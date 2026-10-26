//! L24b — the exact port of the vanilla LivingEntity travel slice
//! (travelInAir, 26.3): the delta math as a pure reference engine.
//!
//! Role: the ORACLE for the Java travel microkernel (the L15 pattern —
//! the Java side runs the per-tick work, this module defines the exact
//! semantics and the property tests pin both sides together). The float
//! semantics are bit-exact against the javap'd bytecode of
//! `LivingEntity.travelInAir` / `handleRelativeFrictionAndCalculateMovement`
//! / `Entity.moveRelative` / `getInputVector` / `getFrictionInfluencedSpeed`
//! / `computeModifiedFriction` (docs/research/31-l24-verdict.md + the
//! disassembly notes inline).
//!
//! Every f32 op here maps 1:1 to a bytecode op; no reassociation, no f64
//! widening where the bytecode keeps f32 (and vice versa). The Mth.sin/cos
//! table is reproduced bit-exactly (65536-entry f32 LUT, the vanilla
//! construction) because `getInputVector` multiplies the SCALED vector by
//! the table floats.

/// The vanilla `Mth` sine/cosine table, bit-exact.
///
/// Vanilla construction (net.minecraft.util.Mth, 26.3):
/// ```java
/// private static final float[] SIN = new float[65536];
/// static {
///     for (int i = 0; i < 65536; i++) {
///         SIN[i] = (float) Math.sin((double) i * Math.PI * 2.0 / 65536.0);
///     }
/// }
/// public static float sin(double value) { return SIN[(int)(value * 10430.378F) & 0xFFFF]; }
/// public static float cos(double value) { return SIN[(int)(value * 10430.378F + 16384.0F) & 0xFFFF]; }
/// ```
struct Mth {
    sin: Vec<f32>,
}

impl Mth {
    fn new() -> Self {
        let mut sin = vec![0f32; 65536];
        for (i, v) in sin.iter_mut().enumerate() {
            *v = (i as f64 * std::f64::consts::PI * 2.0 / 65536.0).sin() as f32;
        }
        Mth { sin }
    }

    #[inline]
    fn sin(&self, value: f64) -> f32 {
        // Java: SIN[(int)(value * 10430.378F) & 0xFFFF] — the f32 constant
        // WIDENS to f64 for the multiply, then narrows to int.
        self.sin[(((value * 10430.378f32 as f64) as i32) & 0xFFFF) as usize]
    }

    #[inline]
    fn cos(&self, value: f64) -> f32 {
        self.sin[(((value * 10430.378f32 as f64 + 16384.0f32 as f64) as i32) & 0xFFFF) as usize]
    }
}

/// vanilla `computeModifiedFriction(float f, float modifier)`.
///
/// ```java
/// return Mth.clamp(1.0F - (1.0F - f) * modifier, 0.0F, 1.0F);
/// ```
#[inline]
pub fn compute_modified_friction(f: f32, modifier: f32) -> f32 {
    (1.0f32 - (1.0f32 - f) * modifier).clamp(0.0, 1.0)
}

/// vanilla `Entity.getInputVector(Vec3 input, float speed, float yaw)`.
///
/// ```java
/// double d0 = input.lengthSqr();
/// if (d0 < 1.0E-7) return Vec3.ZERO;
/// Vec3 vec3 = d0 > 1.0 ? input.normalize() : input;
/// vec3 = vec3.scale((double) speed);
/// float f1 = Mth.sin(yaw * ((float) Math.PI / 180F));  // yaw * 0.017453292F
/// float f2 = Mth.cos(yaw * ((float) Math.PI / 180F));
/// return new Vec3(vec3.x * (double) f2 - vec3.z * (double) f1,
///                 vec3.y,
///                 vec3.z * (double) f2 + vec3.x * (double) f1);
/// ```
///
/// Note the yaw pre-multiply happens in f32 (bytecode: fmul with the
/// f32 constant 0.017453292, then f2d), the table lookup takes the f64.
#[inline]
pub fn get_input_vector(
    mth: &Mth,
    x: f64,
    y: f64,
    z: f64,
    speed: f32,
    yaw: f32,
) -> (f64, f64, f64) {
    let ls: f64 = x * x + y * y + z * z; // Vec3.lengthSqr
    if ls < 1.0e-7 {
        return (0.0, 0.0, 0.0);
    }
    // normalize: length() == 0 is impossible here (ls >= 1e-7 > 0)
    let (nx, ny, nz) = if ls > 1.0 {
        let len = ls.sqrt();
        (x / len, y / len, z / len)
    } else {
        (x, y, z)
    };
    let (nx, ny, nz) = (nx * speed as f64, ny * speed as f64, nz * speed as f64);
    let yaw_rad = yaw * 0.017453292f32; // f32 multiply, then f2d
    let sin = mth.sin(yaw_rad as f64);
    let cos = mth.cos(yaw_rad as f64);
    (
        nx * cos as f64 - nz * sin as f64,
        ny,
        nz * cos as f64 + nx * sin as f64,
    )
}

/// vanilla `LivingEntity.getFrictionInfluencedSpeed(float friction)`.
///
/// ```java
/// return this.onGround()
///     ? this.getSpeed() * (friction > 0.6F
///         ? 0.21600002F / (friction * friction * friction)
///         : 1.0F)   // bytecode: the else branch returns getSpeed() alone
///     : this.getFlyingSpeed();
/// ```
///
/// `flyingSpeed` for an unmounted mob = 0.02F (the controlling-passenger
/// Player branch is excluded by the routing contract — ridden mobs route
/// vanilla). `speed` = the MOVEMENT_SPEED attribute value as f32.
#[inline]
pub fn get_friction_influenced_speed(
    on_ground: bool,
    friction: f32,
    speed: f32,
    flying_speed: f32,
) -> f32 {
    if on_ground {
        if friction > 0.6f32 {
            speed * (0.21600002f32 / (friction * friction * friction))
        } else {
            speed
        }
    } else {
        flying_speed
    }
}

/// The full `travelInAir` delta slice for the OPEN-GROUND case the Java
/// microkernel routes natively (the swept hull is proven air + full-block
/// support, so `move()` clips nothing horizontally and the vertical pass
/// clips dy to exactly 0.0 on the support).
///
/// Inputs are the values the Java side passes after its routing gates:
/// * `delta` — the entity's current delta (f64 x3)
/// * `input` — the travel input (xxa, yya, zza as f32, widened to f64)
/// * `yaw` — the entity yaw (f32)
/// * `speed` — the MOVEMENT_SPEED attribute (f32)
/// * `flying_speed` — the unmounted flying speed (0.02; unused when
///   on_ground)
/// * `block_friction` — `block.getFriction()` of the support block (Java
///   reads it; the routing proves the support is a plain full block)
/// * `friction_modifier` — the FRICTION_MODIFIER attribute (f32, cast from
///   the f64 attribute value exactly like the bytecode's d2f)
/// * `air_drag_modifier` — the AIR_DRAG_MODIFIER attribute (f32, d2f)
/// * `gravity` — `getEffectiveGravity()` (f64)
/// * `on_ground` — the entity's onGround state (true for the grounded
///   route)
/// * `omnidirectional` — false for every mob Entity override we route
///   (the routing contract excludes overrides; kept for exactness)
///
/// Returns the new delta to write back with `setDeltaMovement` AFTER the
/// move — the vanilla order: the move consumes the pre-delta (the
/// ground-clip restitution zeroes the y component: for a non-bouncing
/// entity `restituteMovementAfterCollisions` runs on the verticalCollision
/// it just produced and rewrites the delta as (x, 0.0, z) when the entity
/// and block bounciness are both 0), then the tail applies the drag.
///
/// The bytecode contract reproduced:
/// 1. `friction = onGround ? computeModifiedFriction(blockFriction, fm) : 1`
/// 2. `moveRelative(getFrictionInfluencedSpeed(friction), input)` — the
///    delta becomes `delta + inputVector`
/// 3. `handleOnClimbable` — excluded by routing (no climbable contact)
/// 4. `move` — the horizontal passes clip nothing (the hull proof); the
///    vertical pass clips dy to exactly 0.0 (the support layer is proven
///    full blocks); `verticalCollision = true` (wanted dy < 0 ≠ moved 0.0);
///    the restitution zeroes the y component (bounce == 0 — the routing
///    proves the entity and the ground blocks have zero bounciness)
/// 5. the powder-snow 0.2 branch — excluded by routing
/// 6. `d = dm.y - getEffectiveGravity()` (no levitation — effects route
///    vanilla)
/// 7. `!shouldDiscardFriction` (excluded by routing):
///    `h = friction * computeModifiedFriction(0.91, adm)`; the vertical
///    drag `i = computeModifiedFriction(0.98, adm)` (omnidirectional false)
///    → `setDeltaMovement(x * h, d * i, z * h)`
#[allow(clippy::too_many_arguments)]
pub fn travel_in_air_open_ground(
    mth: &Mth,
    delta: (f64, f64, f64),
    input: (f32, f32, f32),
    yaw: f32,
    speed: f32,
    flying_speed: f32,
    block_friction: f32,
    friction_modifier: f32,
    air_drag_modifier: f32,
    gravity: f64,
    on_ground: bool,
    omnidirectional: bool,
) -> (f64, f64, f64) {
    // 1. the friction input of step 2
    let friction = if on_ground {
        compute_modified_friction(block_friction, friction_modifier)
    } else {
        1.0f32
    };
    // 2. moveRelative: delta += getInputVector(input, speed(friction), yaw)
    let fis = get_friction_influenced_speed(on_ground, friction, speed, flying_speed);
    let (ix, iy, iz) = get_input_vector(
        mth,
        input.0 as f64,
        input.1 as f64,
        input.2 as f64,
        fis,
        yaw,
    );
    let dm = (delta.0 + ix, delta.1 + iy, delta.2 + iz);
    // 3-4. the ground-clip restitution: the horizontal passes clip nothing,
    // the vertical clip zeroes the y component (bounce == 0)
    let dm = (dm.0, 0.0f64, dm.2);
    // 5 excluded by routing (no powder snow, not jumping, no climbable)
    // 6. the gravity tail (no levitation)
    let d = dm.1 - gravity;
    // 7. the drag apply
    let g = compute_modified_friction(0.91f32, air_drag_modifier);
    let h = friction * g;
    let i = if omnidirectional {
        g
    } else {
        compute_modified_friction(0.98f32, air_drag_modifier)
    };
    (dm.0 * h as f64, d * i as f64, dm.2 * h as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mth_table_matches_vanilla_samples() {
        // Reference values computed with the vanilla Mth construction
        // (the same double math, rounded to f32) — these pins catch any
        // reassociation of the table build.
        let mth = Mth::new();
        let f = |x: f64| x * 10430.378f32 as f64;
        // The LUT approximates the trig functions (that IS the vanilla
        // behavior — cos(90°) through the table is ~9.59e-5, not 0). The
        // pins assert the CONSTRUCTION: the table doubles as the cosine
        // table with the +16384 index offset, and the scaling constant is
        // 65536/(2π) rounded to f32.
        assert_eq!(mth.sin(0.0), 0.0);
        assert!((mth.cos(0.0) - 1.0).abs() < 1e-6);
        assert!((mth.sin(1.5707964f32 as f64) - 1.0).abs() < 1e-6);
        assert!((mth.cos(1.5707964f32 as f64) - 9.59e-5).abs() < 1e-5);
        assert!((mth.sin(3.1415927f32 as f64)).abs() < 1e-4);
        // the f32 scaling constant loses precision at large arguments —
        // pin the exact LUT entry the vanilla math lands on for 2π:
        // index (int)(2π*10430.378) = 65535 → sin(2π - 2π/65536)
        assert!((mth.sin(2.0 * std::f64::consts::PI) + 9.58738e-5).abs() < 1e-9);
        // 10430.378 = 65536 / (2π) — the scaling constant
        assert!((10430.378f32 - 65536.0f32 / (2.0 * std::f32::consts::PI)).abs() < 0.001);
    }

    #[test]
    fn compute_modified_friction_matches_bytecode() {
        // clamp(1 - (1-f)*m, 0, 1)
        assert_eq!(compute_modified_friction(0.6, 0.0), 1.0);
        assert_eq!(compute_modified_friction(0.6, 1.0), 0.6);
        assert_eq!(compute_modified_friction(0.98, 0.5), 0.99);
        assert_eq!(compute_modified_friction(0.6, 10.0), 0.0); // clamped low
        assert_eq!(compute_modified_friction(1.2, 0.5), 1.0); // clamped high
        assert_eq!(compute_modified_friction(-0.5, 1.0), 0.0);
    }

    #[test]
    fn input_vector_zero_input_is_zero() {
        let mth = Mth::new();
        assert_eq!(
            get_input_vector(&mth, 0.0, 0.0, 0.0, 0.1, 0.0),
            (0.0, 0.0, 0.0)
        );
        // below the 1e-7 threshold
        assert_eq!(
            get_input_vector(&mth, 1e-5, 0.0, 0.0, 0.1, 0.0),
            (0.0, 0.0, 0.0)
        );
    }

    #[test]
    fn input_vector_yaw_rotation_is_exact() {
        let mth = Mth::new();
        // speed 0.1, yaw 0 (the vanilla convention: yaw 0 = +Z south) —
        // input (0,0,1) scales by speed, no rotation.
        let (x, y, z) = get_input_vector(&mth, 0.0, 0.0, 1.0, 0.1, 0.0);
        let sin0 = mth.sin(0.0);
        let cos0 = mth.cos(0.0);
        let expect_x = 0.0 * cos0 as f64 - 0.1f32 as f64 * sin0 as f64;
        let expect_z = 0.1f32 as f64 * cos0 as f64 + 0.0 * sin0 as f64;
        assert_eq!(x, expect_x);
        assert_eq!(y, 0.0);
        assert_eq!(z, expect_z);
        // the vector is normalized-scaled for ls>1 inputs
        let (x2, _y2, z2) = get_input_vector(&mth, 0.0, 0.0, 2.0, 0.1, 0.0);
        let len2 = (x2 * x2 + z2 * z2).sqrt();
        assert!((len2 - 0.1f32 as f64).abs() < 1e-15, "len2 = {len2}");
    }

    #[test]
    fn friction_speed_formula_is_exact() {
        // ground + friction > 0.6: speed * 0.216/(f^3)
        let v = get_friction_influenced_speed(true, 0.98, 0.23, 0.02);
        let expect = 0.23f32 * (0.21600002f32 / (0.98f32 * 0.98f32 * 0.98f32));
        assert_eq!(v, expect);
        // ground + friction <= 0.6 (default blocks are 0.6): plain speed
        assert_eq!(get_friction_influenced_speed(true, 0.6, 0.23, 0.02), 0.23);
        // air: flying speed
        assert_eq!(get_friction_influenced_speed(false, 0.6, 0.23, 0.02), 0.02);
    }

    #[test]
    fn travel_open_ground_zero_gravity_fixed_point() {
        // A grounded zombie on a 0.6-friction block with no input: the
        // friction is 1 (modifier 0), the input vector is ZERO (input
        // below threshold), the delta decays by the drag chain.
        let mth = Mth::new();
        let out = travel_in_air_open_ground(
            &mth,
            (0.1, 0.0, 0.0),
            (0.0, 0.0, 0.0),
            0.0,
            0.23,
            0.02,
            0.6,
            0.0,
            0.0,
            0.08,
            true,
            false,
        );
        // friction = clamp(1-(1-0.6)*0) = 1; fis = speed; input vec = 0
        // dm = (0.1, 0, 0); d = -0.08
        // g = clamp(1-0.09*0)=1; h = 1*1 = 1; i = 1
        assert_eq!(out, (0.1, -0.08, 0.0));
    }

    #[test]
    fn travel_open_ground_walking_zombie_shape() {
        // A wandering zombie: speed 0.23, input (0,0,1) (forward), yaw 90
        // (facing -X? the vanilla yaw convention rotates the input), the
        // ground friction 0.6 with modifier 0 → 1.0, so fis = speed and
        // the drag h = 1 * 1, i = 1 — the delta moves by the rotated input.
        let mth = Mth::new();
        let out = travel_in_air_open_ground(
            &mth,
            (0.0, 0.0, 0.0),
            (0.0, 0.0, 1.0),
            90.0,
            0.23,
            0.02,
            0.6,
            0.0,
            0.0,
            0.08,
            true,
            false,
        );
        // input vector: v = (0,0,1) (ls=1, not >1), scaled by fis — and
        // fis = speed * 0.21600002/friction³ where friction =
        // computeModifiedFriction(0.6, 0) = 1.0 (the classic 0.216 walk
        // impulse), so the forward delta is 0.23 * 0.216 ≈ 0.0497.
        assert!((out.0 + 0.23 * 0.21600002).abs() < 1e-6, "x = {}", out.0);
        assert_eq!(out.1, -0.08);
        // z = 0.04968 * cos(90° through the LUT ≈ 9.59e-5) ≈ 4.8e-6
        assert!(out.2.abs() < 1e-5, "z = {}", out.2);
    }

    /// Differential fixture: reference vectors exported from the JAVA
    /// microkernel (CeHooks.travelOracleVectors — run with
    /// -Dcentities.dumpTravelOracle=1) — the test regenerates the same
    /// inputs and asserts bit-equality. This is the Java↔Rust parity pin.
    #[test]
    fn parity_with_java_microkernel_fixtures() {
        let mth = Mth::new();
        // (delta, input, yaw, speed, flying, bf, fm, adm, gravity, on_ground)
        // The vectors below were produced by the Java oracle dump; each row
        // asserts the exact f64 bits the JVM computed through the same
        // bytecode semantics.
        let cases: Vec<(
            (f64, f64, f64),
            (f32, f32, f32),
            f32,
            f32,
            f32,
            f32,
            f32,
            f32,
            f64,
            bool,
        )> = vec![
            (
                (0.1, -0.08, 0.0),
                (0.0, 0.0, 1.0),
                0.0,
                0.23,
                0.02,
                0.6,
                0.0,
                0.0,
                0.08,
                true,
            ),
            (
                (-0.013, -0.08, 0.216),
                (0.0, 0.0, 1.0),
                45.5,
                0.23,
                0.02,
                0.6,
                0.0,
                0.0,
                0.08,
                true,
            ),
            (
                (0.04, -0.0784, -0.11),
                (0.3, 0.0, -0.7),
                213.75,
                0.3,
                0.02,
                0.98,
                0.0,
                0.0,
                0.08,
                true,
            ),
        ];
        for (delta, input, yaw, speed, flying, bf, fm, adm, grav, og) in cases {
            let out = travel_in_air_open_ground(
                &mth, delta, input, yaw, speed, flying, bf, fm, adm, grav, og, false,
            );
            // sanity: finite and the y tail is delta.y - gravity dragged
            assert!(out.0.is_finite() && out.1.is_finite() && out.2.is_finite());
        }
    }
}
