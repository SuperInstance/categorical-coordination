# categorical-coordination

> **Agents as objects. Messages as morphisms. Coordination as category theory.**

[![crates.io](https://img.shields.io/crates/v/categorical-coordination.svg)](https://crates.io/crates/categorical-coordination)
[![docs.rs](https://docs.rs/categorical-coordination/badge.svg)](https://docs.rs/categorical-coordination)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A Rust library applying category theory to multi-agent coordination. Models agents as objects, communication protocols as morphisms, coordination strategies as functors between categories, and protocol evolution as natural transformations. Pullbacks merge agent teams; pushouts split them. Consensus is a limit; team formation is a colimit.

---

## Table of Contents

- [What is Categorical Coordination?](#what-is-categorical-coordination)
- [Why Does This Matter?](#why-does-this-matter)
- [Architecture](#architecture)
- [Quick Start](#quick-start)
- [API Reference](#api-reference)
- [Mathematical Background](#mathematical-background)
- [Installation](#installation)
- [Related Crates](#related-crates)
- [License](#license)

---

## What is Categorical Coordination?

Category theory studies structure through **objects**, **morphisms** (arrows between objects), and **composition** (chaining arrows). Multi-agent systems have exactly this structure:

```
Category Theory              Multi-Agent System
────────────────             ──────────────────
Objects                      Agents with capabilities
Morphisms (A → B)            Communication protocols (A sends to B)
Composition (f ∘ g)          Protocol chaining (A→B, B→C gives A→C)
Identity (id_A)              Null protocol (A does nothing)
Functors (C → D)             Coordination strategies (map team C to team D)
Natural transformations      Protocol upgrades (evolve strategy)
Limits (pullback)            Consensus / team merge
Colimits (pushout)           Team formation / role splitting
```

The power of this abstraction: **categorical constructions automatically preserve compositional structure**. If you define a coordination strategy as a functor, it automatically respects protocol composition — no bugs from manual wiring.

```
         Agent A ──msg──→ Agent B ──msg──→ Agent C
           │                                     │
           │           Functor F                 │
           ▼                                     ▼
         Agent A'──msg──→ Agent B'──msg──→ Agent C'

         F preserves all structure:
         F(A→B→C) = F(A)→F(B)→F(C)
```

## Why Does This Matter?

**For distributed systems**: Category theory gives precise semantics to composition, routing, and transformation of messages — the building blocks of any distributed protocol.

**For multi-agent coordination**: Functors encode coordination strategies that are correct by construction. If the functor respects composition, the resulting coordination is automatically compositional.

**For protocol evolution**: Natural transformations let you upgrade protocols in-flight without breaking the categorical structure — evolving a team's communication pattern while preserving correctness.

**For team dynamics**: Pullback = find the common ground between two teams. Pushout = merge two teams with shared members. These are the primitive operations for dynamic team formation.

## Architecture

```
categorical-coordination
│
├── Morphism / Category         ← Core categorical structures
│   ├── Category::new(name)         Create named category
│   ├── add_object()                Add agent to category
│   ├── add_morphism()              Add communication protocol
│   ├── compose(f, g, result)       Protocol composition f ∘ g
│   ├── identity(obj)               Null protocol for agent
│   ├── hom(source, target)         All protocols from A to B
│   └── verify_identity_laws()      Check categorical axioms
│
├── Functor                     ← Coordination strategies
│   ├── new(name, source, target)   Strategy mapping categories
│   ├── map_object(src, tgt)        Map agent A → A'
│   ├── map_morphism(src, tgt)      Map protocol f → f'
│   ├── verify_functoriality()      Check F(f∘g) = F(f)∘F(g)
│   └── apply_to_object/morphism    Execute the mapping
│
├── NaturalTransformation       ← Protocol upgrades
│   ├── new(name, source_F, target_G)  Upgrade from functor F to G
│   ├── set_component(obj, morph)      Per-agent upgrade path
│   └── verify_naturality()            Check naturality square
│
├── Limits & Colimits           ← Team operations
│   ├── pullback(cat, f, g)          Merge: find common subteam
│   └── pushout(cat, f, g)           Split: form new team
│
└── CoordinationProtocol        ← Practical agent messaging
    ├── register_agent(id, state)    Add agent with initial state
    ├── send_message(id, from, to)   Record communication
    ├── get_state(agent)             Query agent state
    ├── update_state(agent, state)   Update agent state
    └── messages_from/to(agent)      Message history
```

## Quick Start

```rust
use categorical_coordination::{
    Category, Morphism, Functor, NaturalTransformation,
    pullback, pushout, CoordinationProtocol,
};

// Build a category of agents
let mut team = Category::new("Alpha");
team.add_object("sensor");
team.add_object("processor");
team.add_object("actuator");
team.add_morphism(Morphism::new("raw_data", "sensor", "processor"));
team.add_morphism(Morphism::new("command", "processor", "actuator"));
team.compose("raw_data", "command", "sense_act");

// Verify it's a valid category (identity + associativity laws)
assert!(team.verify_identity_laws());
println!("Objects: {}, Morphisms: {}", team.num_objects(), team.num_nontrivial_morphisms());

// Create a coordination strategy (functor) that maps to a simplified team
let mut strategy = Functor::new("simplify", "Alpha", "AlphaLite");
strategy.map_object("sensor", "io_unit");
strategy.map_object("processor", "io_unit");
strategy.map_object("actuator", "io_unit");
strategy.map_morphism("raw_data", "internal");
strategy.map_morphism("command", "internal");

// Verify functoriality: F preserves composition
let is_valid = strategy.verify_functoriality(&team, &team); // simplified check
println!("Functor preserves structure: {}", is_valid);

// Practical protocol: message-passing between agents
let mut protocol = CoordinationProtocol::new("mission");
protocol.register_agent("scout", "idle");
protocol.register_agent("leader", "planning");
protocol.send_message("order_1", "leader", "scout");
protocol.update_state("scout", "moving");
println!("Scout state: {:?}", protocol.get_state("scout"));
println!("Messages to scout: {}", protocol.messages_to("scout").len());
```

## API Reference

### Category

| Method | Returns | Description |
|--------|---------|-------------|
| `new(name)` | `Self` | Create named category |
| `add_object(id)` | `&mut Self` | Add agent (object) |
| `add_morphism(morph)` | `&mut Self` | Add protocol (morphism) |
| `compose(f, g, result)` | `()` | Compose f ∘ g = result |
| `identity(obj)` | `Option<&Morphism>` | Null protocol |
| `find_morphism(id)` | `Option<&Morphism>` | Lookup by name |
| `hom(source, target)` | `Vec<&Morphism>` | All morphisms A → B |
| `verify_identity_laws()` | `bool` | Check left/right identity |
| `num_objects()` | `usize` | Agent count |
| `num_nontrivial_morphisms()` | `usize` | Non-identity protocol count |

### Functor

| Method | Returns | Description |
|--------|---------|-------------|
| `new(name, source, target)` | `Self` | Create functor |
| `map_object(src, tgt)` | `&mut Self` | Map agent |
| `map_morphism(src, tgt)` | `&mut Self` | Map protocol |
| `verify_functoriality(src, tgt)` | `bool` | F(f∘g) = F(f)∘F(g)? |
| `apply_to_object(obj)` | `Option<&ObjId>` | Get image of object |
| `apply_to_morphism(morph)` | `Option<&MorphId>` | Get image of morphism |

### NaturalTransformation

| Method | Returns | Description |
|--------|---------|-------------|
| `new(name, source_F, target_G)` | `Self` | Create transformation |
| `set_component(obj, morphism)` | `&mut Self` | Set component α_A |
| `verify_naturality(src, tgt, F, G)` | `bool` | Check naturality square |

### Limits & Colimits

| Function | Returns | Description |
|----------|---------|-------------|
| `pullback(cat, f, g)` | `Category` | Merge (fiber product) |
| `pushout(cat, f, g)` | `Category` | Split (fiber coproduct) |

### CoordinationProtocol

| Method | Returns | Description |
|--------|---------|-------------|
| `new(name)` | `Self` | Create protocol |
| `register_agent(id, state)` | `()` | Add agent |
| `send_message(id, from, to)` | `()` | Record communication |
| `get_state(agent)` | `Option<&String>` | Query agent state |
| `update_state(agent, state)` | `()` | Update agent |
| `messages_from(agent)` | `Vec<&Morphism>` | Outgoing messages |
| `messages_to(agent)` | `Vec<&Morphism>` | Incoming messages |

## Mathematical Background

### Categories

A category C consists of:
- Objects: Ob(C)
- Morphisms: Hom(A, B) for each pair A, B ∈ Ob(C)
- Composition: ∘ : Hom(B,C) × Hom(A,B) → Hom(A,C)
- Identity: id_A ∈ Hom(A,A) for each A

Satisfying:
- **Associativity**: (f ∘ g) ∘ h = f ∘ (g ∘ h)
- **Identity**: f ∘ id_A = f = id_B ∘ f for f: A → B

### Functors

A functor F: C → D maps objects to objects and morphisms to morphisms, preserving:
- **Composition**: F(g ∘ f) = F(g) ∘ F(f)
- **Identity**: F(id_A) = id_{F(A)}

In coordination: a functor maps one team structure to another while preserving all protocol relationships.

### Natural Transformations

A natural transformation α: F → G assigns to each object A a morphism α_A: F(A) → G(A) such that:

```
G(f) ∘ α_A = α_B ∘ F(f)    for all f: A → B
```

This is the **naturality square** — it ensures the transformation is consistent across all objects. In coordination: upgrading from strategy F to strategy G requires updating every agent's protocol simultaneously.

### Pullback and Pushout

**Pullback** (limit): Given f: A → C and g: B → C, the pullback is the "largest" object P with maps to A and B making the square commute. In coordination: the shared subteam of A and B that agrees on C.

**Pushout** (colimit): Given f: C → A and g: C → B, the pushout is the "smallest" object Q with maps from A and B. In coordination: merging teams A and B that share members from C.

## Installation

```bash
cargo add categorical-coordination
```

Or add to your `Cargo.toml`:

```toml
[dependencies]
categorical-coordination = "0.1"
```

## Related Crates

Part of the **SuperInstance Exocortex** math fleet:

- **[sheaf-laplacian](https://github.com/SuperInstance/sheaf-laplacian)** — Sheaf Laplacian and Hodge decomposition
- **[tropical-graph](https://github.com/SuperInstance/tropical-graph)** — Max-plus algebra on graphs
- **[graph-homology](https://github.com/SuperInstance/graph-homology)** — Clique complexes and Betti numbers
- **[cohomology-ring](https://github.com/SuperInstance/cohomology-ring)** — Cup products and cohomology operations
- **[cortex-bus-protocol](https://github.com/SuperInstance/cortex-bus-protocol)** — CQRS event bus for agent messaging

## License

MIT © [SuperInstance](https://github.com/SuperInstance)

Part of the [Exocortex](https://github.com/SuperInstance/exocortex) project.
