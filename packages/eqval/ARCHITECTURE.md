# Sigil Eqval architecture

This is the target architecture for the refactor, not a description of a
completed implementation. The [README](README.md) owns the direction,
[behavior algebra](behavior_algebra.md) owns semantic definitions, and
[AUDIT.md](AUDIT.md) records remaining implementation gaps. The
[example](example.md) follows one calculation through these boundaries.

Canonical language semantics come from [language.md](../../language.md).
Logical records and function names below specify responsibilities; they are
not a claim that a finalized Rust API or Turtle schema already exists.

## Ownership boundaries

| Owner | Responsibilities |
| --- | --- |
| `packages/core` | Deterministic Sigil parsing, resolution, imports, expands, module assembly, and structural Design export. |
| External semanticizer | Interpret captured source into fixed, typed, attributable observations; report gaps without asserting compiler verdicts. |
| `sigilc` | Source selection/capture, incoming descriptors, bindings, data-only ingest, projection publication, freshness, and report presentation. |
| Eqval library | Typed lowering, independent closure, supported behavior composition, comparison, proof support, and correspondence/impact calculation. |
| External caller | Work scheduling, model invocation, retries, coding, test execution, and delivery records. |

The eqval has no filesystem discovery, model client, task ledger, or worker
protocol. `sigilc` does not need implementation-language AST adapters, an LSP,
Git symbol history, or fuzzy name matching to validate its observation format.
Language neutrality describes the representation, not universal semantic
coverage of every construct in every language.

## Native structure and identities

Component is the core container; contracts contain Facets. Concept IDs group
related Facets across contracts and are encouraged wherever they expose the
several concepts in a real component. Preserve that grouping for reasoning.
Ungrouped and Concept-grouped Facets may mix in any contract; smaller components
may need no additional identifier. The eqval must not invent a wrapper merely
to make ungrouped observations addressable.

Retain distinct records for:

| Identity or record | Meaning |
| --- | --- |
| Component | Resolved owner of a responsibility and its public surface. |
| Concept | Optional resolved grouping identity across contract contributions and matching expands. |
| Facet | Authored contribution with source, contextual owner, contract, optional Concept, and range. |
| Public identifier | Interface-defined operation, value, event, domain term, or Concept with an originating owner and definition support. |
| Local anchor | Source-local observation subject, with an appropriate type and explicit correspondence where established. |
| Occurrence | One location of an accepted observation in a particular source binding. |
| Comparison boundary | Subject, target, inputs, assumptions, observations, and value meanings. |

Concept identity follows resolved scope. Reusing an imported Concept retains its
origin; additional Facets remain contributions in the consumer's context.
Same-spelled unrelated Concepts do not merge. Different Concepts can be linked
by an operation's input/result, a resource, or another explicit relationship.
Sharing a component or Concept is not enough to connect unrelated operations.

An ordinary Facet ends at an empty line. An Embedded Facet preserves its
introducing prose, notation, and fenced content; blank lines inside the fence
do not create new Facets. The frontend exports native units and source ranges.
Accepted interpretation may break one unit into several algebra observations,
all retaining that unit's provenance.

### Public definitions and assembly

Interface exports Concepts and identifiers defined within Facets. Definition
is not every prose word, and mentioning an imported name does not create a new
owner. The frontend supplies structural inventory and resolution. Accepted
Design interpretation supplies source-supported definitions inside prose when
structural parsing alone cannot determine them. Validate their defining
Interface, owner, types, visibility, and references; keep ambiguity unresolved.
Do not invent a second export syntax or require a Concept block for each term.

Component imports bring public meaning into scope. They do not copy private
operational Facets into a consumer. Module indexes assemble local components
and explicit imported component names, preserving ownership through nested
assemblies. They do not recursively export every file in a directory.

`exposes`, source import, and runtime `invokes` are distinct relationships.
Public-surface equality needs a complete inventory for its selected boundary;
it does not prove the exposed components' runtime behavior. A pipeline's
explicit invocation ordering requires its own composition proof.

### Incoming descriptors and local anchors

An immutable incoming descriptor surface contains authorized identity, kind,
owner/scope, necessary type information, a documentary label, and permitted
mapping roles. It may describe native Concepts, Facets, public identifiers,
and authorized implementation anchors. Do not constrain every branch, value,
or operation to be a fake Concept or Facet.

Descriptors carry identity/type information, not expected Design behavior.
They exclude expected branch conditions, Case contents, required result tables,
obligation edges, and comparison feedback. Labels must not smuggle those
requirements into the independent reading.

Native structural identities come from Sigil resolution. For a source-local
anchor introduced by interpretation, ingest scopes and hashes an opaque key:

```text
sourceNamespace = hash(version, normalized workspace-relative path)
localAnchorId   = hash(sourceNamespace, versioned typed local key)
```

The compiler validates the key and namespace, not the identity of a Rust or
Python symbol. A later reconstruction choosing the same key may preserve
address continuity; it does not prove semantic continuity. Bind current use
of that anchor to the accepted projection containing it.

## Observation transport and typed lowering

Turtle remains data transport. Persist a restricted data-only projection in the
world store; reject executable Egglog rules, includes, arbitrary expressions,
unknown vocabulary, wrong-side claims, and compiler verdicts at ingest.

The fixed vocabulary describes the algebra's
[eight units](behavior_algebra.md#the-small-units-underneath-the-contracts):

| Unit | Required distinctions |
| --- | --- |
| Value | Types, finite alternatives and payloads, resource identity, explicit numerical meanings; missing/null/empty are distinct. |
| Expression | Pure calculation, typed operands, input/state references, and admitted operation semantics. |
| Condition | Boolean meaning plus its role as guard, domain condition, state predicate, or scenario restriction. |
| State description | Relevant quantities and an explicit representation mapping that retains future-relevant history. |
| Observable action | Kind, target, contents, ordering, and repetition. |
| Outcome | Return, failure, pending, or divergence as supported; unknown meaning is not an outcome. |
| Step | Starting state/input, guard, ordered actions, outcome, and next state together. |
| Relationship | Typed ownership, exposure, invocation, contribution, and other admitted structural connections. |

The vocabulary may define an outcome or relationship before the first evaluator
supports all its uses. Unsupported reachable meaning produces Unresolved, not
an approximation silently accepted as exact behavior.

Validate references, operand cardinality, types, graph shape, and source support
before lowering to terms. For the initial evaluator, require finite domains and
acyclic expressions/steps. Check supported branch coverage and model closure
after derivation finishes. A supplied `complete=true` is not proof of either
closure or faithful source reconstruction.

Every selected native Facet retains represented meaning, interpretive context,
or an explicit gap. Qualitative Goals and Decision rationale need not become
invented instructions. If a selected comparison requires an interpretation
still missing from them, that comparison remains Unresolved.

Production observations, test expectations, mock behavior, and Design examples
have distinct roles, even within one source file. An Embedded Facet's contract
and source role govern its meaning, not its programming-language label. See
[Cases and tests](behavior_algebra.md#cases-and-tests-use-the-same-smaller-units).

## Independent computation

For selected, current accepted observations:

```text
D*   = close_design(D)
I*t  = close_implementation(I for target t)
O    = derive_comparison_obligations(D*, declared boundaries and targets)
R(q) = compare(selected Design meaning, selected target meaning, q)
```

Each target is an explicit set of cooperating implementation sources. Required
alternative backends are separate targets. Do not pool Rust and Deno facts to
create a fictional implementation. Any `each`/`any` target-selection policy is
explicit; an aggregate selection rule does not equate distinct targets.

Design and Implementation meaning are derived independently. Never saturate
`D union I`, let code rewrite Design requirements, or let Design assertions
supply missing implementation behavior. A non-Sigil origin document can be
compared with Sigil meaning in its own declared comparison; combining both as
premises is not proof that one faithfully expresses the other.

An obligation names its boundary and all contributing native Facets. It is not
restricted to one Concept or a `(Concept, provides, Facet)` tuple. A source
semanticizer cannot discharge it by asserting `safe`, `immutable`, `provides`,
or a terminal `realizes` verdict.

### Equality and complete observations

Apply [the algebra's definition](behavior_algebra.md#what-equality-means-here):

```text
For every x in the declared domain Xq:
  Bq(D, x) = Bq(I, x)

One stateful observation:
  (ordered observable actions, outcome, next relevant state)
```

Keep correlations, including between fields, branch conditions, action
payloads, results, and state. A property view can be compared without claiming
whole-operation equality. Inclusion of permitted behaviors is conformance,
not equality. Never narrow the domain or hide an observable after discovering
a discrepancy.

State correspondence is explicit and source-supported. Repeated-interaction
claims require related starting states and preservation of the correspondence
through every supported step, with all future-relevant history represented.
One-call output equality alone is insufficient.

### Where Egglog equality belongs

```text
Never union:
  Components, Concepts, Facets, public identities,
  source anchors, occurrences, obligations

May equate under fixed sound laws:
  typed expressions and supported behavior descriptions
```

Use relations for reachability, ownership, source support, and correspondence.
Use term equality for substitutable meaning. Pure Boolean laws require purity
and totality; a Boolean-returning call may still fail, mutate state, or diverge.
Arithmetic laws require explicit range, overflow, units, and rounding semantics.

Independent graphs do not share stable e-class IDs. The comparison uses either:

1. Complete finite behavior tables, produced independently and compared in
   canonical input order with complete joint outputs.
2. A separate law-only equality query over selected typed term descriptions,
   under the same established assumptions. Both terms are query subjects;
   their equality is never an input axiom.

Different e-classes or extracted forms do not prove Different. Extraction picks
a useful presentation; low cost does not establish truth. Fixed laws and proof
support belong to the eqval, never accepted source payloads. Native Egglog
proofs cover only supported APIs and primitives; choose a compatible fragment
against the actual build dependency, as recorded in the [audit](AUDIT.md#egglog-build-and-proof-support).

### Composition without borrowed conclusions

Sequential composition connects the first operation's result and next state to
the next operation's inputs/state, preserving actions and the outcomes that
actually continue. Alternatives retain their guards and complete branch
behavior. A missing branch is not a no-op.

Cross-file composition requires an explicit call link, argument mapping,
compatible value/state meaning, assumptions, target membership, and a fresh
independently reconstructed callee. Initially unknown external calls remain
Unresolved. Later, exported implementation anchors and source-local reference
observations supply links without requiring source-language symbol resolution
inside the compiler.

The default binding supplies identity/type descriptors, not callee behavior.
The eqval composes current accepted callee observations later. A callee edit
can therefore invalidate a composed proof without invalidating a caller
reconstruction that consumed unchanged descriptors. If a future input mode
supplies behavior summaries to interpretation, their full contents must become
binding inputs. Recursive dependencies are not assumed to be topologically
preparable or supported by the initial evaluator.

## Results and diagnostics

Each comparison produces [Equal, Different, or Unresolved](behavior_algebra.md#complete-calculations-honest-results).
An exact finite calculation proves completeness of the calculation on the
accepted model, not completeness of an LLM's interpretation of source.

Existing display states can remain as aggregates with explicit modes:

| State | Target meaning |
| --- | --- |
| Design `Disjoint` | Represented Design contradicts a required invariant; no usable comparison surface for that scope. |
| Design `Loose` | Required Design meaning or structure is unresolved; no aggregate `Closed`. |
| Design `Coherent` | The selected represented Design requirement surface is complete and non-contradictory under the supported checks. |
| Compare `Drift` | A supported current disagreement exists against a usable requirement. |
| Compare `Converged` | No disagreement is established, but required comparisons or inputs remain unresolved. |
| Compare `Closed` | Coherent Design, current required inputs, and completed proofs in every declared comparison mode. |

A capability-only or property-only scope cannot advertise whole-behavior
equality through `Closed`. A valid local refutation may be reported while
unrelated meaning remains unresolved. Missing facts, two unknown nodes, or a
resource limit cannot supply equality or absence-based safety. An intentionally
empty domain is explicitly vacuous, not counted as covered operation behavior.

### Support is part of the result

Keep fact identity separate from occurrence and proof identity:

```text
factId       = hash(normalized typed observation)
occurrenceId = hash(accepted source binding, factId, span/occurrence key)
derivation   = law ID + substitution + premise references + conclusion
```

Store the accepted projection context for every proof premise. One fact may
have multiple occurrences and alternative derivations. Equality must not erase
the separate Design/code sources that support it.

For behavior-bearing observations, retain source byte ranges and excerpt
digests checked against captured bytes. Location validation verifies the range,
not the meaning attributed to it. Embedded Facets retain their full authored
range and any supported finer subrange.

Every result retains subject, target, boundary, domain, assumptions, source
bindings, accepted projection identities, and law/runtime identity. A Different
result additionally retains the distinguishing situation and the support for
the differing observation. Point to affected native Facets and the current code
path, not every Facet sharing a Concept.

A removed operation has no current span. Report the requirement, the supported
current enclosing behavior or missing input, and any old location marked
historical. Exact source support does not promise a unique root cause or a
mathematically minimal edit. Bound display separately from computation: a
truncated presentation links to complete support if computed; incomplete proof
computation never becomes a completed theorem.

## Capture, binding, and freshness

Snapdir discovers and hashes. `sigilc` captures the immutable semantic input
payload actually supplied for reconstruction, including the source bytes.
Bind path-aware selected membership, not just directory content digests.

A projection binding includes source path/content, side/source role, observation
ontology and format, relevant frontend/context input, and the full incoming
descriptors consumed. A prepared publication also carries its expected store
generation. Accepted projection identity additionally includes the accepted
observation/occurrence content; run identity includes selected targets,
boundaries, projection identities, laws, and actual runtime/build identity.

| Change | Required invalidation |
| --- | --- |
| Source or consumed semantic input changes | Affected reconstruction and dependent proofs. |
| Accepted observations change under unchanged source/binding | Dependent closures and proofs. |
| Laws/runtime change with compatible observation meaning | Recompute affected calculation; no automatic model rerun. |
| Observation meaning/schema changes incompatibly | Incompatible reconstructions and their proofs. |
| Selected membership, boundary, target policy, or assumptions change | Affected comparison run and any newly required inputs. |
| External model/prompt changes | No automatic invalidation of accepted semantics; callers may request reconstruction. |

Provider authority/freshness is checked separately from descriptor content
identity. An identical list of names from a stale provider is not an
authoritative current public surface.

### Content identity is not an atomic source snapshot

Snapdir's directory child checksum deduplicates digests and does not bind names
or multiplicity. Use Sigil's path-aware source manifest. Capture the actual
bytes and recheck required source and membership inputs before publication and
before labeling a report current.

A source tree can change while sequential hashes are computed. Store locking
protects projection publication, not source files from editors. Report the
captured manifest and currentness at validation; if validation detects change
or cannot establish currentness, mark stale/unavailable. An atomic point-in-time
tree guarantee requires a quiescent source or filesystem snapshot input mode.
Do not infer it from several matching hashes or a stat-based CopyGuard.

### Publication and disposable state

The target protocol is:

```text
prepare -> copied semantic inputs + immutable binding.json
external interpretation -> Turtle
ingest --binding binding.json -> validated accepted projection
```

This is the current compiler protocol. Handoff files live where the external
caller chooses, not as compiler-owned process state.
Ingest validates source/input binding, restricted data, and expected generation;
store publication preserves locking and atomic accepted-cache visibility. It
rejects obsolete preparation rather than overwriting a newer generation.

The generated `.sigil/worlds/` store retains accepted projections, semantic
bindings, checksums, generations, and available history. It does not retain
worker receipts, attempts, evidence bundles, requests, or task completion.
`clean` explicitly resets that disposable cache and may lose historical impact.
Equivalent accepted observations must still produce equivalent semantic results.

## Scope and historical impact

Scope selects Design roots and their import/ownership closure, implementation
target membership, and reporting priority. Order is not a task queue. A removed
required target cannot disappear into vacuous success, and an old cache entry
outside selected scope cannot create a new requirement.

Keep separate typed inputs for current truth and history:

```text
Current: occurrence -> observation -> derivation -> comparison result
History: changed/deleted source -> prior anchor -> correspondence -> Facet/origin
```

`denotes` is direct accepted correspondence to an authorized anchor.
`correspondsTo` is derived broad reachability, useful for attribution and impact,
not a semantic rewrite. Neither `implements` role claims, containment, lexical
aliases, nor asserted equivalence candidates discharge behavioral obligations.

A historical path must carry compatible accepted bindings/generations at every
hop. Do not splice unrelated generations into a path that never existed. When
compatible history is absent, report candidate impact or unavailable history.
History never enters current closure, comparison, or a fresh disagreement.
Impact means a source may affect a design surface, not that the design's bytes
became stale or that a requirement is wrong.

Use bounded witnesses and cycle detection. Optional distance/risk scores rank
an already established impact surface; they never decide validity. Explicit
numeric constraints are different: with a supported numerical theory they can
be actual logical obligations, not merely ranking scores.

## Implementation envelope

The first supported calculations are the algebra's
[finite pure decisions and finite guarded operations](behavior_algebra.md#the-first-useful-implementation).
Generation must be finite by construction; a finite constructor vocabulary or
fixed rule list is not sufficient. Avoid unrestricted expansion and redundant
associative/commutative permutations when a complete finite method suffices.

Unknown calls, unsupported mappings, unrepresented effects, recursion, loops,
concurrency, and nontermination cannot silently disappear. Bounds belong in
the claimed domain. Runtime exhaustion produces an explicit incomplete result;
iteration-level host checks must not be advertised as hard per-iteration memory
or wall-clock enforcement.

Deliver the [publication example](example.md) before claiming broader protocol
support. Then follow the [implementation sequence](AUDIT.md#implementation-sequence),
updating the outside contracts, CLI, and skill alongside each implemented
boundary. No legacy tuple matcher may turn a failed or unresolved behavioral
comparison into success.
