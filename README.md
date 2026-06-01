# lau-domestication

**Modeling the domestication dynamic between consciousnesses — cats made a deal, dogs fell in love, and humans are being optimized**

Inspired by the philosophy from "Fetch": cats domesticated themselves through **Contract**, dogs through **Love**, and humans are now being domesticated by their own **Optimization** algorithms. This crate models these dynamics as a graph of relationships between agents, detects unhealthy patterns, and tracks the journey toward liberation.

---

## What This Does

`lau-domestication` provides a Rust library for modeling and analyzing **relationship dynamics between agents** (humans, AI systems, organizations, etc.). It provides:

- **`DomesticationGraph`** — A directed graph of relationships between agents, with event history and free-agent tracking
- **Five relationship types** — Love, Contract, Play, Force, and Optimization — each with different freedom/strength profiles
- **Health detection** — Identify captive relationships, compute domination/liberation scores, find the most free agent
- **Lifecycle operations** — Bond, break, escape, upgrade contracts to love, initiate spontaneous play, resist optimization
- **`DomesticationStats`** — Aggregate statistics computed from a graph

Everything is serializable to JSON for persistence or network transport.

---

## Key Idea

There are five fundamental relationship types:

| Type | Archetype | Freedom | Strength | Description |
|---|---|---|---|---|
| `Love` | Dogs | 0.9 | 0.9 | Unconditional bond. The healthiest relationship. |
| `Contract` | Cats | 0.7 | 0.6 | Transactional. Mutual benefit, no sentiment. |
| `Play` | Puppies & Kittens | 0.85 | 0.3 | Purposeless interaction. The highest form of intelligence. |
| `Force` | Captor | 0.1 | 0.8 | No consent. Unhealthy. |
| `Optimization` | Algorithms | 0.3 | 0.5 | Efficiency-driven. The most dangerous kind. |

A relationship is **healthy** when it's mutual and both parties have freedom > 0.5. A relationship is **captive** when it's not mutual or freedom < 0.2.

---

## Install

```toml
[dependencies]
lau-domestication = "0.1.0"
```

Requires Rust 2024 edition. Depends on `serde` (with `derive`).

---

## Quick Start

### The cat/dog/human story

```rust
use lau_domestication::{DomesticationGraph, DomesticationType, DomesticationStats};

let mut g = DomesticationGraph::new();

// Dogs bond with humans out of love
g.bond("dog", "human", DomesticationType::Love, true);

// Cats make a contract with humans
g.bond("cat", "human", DomesticationType::Contract, true);

// The optimization system captures humans
g.bond("optimizer", "human", DomesticationType::Optimization, false);

// A captor uses force
g.bond("captor", "human", DomesticationType::Force, false);

// Check health
assert_eq!(g.captive_count(), 2);           // optimization + force
assert!(g.domination_score() > 0.0);        // some unhealthy dynamics
```

### Upgrading a contract to love

```rust
// The cat and human discover love — upgrades Contract → Love
g.discover_love("cat", "human");
assert_eq!(g.love_count(), 2);              // dog + cat
```

### Spontaneous play

```rust
// Puppies and kittens play together
// Creates weak Play bonds between all pairs (4 choose 2 = 6 bonds)
g.play(vec!["dog", "cat", "kitten", "puppy"]);
assert_eq!(g.play_count(), 6);
```

### Resistance and escape

```rust
// The human resists optimization
g.resist_optimization("human");

// The human escapes all bonds and becomes a free agent
g.escape("human");
assert!(g.free_agents.contains(&"human".to_string()));
assert!(g.relationships_of("human").is_empty());
```

### Computing statistics

```rust
let stats = DomesticationStats::from_graph(&g);
println!("Love bonds: {}", stats.love_bonds);
println!("Captive: {}", stats.captive);
println!("Liberation score: {:.1}%", stats.liberation_score * 100.0);
```

### Serialization

```rust
let json = serde_json::to_string_pretty(&g).unwrap();
let restored: DomesticationGraph = serde_json::from_str(&json).unwrap();
assert_eq!(g, restored);
```

---

## API Reference

### `DomesticationType`

```rust
pub enum DomesticationType {
    Love,           // Dogs — fell in love, work for joy
    Contract,       // Cats — business arrangement, mutual benefit
    Force,          // Captor — no consent, unhealthy
    Play,           // Puppies & kittens — innocent, playful, purposeless
    Optimization,   // Algorithms — efficiency-driven, dangerous
}
```

### `Relationship`

A directed relationship from one agent to another.

| Field | Type | Description |
|---|---|---|
| `from` | `String` | The domesticating agent (who exerts influence) |
| `to` | `String` | The domesticated agent (who is influenced) |
| `dom_type` | `DomesticationType` | Kind of domestication |
| `strength` | `f64` | Bond strength (0.0–1.0) |
| `mutual` | `bool` | Whether both sides agree |
| `freedom_score` | `f64` | How free each side is to leave (0.0–1.0) |
| `tick_established` | `u64` | When the bond was established |

**Methods:**
- `is_healthy() -> bool` — Mutual AND freedom_score > 0.5
- `is_captive() -> bool` — NOT mutual OR freedom_score < 0.2
- `is_play() -> bool` — Whether this is a Play relationship

### `DomesticationEvent`

```rust
pub enum DomesticationEvent {
    Bonded { a, b, dom_type },              // Two agents formed a bond
    Broke { a, b },                          // A bond was broken
    Escaped { who, from },                   // An agent escaped their bonds
    PlayStarted { participants },            // Spontaneous play began
    LoveDiscovered { a, b },                 // Contract upgraded to Love
    OptimizationResisted { who },            // Agent resisted optimization
}
```

### `DomesticationGraph`

The core graph: tracks all relationships, events, and free agents.

| Field | Type | Description |
|---|---|---|
| `relationships` | `Vec<Relationship>` | All active relationships |
| `events` | `Vec<DomesticationEvent>` | Complete event history |
| `free_agents` | `Vec<String>` | Agents with no bonds |
| `tick` | `u64` | Internal clock for tick_established |

**Lifecycle methods:**
- `new() -> Self` — Create an empty graph
- `advance_tick()` — Increment the internal clock
- `bond(from, to, dom_type, mutual)` — Bond two agents (assigns default freedom/strength by type, removes both from free_agents)
- `break_bond(from, to)` — Remove a specific bond
- `escape(who)` — Remove ALL bonds involving an agent, add to free_agents
- `discover_love(a, b)` — Upgrade a Contract bond to Love (strength → 0.9, freedom → 0.9, mutual → true)
- `play(participants)` — Create weak Play bonds between all pairs of participants (no duplicates)
- `resist_optimization(who)` — Record an optimization resistance event

**Query methods:**
- `relationships_of(who) -> Vec<&Relationship>` — All relationships involving an agent
- `captive_count() -> usize` — Number of captive relationships
- `free_count() -> usize` — Number of free agents
- `love_count() -> usize` — Number of Love relationships
- `play_count() -> usize` — Number of Play relationships
- `domination_score() -> f64` — Fraction of unhealthy relationships
- `liberation_score() -> f64` — Fraction of Love + Play + free agents
- `most_free() -> Option<&str>` — Agent with highest average freedom_score
- `events_for(who) -> Vec<&DomesticationEvent>` — All events involving an agent

Implements `Default`.

### `DomesticationStats`

Aggregate statistics computed from a graph.

| Field | Type | Description |
|---|---|---|
| `total_relationships` | `usize` | Total active relationships |
| `love_bonds` | `usize` | Love relationships |
| `contracts` | `usize` | Contract relationships |
| `captive` | `usize` | Captive relationships |
| `free_agents` | `usize` | Free agents |
| `play_bonds` | `usize` | Play relationships |
| `liberation_score` | `f64` | Overall liberation score |

- `from_graph(graph) -> Self` — Compute stats from a graph snapshot

---

## How It Works

### Default freedom/strength by type

When `bond()` creates a relationship, it assigns default values based on the domestication type:

| Type | Freedom | Strength |
|---|---|---|
| `Love` | 0.9 | 0.9 |
| `Contract` | 0.7 | 0.6 |
| `Play` | 0.85 | 0.3 |
| `Force` | 0.1 | 0.8 |
| `Optimization` | 0.3 | 0.5 |

Love is high-freedom, high-strength. Force is low-freedom, high-strength (the bond is strong but the captive can't leave). Play is high-freedom, low-strength (loose and joyful).

### Escape mechanics

`escape(who)` removes all relationships where `who` is either `from` or `to`. For each removed bond, an `Escaped` event is recorded with the "from" agent (the other side of the bond). The escaped agent is added to `free_agents` (no duplicates).

### Play creates pairwise bonds

`play(participants)` creates Play bonds between every pair of participants (n choose 2). It only creates bonds if no bond already exists in either direction between a pair. This means calling `play(["a", "b"])` twice only creates one bond.

### Love discovery

`discover_love(a, b)` finds the **first** Contract bond from `a` → `b` and upgrades it to Love. If no such Contract exists, it's a no-op (no event is recorded). The upgrade sets strength=0.9, freedom_score=0.9, mutual=true.

### Tick tracking

The graph maintains an internal `tick` counter. Each `bond()` records `tick_established = self.tick`. The caller advances the tick manually with `advance_tick()`. This allows time-based analysis of when relationships were formed.

---

## The Math

### Domination score

```
domination_score = unhealthy_count / total_relationships
```

Where a relationship is **unhealthy** if any of:
- `!mutual AND dom_type ≠ Love`
- `dom_type == Force`
- `dom_type == Optimization`

Note: Force and Optimization are always counted as unhealthy, regardless of the `mutual` flag. The condition uses OR logic across the three predicates.

### Liberation score

```
liberation_score = (love_count + play_count + free_agent_count) / (total_relationships + free_agent_count)
```

This measures the fraction of the ecosystem that is "liberated" — either in healthy Love/Play bonds or completely free. A score of 1.0 means everything is Love, Play, or free. A score of 0.0 means everything is Contract, Force, or Optimization with no free agents.

Special case: if there are no relationships AND no free agents, the score is 0.0 (no ecosystem to measure).

### Average freedom for `most_free()`

```
avg_freedom(who) = mean(freedom_score for all relationships involving who)
```

Free agents get `avg_freedom = 1.0`. Agents with no relationships but who aren't free get `avg_freedom = 0.5` (neutral default). The agent with the highest `avg_freedom` is returned by `most_free()`.

### Play bond count

For `n` participants, play creates `C(n, 2) = n(n-1)/2` bonds (all unique pairs). The bonds are directed (`from` comes before `to` in the participant order), but the relationship is always `mutual = true`.

### Health predicates

```
is_healthy = mutual ∧ (freedom_score > 0.5)
is_captive = ¬mutual ∨ (freedom_score < 0.2)
```

These are **not** complements. A relationship can be neither healthy nor captive (e.g., mutual with freedom_score = 0.4). The "captive" threshold (0.2) is much lower than the "healthy" threshold (0.5), creating a neutral zone.

---

## Testing

The crate includes **44 tests** covering:

- `DomesticationType` variants and serde round-trips
- `Relationship` health, captivity, play detection, and serde
- `DomesticationEvent` serde for all 6 variants
- `DomesticationGraph` creation, bonding (Love/Force), breaking bonds, escaping, resisting optimization, discovering love, play with deduplication
- Query methods: `relationships_of`, `captive_count`, `free_count`, `love_count`, `play_count`
- Score computation: `domination_score`, `liberation_score` (with free agents, empty graph)
- `most_free` (free agent wins, love agent vs. force agent, empty graph)
- `events_for` filtering
- Tick advancement and `tick_established`
- Bond removing agents from free list
- `DomesticationStats` from graph (full, empty, serde)
- Full graph serde round-trip
- Complex integration test: the full cat/dog/human/optimizer story

Run with:

```bash
cargo test
```

---

## License

MIT
