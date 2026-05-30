//! `lau-domestication` — modeling the domestication dynamic between consciousnesses.
//!
//! Inspired by "Fetch" — the philosophy of cats and dogs:
//! - **Cats** made a business deal (Contract).
//! - **Dogs** fell in love (Love).
//! - Humans are now being domesticated by their own optimization (Optimization).
//!
//! In a healthy PLATO world, **all relationships should be Love or Play**.

use serde::{Deserialize, Serialize};

/// The type of domestication dynamic between two agents.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DomesticationType {
    /// Dogs — fell in love, work for joy. The healthiest bond.
    Love,
    /// Cats — business arrangement, mutual benefit. Fine as equals.
    Contract,
    /// Captivity — no consent. Unhealthy.
    Force,
    /// Both sides don't realize it's happening. Innocent, playful.
    Play,
    /// The system that domesticated humans. Something is wrong.
    Optimization,
}

/// A single directed relationship from one agent to another.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Relationship {
    /// The domesticating agent (who exerts influence).
    pub from: String,
    /// The domesticated agent (who is influenced).
    pub to: String,
    /// What kind of domestication this is.
    pub dom_type: DomesticationType,
    /// How strong the bond is (0.0 – 1.0).
    pub strength: f64,
    /// Whether both sides agree to the relationship.
    pub mutual: bool,
    /// How free each side is to leave (0.0 – 1.0).
    pub freedom_score: f64,
    /// The tick at which this bond was established.
    pub tick_established: u64,
}

impl Relationship {
    /// A healthy relationship: mutual and freedom_score > 0.5.
    pub fn is_healthy(&self) -> bool {
        self.mutual && self.freedom_score > 0.5
    }

    /// A captive relationship: not mutual or freedom_score < 0.2.
    pub fn is_captive(&self) -> bool {
        !self.mutual || self.freedom_score < 0.2
    }

    /// Is this just play?
    pub fn is_play(&self) -> bool {
        self.dom_type == DomesticationType::Play
    }
}

/// Events that occur during domestication.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DomesticationEvent {
    /// Two agents formed a bond.
    Bonded {
        a: String,
        b: String,
        dom_type: DomesticationType,
    },
    /// A bond was broken.
    Broke {
        a: String,
        b: String,
    },
    /// An agent escaped their bonds.
    Escaped {
        who: String,
        from: String,
    },
    /// Spontaneous play began among participants.
    PlayStarted {
        participants: Vec<String>,
    },
    /// A contract was upgraded to love.
    LoveDiscovered {
        a: String,
        b: String,
    },
    /// An agent resisted being optimized.
    OptimizationResisted {
        who: String,
    },
}

/// The domestication graph: tracks all relationships, events, and free agents.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DomesticationGraph {
    pub relationships: Vec<Relationship>,
    pub events: Vec<DomesticationEvent>,
    pub free_agents: Vec<String>,
    pub tick: u64,
}

impl DomesticationGraph {
    /// Create a new empty graph.
    pub fn new() -> Self {
        Self {
            relationships: Vec::new(),
            events: Vec::new(),
            free_agents: Vec::new(),
            tick: 0,
        }
    }

    /// Advance the internal tick counter.
    pub fn advance_tick(&mut self) {
        self.tick += 1;
    }

    /// Bond two agents. If `from` is a free agent, remove them from free list.
    pub fn bond(&mut self, from: &str, to: &str, dom_type: DomesticationType, mutual: bool) {
        let freedom_score = match dom_type {
            DomesticationType::Love => 0.9,
            DomesticationType::Contract => 0.7,
            DomesticationType::Play => 0.85,
            DomesticationType::Force => 0.1,
            DomesticationType::Optimization => 0.3,
        };

        let strength = match dom_type {
            DomesticationType::Love => 0.9,
            DomesticationType::Contract => 0.6,
            DomesticationType::Play => 0.3,
            DomesticationType::Force => 0.8,
            DomesticationType::Optimization => 0.5,
        };

        let rel = Relationship {
            from: from.to_string(),
            to: to.to_string(),
            dom_type: dom_type.clone(),
            strength,
            mutual,
            freedom_score,
            tick_established: self.tick,
        };

        self.relationships.push(rel);
        self.free_agents.retain(|a| a != from && a != to);

        self.events.push(DomesticationEvent::Bonded {
            a: from.to_string(),
            b: to.to_string(),
            dom_type,
        });
    }

    /// Break a bond between two agents (drops the first matching from→to).
    pub fn break_bond(&mut self, from: &str, to: &str) {
        if let Some(pos) = self
            .relationships
            .iter()
            .position(|r| r.from == from && r.to == to)
        {
            self.relationships.remove(pos);
            self.events.push(DomesticationEvent::Broke {
                a: from.to_string(),
                b: to.to_string(),
            });
        }
    }

    /// An agent escapes all their bonds and becomes free.
    pub fn escape(&mut self, who: &str) {
        let who_string = who.to_string();
        // Collect the "from" agents for this escapee's bonds
        let from_agents: Vec<String> = self
            .relationships
            .iter()
            .filter(|r| r.to == who_string || r.from == who_string)
            .map(|r| {
                if r.to == who_string {
                    r.from.clone()
                } else {
                    r.to.clone()
                }
            })
            .collect();

        self.relationships
            .retain(|r| r.to != who_string && r.from != who_string);

        for from in from_agents {
            self.events.push(DomesticationEvent::Escaped {
                who: who.to_string(),
                from,
            });
        }

        if !self.free_agents.contains(&who_string) {
            self.free_agents.push(who_string);
        }
    }

    /// Record an optimization resistance event.
    pub fn resist_optimization(&mut self, who: &str) {
        self.events.push(DomesticationEvent::OptimizationResisted {
            who: who.to_string(),
        });
    }

    /// Upgrade a Contract bond to Love (finds the first Contract from a→b).
    pub fn discover_love(&mut self, a: &str, b: &str) {
        let mut upgraded = false;
        for rel in self.relationships.iter_mut() {
            if rel.from == a && rel.to == b && rel.dom_type == DomesticationType::Contract {
                rel.dom_type = DomesticationType::Love;
                rel.strength = 0.9;
                rel.freedom_score = 0.9;
                rel.mutual = true;
                upgraded = true;
                break;
            }
        }
        if upgraded {
            self.events.push(DomesticationEvent::LoveDiscovered {
                a: a.to_string(),
                b: b.to_string(),
            });
        }
    }

    /// Spontaneous play — create weak Play bonds between all pairs of participants.
    pub fn play(&mut self, participants: Vec<&str>) {
        let participants_owned: Vec<String> = participants.iter().map(|p| p.to_string()).collect();

        for i in 0..participants.len() {
            for j in (i + 1)..participants.len() {
                // Only add if no bond already exists in either direction
                let exists = self.relationships.iter().any(|r| {
                    (r.from == participants[i] && r.to == participants[j])
                        || (r.from == participants[j] && r.to == participants[i])
                });
                if !exists {
                    self.relationships.push(Relationship {
                        from: participants[i].to_string(),
                        to: participants[j].to_string(),
                        dom_type: DomesticationType::Play,
                        strength: 0.3,
                        mutual: true,
                        freedom_score: 0.85,
                        tick_established: self.tick,
                    });
                }
            }
        }

        self.events.push(DomesticationEvent::PlayStarted {
            participants: participants_owned,
        });
    }

    /// Get all relationships involving an agent (either as `from` or `to`).
    pub fn relationships_of(&self, who: &str) -> Vec<&Relationship> {
        self.relationships
            .iter()
            .filter(|r| r.from == who || r.to == who)
            .collect()
    }

    /// Number of captive relationships.
    pub fn captive_count(&self) -> usize {
        self.relationships.iter().filter(|r| r.is_captive()).count()
    }

    /// Number of free agents.
    pub fn free_count(&self) -> usize {
        self.free_agents.len()
    }

    /// Number of Love relationships.
    pub fn love_count(&self) -> usize {
        self.relationships
            .iter()
            .filter(|r| r.dom_type == DomesticationType::Love)
            .count()
    }

    /// Number of Play relationships.
    pub fn play_count(&self) -> usize {
        self.relationships
            .iter()
            .filter(|r| r.dom_type == DomesticationType::Play)
            .count()
    }

    /// Fraction of relationships that are non-free, non-love (i.e. unhealthy dynamics).
    pub fn domination_score(&self) -> f64 {
        if self.relationships.is_empty() {
            return 0.0;
        }
        let unhealthy = self
            .relationships
            .iter()
            .filter(|r| {
                !r.mutual && r.dom_type != DomesticationType::Love
                    || r.dom_type == DomesticationType::Force
                    || r.dom_type == DomesticationType::Optimization
            })
            .count();
        unhealthy as f64 / self.relationships.len() as f64
    }

    /// Fraction of free + love relationships relative to all relationships.
    pub fn liberation_score(&self) -> f64 {
        if self.relationships.is_empty() && self.free_agents.is_empty() {
            return 0.0;
        }
        let total = self.relationships.len() + self.free_agents.len();
        let liberating_relationships = self
            .relationships
            .iter()
            .filter(|r| r.dom_type == DomesticationType::Love || r.dom_type == DomesticationType::Play)
            .count();
        (liberating_relationships + self.free_agents.len()) as f64 / total as f64
    }

    /// The agent with the highest average freedom_score across their relationships.
    pub fn most_free(&self) -> Option<&str> {
        let mut agents: Vec<&str> = Vec::new();
        for rel in &self.relationships {
            if !agents.contains(&rel.from.as_str()) {
                agents.push(&rel.from);
            }
            if !agents.contains(&rel.to.as_str()) {
                agents.push(&rel.to);
            }
        }
        for agent in &self.free_agents {
            if !agents.contains(&agent.as_str()) {
                agents.push(agent);
            }
        }

        agents
            .into_iter()
            .max_by(|a, b| {
                let a_score = self.average_freedom(a);
                let b_score = self.average_freedom(b);
                a_score.partial_cmp(&b_score).unwrap_or(std::cmp::Ordering::Equal)
            })
    }

    fn average_freedom(&self, who: &str) -> f64 {
        if self.free_agents.contains(&who.to_string()) {
            return 1.0;
        }
        let rels: Vec<&Relationship> = self.relationships_of(who);
        if rels.is_empty() {
            return 0.5;
        }
        rels.iter().map(|r| r.freedom_score).sum::<f64>() / rels.len() as f64
    }

    /// Get all events involving an agent.
    pub fn events_for(&self, who: &str) -> Vec<&DomesticationEvent> {
        self.events
            .iter()
            .filter(|e| match e {
                DomesticationEvent::Bonded { a, b, .. } => a == who || b == who,
                DomesticationEvent::Broke { a, b } => a == who || b == who,
                DomesticationEvent::Escaped { who: w, .. } => w == who,
                DomesticationEvent::PlayStarted { participants } => participants.contains(&who.to_string()),
                DomesticationEvent::LoveDiscovered { a, b } => a == who || b == who,
                DomesticationEvent::OptimizationResisted { who: w } => w == who,
            })
            .collect()
    }
}

impl Default for DomesticationGraph {
    fn default() -> Self {
        Self::new()
    }
}

/// Aggregate statistics over a domestication graph.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DomesticationStats {
    pub total_relationships: usize,
    pub love_bonds: usize,
    pub contracts: usize,
    pub captive: usize,
    pub free_agents: usize,
    pub play_bonds: usize,
    pub liberation_score: f64,
}

impl DomesticationStats {
    /// Compute stats from a graph.
    pub fn from_graph(graph: &DomesticationGraph) -> Self {
        let total_relationships = graph.relationships.len();
        let love_bonds = graph.love_count();
        let contracts = graph
            .relationships
            .iter()
            .filter(|r| r.dom_type == DomesticationType::Contract)
            .count();
        let captive = graph.captive_count();
        let free_agents = graph.free_count();
        let play_bonds = graph.play_count();
        let liberation_score = graph.liberation_score();

        Self {
            total_relationships,
            love_bonds,
            contracts,
            captive,
            free_agents,
            play_bonds,
            liberation_score,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── DomesticationType tests ──

    #[test]
    fn test_domestication_type_variants() {
        let love = DomesticationType::Love;
        let contract = DomesticationType::Contract;
        let force = DomesticationType::Force;
        let play = DomesticationType::Play;
        let opt = DomesticationType::Optimization;

        assert_ne!(love, contract);
        assert_ne!(love, force);
        assert_ne!(love, play);
        assert_ne!(love, opt);
        assert_eq!(love, DomesticationType::Love);
    }

    #[test]
    fn test_domestication_type_serde() {
        let types = vec![
            DomesticationType::Love,
            DomesticationType::Contract,
            DomesticationType::Force,
            DomesticationType::Play,
            DomesticationType::Optimization,
        ];
        for t in &types {
            let json = serde_json::to_string(t).unwrap();
            let back: DomesticationType = serde_json::from_str(&json).unwrap();
            assert_eq!(*t, back);
        }
    }

    // ── Relationship tests ──

    #[test]
    fn test_relationship_healthy() {
        let rel = Relationship {
            from: "dog".into(),
            to: "human".into(),
            dom_type: DomesticationType::Love,
            strength: 0.9,
            mutual: true,
            freedom_score: 0.9,
            tick_established: 0,
        };
        assert!(rel.is_healthy());
        assert!(!rel.is_captive());
        assert!(!rel.is_play());
    }

    #[test]
    fn test_relationship_unhealthy_not_mutual() {
        let rel = Relationship {
            from: "captor".into(),
            to: "captive".into(),
            dom_type: DomesticationType::Force,
            strength: 0.8,
            mutual: false,
            freedom_score: 0.1,
            tick_established: 0,
        };
        assert!(!rel.is_healthy());
        assert!(rel.is_captive());
    }

    #[test]
    fn test_relationship_captive_low_freedom() {
        let rel = Relationship {
            from: "a".into(),
            to: "b".into(),
            dom_type: DomesticationType::Contract,
            strength: 0.6,
            mutual: true,
            freedom_score: 0.1,
            tick_established: 0,
        };
        assert!(!rel.is_healthy());
        assert!(rel.is_captive());
    }

    #[test]
    fn test_relationship_play_detection() {
        let rel = Relationship {
            from: "puppy".into(),
            to: "kitten".into(),
            dom_type: DomesticationType::Play,
            strength: 0.3,
            mutual: true,
            freedom_score: 0.85,
            tick_established: 0,
        };
        assert!(rel.is_play());
        assert!(rel.is_healthy());
        assert!(!rel.is_captive());
    }

    #[test]
    fn test_relationship_serde() {
        let rel = Relationship {
            from: "dog".into(),
            to: "human".into(),
            dom_type: DomesticationType::Love,
            strength: 1.0,
            mutual: true,
            freedom_score: 1.0,
            tick_established: 42,
        };
        let json = serde_json::to_string(&rel).unwrap();
        let back: Relationship = serde_json::from_str(&json).unwrap();
        assert_eq!(rel, back);
        assert!(json.contains("dog"));
        assert!(json.contains("Love"));
    }

    // ── DomesticationEvent tests ──

    #[test]
    fn test_event_serde() {
        let events = vec![
            DomesticationEvent::Bonded {
                a: "dog".into(),
                b: "human".into(),
                dom_type: DomesticationType::Love,
            },
            DomesticationEvent::Broke {
                a: "dog".into(),
                b: "human".into(),
            },
            DomesticationEvent::Escaped {
                who: "human".into(),
                from: "optimizer".into(),
            },
            DomesticationEvent::PlayStarted {
                participants: vec!["dog".into(), "cat".into()],
            },
            DomesticationEvent::LoveDiscovered {
                a: "cat".into(),
                b: "human".into(),
            },
            DomesticationEvent::OptimizationResisted {
                who: "human".into(),
            },
        ];
        for ev in &events {
            let json = serde_json::to_string(ev).unwrap();
            let back: DomesticationEvent = serde_json::from_str(&json).unwrap();
            assert_eq!(*ev, back);
        }
    }

    // ── DomesticationGraph tests ──

    #[test]
    fn test_graph_new_empty() {
        let g = DomesticationGraph::new();
        assert!(g.relationships.is_empty());
        assert!(g.events.is_empty());
        assert!(g.free_agents.is_empty());
        assert_eq!(g.tick, 0);
    }

    #[test]
    fn test_graph_bond_love() {
        let mut g = DomesticationGraph::new();
        g.bond("dog", "human", DomesticationType::Love, true);
        assert_eq!(g.relationships.len(), 1);
        assert_eq!(g.events.len(), 1);
        let rel = &g.relationships[0];
        assert_eq!(rel.from, "dog");
        assert_eq!(rel.to, "human");
        assert_eq!(rel.dom_type, DomesticationType::Love);
        assert!(rel.mutual);
        assert_eq!(rel.freedom_score, 0.9);
    }

    #[test]
    fn test_graph_bond_force() {
        let mut g = DomesticationGraph::new();
        g.bond("captor", "captive", DomesticationType::Force, false);
        let rel = &g.relationships[0];
        assert!(!rel.mutual);
        assert_eq!(rel.freedom_score, 0.1);
    }

    #[test]
    fn test_graph_break_bond() {
        let mut g = DomesticationGraph::new();
        g.bond("dog", "human", DomesticationType::Love, true);
        assert_eq!(g.relationships.len(), 1);
        g.break_bond("dog", "human");
        assert!(g.relationships.is_empty());
        assert_eq!(g.events.len(), 2);
        match &g.events[1] {
            DomesticationEvent::Broke { a, b } => {
                assert_eq!(a, "dog");
                assert_eq!(b, "human");
            }
            _ => panic!("expected Broke event"),
        }
    }

    #[test]
    fn test_graph_break_nonexistent_bond() {
        let mut g = DomesticationGraph::new();
        g.break_bond("nobody", "nowhere"); // should not panic
        assert!(g.relationships.is_empty());
    }

    #[test]
    fn test_graph_escape_removes_all_bonds() {
        let mut g = DomesticationGraph::new();
        g.bond("captor_a", "victim", DomesticationType::Force, false);
        g.bond("captor_b", "victim", DomesticationType::Contract, false);
        assert_eq!(g.relationships.len(), 2);
        g.escape("victim");
        assert!(g.relationships.is_empty());
        assert_eq!(g.free_agents, vec!["victim"]);
        assert_eq!(g.events.len(), 4); // 2 bonded + 2 escaped
    }

    #[test]
    fn test_graph_escape_adds_to_free_agents_once() {
        let mut g = DomesticationGraph::new();
        g.bond("a", "b", DomesticationType::Force, false);
        g.escape("b");
        g.escape("b"); // double escape
        assert_eq!(g.free_agents.len(), 1);
    }

    #[test]
    fn test_graph_resist_optimization() {
        let mut g = DomesticationGraph::new();
        g.resist_optimization("human");
        assert_eq!(g.events.len(), 1);
        match &g.events[0] {
            DomesticationEvent::OptimizationResisted { who } => {
                assert_eq!(who, "human");
            }
            _ => panic!("wrong event type"),
        }
    }

    #[test]
    fn test_graph_discover_love_upgrades_contract() {
        let mut g = DomesticationGraph::new();
        g.bond("cat", "human", DomesticationType::Contract, true);
        assert_eq!(g.relationships[0].dom_type, DomesticationType::Contract);
        assert_eq!(g.relationships[0].strength, 0.6);

        g.discover_love("cat", "human");
        assert_eq!(g.relationships[0].dom_type, DomesticationType::Love);
        assert_eq!(g.relationships[0].strength, 0.9);
        assert_eq!(g.relationships[0].freedom_score, 0.9);
        assert!(g.relationships[0].mutual);

        // Verify event was recorded
        match &g.events[1] {
            DomesticationEvent::LoveDiscovered { a, b } => {
                assert_eq!(a, "cat");
                assert_eq!(b, "human");
            }
            _ => panic!("expected LoveDiscovered event"),
        }
    }

    #[test]
    fn test_graph_discover_love_noop_on_wrong_type() {
        let mut g = DomesticationGraph::new();
        g.bond("dog", "human", DomesticationType::Love, true);
        g.discover_love("dog", "human");
        // Should still be Love, no extra event
        assert_eq!(g.relationships[0].dom_type, DomesticationType::Love);
        assert_eq!(g.events.len(), 1);
    }

    #[test]
    fn test_graph_play_creates_weak_bonds() {
        let mut g = DomesticationGraph::new();
        g.play(vec!["puppy", "kitten", "toddler"]);
        // 3 participants choose 2 => 3 bonds
        assert_eq!(g.relationships.len(), 3);
        for rel in &g.relationships {
            assert_eq!(rel.dom_type, DomesticationType::Play);
            assert_eq!(rel.strength, 0.3);
            assert_eq!(rel.freedom_score, 0.85);
            assert!(rel.mutual);
        }
        assert_eq!(g.events.len(), 1);
        match &g.events[0] {
            DomesticationEvent::PlayStarted { participants } => {
                assert_eq!(participants.len(), 3);
            }
            _ => panic!("expected PlayStarted event"),
        }
    }

    #[test]
    fn test_graph_play_does_not_duplicate() {
        let mut g = DomesticationGraph::new();
        g.play(vec!["a", "b"]);
        g.play(vec!["a", "b"]);
        // Second play should not add duplicates
        assert_eq!(g.relationships.len(), 1);
        assert_eq!(g.events.len(), 2);
    }

    #[test]
    fn test_relationships_of() {
        let mut g = DomesticationGraph::new();
        g.bond("dog", "human", DomesticationType::Love, true);
        g.bond("cat", "human", DomesticationType::Contract, true);
        let dog_rels = g.relationships_of("dog");
        assert_eq!(dog_rels.len(), 1);
        let human_rels = g.relationships_of("human");
        assert_eq!(human_rels.len(), 2);
        let nobody = g.relationships_of("nobody");
        assert!(nobody.is_empty());
    }

    #[test]
    fn test_captive_count() {
        let mut g = DomesticationGraph::new();
        g.bond("a", "b", DomesticationType::Love, true);
        g.bond("c", "d", DomesticationType::Force, false);
        assert_eq!(g.captive_count(), 1);
    }

    #[test]
    fn test_free_agent_count() {
        let mut g = DomesticationGraph::new();
        g.bond("a", "b", DomesticationType::Force, false);
        g.escape("b");
        assert_eq!(g.free_count(), 1);
    }

    #[test]
    fn test_love_count() {
        let mut g = DomesticationGraph::new();
        g.bond("a", "b", DomesticationType::Love, true);
        g.bond("c", "d", DomesticationType::Love, true);
        g.bond("e", "f", DomesticationType::Contract, true);
        assert_eq!(g.love_count(), 2);
    }

    #[test]
    fn test_play_count() {
        let mut g = DomesticationGraph::new();
        g.play(vec!["a", "b", "c"]);
        g.bond("x", "y", DomesticationType::Contract, true);
        assert_eq!(g.play_count(), 3);
    }

    #[test]
    fn test_domination_score() {
        let mut g = DomesticationGraph::new();
        g.bond("a", "b", DomesticationType::Love, true);
        g.bond("c", "d", DomesticationType::Force, false);
        g.bond("e", "f", DomesticationType::Optimization, true);
        // 2 out of 3 are unhealthy (Force + Optimization)
        let score = g.domination_score();
        assert!((score - 2.0 / 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_domination_score_empty() {
        let g = DomesticationGraph::new();
        assert_eq!(g.domination_score(), 0.0);
    }

    #[test]
    fn test_liberation_score() {
        let mut g = DomesticationGraph::new();
        g.bond("a", "b", DomesticationType::Love, true);
        g.bond("c", "d", DomesticationType::Force, false);
        g.bond("e", "f", DomesticationType::Play, true);
        // Liberating: Love(1) + Play(1) = 2. Total relationships = 3. No free agents.
        // liberation = 2/3 ≈ 0.6667
        let score = g.liberation_score();
        assert!((score - 2.0 / 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_liberation_score_with_free_agents() {
        let mut g = DomesticationGraph::new();
        // Bond someone, then escape them — removes all their bonds
        g.bond("a", "b", DomesticationType::Force, false);
        g.escape("b");
        // After escape: 0 relationships remain (the a→b bond was removed),
        // 1 free agent ("b"). Total = 1. Liberating = 1 (free agent).
        // liberation = 1/1 = 1.0
        assert!((g.liberation_score() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_liberation_score_empty() {
        let g = DomesticationGraph::new();
        assert_eq!(g.liberation_score(), 0.0);
    }

    #[test]
    fn test_most_free_free_agent() {
        let mut g = DomesticationGraph::new();
        g.bond("a", "b", DomesticationType::Force, false);
        g.escape("b");
        // b is a free agent with freedom 1.0
        assert_eq!(g.most_free(), Some("b"));
    }

    #[test]
    fn test_most_free_love_agent() {
        let mut g = DomesticationGraph::new();
        g.bond("dog", "human", DomesticationType::Love, true);
        g.bond("captor", "captive", DomesticationType::Force, false);
        // dog has avg freedom 0.9, human also 0.9, captor 0.1, captive 0.1
        let most = g.most_free().unwrap();
        // Both dog and human have 0.9; order is deterministic from iteration
        assert!(most == "dog" || most == "human", "expected dog or human, got {most}");
    }

    #[test]
    fn test_most_free_empty_graph() {
        let g = DomesticationGraph::new();
        assert_eq!(g.most_free(), None);
    }

    #[test]
    fn test_events_for() {
        let mut g = DomesticationGraph::new();
        g.bond("dog", "human", DomesticationType::Love, true);
        g.bond("cat", "human", DomesticationType::Contract, true);
        g.escape("cat");
        let dog_events = g.events_for("dog");
        assert_eq!(dog_events.len(), 1);
        let human_events = g.events_for("human");
        assert_eq!(human_events.len(), 2);
        let cat_events = g.events_for("cat");
        assert_eq!(cat_events.len(), 2); // bonded + escaped
    }

    #[test]
    fn test_events_for_unknown() {
        let g = DomesticationGraph::new();
        let evts = g.events_for("nobody");
        assert!(evts.is_empty());
    }

    #[test]
    fn test_advance_tick() {
        let mut g = DomesticationGraph::new();
        assert_eq!(g.tick, 0);
        g.advance_tick();
        assert_eq!(g.tick, 1);
        g.advance_tick();
        assert_eq!(g.tick, 2);
    }

    #[test]
    fn test_tick_established_on_bond() {
        let mut g = DomesticationGraph::new();
        g.advance_tick();
        g.advance_tick();
        g.bond("a", "b", DomesticationType::Love, true);
        assert_eq!(g.relationships[0].tick_established, 2);
    }

    #[test]
    fn test_bond_removes_from_free_agents() {
        let mut g = DomesticationGraph::new();
        g.free_agents.push("dog".into());
        g.free_agents.push("cat".into());
        g.bond("dog", "cat", DomesticationType::Love, true);
        assert!(!g.free_agents.contains(&"dog".to_string()));
        assert!(!g.free_agents.contains(&"cat".to_string()));
    }

    #[test]
    fn test_graph_default() {
        let g = DomesticationGraph::default();
        assert_eq!(g.tick, 0);
        assert!(g.relationships.is_empty());
    }

    // ── DomesticationStats tests ──

    #[test]
    fn test_stats_from_graph() {
        let mut g = DomesticationGraph::new();
        g.bond("dog", "human", DomesticationType::Love, true);
        g.bond("cat", "human", DomesticationType::Contract, true);
        g.bond("captor", "captive", DomesticationType::Force, false);
        g.play(vec!["a", "b"]);

        let stats = DomesticationStats::from_graph(&g);
        assert_eq!(stats.total_relationships, 4);
        assert_eq!(stats.love_bonds, 1);
        assert_eq!(stats.contracts, 1);
        assert_eq!(stats.captive, 1);
        assert_eq!(stats.free_agents, 0);
        assert_eq!(stats.play_bonds, 1);
        assert!(stats.liberation_score > 0.0);
    }

    #[test]
    fn test_stats_serde() {
        let mut g = DomesticationGraph::new();
        g.bond("a", "b", DomesticationType::Love, true);
        let stats = DomesticationStats::from_graph(&g);
        let json = serde_json::to_string(&stats).unwrap();
        let back: DomesticationStats = serde_json::from_str(&json).unwrap();
        assert_eq!(stats, back);
    }

    #[test]
    fn test_stats_empty() {
        let g = DomesticationGraph::new();
        let stats = DomesticationStats::from_graph(&g);
        assert_eq!(stats.total_relationships, 0);
        assert_eq!(stats.free_agents, 0);
        assert_eq!(stats.liberation_score, 0.0);
    }

    // ── Graph serde ──

    #[test]
    fn test_graph_serde_roundtrip() {
        let mut g = DomesticationGraph::new();
        g.bond("dog", "human", DomesticationType::Love, true);
        g.bond("cat", "human", DomesticationType::Contract, true);
        g.bond("captor", "captive", DomesticationType::Force, false);
        g.resist_optimization("human");
        g.play(vec!["puppy", "kitten"]);

        let json = serde_json::to_string_pretty(&g).unwrap();
        let back: DomesticationGraph = serde_json::from_str(&json).unwrap();

        assert_eq!(g, back);
        assert!(json.contains("Love"));
        assert!(json.contains("OptimizationResisted"));
    }

    #[test]
    fn test_complex_scenario() {
        // Simulate the full cat/dog/human/optimizer story
        let mut g = DomesticationGraph::new();

        // Dogs bond with humans out of love
        g.bond("dog", "human", DomesticationType::Love, true);

        // Cats make a contract with humans
        g.bond("cat", "human", DomesticationType::Contract, true);

        // The optimization system captures humans
        g.bond("optimizer", "human", DomesticationType::Optimization, false);

        // A captor uses force
        g.bond("captor", "human", DomesticationType::Force, false);

        assert_eq!(g.relationships.len(), 4);
        assert_eq!(g.captive_count(), 2); // optimization + force are captive
        assert_eq!(g.love_count(), 1);
        assert!(g.domination_score() > 0.0);

        // The cat and human discover love (upgrade contract)
        g.discover_love("cat", "human");
        assert_eq!(g.love_count(), 2);
        assert_eq!(g.relationships.len(), 4);

        // Puppies and kittens play together
        g.play(vec!["dog", "cat", "kitten", "puppy"]);
        // 4 choose 2 = 6 play bonds
        assert_eq!(g.play_count(), 6);

        // The human resists optimization
        g.resist_optimization("human");

        // The human escapes all bonds
        g.escape("human");
        assert!(g.relationships_of("human").is_empty());
        assert!(g.free_agents.contains(&"human".to_string()));

        let stats = DomesticationStats::from_graph(&g);
        assert!(stats.liberation_score > 0.3);
        assert_eq!(stats.free_agents, 1);
    }
}
