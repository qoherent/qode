# Sigil Architecture

Sigil is an architecture-level execution language. The authored `.sigil` source is the product source of truth. The structural AST, semantic graph, runtime plans, traces, debugger views, and host-language projections are derived artifacts with explicit source ranges and provenance.

The architecture is built around progressive concretization: unresolved parts of an architecture can remain executable while assumptions are exposed and gradually replaced by deliberate logic, generated implementation code, or real implementation code.

## Layer Overview

```text
.sigil source
→ structural parser with source ranges
→ workspace/import/component index
→ semantic-line model
→ LLM semantic compiler harness
→ normalized semantic graph / AST
→ provenance and assumption ledger
→ closure verification
→ executable architecture runtime
→ trace recorder and replay
→ architecture debugger
→ host-language and real-implementation adapters
→ CLI and LSP
```

Structural layers must work offline. LLM-enhanced layers are additional capabilities, not prerequisites for parsing, indexing, diagnostics, or basic editor behavior.

## Source Model

Sigil files use the `.sigil` extension.

Top-level source contains imports and one or more components:

```sigil
from "./sub/system.sigil" import { Gallery, Reviews }

component ProductPage {
  goal {
    help a shopper decide whether this product is right for them
  }

  interface {
    header {
      +Gallery
    }

    middle {
      +Reviews for the current product
    }
  }
}
```

The conventional section order is:

```text
goal
interface
internal
state
logic
constraints
cases
```

This order is a readability convention only and has no semantic effect.

The mandatory semantic units are `component`, `goal`, and `interface`. `internal`, `state`, `logic`, `constraints`, and `cases` are optional and their omission is meaningful.

## Structural AST

The structural AST is deterministic. It captures what can be parsed reliably without interpreting free-form author intent.

It represents:

- files
- imports
- component declarations
- section declarations
- nested blocks
- semantic lines
- explicit `+` composition references
- source ranges
- structural diagnostics

The structural AST must preserve original text for every semantic line. It must not require semicolons, a universal type grammar, or TypeScript-compatible syntax inside section bodies.

Suggested core records:

```ts
interface SigilFile {
  uri: string
  textVersion: string
  imports: ImportDeclaration[]
  components: ComponentDeclaration[]
  diagnostics: SigilDiagnostic[]
  range: SourceRange
}

interface ComponentDeclaration {
  name: Identifier
  sections: SectionDeclaration[]
  range: SourceRange
}

interface SectionDeclaration {
  name: SectionName
  blocks: NestedBlock[]
  lines: SemanticLine[]
  compositions: CompositionReference[]
  range: SourceRange
}

interface SemanticLine {
  text: string
  lineNumber: number
  indentation: string
  range: SourceRange
}
```

The exact implementation may differ, but every record must preserve source identity well enough for diagnostics, navigation, breakpoints, source maps, traces, and replay.

## Semantic-Line Model

A non-empty authored line inside a section is a semantic line.

Each semantic line is:

- a source unit
- a possible breakpoint location
- a unit of semantic interpretation
- a unit of diffing and review
- a source mapping target

One authored line may normalize into multiple executable semantic nodes. The debugger must support stepping over the authored line and stepping into its normalized operations.

Nested blocks are structural context, not a mandatory grammar for execution. For example, a frontend `interface` may contain `header`, `middle`, and `footer` blocks, while a backend `logic` block may contain prose, pseudocode, arrows, or local notation.

## Composition Model

Imports make component declarations available. A line beginning with `+` composes or instantiates a referenced component.

The containing section determines role:

- `interface +Component`: public or user-visible composition
- `internal +Component`: private composition or dependency

The structural model should record:

- referenced component name
- local instance name, when supplied
- raw configuration or input text, when supplied
- containing component
- containing section
- public versus private placement
- source line and range
- resolution status

The workspace index resolves these references. The semantic compiler later decides what composition means operationally in a specific execution graph.

## Workspace Model

The workspace model indexes parsed files and answers cross-file questions for CLI, LSP, semantic compilation, and future debugger workflows.

It indexes:

- component declarations
- imports
- explicit composition references
- public component relationships from `interface`
- private component relationships from `internal`
- section and semantic-line source references
- optional implementation mappings

Responsibilities:

- resolve named imports
- resolve component references
- detect unresolved imports and references
- detect duplicate exported component names where ambiguous
- detect import cycles without blocking unrelated files
- provide declaration, reference, and relationship queries
- invalidate affected files when source changes

The workspace model must be independent from host-language services. Host-language adapters may add optional mappings, but they cannot define Sigil validity.

## Semantic Graph / AST

The semantic graph is the LLM-normalized internal representation used by runtime and debugger layers. Users do not author this graph directly.

It represents:

- calls
- branches
- state transitions
- waits
- dataflow
- outputs
- side effects
- assertions
- component composition
- delegation
- pretend operations

Every semantic node must map back to one or more structural source ranges. When one source line expands into multiple nodes, the mapping must preserve both the authored line and normalized operation ranges.

The graph must allow unresolved execution. A node may be executable through `LLM.pretend` when source meaning is intentionally or temporarily incomplete.

## Compiler Harness

The compiler harness converts structural source plus workspace context into a normalized semantic graph.

Conceptually separate roles:

- compiler: produce the most faithful executable interpretation
- critic: seek a materially different valid interpretation
- failure analyst: find unspecified failure behavior
- concurrency analyst: find timing, ordering, and race assumptions when relevant
- verifier: determine whether differences are externally material

The harness records:

- source version
- structural AST version
- compiler harness version
- model and model version
- prompt version
- input context
- assumptions
- alternative interpretations
- unresolved ambiguity
- normalized graph version

Semantic compilation is not conventional parsing. The deterministic parser intentionally stops at structure; the LLM semantic pass interprets free-form semantic lines.

## Provenance and Assumption Ledger

The system must never silently hide material invention.

Every normalized semantic node should retain:

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

Provenance categories:

- specified
- derived
- inferred
- pretend
- ambiguous
- conflicting

The ledger is queryable by component, path, trace, semantic line, and execution node. It powers editor warnings, debugger panels, closure reports, and review workflows.

## Closure Verification

Semantic closure measures how much invention remains between authored source and executable model.

Verification compares more than final answers:

- normalized execution graphs
- state transitions
- data dependencies
- side-effect ordering
- failure paths
- representative traces

Suggested statuses:

```text
green:
  no material invention or materially different interpretation detected
  under the configured verification process

yellow:
  executable, but assumptions, ambiguity, or pretend behavior remain

red:
  conflicting meanings, violated constraints, or a material decision
  that cannot be resolved from the source
```

Green is evidence of semantic convergence under the configured process. It is not mathematical proof.

Closure should be reportable at multiple levels:

- semantic line
- execution path
- component
- integration

## Runtime

The runtime executes the normalized semantic graph. It supports hybrid execution across:

- Sigil logic
- `LLM.pretend`
- generated implementation code
- real implementation code

Execution should be able to mix these modes in one running system. Implemented parts run through adapters. Unimplemented or intentionally unresolved parts may run through pretend execution with recorded assumptions and outputs.

Runtime responsibilities:

- evaluate executable semantic nodes
- call composed components
- delegate to host adapters
- invoke pretend execution where needed
- enforce constraints when represented in the graph
- update architectural state
- emit outputs and side effects
- record every material decision for trace replay

The runtime does not reinterpret raw source directly. It executes the normalized graph and consults provenance when decisions are invented, inferred, or unresolved.

## Trace Recorder and Replay

Execution traces must be replayable.

Persist enough information to reproduce a trace:

- source version
- normalized graph version
- compiler harness version
- model and model version
- inputs
- initial state
- assumptions
- pretend results
- state transitions
- outputs
- side effects
- adapter calls
- timestamps or logical ordering

Historical replay should reuse recorded pretend outcomes unless the user explicitly requests resimulation.

Trace storage must support:

- deterministic replay
- inspection by semantic line
- comparison between two runs
- comparison between two graph versions
- debugger time travel
- closure reports over observed paths

## Architecture Debugger

The debugger is source-first. Breakpoints are set on authored semantic lines, not only on normalized internal nodes.

When paused, the user should be able to inspect:

- authored source line
- normalized semantic node or nodes
- component call stack
- current state
- inputs and outputs
- dataflow
- dependencies
- side effects
- active assumptions
- pretend decisions
- expected next state

Stepping modes:

- step over authored semantic line
- step into normalized operations under that line
- step into composed component
- step over adapter call
- replay previous decision
- resimulate pretend decision when requested

Frontend debugging should synchronize:

- current logic line
- current state
- event and dataflow
- rendered interface
- focus state
- accessibility state where available

## Host-Language and Real-Implementation Adapters

Adapters connect Sigil to external implementation systems. They are optional and must not define core Sigil semantics.

Possible adapter responsibilities:

- recognize host-language fragments when explicitly identified
- provide projected types
- navigate to implementation symbols
- call real implementation code during runtime
- call generated implementation code
- map adapter diagnostics back to Sigil ranges
- report side effects and state changes back to the trace recorder

TypeScript may be one adapter for syntax recognition, projected types, implementation navigation, and diagnostics within clearly identified fragments. It is not Sigil's type system and must not be required by the structural parser.

## CLI

The public command is `sigil`.

Expected commands:

- `sigil check`
- `sigil lsp`
- `sigil --help`

`sigil check` should read files or directories, parse and index `.sigil` files, print deterministic diagnostics, and exit non-zero when errors are present.

`sigil lsp` should start the language server over stdio.

The CLI layer should remain thin:

- parse command arguments
- call parser/workspace/LSP services
- format terminal output
- return process status

Command handlers should remain testable in process.

## LSP

The LSP must provide useful structural features before semantic compilation exists.

Structural features:

- document synchronization
- diagnostics
- document symbols
- section navigation
- definitions for components and `+Component` references
- references for component declarations and composition uses
- import navigation
- semantic tokens
- completion for standard section names

LLM-enhanced features may be layered later:

- semantic closure status
- assumption hovers
- trace-linked source annotations
- pretend-decision inspection
- generated test or simulation suggestions
- implementation mapping hints

The LSP layer composes parser, workspace index, semantic artifact store, and adapter APIs. It should not parse host-language fragments directly.

## Diagnostics

Diagnostics should have stable codes.

Suggested prefixes:

- `SIGIL_PARSE_*`
- `SIGIL_SECTION_*`
- `SIGIL_IMPORT_*`
- `SIGIL_COMPOSITION_*`
- `SIGIL_WORKSPACE_*`
- `SIGIL_ADAPTER_*`
- `SIGIL_SEMANTIC_*`

Every diagnostic includes:

- URI or path
- source range
- severity
- code
- message
- optional related information

CLI diagnostics should print file, line, column, code, and message. LSP diagnostics should use native LSP ranges.

## Source Maps

Source maps connect authored Sigil source, structural AST records, normalized semantic nodes, host-language projections, runtime traces, and debugger views.

Mappings must support:

- original URI
- original range
- structural node identity
- semantic node identity when available
- generated/projection URI when available
- generated/projection range when available
- symbol identity when available

Source maps are required for diagnostics, hover, definition, references, runtime traces, breakpoints, replay, and debugger stepping.

## Suggested Source Layout

```text
src/
  main.ts
  cli/
    mod.ts
    commands.ts
    check.ts
    lsp.ts
  parser/
    ast.ts
    lexer.ts
    parser.ts
    ranges.ts
    diagnostics.ts
  workspace/
    file_graph.ts
    component_index.ts
    import_index.ts
    composition_index.ts
    implementation_mappings.ts
  semantic/
    compiler_input.ts
    semantic_graph.ts
    provenance.ts
    closure.ts
    artifacts.ts
  runtime/
    executor.ts
    pretend.ts
    adapters.ts
    state.ts
  trace/
    recorder.ts
    replay.ts
  debugger/
    breakpoints.ts
    stepping.ts
    inspection.ts
  adapters/
    typescript/
      recognition.ts
      projection.ts
      diagnostics.ts
      navigation.ts
  lsp/
    server.ts
    documents.ts
    capabilities.ts
    diagnostics.ts
    document_symbols.ts
    definition.ts
    references.ts
    semantic_tokens.ts
    completion.ts
```

This layout is guidance, not a mandate. Ownership boundaries matter more than exact filenames.

## Testing Strategy

Tests should be grouped by layer:

- CLI command tests
- parser fixture tests
- source range tests
- structural diagnostic recovery tests
- import resolution tests
- composition index tests
- workspace invalidation tests
- LSP protocol tests
- semantic interface serialization tests
- provenance and closure record tests
- adapter boundary tests

Prefer in-process tests for command handlers and LSP request handlers. Add subprocess tests only for binary-level behavior.

Every feature should have a valid fixture and at least one malformed fixture.

## Build Model

The main app is a self-contained Deno-compiled binary.

Expected build command:

```sh
deno task build
```

Expected output:

```text
build/sigil
```
