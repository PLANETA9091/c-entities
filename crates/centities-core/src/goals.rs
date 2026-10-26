//! L22a — the native goal engine (the AI floor's decision core).
//!
//! Pure functions over packed state: the Java packer mirrors the mob's
//! selector state into the input slice, the engine reproduces the vanilla
//! GoalSelector sequences EXACTLY (same order, same conditions, same RNG
//! consumption) and returns an action list + staged requests.
//!
//! Parity contract:
//! * The RNG is the shared `Entity.SHARED_RANDOM` (a ThreadUnsafeRandom —
//!   one plain 48-bit LCG `value` field). The engine advances it with the
//!   exact java.util.Random recurrence and hands the new seed back; the
//!   Java floor applies it before every staged virtual call that consumes
//!   RNG, so both sides drive ONE stream.
//! * Every native decision is bit-exact with the decompiled 26.3 goal
//!   bodies; anything the engine cannot prove becomes a STAGED request the
//!   Java floor resolves with the real virtual call, in the exact vanilla
//!   order.
//! * `start()` always runs as the REAL virtual call (it writes goal-object
//!   state like wantedX/lookTime) — the engine only keeps the lock book.
//! * The kill-switch is trivial: the floor stops calling the engine and the
//!   L15 Java microkernel (byte-exact vanilla mirror) takes over.

#![forbid(unsafe_code)]

// ---------------------------------------------------------------------------
// wire format (shared with CeHooks.java)
// ---------------------------------------------------------------------------

/// actions applied, tick floored
pub const ST_DONE: i32 = 0;
/// resolve out[1] (request opcode) for out[2] (goalIdx), then resume
pub const ST_REQUEST: i32 = 1;
/// run the vanilla serverAiStep mirror this tick
pub const ST_FALLBACK: i32 = -1;

// staged request opcodes
pub const REQ_CC_VANILLA: i32 = 1; // goal.canContinueToUse() → resume res(0/1)
pub const REQ_CANUSE_VANILLA: i32 = 2; // goal.canUse() → resume res(0/1)
pub const REQ_STROLL_POS: i32 = 3; // getPosition() chain → resume res(0/1)
pub const REQ_LOOKAT_QUERY: i32 = 4; // getNearestPlayer chain → resume res(0/1)
pub const REQ_START_VANILLA: i32 = 5; // goal.start() → resume res(=1)
pub const REQ_FIND_TARGET: i32 = 6; // findTarget() chain → resume res(0/1)

// goal kinds (the classified zombie set)
pub const GK_UNKNOWN: i32 = 0;
pub const GK_SPEAR_USE: i32 = 1;
pub const GK_ZOMBIE_ATTACK: i32 = 2;
pub const GK_MOVE_THROUGH_VILLAGE: i32 = 3;
pub const GK_STROLL: i32 = 4;
pub const GK_LOOKAT_PLAYER: i32 = 5;
pub const GK_LOOKAROUND: i32 = 6;
pub const GK_HURT_BY: i32 = 7;
pub const GK_NEAREST_TARGET: i32 = 8;

// goal flags — the vanilla Goal.Flag ordinals as bits
pub const FL_MOVE: i64 = 1;
pub const FL_LOOK: i64 = 1 << 1;
pub const FL_JUMP: i64 = 1 << 2;
pub const FL_TARGET: i64 = 1 << 3;
pub const FL_UNKNOWN: i64 = 1 << 4;

// mob input flags
pub const MF_NO_PASSENGERS: i32 = 1; // !hasControllingPassenger()
pub const MF_MAINHAND_EMPTY: i32 = 1 << 1;
pub const MF_HAS_TARGET: i32 = 1 << 2;
pub const MF_USING_ITEM: i32 = 1 << 3;

// selector order — vanilla runs targetSelector FIRST in both branches
pub const SEL_TARGET: usize = 0;
pub const SEL_GOAL: usize = 1;

// navPathState
pub const NAV_NONE: i32 = 0;
pub const NAV_PATH_DONE: i32 = 1;
pub const NAV_PATH_ACTIVE: i32 = 2;

// per-goal record: 6 i32 — [kind, priority, flagsMask, cfg, running, state]
pub const GOAL_STRIDE: usize = 6;
// cfg word per kind:
//   GK_STROLL         low16 = reducedTickDelay(interval) (60 for 120)
//   GK_LOOKAT_PLAYER  = probability f32 bits (0.02F)
//   GK_NEAREST_TARGET low16 = randomInterval (10)
// state word per kind:
//   GK_LOOKAROUND     = lookTime
//   GK_LOOKAT_PLAYER  bit30 = lookAt alive, bit29 = within range, low16 = lookTime
//   GK_STROLL         bit0 = forceTrigger (classifier refuses triggered strolls)

// input buffer (i32 units), written by the Java packer per call:
//   [0]  resume cursor (0 = fresh; else the out[6] token of the last request)
//   [1]  mobFlags (MF_*)
//   [2]  tickCount
//   [3]  entityId
//   [4]  noActionTime (in/out — the executor writes back the increment)
//   [5]  target selector goal count
//   [6]  goal selector goal count
//   [7]  disabled flags, target selector (low), [8] high
//   [9]  disabled flags, goal selector (low), [10] high
//   [11] navPathState (NAV_*)
//   [12] seed low, [13] seed high
//   [14] request result (0/1), [15] request opcode (the resume dispatch —
//        the packer echoes out[1] here; slot 1 stays the mob flags)
//   [16..22] request result coords x/y/z as f64 bit pairs
//   [22..]  goal records: target selector first, then goal selector
const IN_HEADER: usize = 22;
const IN_SEED_LO: usize = 12;
const IN_SEED_HI: usize = 13;
const IN_RES: usize = 14;
const IN_RES_OP: usize = 15;
const IN_RES_X: usize = 16;

// output buffer (i32 units):
//   [0] status (ST_*)
//   [1] request opcode (ST_REQUEST)
//   [2] request goalIdx
//   [3] seed low, [4] seed high (always current — the floor applies it
//       before any staged virtual call)
//   [5] action count
//   [6] resume cursor (Java writes it into in[0] on resume)
//   [7] fallback reason (FB_* — 0 when no fallback; diagnostics only)
//   [8...] actions, 8 i32 each: [op, p0, p1, p2, p3, d0lo, d0hi, pad]
const OUT_HEADER: usize = 8;
const OUT_ACT: usize = 8;
const MAX_ACTS: usize = 96;

/// fallback reason codes (out[7]) — the Java floor logs the first one so a
/// classified-mob fallback is diagnosable from the CI log alone.
pub const FB_OK: i32 = 0;
pub const FB_BUF_TOO_SMALL: i32 = 1;
pub const FB_GOAL_OVERFLOW: i32 = 2;
pub const FB_LOCK_COLLIDE: i32 = 3;
pub const FB_UNKNOWN_KIND: i32 = 4;
pub const FB_BAD_CURSOR: i32 = 5;
pub const FB_UNKNOWN_REQ: i32 = 6;
pub const FB_ACTS_OVERFLOW: i32 = 7;
pub const FB_STATE_MISMATCH: i32 = 8;
/// Java-side reasons (aiRun) — round cap / bad goal index / unknown op
pub const FB_JAVA_ROUND_CAP: i32 = 101;
pub const FB_JAVA_BAD_GI: i32 = 102;
pub const FB_JAVA_UNKNOWN_REQ: i32 = 103;
pub const FB_JAVA_PREDICATE: i32 = 104;

// action opcodes (applied by the executor after the FINAL round)
pub const ACT_GOAL_START: i32 = 1; // goal.start() virtual (p0 = global goalIdx)
pub const ACT_GOAL_TICK_V: i32 = 3; // goal.tick() virtual (p0 = goalIdx)
pub const ACT_GOAL_STOP: i32 = 2; // goal.stop() virtual (p0 = goalIdx)
pub const ACT_LOOK_TICK_AROUND: i32 = 5; // lookaround tick port: lookTime-- (p1 = new), setLookAt(x+relX, eyeY, z+relZ)
pub const ACT_LOOK_TICK_AT: i32 = 6; // lookat tick port: lookTime-- (p1 = new), setLookAt(lookAt)
pub const ACT_NAV_IDLE: i32 = 7; // navigation.tick idle port done natively (executor does tick++ etc.)
pub const ACT_MOVE_ZZA0: i32 = 8; // moveControl WAIT: setZza(0)
pub const ACT_LOOK_COOL_VANILLA: i32 = 9; // lookControl.tick routes vanilla (cooldown > 0)
pub const ACT_NOOP: i32 = 0;

// phases within one selector tick
const PH_CLEANUP: i32 = 0;
const PH_LOCK_REBUILD: i32 = 1;
const PH_UPDATE: i32 = 2;
const PH_TICK_RUNNING: i32 = 3;

const MAX_GOALS: usize = 16;

fn cursor(sel: usize, phase: i32, gi: usize) -> i32 {
    ((sel as i32 & 1) << 9) | ((phase & 7) << 6) | (gi as i32 & 63)
}

// ---------------------------------------------------------------------------
// the exact RNG (ThreadUnsafeRandom: value = value*25214903917+11 mod 2^48)
// ---------------------------------------------------------------------------

pub struct Lcg {
    pub value: u64,
}

impl Lcg {
    #[inline]
    fn advance(&mut self) -> u64 {
        self.value = self.value.wrapping_mul(25214903917).wrapping_add(11) & 0xFFFF_FFFF_FFFF;
        self.value
    }
    /// BitRandomSource.next(bits)
    #[inline]
    fn next(&mut self, bits: i32) -> i32 {
        (self.advance() >> (48 - bits)) as i32
    }
    /// java.util.Random.nextInt(bound) — the exact biased loop
    #[inline]
    fn next_int_bound(&mut self, bound: i32) -> i32 {
        if bound <= 0 {
            return 0; // vanilla throws; configs are constant positives
        }
        if bound & (bound - 1) == 0 {
            return (((bound as i64) * (self.next(31) as i64)) >> 31) as i32;
        }
        loop {
            let bits = self.next(31);
            let val = bits % bound;
            if bits.wrapping_sub(val).wrapping_add(bound - 1) >= 0 {
                return val;
            }
        }
    }
    /// RandomSource.nextFloat() = next(24) / 2^24 — both exact in f32
    #[inline]
    fn next_f32(&mut self) -> f32 {
        (self.next(24) as f32) / 16777216.0f32
    }
    /// RandomSource.nextDouble() = ((next(26)<<27) + next(27)) * 2^-53
    #[inline]
    fn next_f64(&mut self) -> f64 {
        let hi = (self.next(26) as i64) << 27;
        let lo = self.next(27) as i64;
        ((hi + lo) as f64) * (1.0f64 / 9007199254740992.0f64)
    }
}

// ---------------------------------------------------------------------------
// Mth ports (bit-exact f32 semantics, verified against the decompiled Mth)
// ---------------------------------------------------------------------------

#[inline]
fn wrap_degrees(a: f32) -> f32 {
    let mut n = a % 360.0f32;
    if n >= 180.0f32 {
        n -= 360.0f32;
    }
    if n < -180.0f32 {
        n += 360.0f32;
    }
    n
}

#[inline]
fn degrees_difference(from: f32, to: f32) -> f32 {
    wrap_degrees(to - from)
}

#[inline]
fn clamp_f32(v: f32, lo: f32, hi: f32) -> f32 {
    // Mth.clamp(value, min, max): value < min ? min : value > max ? max : value
    if v < lo {
        lo
    } else if v > hi {
        hi
    } else {
        v
    }
}

/// Control.rotateTowards
#[inline]
fn rotate_towards(from: f32, to: f32, max_rot: f32) -> f32 {
    from + clamp_f32(degrees_difference(from, to), -max_rot, max_rot)
}

/// Mth.positiveCeilDiv
#[inline]
fn positive_ceil_div(a: i32, b: i32) -> i32 {
    -((-a).div_euclid(b))
}

// ---------------------------------------------------------------------------
// the engine
// ---------------------------------------------------------------------------

struct Out<'a> {
    buf: &'a mut [i32],
    n_act: usize,
}

impl<'a> Out<'a> {
    #[inline]
    fn push(&mut self, op: i32, p0: i32, p1: i32, p2: i32, p3: i32) {
        if self.n_act < MAX_ACTS {
            let b = self.n_act * OUT_ACT + OUT_HEADER;
            self.buf[b] = op;
            self.buf[b + 1] = p0;
            self.buf[b + 2] = p1;
            self.buf[b + 3] = p2;
            self.buf[b + 4] = p3;
            self.buf[b + 5] = 0;
            self.buf[b + 6] = 0;
            self.buf[b + 7] = 0;
            self.n_act += 1;
        }
    }
}

struct Ctx<'a> {
    inb: &'a [i32],
    base: usize,        // goal record base
    counts: [usize; 2], // [SEL_TARGET, SEL_GOAL]
    disabled: [i64; 2],
    lcg: Lcg,
    lock: [[i32; MAX_GOALS]; 2], // flag bit -> goal idx, -1 = free
    running: [[bool; MAX_GOALS]; 2],
}

impl<'a> Ctx<'a> {
    #[inline]
    fn off(&self, sel: usize, i: usize) -> usize {
        self.base + (self.offset_of(sel) + i) * GOAL_STRIDE
    }
    #[inline]
    fn offset_of(&self, sel: usize) -> usize {
        if sel == SEL_TARGET {
            0
        } else {
            self.counts[SEL_TARGET]
        }
    }
    /// the GLOBAL goal index (target selector first — matches the packed
    /// record order and the Java executor's goalAt lookup)
    #[inline]
    fn global(&self, sel: usize, i: usize) -> i32 {
        (self.offset_of(sel) + i) as i32
    }
    #[inline]
    fn kind(&self, sel: usize, i: usize) -> i32 {
        self.inb[self.off(sel, i)]
    }
    #[inline]
    fn prio(&self, sel: usize, i: usize) -> i32 {
        self.inb[self.off(sel, i) + 1]
    }
    #[inline]
    fn flags(&self, sel: usize, i: usize) -> i64 {
        self.inb[self.off(sel, i) + 2] as i64
    }
    #[inline]
    fn cfg(&self, sel: usize, i: usize) -> i32 {
        self.inb[self.off(sel, i) + 3]
    }
    #[inline]
    fn is_running(&self, sel: usize, i: usize) -> bool {
        self.inb[self.off(sel, i) + 4] != 0
    }
    #[inline]
    fn state(&self, sel: usize, i: usize) -> i32 {
        self.inb[self.off(sel, i) + 5]
    }
    #[inline]
    fn flag(&self, bit: i32) -> bool {
        self.inb[1] & bit != 0
    }
    fn seed(&self) -> i64 {
        ((self.inb[IN_SEED_HI] as i64) << 32) | (self.inb[IN_SEED_LO] as i64 as u64) as i64
    }
}

enum Cu {
    No,
    /// canUse() == true natively — parked: the START runs as an action
    /// (it writes goal-object state and may consume RNG)
    StartStaged,
    /// the canUse body itself needs the vanilla call (side effects / queries)
    Staged,
}

/// The floor entry. Returns the status written to out_buf[0].
pub fn ai_floor(in_buf: &[i32], out_buf: &mut [i32]) -> i32 {
    if in_buf.len() < IN_HEADER || out_buf.len() < OUT_HEADER + MAX_ACTS * OUT_ACT {
        out_buf[0] = ST_FALLBACK;
        out_buf[7] = FB_BUF_TOO_SMALL;
        return ST_FALLBACK;
    }
    let nt = in_buf[5] as usize;
    let ng = in_buf[6] as usize;
    if nt > MAX_GOALS || ng > MAX_GOALS || in_buf.len() < IN_HEADER + (nt + ng) * GOAL_STRIDE {
        out_buf[0] = ST_FALLBACK;
        out_buf[7] = FB_GOAL_OVERFLOW;
        return ST_FALLBACK;
    }

    let mut ctx = Ctx {
        inb: in_buf,
        base: IN_HEADER,
        counts: [nt, ng],
        disabled: [
            (in_buf[7] as i64) | ((in_buf[8] as i64) << 32),
            (in_buf[9] as i64) | ((in_buf[10] as i64) << 32),
        ],
        lcg: Lcg {
            value: ((in_buf[IN_SEED_HI] as u64) << 32) | (in_buf[IN_SEED_LO] as u64 & 0xFFFF_FFFF),
        },
        lock: [[-1; MAX_GOALS]; 2],
        running: [[false; MAX_GOALS]; 2],
    };
    for sel in 0..2usize {
        for i in 0..ctx.counts[sel] {
            ctx.running[sel][i] = ctx.is_running(sel, i);
        }
    }

    let mut out = Out {
        buf: out_buf,
        n_act: 0,
    };

    // parity: vanilla serverAiStep —
    //   odd  (tickCount+id)%2 != 0 && tickCount > 1: tickRunningGoals(false) only
    //   even (otherwise):                            full tick()
    let odd_branch = ((ctx.inb[2].wrapping_add(ctx.inb[3])) & 1 != 0) && ctx.inb[2] > 1;

    let resume = in_buf[0] != 0;
    let (mut sel, mut phase, mut gi) = if resume {
        let c = in_buf[0];
        (((c >> 9) & 1) as usize, (c >> 6) & 7, (c & 63) as usize)
    } else {
        (SEL_TARGET, PH_CLEANUP, 0)
    };
    if !resume && odd_branch {
        phase = PH_TICK_RUNNING;
    }

    if resume {
        // the fresh pack carries the post-continuation running state —
        // rebuild the lock table for the parked selector before continuing
        // (the displaced goals were stopped with their flags released)
        let count = ctx.counts[sel];
        for bit in 0..MAX_GOALS {
            ctx.lock[sel][bit] = -1;
        }
        for i in 0..count {
            if ctx.running[sel][i] {
                let mut fl = ctx.flags(sel, i);
                while fl != 0 {
                    let bit = fl.trailing_zeros() as usize;
                    if bit >= MAX_GOALS || ctx.lock[sel][bit] != -1 {
                        return fallback(&mut out, FB_LOCK_COLLIDE);
                    }
                    ctx.lock[sel][bit] = i as i32;
                    fl &= fl - 1;
                }
            }
        }
        let res = in_buf[IN_RES] != 0;
        match in_buf[IN_RES_OP] {
            REQ_CC_VANILLA => {
                if !res && stop_goal(&mut ctx, &mut out, sel, gi).is_err() {
                    return fallback(&mut out, FB_STATE_MISMATCH);
                }
            }
            REQ_CANUSE_VANILLA | REQ_STROLL_POS | REQ_LOOKAT_QUERY | REQ_FIND_TARGET
            | REQ_START_VANILLA => {
                // the continuation resolved the PREDICATE only — the lock
                // transition (displaced stops, then start) is the engine's
                // job, emitted as actions the floor applies IMMEDIATELY
                if res && lock_only(&mut ctx, &mut out, sel, gi).is_err() {
                    return fallback(&mut out, FB_STATE_MISMATCH);
                }
            }
            _ => {}
        }
        gi += 1;
    }

    loop {
        let count = ctx.counts[sel];
        match phase {
            PH_CLEANUP => {
                while gi < count {
                    if ctx.running[sel][gi] {
                        if ctx.flags(sel, gi) & ctx.disabled[sel] != 0 {
                            if stop_goal(&mut ctx, &mut out, sel, gi).is_err() {
                                return fallback(&mut out, FB_STATE_MISMATCH);
                            }
                        } else if needs_cc_vanilla(ctx.kind(sel, gi)) {
                            return park(&ctx, &mut out, sel, PH_CLEANUP, gi, REQ_CC_VANILLA);
                        } else if !cc_ported(&ctx, sel, gi) {
                            if stop_goal(&mut ctx, &mut out, sel, gi).is_err() {
                                return fallback(&mut out, FB_STATE_MISMATCH);
                            }
                        }
                    }
                    gi += 1;
                }
                phase = PH_LOCK_REBUILD;
                gi = 0;
            }
            PH_LOCK_REBUILD => {
                for bit in 0..MAX_GOALS {
                    ctx.lock[sel][bit] = -1;
                }
                for i in 0..count {
                    if ctx.running[sel][i] {
                        let mut fl = ctx.flags(sel, i);
                        while fl != 0 {
                            let bit = fl.trailing_zeros() as usize;
                            if bit >= MAX_GOALS || ctx.lock[sel][bit] != -1 {
                                // two running goals on one flag — impossible
                                // under vanilla semantics; fall back
                                return fallback(&mut out, FB_LOCK_COLLIDE);
                            }
                            ctx.lock[sel][bit] = i as i32;
                            fl &= fl - 1;
                        }
                    }
                }
                phase = PH_UPDATE;
                gi = 0;
            }
            PH_UPDATE => {
                while gi < count {
                    if !ctx.running[sel][gi] {
                        let fl = ctx.flags(sel, gi);
                        if fl & ctx.disabled[sel] == 0 && can_replace_all(&ctx, sel, gi) {
                            match can_use(&mut ctx, &mut out, sel, gi, phase) {
                                Ok(Cu::No) => {}
                                Ok(Cu::StartStaged) | Ok(Cu::Staged) => {
                                    return out.buf[0]; // parked
                                }
                                Err(()) => return fallback(&mut out, FB_UNKNOWN_KIND),
                            }
                        }
                    }
                    gi += 1;
                }
                phase = PH_TICK_RUNNING;
                gi = 0;
            }
            PH_TICK_RUNNING => {
                let force = !odd_branch;
                while gi < count {
                    if ctx.running[sel][gi] && (force || every_tick(ctx.kind(sel, gi))) {
                        if !tick_ported(&mut ctx, &mut out, sel, gi) {
                            out.push(ACT_GOAL_TICK_V, ctx.global(sel, gi), 0, 0, 0);
                        }
                    }
                    gi += 1;
                }
                if sel == SEL_TARGET {
                    sel = SEL_GOAL;
                    phase = if odd_branch {
                        PH_TICK_RUNNING
                    } else {
                        PH_CLEANUP
                    };
                    gi = 0;
                    continue;
                }
                return done(&mut ctx, &mut out);
            }
            _ => return fallback(&mut out, FB_BAD_CURSOR),
        }
    }
}

fn done(ctx: &mut Ctx, out: &mut Out) -> i32 {
    // the engine's ported control decisions ride the actions; the executor's
    // Java tail (nav tick++, look idle rotate, jump reset) finishes the rest.
    out.buf[0] = ST_DONE;
    out.buf[1] = 0;
    out.buf[2] = 0;
    out.buf[3] = (ctx.lcg.value & 0xFFFF_FFFF) as i32;
    out.buf[4] = (ctx.lcg.value >> 32) as i32;
    out.buf[5] = out.n_act as i32;
    out.buf[6] = 0;
    ST_DONE
}

fn fallback(out: &mut Out, reason: i32) -> i32 {
    out.buf[0] = ST_FALLBACK;
    out.buf[5] = 0;
    out.buf[7] = reason;
    ST_FALLBACK
}

fn park(ctx: &Ctx, out: &mut Out, sel: usize, phase: i32, gi: usize, req: i32) -> i32 {
    out.buf[0] = ST_REQUEST;
    out.buf[1] = req;
    out.buf[2] = gi as i32;
    out.buf[3] = (ctx.lcg.value & 0xFFFF_FFFF) as i32;
    out.buf[4] = (ctx.lcg.value >> 32) as i32;
    out.buf[5] = out.n_act as i32;
    out.buf[6] = cursor(sel, phase, gi);
    ST_REQUEST
}

fn needs_cc_vanilla(kind: i32) -> bool {
    matches!(
        kind,
        GK_SPEAR_USE | GK_ZOMBIE_ATTACK | GK_MOVE_THROUGH_VILLAGE | GK_HURT_BY | GK_NEAREST_TARGET
    )
}

fn every_tick(kind: i32) -> bool {
    kind == GK_LOOKAROUND
}

/// the ported canContinueToUse bodies
fn cc_ported(ctx: &Ctx, sel: usize, gi: usize) -> bool {
    match ctx.kind(sel, gi) {
        GK_STROLL => {
            // !nav.isDone() && !hasControllingPassenger()
            ctx.inb[11] == NAV_PATH_ACTIVE && ctx.flag(MF_NO_PASSENGERS)
        }
        GK_LOOKAROUND => ctx.state(sel, gi) >= 0,
        GK_LOOKAT_PLAYER => {
            let st = ctx.state(sel, gi);
            (st & (1 << 31)) != 0 && (st & (1 << 30)) != 0 && (st & 0x3FFF_FFFF) > 0
        }
        _ => true,
    }
}

/// the ported canUse bodies
fn can_use(ctx: &mut Ctx, out: &mut Out, sel: usize, gi: usize, phase: i32) -> Result<Cu, ()> {
    match ctx.kind(sel, gi) {
        GK_SPEAR_USE => {
            // ableToAttack() && !isUsingItem() — native only when the
            // mainhand is EMPTY (has(KINETIC_WEAPON) provably false);
            // a non-empty mainhand stages the component check
            if ctx.flag(MF_MAINHAND_EMPTY) {
                Ok(Cu::No)
            } else {
                park(ctx, out, sel, phase, gi, REQ_CANUSE_VANILLA);
                Ok(Cu::Staged)
            }
        }
        GK_ZOMBIE_ATTACK | GK_MOVE_THROUGH_VILLAGE | GK_HURT_BY => {
            park(ctx, out, sel, phase, gi, REQ_CANUSE_VANILLA);
            Ok(Cu::Staged)
        }
        GK_NEAREST_TARGET => {
            // if (randomInterval > 0 && nextInt(randomInterval) != 0) false
            let ri = ctx.cfg(sel, gi) & 0xFFFF;
            if ri > 0 && ctx.lcg.next_int_bound(ri) != 0 {
                return Ok(Cu::No);
            }
            // findTarget() — a level query, staged (the gate pull already
            // happened; the seed handoff keeps the stream exact)
            park(ctx, out, sel, phase, gi, REQ_FIND_TARGET);
            Ok(Cu::Staged)
        }
        GK_STROLL => {
            // RandomStrollGoal.canUse: passenger → noActionTime ≥ 100 →
            // nextInt gate → getPosition (staged when passed)
            if !ctx.flag(MF_NO_PASSENGERS) {
                return Ok(Cu::No);
            }
            // checkNoActionTime is always on for the classified stroll
            if ctx.inb[4] >= 100 {
                return Ok(Cu::No);
            }
            let interval = ctx.cfg(sel, gi) & 0xFFFF;
            if interval > 0 && ctx.lcg.next_int_bound(interval) != 0 {
                return Ok(Cu::No);
            }
            // forceTrigger == false for the classified goal (verified at
            // classification; a triggered stroll routes vanilla forever)
            park(ctx, out, sel, phase, gi, REQ_STROLL_POS);
            Ok(Cu::Staged)
        }
        GK_LOOKAT_PLAYER => {
            // nextFloat() >= probability → false; the query stages
            let prob = f32::from_bits(ctx.cfg(sel, gi) as u32);
            if ctx.lcg.next_f32() >= prob {
                return Ok(Cu::No);
            }
            park(ctx, out, sel, phase, gi, REQ_LOOKAT_QUERY);
            Ok(Cu::Staged)
        }
        GK_LOOKAROUND => {
            // nextFloat() < 0.02F → start (staged — start() writes
            // relX/relZ/lookTime and consumes nextDouble + nextInt(20))
            if ctx.lcg.next_f32() < 0.02f32 {
                park(ctx, out, sel, phase, gi, REQ_START_VANILLA);
                Ok(Cu::StartStaged)
            } else {
                Ok(Cu::No)
            }
        }
        _ => Err(()),
    }
}

/// WrappedGoal.canBeReplacedBy: other.priority <= locked.priority
fn can_replace_all(ctx: &Ctx, sel: usize, gi: usize) -> bool {
    let mut fl = ctx.flags(sel, gi);
    while fl != 0 {
        let bit = fl.trailing_zeros() as usize;
        if bit < MAX_GOALS {
            let locked = ctx.lock[sel][bit];
            if locked != -1 && ctx.prio(sel, locked as usize) < ctx.prio(sel, gi) {
                return false;
            }
        }
        fl &= fl - 1;
    }
    true
}

fn lock_only(ctx: &mut Ctx, out: &mut Out, sel: usize, gi: usize) -> Result<(), ()> {
    // vanilla update-pass transition: for each flag — currentGoal.stop();
    // lock[flag] = goalx — then goalx.start(). The stops ride first so the
    // executor applies stop→start in the exact vanilla order.
    let mut fl = ctx.flags(sel, gi);
    while fl != 0 {
        let bit = fl.trailing_zeros() as usize;
        if bit >= MAX_GOALS {
            return Err(());
        }
        let cur = ctx.lock[sel][bit];
        if cur != -1 && stop_goal(ctx, out, sel, cur as usize).is_err() {
            return Err(());
        }
        ctx.lock[sel][bit] = gi as i32;
        fl &= fl - 1;
    }
    ctx.running[sel][gi] = true;
    out.push(ACT_GOAL_START, ctx.global(sel, gi), 0, 0, 0);
    Ok(())
}

fn stop_goal(ctx: &mut Ctx, out: &mut Out, sel: usize, gi: usize) -> Result<(), ()> {
    ctx.running[sel][gi] = false;
    for bit in 0..MAX_GOALS {
        if ctx.lock[sel][bit] == gi as i32 {
            ctx.lock[sel][bit] = -1;
        }
    }
    out.push(ACT_GOAL_STOP, ctx.global(sel, gi), 0, 0, 0);
    Ok(())
}

/// the ported tick() bodies — false → the virtual call
fn tick_ported(ctx: &mut Ctx, out: &mut Out, sel: usize, gi: usize) -> bool {
    match ctx.kind(sel, gi) {
        GK_LOOKAROUND => {
            // lookTime--; lookControl.setLookAt(x+relX, eyeY, z+relZ)
            let st = ctx.state(sel, gi);
            out.push(ACT_LOOK_TICK_AROUND, ctx.global(sel, gi), st - 1, 0, 0);
            true
        }
        GK_LOOKAT_PLAYER => {
            // if (lookAt.isAlive()) setLookAt(lookAt); lookTime--
            let st = ctx.state(sel, gi);
            if (st & (1 << 31)) != 0 {
                out.push(
                    ACT_LOOK_TICK_AT,
                    ctx.global(sel, gi),
                    (st & 0x3FFF_FFFF) - 1,
                    0,
                    0,
                );
            }
            true
        }
        _ => false,
    }
}

// ---------------------------------------------------------------------------
// tests — the LCG and the wrap math against java.util.Random reference
// vectors (generated from the JDK, fixed below)
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // -- buffer builders (mirror the Java packer) -------------------------

    const MF_NO_PASSENGERS: i32 = 1;
    const MF_MAINHAND_EMPTY: i32 = 2;

    fn rec(kind: i32, prio: i32, flags: i64, cfg: i32, running: bool, state: i32) -> [i32; 6] {
        [
            kind,
            prio,
            flags as i32,
            cfg,
            if running { 1 } else { 0 },
            state,
        ]
    }

    /// idle zombie: target selector = hurtby + 3 nearest; goal selector =
    /// spear, attack, turtleEgg, village, stroll, lookat, lookaround
    fn build_in(seed: u64, tick_count: i32, id: i32, no_action: i32) -> Vec<i32> {
        let mut b = vec![0i32; IN_HEADER + 11 * GOAL_STRIDE];
        b[0] = 0;
        b[1] = MF_NO_PASSENGERS | MF_MAINHAND_EMPTY; // no target, not using item
        b[2] = tick_count;
        b[3] = id;
        b[4] = no_action;
        b[5] = 4; // target selector count
        b[6] = 7; // goal selector count
        b[7] = 0; // disabled target flags
        b[8] = 0;
        b[9] = 0; // disabled goal flags
        b[10] = 0;
        b[11] = NAV_NONE;
        b[12] = seed as i32;
        b[13] = (seed >> 32) as i32;
        let fl_target = FL_TARGET;
        let fl_ml = FL_MOVE | FL_LOOK;
        let mut off = IN_HEADER;
        for r in [
            rec(GK_HURT_BY, 1, fl_target, 0, false, 0),
            rec(GK_NEAREST_TARGET, 2, fl_target, 10, false, 0),
            rec(GK_NEAREST_TARGET, 3, fl_target, 10, false, 0),
            rec(GK_NEAREST_TARGET, 5, fl_target, 10, false, 0),
        ] {
            b[off..off + 6].copy_from_slice(&r);
            off += 6;
        }
        for r in [
            rec(GK_SPEAR_USE, 2, fl_ml, 0, false, 0),
            rec(GK_ZOMBIE_ATTACK, 3, fl_ml, 0, false, 0),
            rec(GK_ZOMBIE_ATTACK, 4, fl_ml, 0, false, 0),
            rec(GK_MOVE_THROUGH_VILLAGE, 6, FL_MOVE, 0, false, 0),
            rec(GK_STROLL, 7, FL_MOVE, 60, false, 0),
            rec(
                GK_LOOKAT_PLAYER,
                8,
                FL_LOOK,
                0.02f32.to_bits() as i32,
                false,
                0,
            ),
            rec(GK_LOOKAROUND, 8, fl_ml, 0, false, 0),
        ] {
            b[off..off + 6].copy_from_slice(&r);
            off += 6;
        }
        b
    }

    /// drive the staged protocol the way CeHooks.aiRun does; `resolve`
    /// maps the request opcode to the continuation result
    fn drive_res(in_buf: &mut Vec<i32>, out: &mut Vec<i32>, resolve: &dyn Fn(i32) -> i32) -> i32 {
        loop {
            let st = ai_floor(in_buf, out);
            if st != ST_REQUEST {
                return st;
            }
            in_buf[0] = out[6]; // cursor
            in_buf[IN_RES_OP] = out[1]; // the request op for the resume dispatch
            in_buf[IN_RES] = resolve(out[1]);
            let seed = ((out[4] as u64) << 32) | (out[3] as u64 & 0xFFFF_FFFF);
            in_buf[12] = seed as i32;
            in_buf[13] = (seed >> 32) as i32;
        }
    }

    fn drive(in_buf: &mut Vec<i32>, out: &mut Vec<i32>, res: i32) -> i32 {
        drive_res(in_buf, out, &|_| res)
    }

    #[test]
    fn idle_even_tick_pulls_exactly_six_values() {
        // the vanilla even-tick pull order for an idle zombie: 3 × nextInt(10)
        // (nearest gates) + nextInt(60) (stroll) + nextFloat (lookat) +
        // nextFloat (lookaround) — staged goals pull NOTHING.
        let seed0 = 777u64;
        let mut inb = build_in(seed0, 100, 2, 42);
        let mut out = vec![0i32; OUT_HEADER + MAX_ACTS * OUT_ACT];
        let st = drive(&mut inb, &mut out, 0);
        assert_eq!(st, ST_DONE);

        // reference: advance the LCG with the exact pull sequence
        let mut r = Lcg { value: seed0 };
        for _ in 0..3 {
            r.next_int_bound(10);
        }
        r.next_int_bound(60);
        r.next_f32();
        r.next_f32();
        let seed1 = ((out[4] as u64) << 32) | (out[3] as u64 & 0xFFFF_FFFF);
        assert_eq!(seed1, r.value, "the engine must consume exactly 6 pulls");

        // nothing started, nothing stopped
        assert_eq!(out[5], 0, "no actions expected on a quiet tick");
    }

    #[test]
    fn odd_tick_runs_running_goals_only() {
        // (tickCount + id) odd → both selectors run tickRunningGoals(false)
        // → the running lookaround ticks (lookTime--), nothing else
        let seed0 = 4242u64;
        let mut inb = build_in(seed0, 101, 2, 42); // 101+2 = odd, tickCount > 1
                                                   // mark the lookaround (global idx 4+6=10) running with lookTime 7
        let off = IN_HEADER + 10 * GOAL_STRIDE;
        inb[off + 4] = 1;
        inb[off + 5] = 7;
        let mut out = vec![0i32; OUT_HEADER + MAX_ACTS * OUT_ACT];
        let st = ai_floor(&inb, &mut out);
        assert_eq!(st, ST_DONE);
        assert_eq!(out[5], 1, "exactly one action: the lookaround tick port");
        assert_eq!(out[OUT_HEADER], ACT_LOOK_TICK_AROUND);
        assert_eq!(out[OUT_HEADER + 1], 10); // global idx
        assert_eq!(out[OUT_HEADER + 2], 6); // lookTime 7 - 1

        // the seed must be UNTOUCHED on odd ticks
        let seed1 = ((inb[13] as u64) << 32) | (inb[12] as u64 & 0xFFFF_FFFF);
        let done_seed = ((out[4] as u64) << 32) | (out[3] as u64 & 0xFFFF_FFFF);
        assert_eq!(seed1, done_seed);
    }

    #[test]
    fn stroll_gate_pass_starts_with_lock_transition() {
        // choose a seed where nextInt(60) == 0 on the stroll gate: brute a
        // seed whose 4th pull (after the 3 nearest gates) yields 0
        let mut seed = 1u64;
        loop {
            let mut r = Lcg { value: seed };
            for _ in 0..3 {
                r.next_int_bound(10);
            }
            if r.next_int_bound(60) == 0 {
                break;
            }
            seed += 1;
        }
        let mut inb = build_in(seed, 100, 2, 42);
        let mut out = vec![0i32; OUT_HEADER + MAX_ACTS * OUT_ACT];
        // only the stroll's getPosition succeeds — every staged canUse says no
        let st = drive_res(&mut inb, &mut out, &|req| {
            if req == REQ_STROLL_POS {
                1
            } else {
                0
            }
        });
        assert_eq!(st, ST_DONE);
        // actions: [start stroll] — the stroll's global idx = 4 + 4 = 8
        let acts = out[5] as usize;
        let mut saw_start = false;
        for a in 0..acts {
            let b = OUT_HEADER + a * OUT_ACT;
            if out[b] == ACT_GOAL_START {
                saw_start = true;
                assert_eq!(out[b + 1], 8, "the stroll goal starts");
            }
        }
        assert!(saw_start, "the stroll start action must be emitted");
    }

    #[test]
    fn no_action_time_blocks_stroll_but_not_looks() {
        // noActionTime >= 100 → the stroll gate short-circuits BEFORE the
        // nextInt pull (the vanilla body returns false without consuming RNG)
        let seed0 = 555u64;
        let mut inb = build_in(seed0, 100, 2, 200);
        let mut out = vec![0i32; OUT_HEADER + MAX_ACTS * OUT_ACT];
        let st = drive(&mut inb, &mut out, 0);
        assert_eq!(st, ST_DONE);
        // reference: 3 nearest gates + lookat + lookaround (NO stroll pull)
        let mut r = Lcg { value: seed0 };
        for _ in 0..3 {
            r.next_int_bound(10);
        }
        r.next_f32();
        r.next_f32();
        let seed1 = ((out[4] as u64) << 32) | (out[3] as u64 & 0xFFFF_FFFF);
        assert_eq!(seed1, r.value);
    }

    #[test]
    fn fallback_on_unknown_kind() {
        let mut inb = build_in(1, 100, 2, 0);
        inb[IN_HEADER] = GK_UNKNOWN; // the first target goal's kind slot
        let mut out = vec![0i32; OUT_HEADER + MAX_ACTS * OUT_ACT];
        let st = drive(&mut inb, &mut out, 0);
        assert_eq!(st, ST_FALLBACK);
    }

    #[test]
    fn lcg_matches_java_random() {
        // java.util.Random(42) reference: nextInt(60), nextFloat(), nextDouble()
        // computed with the same recurrence (value starts at seed ^ MUL & MASK)
        let mut r = Lcg {
            value: (42u64 ^ 25214903917) & 0xFFFF_FFFF_FFFF,
        };
        let a = r.next_int_bound(60);
        let b = r.next_f32();
        let c = r.next_f64();
        // JDK reference vectors (Random(42): nextInt(60), nextFloat, nextDouble)
        assert_eq!(a, 50);
        assert_eq!(b.to_bits(), 0.054665208f32.to_bits());
        assert_eq!(c.to_bits(), 0.6832234717598454f64.to_bits());
    }

    #[test]
    fn wrap_matches_mth() {
        assert_eq!(wrap_degrees(190.0f32), -170.0f32);
        assert_eq!(wrap_degrees(-190.0f32), 170.0f32);
        assert_eq!(wrap_degrees(360.0f32 * 3.0f32 + 10.0f32), 10.0f32);
        assert_eq!(degrees_difference(350.0f32, 10.0f32), 20.0f32);
    }

    #[test]
    fn rotate_matches_control() {
        assert_eq!(rotate_towards(350.0f32, 10.0f32, 5.0f32), 355.0f32);
        assert_eq!(rotate_towards(0.0f32, 90.0f32, 5.0f32), 5.0f32);
    }

    #[test]
    fn positive_ceil_div_matches() {
        assert_eq!(positive_ceil_div(120, 2), 60);
        assert_eq!(positive_ceil_div(121, 2), 61);
        assert_eq!(positive_ceil_div(40, 2), 20);
    }
}
