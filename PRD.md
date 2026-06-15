# PRD: Qode LSP Milestone

This PRD describes the first implementation milestone for Qode: parser, validation, and Language Server Protocol support for concept files. It intentionally excludes LLM semantic execution, code generation, conformance checking, and runtime stubs. Those are future milestones described in [README.md](./README.md).

Long-lived technical design belongs in [ARCHITECTURE.md](./ARCHITECTURE.md). This PRD is a working execution plan and may be deleted after the milestone is complete.

## Objective

Deliver a self-contained `qode` binary that can parse concept specifications, validate their TypeScript-compatible regions, and provide editor support through LSP.

The first LSP milestone is successful when a developer can open a `.qode` file and get useful diagnostics, symbol navigation, hover information, and cross-file navigation between concept declarations and TypeScript implementation files.

## Non-Goals

- No LLM execution runtime.
- No `LLM.pretend()` integration.
- No generated production implementation code.
- No full formal verification.
- No new TypeScript syntax inside `.ts` files.
- No requirement that concepts map to classes. Concepts may describe modules, functions, structs, services, protocols, or other architectural surfaces.

## Product Requirements

- Parse `.qode` concept files with stable source ranges.
- Preserve prose sections as prose, not code.
- Parse and validate TypeScript-compatible fragments inside `definition`, `interface`, and state payload types.
- Represent public concept shape without committing to implementation form.
- Project concept surfaces into virtual TypeScript declarations for editor intelligence.
- Provide an LSP server through `qode lsp`.
- Provide a CLI validation command through `qode check`.
- Support navigation from `.qode` symbols to real `.ts` symbols when mappings are configured.

## Agent Workflow

Agent execution rules, startup validation, implementation tracking, and commit workflow live in [AGENTS.md](./AGENTS.md).

## Goal1: CLI Foundation

Create the command structure for the self-contained `qode` binary.

### Scope

- Add `qode check`.
- Add `qode lsp`.
- Add `qode --help`.
- Keep the current binary entrypoint in `src/main.ts`.
- Introduce a command dispatcher without implementing parser behavior yet.

### Testable Iteration

Create CLI tests that execute the command handler in-process:

- `qode --help` returns usage text.
- `qode check` with no files reports a clear error.
- `qode lsp --help` documents that it starts the language server.

### Exit Criteria

- CLI command parsing is covered by tests.
- Invalid commands return non-zero status through the command handler.
- `deno task build` produces `build/qode`.

## Goal2: Concept Lexer and Parser

Parse the outer Qode syntax into a source-ranged AST.

### Scope

- Parse `concept Name<T> { ... }`.
- Parse section headers: `goal:`, `definition {}`, `interface {}`, `state {}`, `behavior:`.
- Preserve source ranges for concepts, sections, names, type parameters, and section bodies.
- Preserve prose sections as raw text plus paragraph ranges.
- Recover from malformed input enough to return multiple diagnostics.

### Testable Iteration

Add parser fixture tests:

- Parses the Promise example into one `ConceptDeclaration`.
- Captures all five standard sections.
- Captures generic parameters without interpreting them as implementation details.
- Reports diagnostics for unclosed concept braces.
- Reports diagnostics for duplicate standard sections.

### Exit Criteria

- Parser tests assert AST shape and source ranges.
- Invalid syntax tests assert diagnostic code, message, and range.
- No TypeScript parser is required for this goal.

## Goal3: State Section Model

Parse architectural state declarations as first-class concept data.

### Scope

- Parse state variants such as `Pending;`.
- Parse payload variants such as `Resolved(value: T);`.
- Support multiple typed fields when needed.
- Preserve field type text and source ranges.
- Do not prescribe storage representation.

### Testable Iteration

Add state parser tests:

- Parses empty payload variants.
- Parses single and multiple typed fields.
- Reports missing semicolon.
- Reports invalid variant names.
- Preserves type text exactly for later TypeScript parsing.

### Exit Criteria

- State AST is independent from TypeScript AST.
- Payload type fragments are preserved with exact spans.
- Parser diagnostics remain stable for malformed state declarations.

## Goal4: TypeScript Fragment Parser

Validate TypeScript-compatible fragments without forcing a concept to be a class.

### Scope

- Use `npm:typescript` for TypeScript AST parsing.
- Parse type aliases in `definition`.
- Parse function signatures in `definition`.
- Parse public surface signatures in `interface`.
- Parse state payload type fragments.
- Return mapped diagnostics against original `.qode` ranges.

### Testable Iteration

Add TypeScript fragment tests:

- Validates `type Reason = any`.
- Validates generic handler aliases.
- Validates function signatures with generic and union types.
- Validates state payload type `T`.
- Reports TypeScript syntax errors against `.qode` ranges.

### Exit Criteria

- TypeScript parsing is hidden behind a Qode-owned adapter.
- No generated TypeScript projection assumes class implementation.
- Every TypeScript diagnostic returned to the user maps back to the original concept file.

## Goal5: Concept Surface Projection

Generate implementation-agnostic virtual TypeScript declarations for language intelligence.

### Scope

- Convert a concept AST into a `ConceptSurface`.
- Project the surface into virtual `.d.ts` text.
- Represent constructable values, callable values, static members, functions, modules, and records as surface capabilities, not implementation categories.
- Preserve source maps from virtual TypeScript declarations back to `.qode`.

### Testable Iteration

Add projection tests:

- Promise interface members project to usable TypeScript declarations.
- Constructor-like syntax projects without requiring the implementation to be a class.
- Static-like members project as value-side capabilities.
- Virtual declaration positions map back to `.qode` positions.

### Exit Criteria

- TypeScript can parse the generated virtual declarations.
- Projection tests prove no hardcoded `class` normalizer is required.
- Source maps cover every projected identifier.

## Goal6: TypeScript Language Service Host

Create the internal TypeScript service that sees both real `.ts` files and virtual `.qode.d.ts` files.

### Scope

- Build a `LanguageServiceHost` backed by workspace files and virtual files.
- Include generated concept projections in the service.
- Keep the Qode AST as the source of truth.
- Add cache invalidation when a `.qode` file changes.

### Testable Iteration

Add language service tests:

- A real `.ts` file can reference a concept-projected type.
- Hover text resolves for projected members.
- Definition from a projected member maps back to `.qode`.
- Updating a concept invalidates and rebuilds the virtual file.

### Exit Criteria

- Tests demonstrate TypeScript semantic APIs working over virtual Qode declarations.
- No editor process is required for these tests.
- Cached virtual files update deterministically.

## Goal7: LSP Server Core

Expose parser, diagnostics, and document synchronization through LSP.

### Scope

- Implement `qode lsp` over stdio.
- Support initialize/shutdown.
- Track opened `.qode` documents.
- Publish parser and TypeScript fragment diagnostics.
- Return document symbols for concepts, sections, states, and interface members.

### Testable Iteration

Add LSP protocol tests using an in-process connection harness:

- `initialize` advertises Qode capabilities.
- `textDocument/didOpen` triggers diagnostics.
- Valid concept files return no diagnostics.
- Invalid concept files return parser diagnostics.
- `textDocument/documentSymbol` returns concept structure.

### Exit Criteria

- LSP tests cover initialization, diagnostics, and document symbols.
- The server exits cleanly on shutdown.
- `qode lsp --help` remains available from Goal1.

## Goal8: Navigation Across Qode and TypeScript

Implement editor navigation between concept files and TypeScript implementation files.

### Scope

- Support `textDocument/definition`.
- Support `textDocument/typeDefinition`.
- Support `textDocument/implementation`.
- Support `textDocument/references` for Qode symbols and configured TypeScript links.
- Introduce explicit implementation mapping config.

### Testable Iteration

Add navigation tests:

- From a state payload type in `.qode`, go to the real TypeScript type.
- From an interface member in `.qode`, go to the configured `.ts` implementation.
- From a `.ts` implementation symbol, find the related concept declaration when the mapping is configured.
- References merge Qode references and TypeScript references without duplicates.

### Exit Criteria

- Navigation tests assert exact target URI and range.
- Missing mappings produce no crash and a clear diagnostic or null result.
- Cross-file navigation works without assuming concepts are classes.

## Goal9: Hover, Completion, and Semantic Tokens

Improve editor usefulness after core navigation works.

### Scope

- Hover for concept names, sections, states, and interface members.
- Completion for section names inside concept bodies.
- Completion for known symbols in TypeScript-compatible regions.
- Semantic tokens for concept syntax and section names.

### Testable Iteration

Add editor feature tests:

- Hover on `concept Promise` returns concept summary.
- Hover on a TypeScript type delegates to TypeScript service.
- Completion suggests missing section names.
- Semantic token snapshots are stable for the Promise fixture.

### Exit Criteria

- Tests cover hover, completion, and semantic tokens.
- Feature output remains useful when prose sections contain arbitrary English.
- No feature depends on LLM interpretation.

## Goal10: Workspace CLI Validation

Make `qode check` useful outside the editor.

### Scope

- Discover `.qode` files from paths or workspace config.
- Parse and validate all discovered files.
- Include TypeScript fragment diagnostics.
- Print human-readable diagnostics.
- Return non-zero exit code on errors.

### Testable Iteration

Add CLI validation tests:

- `qode check examples/promise.qode` succeeds.
- `qode check examples/bad.qode` fails with stable output.
- Multiple files aggregate diagnostics.
- Missing files produce a clear error.

### Exit Criteria

- `qode check` is usable in CI.
- Diagnostic output includes file, line, column, code, and message.
- The compiled binary can run `check` against fixtures.

## Goal11: Documentation and Packaging

Make the LSP milestone installable and understandable.

### Scope

- Keep [README.md](./README.md) product-facing.
- Keep [ARCHITECTURE.md](./ARCHITECTURE.md) technical.
- Document editor integration basics.
- Document supported syntax and known limitations.
- Ensure `deno task build` produces a self-contained binary.

### Testable Iteration

Add documentation checks where practical:

- README links resolve locally.
- CLI help references the same commands documented in README.
- Build task succeeds after docs and packaging changes.

### Exit Criteria

- A developer can build `qode`, run `qode check`, and start `qode lsp` from docs alone.
- README positions LLM execution as future scope, not current LSP functionality.
- Architecture doc contains the durable module and service design.

## Final Milestone Acceptance

The LSP milestone is complete when:

- `qode check` validates `.qode` files from the CLI.
- `qode lsp` provides diagnostics, document symbols, hover, completion, semantic tokens, and navigation.
- TypeScript-compatible fragments are validated with TypeScript tooling.
- Cross-navigation between concepts and configured TypeScript implementations works.
- Tests cover parser, projection, TypeScript service integration, LSP protocol behavior, and CLI behavior.
- The compiled binary is self-contained.
