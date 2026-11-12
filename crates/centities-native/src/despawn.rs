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

/// The pure decision — the faithful `checkDespawn` body (javap 26.3,
/// bc 0-309, pinned from the paper-26.3-249 mapped jar).
///
/// The caller guarantees:
///   - `nearest` is the PLAYER_AFFECTS_SPAWNING nearest player — the
///     wire pushes the exact predicate bit (javap EntitySelector
///     lambda$static$7: `!isSpectator() && isAlive() && affectsSpawning`);
///     the NEAREST by 3D squared distance with the strict-< update rule
///     (the first player in `level.players()` order wins ties — the
///     getNearestPlayer bytecode keeps the earlier player),
///   - `difficulty` is the level's Difficulty ordinal (0 = PEACEFUL),
///   - `peaceful_despawn` is the per-mob resolved
///     `despawnInPeacefulOverride.toBooleanOrElse(!type.isAllowedInPeaceful())`
///     (the wire resolves the TriState at classification),
///   - `seed` is the mob's OWN LCG mirror — the SHARED_RANDOM stand-in
///     (the shipped per-mob semantics; the batch.rs honesty note). The
///     `random.nextInt(800)` soft-roll draws FROM IT INSIDE this function
///     — bc 270-282: the draw runs IFF the soft range hit && !persistent
///     && noActionTime > 600, BEFORE the removeWhenFarAway check — and
///     lands in the mob's timeline BEFORE its AI draws (vanilla
///     lambda$tick$0: checkDespawn -> guardEntityTick(tick)). The
///     doc-61 §4 walk-order note was written for the shared-stream
///     frame; under the per-mob mirror frame the order requirement is
///     trivially satisfied (a per-mob chain has no cross-mob order),
///     which ALSO keeps the wave15 parallel executor untouched: the
///     despawn draw is per-mob state, never a shared-stream drawer.
///
/// On the in-soft-range miss the outcome carries `reset_no_action` (the
/// vanilla `noActionTime = 0` else-branch, bc 304-306) — the executor
/// zeroes its mirror and the applier writes the Java field.
///
/// The hard branch does NOT early-return in vanilla (bc 233 discard
/// falls through to the soft check) — the difference is only state on a
/// already-discarded mob (the extra 1/800 draw on the shared stream and
/// the possible second discard): under the mirror model the discarded
/// mob's chain dies with it, so the early return is observationally
/// identical. Documented in the doc 61 addendum.
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
        peaceful_despawn: bool,
        difficulty: i32,
        seed: &mut u64,
    ) -> DespawnOutcome {
        // bc 0-45: the PEACEFUL branch — difficulty == PEACEFUL && the
        // resolved despawnInPeacefulOverride -> discard(DESPAWN).
        if difficulty == 0 && peaceful_despawn {
            return DespawnOutcome {
                event: Some(DespawnEvent::Peaceful),
                reset_no_action: false,
            };
        }
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
        // dist2 (dx2+dz2+dy2) feeds vanilla's removeWhenFarAway(dist2) —
        // a per-mob bool resolved by the wire at classification
        let _dist2 = dx2 + dz2 + dy2;
        if !persistent
            && pair
                .hard
                .should_despawn(self.ellipsoid, dx2, dy2, dz2, dy)
            && remove_when_far
        {
            return DespawnOutcome { event: Some(DespawnEvent::Hard), reset_no_action: false };
        }
        if pair.soft.should_despawn(self.ellipsoid, dx2, dy2, dz2, dy) {
            // bc 256-282: the roll runs IFF soft-hit && !persistent &&
            // noActionTime > 600 — the draw lands on the mob's own mirror
            // exactly here, before the removeWhenFarAway check.
            if !persistent && no_action > 600 {
                let draw = swarm_sim::mob::lcg_next_int(seed, 800);
                if draw == 0 && remove_when_far {
                    return DespawnOutcome {
                        event: Some(DespawnEvent::Soft),
                        reset_no_action: false,
                    };
                }
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

    /// The zombie defaults for the wire: MONSTER(0), peaceful-despawn on,
    /// NORMAL difficulty (2). `seed` is the draw-independent starter for
    /// tests that never reach the roll.
    const NORMAL: i32 = 2;
    fn d(
        cfg: &DespawnCfg,
        nearest: Option<(f64, f64, f64)>,
        mob: (f64, f64, f64),
        persistent: bool,
        no_action: i32,
        rwf: bool,
        seed: u64,
    ) -> (DespawnOutcome, u64) {
        let mut s = seed;
        let o = cfg.decide(0, nearest, mob, persistent, no_action, rwf, true, NORMAL, &mut s);
        (o, s)
    }

    /// A seed whose FIRST nextInt(800) draw lands on `want` (bound 800 is
    /// even -> the Java rejection loop never fires — exactly ONE 48-bit
    /// LCG step per draw, so the draw value is (seed' >> 17) % 800).
    fn seed_for_draw(want: i32) -> u64 {
        for s in 0xC0FFEEu64.. {
            let mut t = s;
            if swarm_sim::mob::lcg_next_int(&mut t, 800) == want {
                return s;
            }
        }
        unreachable!()
    }

    #[test]
    fn cylinder_hard_edge_is_exact() {
        let cfg = monster_cfg(false);
        // 128 blocks horizontal: dx2+dz2 = 16384.0 == h2 -> NOT > h2 -> alive.
        // Outside the SOFT range too (128 > 32) -> the soft-if body runs
        // (noActionTime 0 <= 600: no roll, no despawn) — no reset.
        let (o, _) = d(&cfg, Some((128.0, 0.0, 0.0)), (0.0, 0.0, 0.0), false, 0, true, 0xC0FFEE);
        assert_eq!(o.event, None);
        assert!(!o.reset_no_action);
        // 128.0001: strictly outside -> hard despawn
        let (o, _) = d(&cfg, Some((128.0001, 0.0, 0.0)), (0.0, 0.0, 0.0), false, 0, true, 0xC0FFEE);
        assert_eq!(o.event, Some(DespawnEvent::Hard));
        // vertical: |dy| = 128 == v -> alive; 128.5 -> hard (the CYLINDER
        // vertical gate uses the SIGNED compare on the abs'ed dy)
        let (o, _) = d(&cfg, Some((0.0, 128.0, 0.0)), (0.0, 0.0, 0.0), false, 0, true, 0xC0FFEE);
        assert_eq!(o.event, None);
        let (o, _) = d(&cfg, Some((0.0, -128.5, 0.0)), (0.0, 0.0, 0.0), false, 0, true, 0xC0FFEE);
        assert_eq!(o.event, Some(DespawnEvent::Hard), "the abs at the call site makes it symmetric");
    }

    #[test]
    fn soft_needs_no_action_and_the_draw() {
        let cfg = monster_cfg(false);
        // 100 blocks out: past the soft 32 (soft-if), inside the hard 128
        let far = Some((100.0, 0.0, 0.0));
        // noActionTime 600 is NOT > 600 -> alive, no reset (outside soft);
        // the roll does NOT run (no mirror consumption)
        let (o, s) = d(&cfg, far, (0.0, 0.0, 0.0), false, 600, true, 0xC0FFEE);
        assert_eq!(o.event, None);
        assert!(!o.reset_no_action);
        assert_eq!(s, 0xC0FFEE, "no roll: the mirror must stay untouched");
        // 601 + draw 0 -> soft despawn; the roll consumed ONE LCG step
        let s0 = seed_for_draw(0);
        let (o, s) = d(&cfg, far, (0.0, 0.0, 0.0), false, 601, true, s0);
        assert_eq!(o.event, Some(DespawnEvent::Soft));
        assert_ne!(s, s0, "the roll advances the mirror exactly once");
        // 601 + draw != 0 -> alive (the roll still consumed its step)
        let s1 = seed_for_draw(1);
        let (o, s) = d(&cfg, far, (0.0, 0.0, 0.0), false, 601, true, s1);
        assert_eq!(o.event, None);
        assert_ne!(s, s1, "a missed roll still consumed its draw");
        // removeWhenFarAway=false: the roll RUNS (bc order) but the despawn
        // declines
        let (o, s) = d(&cfg, far, (0.0, 0.0, 0.0), false, 601, false, s0);
        assert_eq!(o.event, None);
        assert_ne!(s, s0, "the roll runs before the removeWhenFarAway check");
        // the soft range's VERTICAL gate: dy = 60 > soft v 56 -> the soft-if
        // body with a qualifying noActionTime + draw -> soft despawn
        let (o, _) = d(&cfg, Some((0.0, -60.0, 0.0)), (0.0, 0.0, 0.0), false, 9000, true, s0);
        assert_eq!(o.event, Some(DespawnEvent::Soft));
        // INSIDE the soft range (20 blocks): the ELSE — the reset fires for
        // persistent mobs too (the reset is the soft-IF's else, not gated)
        let (o, s) = d(&cfg, Some((20.0, 0.0, 0.0)), (0.0, 0.0, 0.0), true, 9000, true, 0xC0FFEE);
        assert_eq!(o.event, None);
        assert!(o.reset_no_action);
        assert_eq!(s, 0xC0FFEE, "the else-branch draws nothing");
        // persistent + far (soft range hit): no reset, no despawn, no roll
        let (o, s) = d(&cfg, far, (0.0, 0.0, 0.0), true, 9000, true, 0xC0FFEE);
        assert_eq!(o.event, None);
        assert!(!o.reset_no_action);
        assert_eq!(s, 0xC0FFEE);
        // removeWhenFarAway=false but persistent=false, far, noAct 0:
        // outside the soft? no — 100 > 32: soft-if body, noAct 0 <= 600:
        // no roll, no despawn
        let (o, _) = d(&cfg, Some((200.0, 0.0, 0.0)), (0.0, 0.0, 0.0), false, 0, false, 0xC0FFEE);
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
        let (o, _) = d(&cfg, Some((0.0, 128.0, 0.0)), (0.0, 0.0, 0.0), false, 0, true, 0xC0FFEE);
        assert_eq!(o.event, None);
        let (o, _) = d(&cfg, Some((0.0, 128.5, 0.0)), (0.0, 0.0, 0.0), false, 0, true, 0xC0FFEE);
        assert_eq!(o.event, Some(DespawnEvent::Hard));
        // the SOFT ellipsoid's vertical edge: dy2 * r > soft h2 <=>
        // |dy| > sqrt(1024 / 0.32653) = 56 — the ratio IS the stretch
        let (o, _) = d(&cfg, Some((0.0, 56.0, 0.0)), (0.0, 0.0, 0.0), false, 9000, true, 0xC0FFEE);
        assert_eq!(o.event, None, "dy == 56: 3136 * r == 1024, not > — inside the soft: reset");
        assert!(o.reset_no_action);
        let s0 = seed_for_draw(0);
        let (o, _) = d(&cfg, Some((0.0, 56.5, 0.0)), (0.0, 0.0, 0.0), false, 9000, true, s0);
        assert_eq!(o.event, Some(DespawnEvent::Soft), "56.5: outside the soft ellipsoid, the roll fires");
        // inside the soft ELLIPSOID: 20 blocks horizontal (400 < 1024) ->
        // the else -> the reset
        let (o, _) = d(&cfg, Some((20.0, 0.0, 0.0)), (0.0, 0.0, 0.0), false, 9000, true, 0xC0FFEE);
        assert_eq!(o.event, None);
        assert!(o.reset_no_action);
        // diagonal (90, 60): dx2 8100 + dy2*r 1174 = 9274 < hard h2 16384
        // (alive) but > soft h2 1024 (outside the soft) — the soft-if body
        let (o, _) = d(&cfg, Some((90.0, 60.0, 0.0)), (0.0, 0.0, 0.0), false, 601, true, s0);
        assert_eq!(o.event, Some(DespawnEvent::Soft));
    }

    /// bc 0-45: the PEACEFUL branch — difficulty PEACEFUL + the resolved
    /// despawnInPeacefulOverride. NORMAL difficulty leaves it dead even
    /// with the bit set; the bit false keeps a peaceful-despawn mob alive
    /// (a per-mob TriState FALSE override).
    #[test]
    fn peaceful_fires_first() {
        let cfg = monster_cfg(false);
        let mut s = 0xC0FFEEu64;
        // PEACEFUL (0) + zombie semantics -> Peaceful before ANY geometry
        let o = cfg.decide(0, Some((10.0, 0.0, 0.0)), (0.0, 0.0, 0.0), false, 0, true, true, 0, &mut s);
        assert_eq!(o.event, Some(DespawnEvent::Peaceful));
        assert!(!o.reset_no_action);
        // the same state on NORMAL: inside the soft range -> just the reset
        let (o, _) = d(&cfg, Some((10.0, 0.0, 0.0)), (0.0, 0.0, 0.0), false, 0, true, 0xC0FFEE);
        assert_eq!(o.event, None);
        assert!(o.reset_no_action);
        // PEACEFUL but the per-mob override declined the despawn (TriState
        // FALSE — a plugin kept it): the body falls through to the geometry
        let o = cfg.decide(0, Some((200.0, 0.0, 0.0)), (0.0, 0.0, 0.0), false, 0, true, false, 0, &mut s);
        assert_eq!(o.event, Some(DespawnEvent::Hard), "the hard range still runs under the declined override");
    }

    #[test]
    fn no_player_and_no_pair_decline() {
        let cfg = monster_cfg(false);
        let (o, _) = d(&cfg, None, (0.0, 0.0, 0.0), false, 9000, true, 0xC0FFEE);
        assert_eq!(o.event, None);
        assert!(!o.reset_no_action);
        // category 3 has no pair (the config push refused the arm; this
        // arm is the defensive no-op)
        let mut s = 0xC0FFEEu64;
        let o = cfg.decide(3, Some((500.0, 0.0, 0.0)), (0.0, 0.0, 0.0), false, 9000, true, true, NORMAL, &mut s);
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
