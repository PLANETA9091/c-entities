//! L6 — native item tick core.
//!
//! A bit-exact port of the Paper 1.21.8 ItemEntity tick COMPUTATION:
//! ItemEntity.tick + Entity.tick/baseTick/move/collide + moonrise
//! performCollisions, as pure Rust over caller-owned state. No unsafe, no
//! deps, no allocation on the steady-state path.
//!
//! Owner directive (binding): the tick is COMPUTED here, never skipped.
//! Items whose tick cannot be computed exactly by this core (fluids, complex
//! collision shapes, world border, portals, fire, vehicles) are routed to
//! the vanilla Java path by the module — they still tick, in Java.
//!
//! Exactness notes (all verified against the decompiled sources):
//! * Java float literals stay f32 and are promoted to f64 exactly like Java
//!   does: `(0.98f32 as f64) == 0.9800000190734863`.
//! * Gravity 0.04 is a Java double literal.
//! * moonrise collide epsilons are 1.0e-7 (double) everywhere.
//! * Mth.floor == `x.floor() as i32` for our ranges (floor div matches on
//!   negatives; Java Math.floor of NaN never occurs here).
//! * The move gate is `!onGround || horizSqr > (1.0e-5f32 as f64)
//!   || (tickCount + id) % 4 == 0`.
//! * Merge cadence: `tickCount % (floor_changed ? 2 : 40) == 0`.
//! * checkFallDamage casts the fall delta to f32 before subtracting.
//! * Friction on ground: `(block_friction_f32 * 0.98f32) as f64` (the f32
//!   multiply happens in float in Java), else `(0.98f32 as f64)`.

#![allow(clippy::excessive_precision)]

/// moonrise collision epsilon (double), used by gather + all axis clips.
pub const EPS7: f64 = 1.0e-7;
/// `Entity.move` setPos threshold (double literal 1.0E-7).
pub const MOVE_EPS: f64 = 1.0e-7;
/// ItemEntity gravity (Java double literal 0.04).
pub const ITEM_GRAVITY: f64 = 0.04;
/// Java f32 literal 0.98 promoted to f64 — `multiply(f, 0.98, f)` uses the
/// f64-promoted float, NOT the f64 0.98.
pub const FRICTION_F32_AS_F64: f64 = 0.98f32 as f64;
/// Air/ground drag applied to Y every friction step (double literal 0.98).
/// Vanilla Y drag = `getAirDrag()` — a FLOAT 0.98F widened to double
/// (0.9800000190734863). The f64 literal 0.98 is a DIFFERENT number and
/// shifts the item's vy by 1 ULP per tick (the parity trace caught it:
/// items-T389 id 2, t=142, first ULP divergence of vy mid-air).
pub const Y_DRAG: f64 = 0.98f32 as f64;
/// Move gate float threshold: `horizontalDistanceSqr() > 1.0E-5F`.
pub const GATE_HORZ_SQR: f64 = 1.0e-5f32 as f64;
/// ItemEntity dimensions: width 0.25F, height 0.25F (EntityType.ITEM).
pub const ITEM_HALF_W: f64 = 0.125;
pub const ITEM_HEIGHT: f64 = 0.25;
/// World-space AABB (6 doubles), same field order as Java AABB.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Aabb6 {
    pub x0: f64,
    pub y0: f64,
    pub z0: f64,
    pub x1: f64,
    pub y1: f64,
    pub z1: f64,
}

impl Aabb6 {
    #[inline]
    pub fn new(x0: f64, y0: f64, z0: f64, x1: f64, y1: f64, z1: f64) -> Self {
        Aabb6 {
            x0,
            y0,
            z0,
            x1,
            y1,
            z1,
        }
    }

    /// `EntityDimensions.makeBoundingBox` for ItemEntity (w/2 and h are
    /// floats: 0.125 and 0.25 are exact in f32, promotion is exact).
    #[inline]
    pub fn item_box(x: f64, y: f64, z: f64) -> Self {
        Aabb6 {
            x0: x - ITEM_HALF_W,
            y0: y,
            z0: z - ITEM_HALF_W,
            x1: x + ITEM_HALF_W,
            y1: y + ITEM_HEIGHT,
            z1: z + ITEM_HALF_W,
        }
    }

    #[inline]
    pub fn deflate(&self, d: f64) -> Aabb6 {
        Aabb6 {
            x0: self.x0 + d,
            y0: self.y0 + d,
            z0: self.z0 + d,
            x1: self.x1 - d,
            y1: self.y1 - d,
            z1: self.z1 - d,
        }
    }

    #[inline]
    pub fn expand_towards(&self, dx: f64, dy: f64, dz: f64) -> Aabb6 {
        Aabb6 {
            x0: if dx < 0.0 { self.x0 + dx } else { self.x0 },
            y0: if dy < 0.0 { self.y0 + dy } else { self.y0 },
            z0: if dz < 0.0 { self.z0 + dz } else { self.z0 },
            x1: if dx > 0.0 { self.x1 + dx } else { self.x1 },
            y1: if dy > 0.0 { self.y1 + dy } else { self.y1 },
            z1: if dz > 0.0 { self.z1 + dz } else { self.z1 },
        }
    }
}

/// moonrise `CollisionUtil.performCollisions` for the pure-AABB universe
/// (single-cuboid shapes only; complex shapes route vanilla before this
/// runs). Axis order: Y first, then the SMALLER of |x|/|z|, then the other;
/// the reference box shifts after each applied axis exactly as in the
/// decompiled body.
pub fn perform_collisions(
    mv: (f64, f64, f64),
    box_: &Aabb6,
    colliders: &[Aabb6],
) -> (f64, f64, f64) {
    let mut x = mv.0;
    let mut y = mv.1;
    let mut z = mv.2;
    let mut b = *box_;
    if y != 0.0 {
        for t in colliders {
            y = collide_y_axis(t, &b, y);
        }
        if y != 0.0 {
            b.y0 += y;
            b.y1 += y;
        }
    }
    let x_smaller = x.abs() < z.abs();
    if x_smaller && z != 0.0 {
        for t in colliders {
            z = collide_z_axis(t, &b, z);
        }
        if z != 0.0 {
            b.z0 += z;
            b.z1 += z;
        }
    }
    if x != 0.0 {
        for t in colliders {
            x = collide_x_axis(t, &b, x);
        }
        if !x_smaller && x != 0.0 {
            b.x0 += x;
            b.x1 += x;
        }
    }
    if !x_smaller && z != 0.0 {
        for t in colliders {
            z = collide_z_axis(t, &b, z);
        }
    }
    (x, y, z)
}

/// Java `Mth.floor(double)`: truncate toward zero, decrement if below.
#[inline]
pub fn mth_floor(v: f64) -> i32 {
    let i = v as i32;
    if v < i as f64 {
        i - 1
    } else {
        i
    }
}

/// Java `Mth.equal(double, double)` — used for horizontalCollision flags.
#[inline]
pub fn mth_equal(a: f64, b: f64) -> bool {
    (a - b).abs() < 1.0e-7
}

/// Moonrise AABB-vs-AABB gather/intersect epsilon semantics
/// (CollisionUtil.voxelShapeIntersect).
#[inline]
pub fn voxel_shape_intersect(a: &Aabb6, b: &Aabb6) -> bool {
    a.x0 - b.x1 < -EPS7
        && a.x1 - b.x0 > EPS7
        && a.y0 - b.y1 < -EPS7
        && a.y1 - b.y0 > EPS7
        && a.z0 - b.z1 < -EPS7
        && a.z1 - b.z0 > EPS7
}

/// moonrise `collideY(AABB target, AABB source, double move)`.
#[inline]
fn collide_y_axis(t: &Aabb6, src: &Aabb6, mv: f64) -> f64 {
    if !(src.x0 - t.x1 < -EPS7)
        || !(src.x1 - t.x0 > EPS7)
        || !(src.z0 - t.z1 < -EPS7)
        || !(src.z1 - t.z0 > EPS7)
    {
        return mv;
    }
    if mv >= 0.0 {
        let max_move = t.y0 - src.y1;
        if max_move < -EPS7 {
            mv
        } else {
            max_move.min(mv)
        }
    } else {
        let max_move = t.y1 - src.y0;
        if max_move > EPS7 {
            mv
        } else {
            max_move.max(mv)
        }
    }
}

/// moonrise `collideX(AABB target, AABB source, double move)`.
#[inline]
fn collide_x_axis(t: &Aabb6, src: &Aabb6, mv: f64) -> f64 {
    if !(src.y0 - t.y1 < -EPS7)
        || !(src.y1 - t.y0 > EPS7)
        || !(src.z0 - t.z1 < -EPS7)
        || !(src.z1 - t.z0 > EPS7)
    {
        return mv;
    }
    if mv >= 0.0 {
        let max_move = t.x0 - src.x1;
        if max_move < -EPS7 {
            mv
        } else {
            max_move.min(mv)
        }
    } else {
        let max_move = t.x1 - src.x0;
        if max_move > EPS7 {
            mv
        } else {
            max_move.max(mv)
        }
    }
}

/// moonrise `collideZ(AABB target, AABB source, double move)`.
#[inline]
fn collide_z_axis(t: &Aabb6, src: &Aabb6, mv: f64) -> f64 {
    if !(src.x0 - t.x1 < -EPS7)
        || !(src.x1 - t.x0 > EPS7)
        || !(src.y0 - t.y1 < -EPS7)
        || !(src.y1 - t.y0 > EPS7)
    {
        return mv;
    }
    if mv >= 0.0 {
        let max_move = t.z0 - src.z1;
        if max_move < -EPS7 {
            mv
        } else {
            max_move.min(mv)
        }
    } else {
        let max_move = t.z1 - src.z0;
        if max_move > EPS7 {
            mv
        } else {
            max_move.max(mv)
        }
    }
}
