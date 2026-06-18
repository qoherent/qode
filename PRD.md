# PRD: Sigil Structural Tooling Milestone

Sigil is an architecture-level execution language designed for iteration. It lets architects describe and execute a system before every part has a real implementation.

The product loop is:

```text
describe
→ compile semantically
→ run
→ pause and inspect
→ expose assumptions
→ refine
→ implement selected parts
→ repeat
```

The central value is not ordinary code generation. Unresolved parts of an architecture remain executable while the architect progressively removes invention and replaces pretend execution with deliberate logic, generated implementation code, or real implementation code.

This PRD defines the next coherent milestone: rename the public command surface to Sigil, parse `.sigil` files structurally, index components and composition, and provide useful CLI/LSP feedback without depending on an online semantic compiler.

## Current State

The repository currently contains a small Deno/TypeScript command scaffold:

- `src/main.ts` delegates to a CLI module.
- `src/cli/*` contains placeholder `check` and `lsp` command handlers.
- `tests/cli_test.ts` covers command help, missing file arguments, and invalid commands.

That work remains useful as scaffolding. The parser, workspace index, language server, semantic compiler, runtime, debugger, and host-language adapters are not implemented yet.

Pre-Sigil command names, package metadata, build output names, test names, and examples are migration debt. The first implementation goal below moves the public command surface to Sigil before any parser work starts.

## Milestone Scope

This milestone delivers deterministic structural tooling for Sigil:

- public CLI/binary name `sigil`
- source extension `.sigil`
- canonical unit `component`
- imports
- all canonical section names
- free-form semantic lines inside sections
- explicit `+Component` composition references
- stable source ranges
- structural diagnostics and recovery
- workspace component/import/composition index
- structural LSP features that work offline
- interfaces for future semantic normalization

The milestone should not pretend the full execution system exists. It should create clean boundaries so later milestones can add LLM semantic normalization, closure verification, runtime execution, traces, and debugger features without replacing the structural foundation.

## Non-Goals

- No full LLM semantic compiler implementation.
- No `LLM.pretend` runtime.
- No generated production implementation code.
- No architecture debugger UI.
- No formal verification claim.
- No mandatory TypeScript-compatible syntax inside Sigil sections.
- No universal Sigil-owned type system.
- No requirement that components map to classes, functions, services, endpoints, UI components, or any one implementation shape.

## Canonical Source Shape

Imports are explicit:

```sigil
from "./sub/system.sigil" import { Gallery, Reviews }
```

Components use this conventional section order:

```sigil
component Name {
  goal {
  }

  interface {
  }

  internal {
  }

  state {
  }

  logic {
  }

  constraints {
  }

  cases {
  }
}
```

The conventional order is:

```text
goal
interface
internal
state
logic
constraints
cases
```

The order above is a readability convention only. It has no semantic effect.

The mandatory semantic units are:

- `component`
- `goal`
- `interface`

Optional sections:

- `internal`
- `state`
- `logic`
- `constraints`
- `cases`

Omission has meaning:

- no `internal`: no private structure has been committed
- no `state`: stateless or state intentionally unspecified
- no `logic`: execution may continue through `LLM.pretend` in a later runtime
- no `constraints`: no universal invariants declared
- no `cases`: no representative acceptance cases captured

## Section Semantics

`goal` explains why the component exists and what outcome it enables. It guides pretend execution where lower-level meaning remains unspecified.

`interface` is the component boundary. It contains anything the surrounding environment can see, call, supply, receive, render, interact with, subscribe to, or publicly depend upon. For frontend components, it includes visible composition and user interaction. For backend components, it includes functions, endpoints, commands, queries, messages, events, streams, outputs, and externally visible effects.

`internal` names private things inside the component that may be referenced elsewhere: private components, dependencies, resources, services, capabilities, functions, types, domain vocabulary, and static relationships.

Use the strict distinction:

```text
internal = what privately exists
state    = what changes over time
logic    = what executes
```

`state` describes architecturally meaningful configurations that persist or change during execution. It is not storage layout. It should support state-machine visualization, transition inspection, debugger state views, and lifecycle reasoning.

`logic` is executable architecture-level decision-making: ordering, dataflow, delegation, and state transitions. It does not require a mandatory pseudocode grammar. Authors may use concise English, pseudocode, mathematical notation, arrows, host-language-like syntax, locally invented notation, or combinations.

`constraints` are universal truths over all valid executions: invariants, forbidden outcomes, limits, at-most-once rules, eventual requirements, privacy rules, ordering guarantees, and accessibility requirements.

`cases` are representative externally observable situations and acceptance criteria. They should be suitable for generating tests, simulations, interaction scripts, trace assertions, frontend screenshots, or accessibility checks. Cases are examples, not the full behavior of the system.

## Semantic Lines

Inside every section, authors may use whatever notation best expresses the idea. A new line separates semantically distinct things.

Each non-empty semantic line is:

- a source unit
- a possible breakpoint location
- a unit of semantic interpretation
- a unit of diffing and review
- a source mapping target

One authored line may normalize into multiple executable semantic nodes later. The debugger must eventually support both stepping over the authored line and stepping into its normalized operations.

The structural parser must not require semicolons or a universal type grammar inside sections.

## Composition

A line beginning with `+` explicitly composes a referenced component:

```sigil
interface {
  +Gallery

  +Reviews for the current product
}
```

```sigil
internal {
  +ProductRepository

  +Analytics
}
```

The containing section determines the architectural role:

- `interface +Component`: public or user-visible composition
- `internal +Component`: private composition or dependency

Imports make component declarations available. `+` composes or instantiates them.

The semantic model must distinguish:

- imported component declaration
- composed component instance
- instance name, when supplied
- inputs or configuration, when supplied
- public versus private placement

## Frontend Support

Sigil is not backend-only. Frontend `interface` blocks may describe visuals through free-form nested regions and relationships:

```sigil
interface {
  header {
    +Search
  }

  middle {
    +Products
  }

  footer {
    +Pagination
  }
}
```

Authors may describe spatial relationships, responsive layouts, visual hierarchy, attention flow, state-dependent views, progressive disclosure, overlays, focus behavior, accessibility, motion, design tokens, ASCII layout sketches, and direct CSS-like details when intentionally chosen.

These are authoring idioms, not mandatory visual grammar. Later frontend debugging should synchronize the current logic line, current state, event and dataflow, and rendered interface.

## Type and Notation Policy

Sigil does not own a universal type system.

Authors may write TypeScript-style declarations, Rust-style declarations, Python typing, Java declarations, mathematical notation, domain notation, or ordinary English. Known host-language syntax may later be delegated to host-language tooling, but it must not define Sigil's core semantics.

The structural parser must not require a TypeScript compiler to understand a valid Sigil component. TypeScript support may remain useful only as an optional host-language adapter for syntax recognition, projected types, implementation navigation, or diagnostics inside clearly identified host-language fragments.

## LSP Requirements

The LSP must be useful before semantic compilation is available and must not depend on an online LLM for structural features.

Required structural features:

- document symbols for components, imports, sections, nested blocks, semantic lines, and composition references
- section navigation
- structural diagnostics
- import resolution diagnostics
- component declaration lookup
- component reference lookup
- `+Component` reference lookup
- references across imports and compositions
- semantic tokens for imports, components, section names, nested block labels, and composition references
- completion for missing standard section names

LLM-enhanced features can be layered later, but structural editor support must stay deterministic.

## Semantic Compiler Boundary

The human-authored source is not the final runtime AST.

Future pipeline:

```text
Sigil source
→ structural parsing
→ semantic normalization by an LLM compiler harness
→ normalized semantic graph / AST
→ executable architecture
```

This milestone should define interfaces for the future semantic compiler without implementing the full harness.

The deterministic parser owns only what must be structurally reliable:

- imports
- component declarations
- named section blocks
- nested braces
- semantic-line source ranges
- explicit `+` composition references
- syntax recovery and structural diagnostics

The future LLM semantic pass interprets semantic-line contents. Free-form text must not be represented as if it can be fully normalized by a conventional parser.

## Provenance and Closure Requirements

Future normalized semantic nodes must record whenever the compiler introduces meaning not directly present in the source.

Each normalized node should retain:

- source component
- source section
- source line and range
- normalized meaning
- relevant inputs and outputs
- inferred types or shapes, if any
- dependencies
- state effects
- side effects
- assumptions
- alternative interpretations
- compiler/model provenance

Use provenance categories such as:

- specified
- derived
- inferred
- pretend
- ambiguous
- conflicting

The quality metric is semantic closure: how much semantic invention remains between the authored component and its executable model.

Future compiler harness roles:

- compiler: produce the most faithful executable interpretation
- critic: seek a materially different valid interpretation
- failure analyst: find unspecified failure behavior
- concurrency analyst: find timing, ordering, and race assumptions when relevant
- verifier: determine whether differences are externally material

Verification should compare normalized execution graphs, state transitions, data dependencies, side-effect ordering, failure paths, and representative traces. Green status means evidence of semantic convergence under the configured process, not mathematical proof.

## Goal1: Rename Command Surface to Sigil

Make `sigil` the public command and binary before structural language work starts.

Scope:

- update command help to `sigil`
- update build output to `build/sigil`
- update package metadata where it names the public command
- update command tests to assert `sigil`
- keep the existing command dispatch shape
- preserve `sigil check` and `sigil lsp` as placeholder commands until later goals make them functional

Testable iteration:

- `sigil --help` returns usage text
- `sigil check` with no files reports a clear error
- `sigil lsp --help` documents language server startup
- invalid commands return non-zero status

Exit criteria:

- command tests pass with Sigil names only
- `deno task build` produces `build/sigil`
- no public docs, help text, or tests use pre-Sigil command terminology

## Goal2: Structural Parser

Parse `.sigil` files into a source-ranged structural AST.

Scope:

- parse imports
- parse `component Name { ... }`
- parse required `goal` and `interface` sections
- parse optional `internal`, `state`, `logic`, `constraints`, and `cases` sections
- treat section order as non-semantic
- preserve nested blocks inside sections
- preserve semantic lines with stable source ranges
- recover from malformed braces, unknown sections, duplicate sections, missing mandatory sections, and unterminated imports
- avoid host-language parsing in the core parser

Testable iteration:

- fixture with imports, one component, all canonical sections, nested frontend blocks, and `+Component` lines parses
- source ranges cover file, import, component name, section header, nested block, semantic line, and composition reference
- diagnostics have stable codes and ranges
- malformed input returns multiple diagnostics where practical

Exit criteria:

- parser tests assert AST shape and ranges
- valid free-form section bodies do not require semicolons or TypeScript-compatible syntax
- TypeScript is not imported by parser modules

## Goal3: Workspace Index and Composition

Resolve files, imports, component declarations, and explicit composition references.

Scope:

- discover `.sigil` files from explicit paths and workspace roots
- build an index of component declarations
- resolve named imports
- resolve `+Component` references against local and imported declarations
- record public composition from `interface`
- record private composition from `internal`
- preserve unresolved references as diagnostics without crashing
- track source references for navigation
- define optional implementation mapping records without requiring any host language

Testable iteration:

- imported components resolve across files
- unresolved imports and compositions produce stable diagnostics
- references distinguish imported declarations from composed instances
- `interface +Component` and `internal +Component` produce different relationship records

Exit criteria:

- workspace index supports declaration, reference, and relationship queries
- incremental re-indexing invalidates changed files and dependent imports
- cycles are diagnosed without preventing unrelated files from being indexed

## Goal4: Structural LSP

Expose parser and workspace index behavior through Language Server Protocol.

Scope:

- implement `sigil lsp` over stdio
- support initialize/shutdown
- track opened `.sigil` documents
- publish structural diagnostics
- provide document symbols
- provide definition for component and `+Component` references
- provide references for components and compositions
- provide semantic tokens
- provide section-name completion

Testable iteration:

- in-process LSP tests cover initialize, didOpen, diagnostics, document symbols, definition, references, semantic tokens, and completion
- valid Sigil fixtures return no structural diagnostics
- invalid fixtures return parser and import diagnostics
- structural features work without network or model access

Exit criteria:

- LSP capabilities match implemented behavior
- server shuts down cleanly
- all LSP locations use stable source ranges from parser/index layers

## Goal5: CLI Validation

Make `sigil check` useful in terminals and CI.

Scope:

- accept one or more files or directories
- discover `.sigil` files
- parse and index discovered files
- print human-readable diagnostics
- return non-zero status on errors
- include file, line, column, diagnostic code, and message

Testable iteration:

- valid fixture succeeds
- invalid fixture fails with stable output
- directory input discovers `.sigil` files
- missing files produce clear diagnostics
- multiple files aggregate diagnostics

Exit criteria:

- `sigil check` is usable in CI for structural validation
- diagnostic output is deterministic
- compiled binary can run `check` against fixtures

## Goal6: Semantic Compiler Interfaces

Establish contracts for future semantic normalization without implementing the full LLM runtime.

Scope:

- define structural-to-semantic input records
- define normalized semantic graph interfaces
- define provenance and assumption ledger interfaces
- define semantic closure status records
- define invalidation rules when source changes
- define persisted artifact metadata: source version, compiler harness version, model identity, prompt version, and normalization time

Testable iteration:

- structural AST can be converted into compiler input records with source mappings
- placeholder semantic graph records preserve component, section, line, and range identity
- provenance categories are typed and covered by tests
- serialization round trip preserves ranges and provenance metadata

Exit criteria:

- future compiler work has stable interfaces to target
- no interface claims semantic normalization has already happened
- no runtime execution is introduced in this milestone

## Goal7: Optional Host-Language Adapter Boundary

Keep host-language tooling useful without making it core Sigil semantics.

Scope:

- define adapter interfaces for host-language syntax recognition
- allow adapters to attach diagnostics only to explicitly identified fragments
- allow adapters to project types or implementation links when configured
- keep adapters outside parser, structural AST, and workspace core
- start with TypeScript only if it directly supports implementation navigation or projected types

Testable iteration:

- parser/index tests pass without loading host-language adapters
- adapter diagnostics map back to Sigil ranges
- missing adapter configuration does not reduce structural LSP behavior

Exit criteria:

- TypeScript support, if added, is clearly optional
- Sigil validity never depends on TypeScript-compatible syntax
- adapter boundaries are covered by tests

## Final Milestone Acceptance

The milestone is complete when:

- public command and binary are `sigil`
- `.sigil` files parse structurally with stable source ranges
- `component`, `goal`, and `interface` are enforced as mandatory semantic units
- `internal`, `state`, `logic`, `constraints`, and `cases` are recognized as optional sections
- section order is accepted as a readability convention only
- section bodies are preserved as free-form semantic lines
- imports and `+Component` composition are indexed
- public versus private composition is represented
- structural diagnostics recover from common malformed files
- `sigil check` validates workspaces structurally
- `sigil lsp` provides deterministic offline editor features
- semantic compiler interfaces exist without claiming runtime execution
- TypeScript, if present, is isolated as an optional adapter

Required checks for each implementation goal:

```sh
deno fmt --check
deno check src/main.ts tests/**/*.ts
deno test --allow-net tests/
```

Goals that touch the compiled binary must also run:

```sh
deno task build
```
