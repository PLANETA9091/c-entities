//! centities-native — the L110 architecture pivot: ZERO Java in the entity
//! tick.
//!
//! The study of Horizon (Minecraft 26.3 rewritten in Rust, doc 42) showed the
//! winning server-side entity shape: a plain AoS `Vec<Mob>` in insertion
//! order, a per-tick flattened snapshot for cross-mob reads, a uniform 4-block
//! XZ grid for spatial candidates, per-mob flat attribute fields instead of
//! any map, and side effects deferred into an event outbox drained once per
//! tick by the host. This crate ports that shape onto the parity-proven
//! decision layers we already own (`centities_core::goals` — the bit-exact
//! goal-selector floor — and `centities_core::mobtravel` / `pushcore`).
//!
//! The layers, bottom-up:
//! 1. [`store`] — `MobStore`: persistent AoS registry (`Vec<MobSlot>` +
//!    id→slot map), vanilla insertion order, plus `SeenGrid` (the Horizon
//!    SeenGrid port, 4.0-block XZ cells) and a `PushCore` mirror.
//! 2. [`world`] — `WorldRes`: the world seam as a `Resolver` (swarm-sim's
//!    protocol). Player targeting / lookat / LoS resolve against the REAL
//!    player snapshot; block-safety walks go through the `BlockFacts` seam
//!    (v0: the open-ground contract, the seam is where the section-copy
//!    protocol plugs in — doc 42 §6).
//! 3. [`exec`] — `NativeTick`: one call ticks the WHOLE managed population:
//!    AI floor (the bit-exact selector walk) → travel slice → apply-back →
//!    grid refresh → events outbox. The host (Java) shrinks to a copy layer:
//!    per tick it pushes diffs (add/remove/players) and reads back positions
//!    + events — no entity logic in Java at all.
//! 4. [`profile`] — the Rust profiler the pivot demands: Horizon-style
//!    thread-local phase sampling (1-in-N, keyed by phase) + a per-function
//!    self-time path via `perf` (scripts/rust_perf_top.py + the
//!    native-profile CI job). Every hotspot report now points at RUST
//!    functions to optimize, not Java frames behind JNI.
//!
//! Honesty ledger (doc 42 §8): the AI floor and travel slice are the
//! parity-proven machinery (CI run #369 and the L37-L109 gate history); the
//! world seam v0 resolves stroll targets and LoS under the open-ground
//! contract the swarm bench used — real block reads are the N1 phase and
//! they change WHERE mobs walk, never the tick cost structure. The crate
//! states this in its API (`WorldRes::open_ground_v0` is a named constructor).
pub mod exec;
pub mod facts;
pub mod profile;
pub mod store;
pub mod world;

pub use exec::NativeTick;
pub use profile::{drain, Phases};
pub use store::{MobSlot, MobStore, SeenGrid};
pub use facts::SectionFacts;
pub use world::{BlockFacts, OpenGround, WorldRes};
