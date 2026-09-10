# Sigil Eqval

Design says what should happen. Code says what does happen. The eqval's job is
not to notice that both mention the same thing. It is to **calculate whether
their represented behavior is equal**, and show the distinguishing situation
and source support when it is not.

This folder is the current refactor design, not an implemented Rust package.
The running foundations are in [sigilc](../sigilc/README.md), with the structural
language frontend in `packages/core/`. The target is a Rust library consumed by
`sigilc`, not another binary. Existing code is migration material, not a
compatibility requirement for the old semantic matcher.

## Read the picture

| Document | Owns |
| --- | --- |
| [Language](../../language.md) | Canonical authored language, including the author's clarifications. |
| [Behavior algebra](behavior_algebra.md) | Meaning, comparison boundaries, composition laws, and proof requirements. |
| [Architecture](ARCHITECTURE.md) | Target inputs, identities, independent computation, freshness, and reporting. |
| [End-to-end example](example.md) | Publication reconstructed from Sigil, Markdown, and differently structured code. |
| [Audit and migration](AUDIT.md) | Evidence from the existing implementation, remaining gaps, exact change surfaces, and acceptance criteria. |

These documents describe one target. The audit does not maintain an alternative
proposal. Older language specifications and current tooling limitations do not
narrow the canonical language. None of the example proofs is reported as an
executed compiler result.

## Start with Sigil, not another organizing language

Component is the core container; contracts contain Facets. Concept IDs group
related Facets across contracts and should be used to expose the several
concepts that real components commonly contain. Preserve that authored grouping
as a first-class connection for the calculation. Smaller components may not
need another identifier, and every contract can freely mix ungrouped and
Concept-grouped Facets without warnings or artificial wrappers.

```text
Component
  Contracts containing Facets
  Optional Concept A grouping related Facets across contracts
  Optional Concept B distinguishing another set of contributions
```

A Concept's identity is resolved, not guessed from spelling. Imported Concepts
keep their originating identity, while consumer contributions keep consumer
ownership. Matching expands contribute collectively, without overriding prior
Facets. Different Concepts connect through explicit operations, values,
resources, and other relationships; sharing a component does not equate them.

An ordinary Facet ends at one empty line. An Embedded Facet contains introducing
prose and fenced content in its chosen notation. Its contract determines its
role: code in Cases is not automatically production code, and a diagram in
Logic is not automatically executable.

Interface exports both Concepts and identifiers defined within Facets, including
operations and domain terms. Imported vocabulary retains its provider's
identity. `_module.sigil` assembles explicit public component surfaces and
preserves their owners; it does not sweep a directory or invent runtime calls.

## Seven views, one set of smaller units

The algebra uses **values, expressions, conditions, state descriptions,
observable actions, outcomes, steps, and relationships**. They are calculation
units, not new Sigil keywords or a universal source-language AST.

| Contract | What its Facets contribute |
| --- | --- |
| Goal | Intended observable outcomes or properties; qualitative purpose remains context until made precise. |
| Interface | Public identities, input/result meanings, actions, failures, and interaction promises. |
| State | Relevant quantities, configurations, and starting conditions. |
| Logic | Calculations, guarded steps, ordering, and delegation. |
| Constraint | Restrictions on states, steps, traces, or relationships. |
| Decision | Applicable choices, assumptions, rationale, and links to binding restrictions; not requirements to implement discarded alternatives. |
| Case | Happy, sad, boundary, and recovery scenarios: given conditions, actions, expected observations. |

One Facet can contribute several units. Several Facets, possibly under different
Concepts, can jointly describe one operation. Decomposition retains the native
Facet's provenance. Cases need not have tests; comparing a test's expectations
with a Case is distinct from proving production behavior or executing a test.

See [contract contributions](behavior_algebra.md#how-each-contract-contributes-to-the-calculation)
and [joint observations](behavior_algebra.md#compare-related-observations-together).

## What we mean by equality

A comparison fixes its subject, implementation target, inputs, assumptions,
observations, and value meanings before considering a discrepancy.

```text
For every permitted input and starting situation:
  Design's possible complete observations
    = Implementation's possible complete observations
```

For a stateful operation, a complete observation keeps **ordered actions,
outcome, and next relevant state together**. Equal sets of statuses and equal
sets of payloads do not prove equality of their allowed pairs. State mappings
must preserve distinctions that affect later interactions.

A partial prohibition can support equality of its property view without
specifying a whole operation. An implementation choosing a subset of permitted
behavior may conform without being equal. Neither property coverage nor
capability names may be promoted into a whole-program equality claim.

Each declared comparison returns:

| Result | Meaning |
| --- | --- |
| Equal | A proof covers the full stated boundary and domain. |
| Different | A distinguishing input, situation, trace, or other supported refutation exists. |
| Unresolved | Missing meaning, unsupported behavior, ambiguous mapping, incomplete inputs, or exhausted computation prevents a conclusion. |

The useful product claim is scoped: sigilc calculates equality between
independently reconstructed descriptions, under fixed laws, and locates
supported disagreements in code and native design Facets. The proof remains
conditional on faithful source interpretation. Hashes establish input identity;
graph validation establishes admissibility; neither makes interpretation
infallible. See [the definition](behavior_algebra.md#what-equality-means-here).

## Why Egglog and Snapdir stay

Egglog supplies relational inference, term equality, and congruence. Fixed laws
can recognize an early-return sequence and a positive guard as the same pure
calculation, then compose that condition with its actions and outcome. The
model supplies local observations, not `provides(Safety)` or an equality verdict.

Keep source identities outside e-classes. Only typed expressions and supported
behavior descriptions may be equated by sound laws. Correspondence explains
which sources concern a subject; it is not a behavioral proof.

Snapdir supplies discovery and content identity. Sigil binds the exact captured
bytes and all consumed semantic inputs to accepted observations. A path-aware
manifest matters: a directory's deduplicated child digest does not preserve
names or multiplicity. Sequential hashing does not create an atomic filesystem
snapshot. Results name their captured inputs and qualify currentness at
validation.

```text
Fresh accepted observations -> current semantic calculation
Stale observations          -> excluded from current truth
Last-known observations     -> historical impact explanation only
```

Native Facet and code locations are required immediately. Facet-granular cache
reuse is a later optimization, not the mechanism that makes diagnostics precise.

## The computation stays independent

```text
Captured Design -> accepted observations -> Design meaning -> obligations
Captured code   -> accepted observations -> target-local Implementation meaning

Selected independent meanings -> fixed comparison -> result and support
Changed source -> compatible last-known correspondence -> impact only
```

Implementation reconstruction receives source, fixed vocabulary, and authorized
identity/type descriptors. It does not receive Design's expected conditions,
Cases, behavior tables, obligations, or verdicts. Target backends remain
separate: correct Rust cannot discharge missing Deno behavior.

Separate e-graphs do not share meaningful e-class IDs. Compare complete finite
behavior tables, or use a third law-only equality query over selected terms.
That query does not assert equality or import Design requirements as code facts.
See [the architecture](ARCHITECTURE.md#independent-computation).

## First deliver something we can calculate

1. **Finite pure decisions.** Prove SearchPublication's admission condition equal
   across a sentence, early returns, and a positive branch. Removing cancellation
   produces a distinguishing Boolean assignment.
2. **Finite guarded operations.** Compare publication's conditions, payload,
   ordered actions, result alternative, and next state jointly. Detect a wrong
   write even when admission is correct.
3. **Supported composition.** Connect independently reconstructed callees through
   explicit links and compatible inputs/state. Compare Case and test expectations
   without substituting tests for production. Extend to PreparedBinding's
   publication protocol only after representing its ordering and concurrency
   requirements.

The first two calculations are finite and acyclic. Unsupported loops,
recursion, concurrency, effects, or representation mappings remain Unresolved.
A small vocabulary does not guarantee finite term generation. Use complete
finite decision procedures where available and equality saturation to recognize
and explain formulations, not to generate endless permutations.

The [example](example.md) specifies the first observable win. The
[audit's migration sequence](AUDIT.md#implementation-sequence) identifies the
actual code boundaries that must change.

## Keep the compiler out of the work queue

`sigilc` owns source capture, semantic bindings, freshness, restricted ingest,
projection publication, scope, comparison, and impact. The eqval owns the
calculation. External callers own models, workers, prompts, attempts, retries,
tests, and scheduling.

The implemented handoff is `prepare` with copied semantic inputs and an
immutable `binding.json`, followed by `ingest --binding binding.json`. Retain
source validation, generation checks, locking, and atomic cache publication.
The compiler does not retain request ledgers, worker receipts, artifact
evidence, or process state.

Scope stays because it defines semantic membership and implementation targets,
not task order. The generated world store stays disposable. Changes to the
current CLI, authored contracts, and repository-owned skill must accompany the
implementation, as detailed in the [migration map](AUDIT.md#outside-this-folder).
