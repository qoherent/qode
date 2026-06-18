# Sigil

Sigil is an architecture-level execution language designed for iteration.

It lets architects describe and execute a system before every part has a real implementation. Unresolved parts remain executable while the architect progressively removes invention and replaces pretend execution with deliberate logic, generated implementation code, or real implementation code.

The loop:

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

## Language Shape

Sigil source files use `.sigil`.

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

  internal {
    +ProductRepository
  }

  state {
    loading
    ready with product, gallery, reviews
    failed with recoverable reason
  }

  logic {
    load product details
    load gallery and reviews in parallel
    show ready view when required data is present
  }

  constraints {
    never expose private customer data
    product title remains visible before reviews
  }

  cases {
    product loads with reviews and gallery
    reviews fail while product details remain usable
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

This order is only a readability convention. It has no semantic effect.

The mandatory semantic units are `component`, `goal`, and `interface`.

Optional sections:

- `internal`
- `state`
- `logic`
- `constraints`
- `cases`

## Section Meaning

`goal` explains why the component exists and what outcome it enables.

`interface` is the component boundary: everything the environment can see, call, supply, receive, render, interact with, subscribe to, or publicly depend on.

`internal` names private things that exist inside the component: private components, dependencies, resources, services, capabilities, functions, types, domain vocabulary, and static relationships.

```text
internal = what privately exists
state    = what changes over time
logic    = what executes
```

`state` describes architecturally meaningful configurations that persist or change during execution. It is not storage layout.

`logic` describes executable architecture-level decisions, ordering, dataflow, delegation, and state transitions.

`constraints` are universal truths over all valid executions.

`cases` are representative externally observable situations and acceptance criteria. They are examples, not the complete behavior of the component.

## Semantic Lines

Inside each section, authors may use whatever notation best expresses the idea: concise English, pseudocode, math, arrows, host-language-like syntax, domain notation, ASCII sketches, or combinations.

A new line separates semantically distinct things. Each non-empty semantic line is a source unit, breakpoint location, interpretation unit, diff unit, and source mapping target.

Sigil does not require semicolons or a universal type grammar inside sections.

## Composition

Imports make component declarations available. A line beginning with `+` composes a referenced component.

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

The containing section determines architectural role:

- `interface +Component`: public or user-visible composition
- `internal +Component`: private composition or dependency

## Frontend and Backend

Sigil is not backend-only.

Frontend `interface` blocks may describe visual composition, spatial relationships, responsive layouts, state-dependent views, overlays, focus behavior, accessibility, motion, design tokens, ASCII layout sketches, or direct CSS-like details when useful.

Backend `interface` blocks may describe functions, endpoints, commands, queries, messages, events, streams, outputs, and externally visible effects.

These are authoring idioms, not mandatory subgrammars.

## Type Policy

Sigil does not own a universal type system.

Authors may write TypeScript-style declarations, Rust-style declarations, Python typing, Java declarations, mathematical notation, domain notation, or ordinary English. Host-language tooling may be added later as an adapter, but no host language defines core Sigil semantics.

## Execution Model

The authored source is not the final runtime AST.

```text
Sigil source
→ structural parsing
→ semantic normalization by an LLM compiler harness
→ normalized semantic graph / AST
→ executable architecture
```

A running system may mix:

- real implementation code
- generated implementation code
- executable Sigil logic
- `LLM.pretend`

The compiler must record assumptions and provenance whenever it introduces meaning not directly present in the source. Semantic closure measures how much invention remains. Green closure is evidence of convergence under the configured process, not mathematical proof.

## Current Milestone

The repository is at the command-scaffold stage. The next milestone is structural tooling, not the full runtime.

Milestone breakdown:

1. Rename the public command and binary to `sigil`.
2. Parse `.sigil` files into a source-ranged structural AST.
3. Index imports, component declarations, and `+Component` composition.
4. Provide offline structural LSP features.
5. Make `sigil check` useful for deterministic structural validation.
6. Define interfaces for future semantic normalization, provenance, closure, traces, and replay.
7. Keep host-language tooling optional and adapter-based.

See [PRD.md](./PRD.md) for the execution plan and [ARCHITECTURE.md](./ARCHITECTURE.md) for the durable technical design.

## Target CLI

```sh
sigil check path/to/system.sigil
sigil lsp
```

The first implementation task in the current milestone makes the built binary and help text match this public surface.

## Development

```sh
deno task test
deno task check
deno task build
```

After the command rename lands, the build output should be:

```text
build/sigil
```

## Not This

Sigil is not:

- a general-purpose programming language
- a formal proof system
- a mandatory pseudocode grammar
- a prompt file format
- a universal type system
- a replacement for host implementation languages
