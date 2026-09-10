use crate::{
    assertions::quote,
    eqval::{self, DesignState, DesignWorld, Limits, SaturatedWorld},
    sources::hash,
};
use egglog::EGraph;
use serde::Serialize;
use serde_json::Value;
use std::{collections::BTreeMap, time::Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum ImplementationState {
    Drift,
    Converged,
    Closed,
}

/// Assembly computes these flags after excluding stale/corrupt/unselected facts.
/// They are not semantic assertions and are never supplied by an interpreter.
#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FreshInputs {
    pub all_design_fresh: bool,
    pub all_implementation_fresh: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Comparison {
    pub version: u32,
    pub kernel_fingerprint: String,
    pub design: DesignState,
    pub implementation: Option<ImplementationState>,
    pub fresh_inputs: FreshInputs,
    pub obligations: BTreeMap<String, Vec<Value>>,
    pub satisfied: Vec<String>,
    pub unresolved: Vec<String>,
    pub disagreements: Vec<Vec<Value>>,
    pub implementation_contradictions: Vec<Vec<Value>>,
}

// @sigil implements packages/sigilc/eqval.sigil::SigilWorldClosure::IndependentComparison interface
pub fn compare(
    design: &DesignWorld,
    implementation: &SaturatedWorld,
    fresh: FreshInputs,
    limits: Limits,
) -> Result<Comparison, String> {
    if implementation.is_design || !design.closure.is_design {
        return Err(
            "comparison requires independently saturated Design and Implementation worlds".into(),
        );
    }
    let fingerprint = eqval::fingerprint();
    if design.closure.kernel_fingerprint != fingerprint
        || implementation.kernel_fingerprint != fingerprint
    {
        return Err("world closure belongs to a different compiler runtime".into());
    }
    let mut result = Comparison {
        version: 1,
        kernel_fingerprint: fingerprint,
        design: design.state,
        implementation: None,
        fresh_inputs: fresh,
        obligations: BTreeMap::new(),
        satisfied: Vec::new(),
        unresolved: Vec::new(),
        disagreements: Vec::new(),
        implementation_contradictions: Vec::new(),
    };
    if design.state == DesignState::Disjoint {
        return Ok(result);
    }
    if !fresh.all_design_fresh {
        result.design = DesignState::Loose;
    }
    let started = Instant::now();
    let mut program = String::from(include_str!("comparison.egg"));
    for row in &design.closure.tables["coverage"] {
        let id =
            hash(&serde_json::to_vec(&("sigil-obligation-v1", row)).map_err(|e| e.to_string())?);
        if row.len() != 6 {
            return Err("invalid coverage row".into());
        }
        // Drop provenance origin from the eqval input, retaining it in the report.
        let args = row[1..].iter().map(string).collect::<Result<Vec<_>, _>>()?;
        program.push_str(&format!("\n(obligation {} {})", quote(&id), args.join(" ")));
        result.obligations.insert(id, row.clone());
    }
    for row in &design.closure.tables["numeric-obligation"] {
        let id = hash(
            &serde_json::to_vec(&("sigil-numeric-obligation-v1", row))
                .map_err(|e| e.to_string())?,
        );
        if row.len() != 4 {
            return Err("invalid numeric obligation row".into());
        }
        program.push_str(&format!(
            "\n(budget {} {} {} {:?})",
            quote(&id),
            string(&row[0])?,
            string(&row[1])?,
            number(&row[2])?
        ));
        result.obligations.insert(id, row.clone());
    }
    // Deliberately no iteration over Design known, number, text, or edge facts.
    for row in &implementation.tables["known"] {
        if row.len() != 3 {
            return Err("invalid Implementation known row".into());
        }
        program.push_str(&format!(
            "\n(actual {})",
            row.iter()
                .map(string)
                .collect::<Result<Vec<_>, _>>()?
                .join(" ")
        ));
    }
    for row in &implementation.tables["number"] {
        if row.len() != 4 {
            return Err("invalid Implementation number row".into());
        }
        program.push_str(&format!(
            "\n(actual-number {} {} {:?} {})",
            string(&row[0])?,
            string(&row[1])?,
            number(&row[2])?,
            string(&row[3])?
        ));
    }
    let mut graph = EGraph::default();
    graph
        .parse_and_run_program(Some("sigil-comparison".into()), &program)
        .map_err(|e| e.to_string())?;
    eqval::fixedpoint(&mut graph, limits, started)?;
    result.satisfied = ids(eqval::rows(&graph, "satisfied", 1, limits)?)?;
    result.unresolved = ids(eqval::rows(&graph, "unresolved", 1, limits)?)?;
    result.disagreements = eqval::rows(&graph, "disagreement", 3, limits)?;
    result.implementation_contradictions = implementation.tables["violation"].clone();
    result.implementation = Some(
        if !result.disagreements.is_empty() || !result.implementation_contradictions.is_empty() {
            ImplementationState::Drift
        } else if result.design == DesignState::Coherent
            && fresh.all_implementation_fresh
            && result.unresolved.is_empty()
        {
            ImplementationState::Closed
        } else {
            ImplementationState::Converged
        },
    );
    eqval::check_limits(&graph, limits, started)?;
    Ok(result)
}

fn string(value: &Value) -> Result<String, String> {
    value
        .as_str()
        .map(quote)
        .ok_or_else(|| "invalid native string cell".into())
}
fn number(value: &Value) -> Result<f64, String> {
    value
        .as_f64()
        .filter(|n| n.is_finite())
        .ok_or_else(|| "invalid native number cell".into())
}
fn ids(rows: Vec<Vec<Value>>) -> Result<Vec<String>, String> {
    rows.into_iter()
        .map(|r| {
            r[0].as_str()
                .map(str::to_owned)
                .ok_or_else(|| "invalid native ID".into())
        })
        .collect()
}
