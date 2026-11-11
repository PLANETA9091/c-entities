//! N3-N1 (doc 61): the native despawn decision — the EXACT port of the
//! Paper 26.3 `Mob.checkDespawn` geometry and gates.
//!
//! Ground truth: javap of the pinned paper-26.3-218 jar
//! (`io.papermc.paper.configuration.type.DespawnRange`). The decision is
//! a pure per-mob function of (config, players, mob state) plus ONE RNG
//! draw on the soft path; the draw itself stays with the caller (the
//! executor owns the SHARED_RANDOM stream order — the wave13/14 prefix
//! assignment), so this module is total and testable without a stream.
//!
//! The virtuals (`requiresCustomPersistence()`, `removeWhenFarAway()`)
//! are caller-resolved (the class-cache design in doc 61 §3) and arrive
//! as plain bools here.

/// One Paper despawn range: the precomputed geometry of
/// `DespawnRange.preComputed(vanillaDefault, path)`.
#[derive(Clone, Copy, Debug)]
pub struct DespawnRange {
    /// horizontalLimit^2 (the config value or the vanilla default).
    pub h2: f64,
    /// horizontalLimit^2 / verticalLimit^2 — 1.0 when NEITHER limit was
    /// defined in the config (the vanilla-ellipsoid identity).
    pub ratio: f64,
    /// the vanilla default vertical limit for this category/range.
    pub v_default: i32,
    /// verticalLimit.or(v_default) — resolved by the config push.
    pub v: i32,
}

impl DespawnRange {
    /// `DespawnRange.shouldDespawn(shape, dx2, dy2, dz2, dy)` — the
    /// bytecode is ground truth:
    ///   ELLIPSOID: `dx2 + dz2 + dy2 * ratio > h2`  (dy2 = dy*dy, dy = |Δy|)
    ///   CYLINDER:  `dx2 + dz2 > h2 || dy > v`      (dy = |Δy|, SIGNED compare
    ///               on a non-negative value — symmetric by the abs at the
    ///               call site, bc 146)
    #[inline]
    pub fn should_despawn(&self, ellipsoid: bool, dx2: f64, dy2: f64, dz2: f64, dy: f64) -> bool {
        if ellipsoid {
            dx2 + dz2 + dy2 * self.ratio > self.h2
        } else {
            dx2 + dz2 > self.h2 || dy > self.v as f64
        }
    }
}

/// The hard+soft pair for one MobCategory.
#[derive(Clone, Copy, Debug)]
pub struct DespawnRangePair {
    pub hard: DespawnRange,
    pub soft: DespawnRange,
}

/// The per-level despawn config, pushed once at wire arm (the wave13
/// shared-cfg-table pattern). Indexed by the MobCategory ordinal — the
/// Java side reads `despawnRanges.get(category)` and serializes the
/// resolved numbers; `pairs[cat]` is `None` for a category with no
/// configured pair (vanilla fills every category the config defines;
/// a missing pair means the vanilla map.get returned null — the vanilla
/// body would NPE, so the config push REJECTS the arm instead).
#[derive(Clone, Debug)]
pub struct DespawnCfg {
    pub ellipsoid: bool,
    pub pairs: [Option<DespawnRangePair>; 8],
}

impl DespawnCfg {
    pub fn pair(&self, category: usize) -> Option<&DespawnRangePair> {
        self.pairs.get(category).and_then(|p| p.as_ref())
    }
}

/// The decision outcome (the outbox event; NONE leaves the mob alone).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DespawnEvent {
    /// PEACEFUL difficulty + the type is not allowed in peaceful (or the
    /// per-mob override says so) — the vanilla body's first branch.
    Peaceful,
    /// outside the hard range (+ removeWhenFarAway) — the instant despawn.
    Hard,
    /// outside the soft range with noActionTime > 600 and the 1/800 draw.
    Soft,
}

/// The pure decision — the faithful `checkDespawn` tail after the
/// findNearbyPlayer gate. The caller guarantees:
///   - `nearest` is the PLAYER_AFFECTS_SPAWNING nearest player (the wire
///     pushes the exact predicate bit; vanilla returns early when none),
///   - `soft_draw` is the `random.nextInt(800)` result drawn IN THE
///     VANILLA ORDER (the walk-order position, before the mob's AI
///     draws) — pass 0 when the caller knows the draw is skipped? NO:
///     the caller must draw IFF `needs_draw` says so — peek `needs_draw`
///     first, draw, then call. The function never invents a draw.
///
/// `no_action` is the mob's current noActionTime; on the in-soft-range
/// miss the caller must apply the `reset_no_action` side effect (the
/// vanilla `noActionTime = 0`), mirrored back to Java by the applier.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DespawnOutcome {
    pub event: Option<DespawnEvent>,
    /// the vanilla `noActionTime = 0` else-branch fired (the player is
    /// INSIDE the soft range): the executor zeroes its mirror and the
    /// applier writes the Java field.
    pub reset_no_action: bool,
}

impl DespawnCfg {
    pub fn decide(
        &self,
        category: usize,
        nearest: Option<(f64, f64, f64)>,
        mob: (f64, f64, f64),
        persistent: bool,
        no_action: i32,
        remove_when_far: bool,
        soft_draw: i32,
    ) -> DespawnOutcome {
        let Some((px, py, pz)) = nearest else {
            return DespawnOutcome { event: None, reset_no_action: false };
        };
        let Some(pair) = self.pair(category) else {
            // the vanilla body NPEs on a missing pair (map.get == null);
            // the config push refuses the arm — this arm is unreachable.
            return DespawnOutcome { event: None, reset_no_action: false };
        };
        let dx = px - mob.0;
        let dy_raw = py - mob.1;
        let dz = pz - mob.2;
        let dy = dy_raw.abs();
        let dy2 = dy * dy;
        let dx2 = dx * dx;
        let dz2 = dz * dz;
        let dist2 = dx2 + dz2 + dy2;
        if !persistent
            && pair
                .hard
                .should_despawn(self.ellipsoid, dx2, dy2, dz2, dy)
            && remove_when_far
        {
            return DespawnOutcome { event: Some(DespawnEvent::Hard), reset_no_action: false };
        }
        if pair.soft.should_despawn(self.ellipsoid, dx2, dy2, dz2, dy) {
            if !persistent
                && no_action > 600
                && soft_draw == 0
                && remove_when_far
            {
                return DespawnOutcome { event: Some(DespawnEvent::Soft), reset_no_action: false };
            }
            DespawnOutcome { event: None, reset_no_action: false }
        } else {
            DespawnOutcome { event: None, reset_no_action: true }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn range(h: i32, v: i32, v_defined: bool, h_defined: bool, v_default: i32) -> DespawnRange {
        let h2 = (h as f64) * (h as f64);
        let ratio = if h_defined || v_defined {
            h2 / ((v as f64) * (v as f64))
        } else {
            1.0
        };
        DespawnRange { h2, ratio, v_default, v: if v_defined { v } else { v_default } }
    }

    /// The MONSTER defaults as the config resolves them on a stock 26.3
    /// server: hard 128/128, soft 32/56 (the Paper defaults; the config
    /// push reads the RESOLVED values — these numbers are for the tests).
    fn monster_cfg(ellipsoid: bool) -> DespawnCfg {
        DespawnCfg {
            ellipsoid,
            pairs: [
                Some(DespawnRangePair {
                    hard: range(128, 128, true, true, 128),
                    soft: range(32, 56, true, true, 32),
                }),
                None, None, None, None, None, None, None,
            ],
        }
    }

    #[test]
    fn cylinder_hard_edge_is_exact() {
        let cfg = monster_cfg(false);
        // 128 blocks horizontal: dx2+dz2 = 16384.0 == h2 -> NOT > h2 -> alive.
        // Outside the SOFT range too (128 > 32) -> the soft-if body runs
        // (noActionTime 0 <= 600: no roll, no despawn) — no reset.
        let o = cfg.decide(0, Some((128.0, 0.0, 0.0)), (0.0, 0.0, 0.0), false, 0, true, 1);
        assert_eq!(o.event, None);
        assert!(!o.reset_no_action);
        // 128.0001: strictly outside -> hard despawn
        let o = cfg.decide(0, Some((128.0001, 0.0, 0.0)), (0.0, 0.0, 0.0), false, 0, true, 1);
        assert_eq!(o.event, Some(DespawnEvent::Hard));
        // vertical: |dy| = 128 == v -> alive; 128.5 -> hard (the CYLINDER
        // vertical gate uses the SIGNED compare on the abs'ed dy)
        let o = cfg.decide(0, Some((0.0, 128.0, 0.0)), (0.0, 0.0, 0.0), false, 0, true, 1);
        assert_eq!(o.event, None);
        let o = cfg.decide(0, Some((0.0, -128.5, 0.0)), (0.0, 0.0, 0.0), false, 0, true, 1);
        assert_eq!(o.event, Some(DespawnEvent::Hard), "the abs at the call site makes it symmetric");
    }

    #[test]
    fn soft_needs_no_action_and_the_draw() {
        let cfg = monster_cfg(false);
        // 100 blocks out: past the soft 32 (soft-if), inside the hard 128
        let far = Some((100.0, 0.0, 0.0));
        // noActionTime 600 is NOT > 600 -> alive, no reset (outside soft)
        let o = cfg.decide(0, far, (0.0, 0.0, 0.0), false, 600, true, 0);
        assert_eq!(o.event, None);
        assert!(!o.reset_no_action);
        // 601 + draw 0 -> soft despawn
        let o = cfg.decide(0, far, (0.0, 0.0, 0.0), false, 601, true, 0);
        assert_eq!(o.event, Some(DespawnEvent::Soft));
        // 601 + draw 1 -> alive
        let o = cfg.decide(0, far, (0.0, 0.0, 0.0), false, 601, true, 1);
        assert_eq!(o.event, None);
        // the soft range's VERTICAL gate: dy = 60 > soft v 56 -> the soft-if
        // body with a qualifying noActionTime + draw -> soft despawn
        let o = cfg.decide(0, Some((0.0, -60.0, 0.0)), (0.0, 0.0, 0.0), false, 9000, true, 0);
        assert_eq!(o.event, Some(DespawnEvent::Soft));
        // INSIDE the soft range (20 blocks): the ELSE — the reset fires for
        // persistent mobs too (the reset is the soft-IF's else, not gated)
        let o = cfg.decide(0, Some((20.0, 0.0, 0.0)), (0.0, 0.0, 0.0), true, 9000, true, 0);
        assert_eq!(o.event, None);
        assert!(o.reset_no_action);
        // persistent + far (soft range hit): no reset, no despawn
        let o = cfg.decide(0, far, (0.0, 0.0, 0.0), true, 9000, true, 0);
        assert_eq!(o.event, None);
        assert!(!o.reset_no_action);
        // removeWhenFarAway=false: hard and soft both decline
        let o = cfg.decide(0, Some((200.0, 0.0, 0.0)), (0.0, 0.0, 0.0), false, 9000, false, 0);
        assert_eq!(o.event, None);
    }

    #[test]
    fn ellipsoid_uses_the_ratio() {
        // ratio = h2/v2 = (32/56)^2 for the soft range: the ellipsoid
        // stretches the vertical axis by h/v
        let cfg = monster_cfg(true);
        let pair = cfg.pair(0).unwrap();
        let expected_ratio = (32.0 * 32.0) / (56.0 * 56.0);
        assert!((pair.soft.ratio - expected_ratio).abs() < 1e-12);
        // the ELLIPSOID hard gate here is a SPHERE (hard 128/128 -> r = 1.0):
        // dy = 128 -> 16384 == h2, not > -> no hard; noAct = 0 keeps the
        // soft-if's roll dead -> alive; 128.5 -> the HARD fires first
        let o = cfg.decide(0, Some((0.0, 128.0, 0.0)), (0.0, 0.0, 0.0), false, 0, true, 0);
        assert_eq!(o.event, None);
        let o = cfg.decide(0, Some((0.0, 128.5, 0.0)), (0.0, 0.0, 0.0), false, 0, true, 0);
        assert_eq!(o.event, Some(DespawnEvent::Hard));
        // the SOFT ellipsoid's vertical edge: dy2 * r > soft h2 <=>
        // |dy| > sqrt(1024 / 0.32653) = 56 — the ratio IS the stretch
        let o = cfg.decide(0, Some((0.0, 56.0, 0.0)), (0.0, 0.0, 0.0), false, 9000, true, 0);
        assert_eq!(o.event, None, "dy == 56: 3136 * r == 1024, not > — inside the soft: reset");
        assert!(o.reset_no_action);
        let o = cfg.decide(0, Some((0.0, 56.5, 0.0)), (0.0, 0.0, 0.0), false, 9000, true, 0);
        assert_eq!(o.event, Some(DespawnEvent::Soft), "56.5: outside the soft ellipsoid, the roll fires");
        // inside the soft ELLIPSOID: 20 blocks horizontal (400 < 1024) ->
        // the else -> the reset
        let o = cfg.decide(0, Some((20.0, 0.0, 0.0)), (0.0, 0.0, 0.0), false, 9000, true, 0);
        assert_eq!(o.event, None);
        assert!(o.reset_no_action);
        // diagonal (90, 60): dx2 8100 + dy2*r 1174 = 9274 < hard h2 16384
        // (alive) but > soft h2 1024 (outside the soft) — the soft-if body
        let o = cfg.decide(0, Some((90.0, 60.0, 0.0)), (0.0, 0.0, 0.0), false, 601, true, 0);
        assert_eq!(o.event, Some(DespawnEvent::Soft));
    }

    #[test]
    fn no_player_and_no_pair_decline() {
        let cfg = monster_cfg(false);
        let o = cfg.decide(0, None, (0.0, 0.0, 0.0), false, 9000, true, 0);
        assert_eq!(o.event, None);
        assert!(!o.reset_no_action);
        // category 3 has no pair (the config push refused the arm; this
        // arm is the defensive no-op)
        let o = cfg.decide(3, Some((500.0, 0.0, 0.0)), (0.0, 0.0, 0.0), false, 9000, true, 0);
        assert_eq!(o.event, None);
    }

    /// The identity-ratio degenerate form (neither limit defined in the
    /// config): ratio == 1.0 — the vanilla preComputed bc 94-96.
    #[test]
    fn undefined_limits_get_ratio_one() {
        let r = range(128, 128, false, false, 128);
        assert_eq!(r.ratio, 1.0);
        // v falls back to the vanilla default
        assert_eq!(r.v, 128);
    }
}
