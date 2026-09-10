<!-- @sigil implements integrations/skills/sigil/compilation-execution.sigil::SigilCompilationExecution interface,logic,constraints,cases -->

# Native compilation execution

Run from the selected workspace, or pass its absolute path with `--root` to
native commands. Keep exported bundles, scopes, preparations, Turtle results,
and reports in a caller-owned directory such as `.sigil/tmp/<run-id>/`.
Preparation output must use a new directory for every input binding.

## Capture and scope

```sh
mkdir -p .sigil/tmp/<run-id>
sigil export design . > .sigil/tmp/<run-id>/frontend.json
sigilc scope --frontend .sigil/tmp/<run-id>/frontend.json --scope .sigil/tmp/<run-id>/scope.json
sigilc stale design --frontend .sigil/tmp/<run-id>/frontend.json --scope .sigil/tmp/<run-id>/scope.json
```

Create the run directory before capture. Require successful export and
regenerate the bundle after changing authored files, imports, configuration, or
glossary. Export captures the workspace; native scope selects semantic focus.
For example:

```json
{
  "version": 1,
  "design": { "paths": ["architecture/a.sigil", "architecture/b.sigil"] },
  "implementation": { "dirs": ["src"], "vendorDirs": ["vendor"] }
}
```

Design paths are ordered roots. Read `scope.design.focus_order`; native scope
preserves explicit priorities and appends import and owner dependencies with
reasons. Order and membership have separate fingerprints. Reordering alone does
not invalidate unchanged semantic bindings. Scope is read-only semantic
membership; it does not schedule or record external work.

Implementation selection supports `paths`, `dirs`, `include`, `exclude`,
`vendorDirs`, and `allowEmpty`. Paths are workspace-relative and glob filters
use `*`, `**`, and `?`. Use the same `--scope` on every operation below. Without
a paired scope, Implementation inspection and comparison require
`--selection FILE`.

## Prepare and ingest

At the start of a fresh semantic round, run `sigilc stale` for the selected
scope. Preserve fresh projections. Prepare only stale, missing, or
dependency-invalid rows into new directories.

```sh
sigilc prepare design --frontend .sigil/tmp/<run-id>/frontend.json --scope .sigil/tmp/<run-id>/scope.json --source architecture/a.sigil --out .sigil/tmp/<run-id>/design-a
sigilc ingest design --frontend .sigil/tmp/<run-id>/frontend.json --scope .sigil/tmp/<run-id>/scope.json --source architecture/a.sigil --binding .sigil/tmp/<run-id>/design-a/binding.json --turtle .sigil/tmp/<run-id>/design-a/result.ttl
sigilc prepare design --frontend .sigil/tmp/<run-id>/frontend.json --scope .sigil/tmp/<run-id>/scope.json --source architecture/b.sigil --out .sigil/tmp/<run-id>/design-b
sigilc ingest design --frontend .sigil/tmp/<run-id>/frontend.json --scope .sigil/tmp/<run-id>/scope.json --source architecture/b.sigil --binding .sigil/tmp/<run-id>/design-b/binding.json --turtle .sigil/tmp/<run-id>/design-b/result.ttl
sigilc compile design --frontend .sigil/tmp/<run-id>/frontend.json --scope .sigil/tmp/<run-id>/scope.json
sigilc entities --frontend .sigil/tmp/<run-id>/frontend.json --scope .sigil/tmp/<run-id>/scope.json
```

Design preparation contains exactly the copied Design JSON, ontology, and
`binding.json`. An external interpreter may use those files to construct a
Turtle result. The caller passes the unchanged binding to `ingest`.

A current Design catalog is required before Implementation preparation:

```sh
sigilc prepare implementation --frontend .sigil/tmp/<run-id>/frontend.json --scope .sigil/tmp/<run-id>/scope.json --source src/main.rs --out .sigil/tmp/<run-id>/implementation-main
sigilc ingest implementation --frontend .sigil/tmp/<run-id>/frontend.json --scope .sigil/tmp/<run-id>/scope.json --source src/main.rs --binding .sigil/tmp/<run-id>/implementation-main/binding.json --turtle .sigil/tmp/<run-id>/implementation-main/result.ttl
sigilc stale implementation --frontend .sigil/tmp/<run-id>/frontend.json --scope .sigil/tmp/<run-id>/scope.json
sigilc compile implementation --frontend .sigil/tmp/<run-id>/frontend.json --scope .sigil/tmp/<run-id>/scope.json
sigilc compare --frontend .sigil/tmp/<run-id>/frontend.json --scope .sigil/tmp/<run-id>/scope.json
```

Implementation preparation contains exactly captured `source`, `ontology.json`,
`catalog.json`, and `binding.json`. Do not provide Design prose, neighboring
code, or caller records. A truthful zero-fact Turtle result is valid when the
source has no supported assertions.

Ingest validates the binding, source bytes, restricted data, and expected
projection generation before publication. A rejection includes an actionable
hint. The external caller may repair only its temporary Turtle and retry with
the same binding. If a bound source, frontend, catalog, or scope changes,
discard the preparation and begin a fresh round. Never edit source bytes,
`binding.json`, or published projections to force acceptance.

## Interpret results

`compile design` exits 0 for Coherent or Loose and 1 for Disjoint.
`compile implementation` and `compare` exit 0 for Closed or Converged and 1
for Drift. `stale` exits 1 when freshness work remains. `entities` exits 1
when no usable catalog is available. Usage errors exit 2; runtime failures and
unavailable comparisons exit 3. Inspect the JSON state and diagnostics instead
of inferring a semantic color from a generic exit code.

The external host owns interpreters, isolation, scheduling, retries, source
changes, and any records about those activities. The generated `.sigil/worlds/`
directory is an ignored disposable compiler cache containing accepted semantic
projections, bindings, checksums, generations, and available history. Use
`clean` when discarding that cache is intended:

```sh
sigilc clean --root .
```
