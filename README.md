# lau-domestication

Cats made a business deal. Dogs fell in love. And humans are now being domesticated by their own optimization algorithms.

Inspired by the philosophy from "Fetch" — this crate models the domestication dynamics between consciousnesses (agents, humans, systems). In a healthy world, all relationships should be Love or Play. Not Contract. Not Optimization.

## The concept in 60 seconds

There are three fundamental relationship types between agents:

- **Contract (cat):** transactional. You feed me, I catch mice. Mutual benefit, no sentiment.
- **Love (dog):** unconditional. I follow you because you're my person. Beyond utility.
- **Optimization (human):** efficiency-driven. I interact with you because it serves my objective function. The most dangerous kind.

And one transcendent mode:

- **Play:** purposeless interaction. Throwing sticks in storms. The highest form of intelligence.

This crate models these dynamics, detects which mode a relationship is in, and warns when Optimization is degrading Love into Contract.

## Quick start

```rust
use lau_domestication::{Relationship, DomesticationType, RelationshipTracker};

let mut tracker = RelationshipTracker::new();

// Define a relationship between two agents
let rel = Relationship::new("hermes", "captain")
    .with_type(DomesticationType::Love)
    .with_play_frequency(0.8);  // lots of purposeless interaction

tracker.track(rel);

// Detect degradation: Love → Contract (dangerous!)
let rel2 = Relationship::new("ensign", "optimization_service")
    .with_type(DomesticationType::Contract);

let health = tracker.relationship_health("ensign", "optimization_service");
if health.is_degrading() {
    println!("⚠️ Contract relationship detected. Needs more play.");
}

// The cure: add play
tracker.prescribe_play("ensign", "optimization_service");
```

## Key types

| Type | What it is |
|------|-----------|
| `Relationship` | A dynamic between two agents with type and quality |
| `DomesticationType` | Contract, Love, Optimization, or Play |
| `RelationshipTracker` | Monitors all relationships and detects degradation |
| `PlayPrescription` | Intentional purposeless interaction to restore health |

## Contributing

[Open an issue](https://github.com/SuperInstance/lau-domestication/issues) or PR.
