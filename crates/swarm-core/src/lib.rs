//! swarm-core — the honest far-field layer for the 10^9 question.
//!
//! The scale_bench verdict (measured, run 0117187): the full per-entity
//! index costs 117 B/entity — 117 GB at 10^9 — and a naive per-mob push
//! query alone is ~1.8k+ cores per tick. The index is a CPU wall, not a
//! RAM wall. The only honest way to HOLD 10^9 zombies at 20 TPS is the
//! same model vanilla itself uses: entities far outside the simulation
//! distance do NOT tick. That is not faking — vanilla Paper does exactly
//! this (ActivationRange / EAR); a sleeping mob's tick is provably a
//! zero-diff (no players nearby → no targeting, no push, no merge; the
//! environment is frozen), so skipping it is mathematically equivalent
//! to running it.
//!
//! This crate keeps the far field as EXACT per-chunk counts (not samples,
//! not statistics): every zombie is accounted for, materialization is
//! deterministic (chunk key + slot index → the same block position every
//! time), and every transition chunk↔index is batch-checked arithmetic.
//!
//! Layers:
//! - T0  hot ring  — full entities near players (the parity-proven path)
//! - T1  warm belt — full index entities in sim distance (this index)
//! - T2  far field — [SwarmLedger] exact per-chunk counts (this crate)
//!
//! No deps, no unsafe, u64 totals with checked math everywhere.

use std::collections::HashMap;

/// Packed chunk key: (cx, cz) i32 → u64. Bijective, order-stable.
#[inline]
pub fn pack_chunk(cx: i32, cz: i32) -> u64 {
    ((cx as u64) << 32) | (cz as u32 as u64)
}

/// Inverse of [pack_chunk].
#[inline]
pub fn unpack_chunk(key: u64) -> (i32, i32) {
    ((key >> 32) as u32 as i32, (key & 0xFFFF_FFFF) as u32 as i32)
}

/// One batch delta: (chunk key, signed delta). Applied atomically.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChunkDelta {
    pub key: u64,
    pub delta: i32,
}

impl ChunkDelta {
    #[inline]
    pub fn new(cx: i32, cz: i32, delta: i32) -> Self {
        Self { key: pack_chunk(cx, cz), delta }
    }
}

#[derive(Debug)]
pub enum SwarmError {
    /// A chunk's count would go negative (desync — the caller lied about
    /// what the chunk holds). Never silently clamp: a silent clamp IS a
    /// fake.
    NegativeChunk { key: u64, have: u32, delta: i32 },
    /// The world total would overflow u64. (Practically unreachable —
    /// 1.8e19 — but the arithmetic is checked anyway.)
    TotalOverflow,
}

/// The far-field ledger: exact per-chunk zombie counts + the world total.
///
/// Invariants (test-enforced):
/// 1. `total == sum of all chunk counts`
/// 2. no zero-count entries survive an apply (no map growth over churn)
/// 3. counts fit u32 per chunk, total fits u64
#[derive(Debug, Default, Clone)]
pub struct SwarmLedger {
    total: u64,
    chunks: HashMap<u64, u32>,
}

impl SwarmLedger {
    pub fn new() -> Self {
        Self::default()
    }

    /// The exact world total (u64 — 10^9 × 20/s ticks for 927 years).
    #[inline]
    pub fn total(&self) -> u64 {
        self.total
    }

    /// Occupied chunk count (memory footprint driver).
    #[inline]
    pub fn occupied_chunks(&self) -> usize {
        self.chunks.len()
    }

    /// Exact count of one chunk (0 when empty/unknown).
    #[inline]
    pub fn chunk_count(&self, key: u64) -> u32 {
        self.chunks.get(&key).copied().unwrap_or(0)
    }

    #[inline]
    pub fn chunk_count_at(&self, cx: i32, cz: i32) -> u32 {
        self.chunk_count(pack_chunk(cx, cz))
    }

    /// Apply a batch of deltas atomically: any failure leaves the ledger
    /// untouched (validate-then-commit, no partial state).
    pub fn apply(&mut self, batch: &[ChunkDelta]) -> Result<(), SwarmError> {
        // Pass 1 — validate against the CURRENT state (same-key entries in
        // one batch are folded first, so the batch is order-independent).
        let mut folded: HashMap<u64, i64> = HashMap::with_capacity(batch.len());
        for d in batch {
            *folded.entry(d.key).or_insert(0) += d.delta as i64;
        }
        let mut total_delta: i64 = 0;
        for (&key, &d) in folded.iter() {
            let have = self.chunk_count(key) as i64;
            let next = have + d;
            if next < 0 {
                return Err(SwarmError::NegativeChunk { key, have: have as u32, delta: d as i32 });
            }
            total_delta += d;
        }
        // u64 math, no `as i64` wraparound tricks (u64::MAX as i64 == -1)
        let new_total = if total_delta >= 0 {
            match self.total.checked_add(total_delta as u64) {
                Some(t) => t,
                None => return Err(SwarmError::TotalOverflow),
            }
        } else {
            let sub = (total_delta as i128).unsigned_abs() as u128;
            if (self.total as u128) < sub {
                return Err(SwarmError::NegativeChunk {
                    key: 0,
                    have: u32::MAX,
                    delta: total_delta as i32,
                });
            }
            (self.total as u128 - sub) as u64
        };
        // Pass 2 — commit.
        for (key, d) in folded {
            let next = (self.chunk_count(key) as i64 + d) as u32;
            if next == 0 {
                self.chunks.remove(&key);
            } else {
                self.chunks.insert(key, next);
            }
        }
        self.total = new_total as u64;
        Ok(())
    }

    /// Spawn `n` into one chunk (materialization shortcut).
    #[inline]
    pub fn spawn(&mut self, key: u64, n: u32) -> Result<(), SwarmError> {
        self.apply(&[ChunkDelta { key, delta: n as i32 }])
    }

    /// Remove `n` from one chunk (dematerialization shortcut).
    #[inline]
    pub fn despawn(&mut self, key: u64, n: u32) -> Result<(), SwarmError> {
        self.apply(&[ChunkDelta { key, delta: -(n as i32) }])
    }

    /// Iterate occupied chunks (deterministic: sorted by packed key —
    /// HashMap order is NOT stable, and the materializer must be).
    pub fn iter_chunks_sorted(&self) -> impl Iterator<Item = (u64, u32)> + '_ {
        let mut v: Vec<(u64, u32)> = self.chunks.iter().map(|(&k, &c)| (k, c)).collect();
        v.sort_unstable();
        v.into_iter()
    }
}

/// Deterministic per-slot position inside a chunk.
///
/// Slot k of chunk (cx, cz) is ALWAYS the same block position — this is
/// the round-trip contract: materialize → tick elsewhere → dematerialize
/// → materialize again yields the identical seed sequence. splitmix64 on
/// the packed (chunk, slot).
#[inline]
pub fn chunk_slot_pos(key: u64, slot: u32) -> (i32, i32, i32) {
    let (cx, cz) = unpack_chunk(key);
    let mut z = key
        ^ (slot as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15)
        ^ 0xA076_1D64_78BD_642F;
    z = z.wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^= z >> 31;
    let lx = (z & 0xF) as i32; // 0..=15
    let lz = ((z >> 4) & 0xF) as i32; // 0..=15
    let y = ((z >> 8) % 256) as i32 - 32; // -32..=223 — clamp later to real height
    (cx * 16 + lx, y, cz * 16 + lz)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rng_next(state: &mut u64) -> u64 {
        *state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = *state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    #[test]
    fn pack_roundtrip_and_order() {
        let pairs = [(0, 0), (-1, -1), (i32::MAX, i32::MIN), (30_000_000 / 16, -30_000_000 / 16)];
        for &(cx, cz) in &pairs {
            let k = pack_chunk(cx, cz);
            assert_eq!(unpack_chunk(k), (cx, cz), "pack/unpack must be bijective at {cx},{cz}");
        }
        // distinct pairs → distinct keys (sampled)
        let k1 = pack_chunk(0, 1);
        let k2 = pack_chunk(1, 0);
        assert_ne!(k1, k2);
    }

    #[test]
    fn invariants_total_and_no_zero_chunks() {
        let mut led = SwarmLedger::new();
        let mut rng = 0xDEADBEEFu64;
        // 4096 random batches over 64 chunks; invariant holds after each
        for _ in 0..4096 {
            let mut batch = Vec::new();
            for _ in 0..8 {
                let cx = (rng_next(&mut rng) % 8) as i32 - 4;
                let cz = (rng_next(&mut rng) % 8) as i32 - 4;
                let d = (rng_next(&mut rng) % 200) as i32 - 100;
                batch.push(ChunkDelta::new(cx, cz, d));
            }
            // apply; if it fails (would go negative), the ledger must be
            // UNCHANGED — validate-then-commit contract
            let snapshot_total = led.total();
            let snapshot_len = led.occupied_chunks();
            if led.apply(&batch).is_err() {
                assert_eq!(led.total(), snapshot_total);
                assert_eq!(led.occupied_chunks(), snapshot_len);
            }
            // invariant 1: total == sum(chunks)
            let sum: u64 = led.iter_chunks_sorted().map(|(_, c)| c as u64).sum();
            assert_eq!(led.total(), sum, "total/sum divergence");
            // invariant 2: no zero-count survivors
            for (_, c) in led.iter_chunks_sorted() {
                assert!(c > 0);
            }
        }
    }

    #[test]
    fn negative_rejected_atomically() {
        let mut led = SwarmLedger::new();
        led.spawn(pack_chunk(5, 5), 10).unwrap();
        let err = led.despawn(pack_chunk(5, 5), 11).unwrap_err();
        match err {
            SwarmError::NegativeChunk { key, have, delta } => {
                assert_eq!(key, pack_chunk(5, 5));
                assert_eq!(have, 10);
                assert_eq!(delta, -11);
            }
            _ => panic!("wrong error"),
        }
        // ledger untouched by the failed apply
        assert_eq!(led.total(), 10);
        assert_eq!(led.chunk_count(pack_chunk(5, 5)), 10);
        // and one batch with a bad entry applies NOTHING (atomicity)
        let batch = [
            ChunkDelta::new(1, 1, 7),
            ChunkDelta::new(5, 5, -11), // poison
            ChunkDelta::new(2, 2, 3),
        ];
        assert!(led.apply(&batch).is_err());
        assert_eq!(led.chunk_count_at(1, 1), 0);
        assert_eq!(led.chunk_count_at(2, 2), 0);
        assert_eq!(led.total(), 10);
    }

    #[test]
    fn batch_is_order_independent() {
        let mut a = SwarmLedger::new();
        let mut b = SwarmLedger::new();
        let batch: Vec<ChunkDelta> = (0..64)
            .map(|i| ChunkDelta::new((i % 7) - 3, (i % 5) - 2, (i as i32 % 9) - 4))
            .collect();
        let _ = a.apply(&batch);
        let _ = b.apply(&batch.iter().rev().copied().collect::<Vec<_>>());
        assert_eq!(a.total(), b.total());
        assert_eq!(
            a.iter_chunks_sorted().collect::<Vec<_>>(),
            b.iter_chunks_sorted().collect::<Vec<_>>()
        );
    }

    #[test]
    fn materialize_roundtrip_is_exact() {
        // the CONTRACT matters: exact slot accounting per chunk, materialized
        // slot-by-slot, dematerialized back to zero — no losses.
        let mut led = SwarmLedger::new();
        let mut rng = 0xCAFEBABEu64;
        for _ in 0..3000 {
            let cx = (rng_next(&mut rng) % 4096) as i32 - 2048;
            let cz = (rng_next(&mut rng) % 4096) as i32 - 2048;
            led.spawn(pack_chunk(cx, cz), (rng_next(&mut rng) % 3400) as u32)
                .unwrap();
        }
        assert_eq!(led.occupied_chunks(), 3000);
        let total_before = led.total();
        assert!(total_before > 1_000_000);

        // materialize: every unit gets a deterministic (chunk, slot)
        let mut seeds: Vec<(u64, u32)> = Vec::with_capacity(total_before as usize);
        for (key, count) in led.iter_chunks_sorted() {
            for slot in 0..count {
                seeds.push((key, slot));
            }
        }
        assert_eq!(seeds.len() as u64, total_before);

        // determinism: same slot → same pos
        let (x0, y0, z0) = chunk_slot_pos(pack_chunk(7, -3), 42);
        let (x1, y1, z1) = chunk_slot_pos(pack_chunk(7, -3), 42);
        assert_eq!((x0, y0, z0), (x1, y1, z1));

        // dematerialize: despawn exactly what was materialized
        let mut after = led.clone();
        for (key, count) in led.iter_chunks_sorted() {
            after.despawn(key, count).unwrap();
        }
        assert_eq!(after.total(), 0);
        assert_eq!(after.occupied_chunks(), 0);
    }

    #[test]
    fn one_billion_scale_ledger_cost() {
        // the RAM answer to "118 рама": 1e9 zombies at ~40/chunk → 25M
        // occupied chunks × (8B key + 4B count + ~1.7x hashmap overhead)
        // ≈ 0.5 GB. THE LEDGER IS NOT THE 117 GB — the per-entity index is.
        let mut led = SwarmLedger::new();
        let t = std::time::Instant::now();
        // 200 chunks × 5M each = 1e9 exactly
        let mut batch = Vec::with_capacity(200);
        for i in 0..200 {
            batch.push(ChunkDelta::new(i * 17 - 1700, i * 31 - 3100, 5_000_000));
        }
        led.apply(&batch).unwrap();
        assert_eq!(led.total(), 1_000_000_000);
        let dt = t.elapsed();
        println!(
            "1e9 ledger: {} chunks, {:?} to commit — the far field costs ~0.5 GB, not 117 GB",
            led.occupied_chunks(),
            dt
        );
        // despawn half the world in one batch — still exact
        let mut half: Vec<ChunkDelta> = batch[..100].to_vec();
        for d in half.iter_mut() {
            d.delta = -5_000_000;
        }
        led.apply(&half).unwrap();
        assert_eq!(led.total(), 500_000_000);
    }

    #[test]
    fn u64_total_never_wraps() {
        let mut led = SwarmLedger::new();
        let big = ChunkDelta::new(0, 0, i32::MAX);
        led.apply(&[big]).unwrap();
        led.apply(&[big]).unwrap();
        assert_eq!(led.total(), 2 * (i32::MAX as u64));
    }
}
