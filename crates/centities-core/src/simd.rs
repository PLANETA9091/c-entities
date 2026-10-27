//! AVX2 candidate prefilter for the exact query scan (doc 21).
//!
//! The cell box store is a blocked-SoA layout: one block of 8 candidates,
//! six contiguous segments of 8 f32 each (x0,y0,z0,x1,y1,z1):
//!
//! ```text
//!   offset(candidate i, segment s) = (i >> 3) * 48 + (s << 3) + (i & 7)
//! ```
//!
//! The prefilter runs the vanilla intersects test on 8 candidates at once
//! with 6 aligned-ish loads and zero shuffles, then every survivor is
//! re-checked by the EXACT f64-promoted compare (the pre-L5 semantics,
//! bit-for-bit). The prefilter is a provable SUPERSET of the exact check:
//!
//! * stored coords are f32 cell-relative (|v| < 96) or absolute f32 y;
//! * prefilter bounds are the exact bounds expanded by EPSF = 1e-2 blocks;
//! * the f32 rounding of the (f64 - f64 cell origin) bound is <= ULP(64)
//!   = 7.6e-6, which is 1300x below EPSF;
//! * so `x0f < round_f32(qx1 - ox) + EPSF` holds whenever the exact
//!   `x0f + ox < qx1` holds (and likewise for the five other faces).
//!
//! Therefore the prefilter can never drop a candidate the exact check
//! would keep — no false negatives, result sets identical. Padded lanes
//! (never-allocated block slots) store x0 = +INF / x1 = -INF and reject
//! themselves; lanes past `n` in the last block are masked out of the
//! movemask, so stale data after swap-removes is unreachable.

#![allow(unsafe_code)] // masked AVX2 loads over in-bounds blocked-SoA arrays

use std::sync::atomic::{AtomicU8, Ordering};

/// Runtime feature state: 0 = unknown, 1 = absent, 2 = present.
static AVX2: AtomicU8 = AtomicU8::new(0);

#[inline]
pub fn use_avx2() -> bool {
    match AVX2.load(Ordering::Relaxed) {
        2 => true,
        1 => false,
        _ => {
            #[cfg(target_arch = "x86_64")]
            {
                let has = std::arch::is_x86_feature_detected!("avx2");
                AVX2.store(if has { 2 } else { 1 }, Ordering::Relaxed);
                has
            }
            #[cfg(not(target_arch = "x86_64"))]
            {
                AVX2.store(1, Ordering::Relaxed);
                false
            }
        }
    }
}

/// Test hook: force a mode (0 = auto-detect, 1 = scalar, 2 = AVX2).
#[doc(hidden)]
pub fn set_simd_mode(m: u8) {
    AVX2.store(m, Ordering::Relaxed);
}

/// The exact f64-promoted intersect on the blocked-SoA layout — the single
/// source of truth for what gets pushed (both scans funnel here).
#[inline]
pub(crate) fn exact_keep(
    bx: &[f32],
    i: usize,
    ox: f64,
    oz: f64,
    qx0: f64,
    qx1: f64,
    qz0: f64,
    qz1: f64,
    my0: f32,
    my1: f32,
) -> bool {
    let b = (i >> 3) * 48 + (i & 7);
    // y first: most rejections win here (same ordering as the pre-L5 scan)
    if !(bx[b + 8] < my1 && bx[b + 32] > my0) {
        return false;
    }
    let x0 = bx[b] as f64 + ox;
    let x1 = bx[b + 24] as f64 + ox;
    let z0 = bx[b + 16] as f64 + oz;
    let z1 = bx[b + 40] as f64 + oz;
    x0 < qx1 && x1 > qx0 && z0 < qz1 && z1 > qz0
}

/// Scalar exact scan (the reference path; also the non-x86_64 fallback).
#[inline]
pub(crate) fn scan_scalar(
    bx: &[f32],
    ids: &[i64],
    n: usize,
    ox: f64,
    oz: f64,
    qx0: f64,
    qx1: f64,
    qz0: f64,
    qz1: f64,
    my0: f32,
    my1: f32,
    out: &mut Vec<i64>,
) {
    for i in 0..n {
        if exact_keep(bx, i, ox, oz, qx0, qx1, qz0, qz1, my0, my1) {
            out.push(ids[i]);
        }
    }
}

/// Blocked-SoA accessor (single source of truth for the layout math).
#[inline]
pub(crate) fn box_at(bx: &[f32], i: usize, s: usize) -> f32 {
    bx[(i >> 3) * 48 + (s << 3) + (i & 7)]
}

#[cfg(target_arch = "x86_64")]
mod avx2 {
    use std::arch::x86_64::*;

    /// AVX2 prefilter over whole blocks of 8, scalar-exact re-check for
    /// survivors. `n_blocks` covers ceil(n/8); lanes of the last block past
    /// `rem` are masked out. Every load is within the allocated block
    /// (cell_add extends the full 48-f32 block before the first write).
    ///
    /// SAFETY: caller guarantees AVX2 is available and
    /// bx.len() >= n_blocks * 48. Loads stay in-bounds by construction.
    #[target_feature(enable = "avx2")]
    pub(crate) unsafe fn scan_blocks(
        bx: &[f32],
        ids: &[i64],
        n_blocks: usize,
        last_lane_mask: u32,
        px0: f32,
        px1: f32,
        pz0: f32,
        pz1: f32,
        py0: f32,
        py1: f32,
        ox: f64,
        oz: f64,
        qx0: f64,
        qx1: f64,
        qz0: f64,
        qz1: f64,
        my0: f32,
        my1: f32,
        out: &mut Vec<i64>,
    ) {
        let vx0 = _mm256_set1_ps(px0);
        let vx1 = _mm256_set1_ps(px1);
        let vz0 = _mm256_set1_ps(pz0);
        let vz1 = _mm256_set1_ps(pz1);
        let vy0 = _mm256_set1_ps(py0);
        let vy1 = _mm256_set1_ps(py1);

        for blk in 0..n_blocks {
            let p = bx.as_ptr().add(blk * 48);
            let x0 = _mm256_loadu_ps(p);
            let y0 = _mm256_loadu_ps(p.add(8));
            let z0 = _mm256_loadu_ps(p.add(16));
            let x1 = _mm256_loadu_ps(p.add(24));
            let y1 = _mm256_loadu_ps(p.add(32));
            let z1 = _mm256_loadu_ps(p.add(40));
            // vanilla intersects: strict inequalities, both faces
            let mx = _mm256_and_ps(
                _mm256_cmp_ps(x0, vx1, _CMP_LT_OS),
                _mm256_cmp_ps(x1, vx0, _CMP_GT_OS),
            );
            let mz = _mm256_and_ps(
                _mm256_cmp_ps(z0, vz1, _CMP_LT_OS),
                _mm256_cmp_ps(z1, vz0, _CMP_GT_OS),
            );
            let my = _mm256_and_ps(
                _mm256_cmp_ps(y0, vy1, _CMP_LT_OS),
                _mm256_cmp_ps(y1, vy0, _CMP_GT_OS),
            );
            let m = _mm256_and_ps(mx, _mm256_and_ps(mz, my));
            let mut bits = _mm256_movemask_ps(m) as u32;
            if blk + 1 == n_blocks {
                bits &= last_lane_mask; // stale/garbage lanes past n: unreachable
            }
            let base = blk << 3;
            while bits != 0 {
                let j = bits.trailing_zeros() as usize;
                bits &= bits - 1;
                let k = base + j;
                if super::exact_keep(bx, k, ox, oz, qx0, qx1, qz0, qz1, my0, my1) {
                    out.push(ids[k]);
                }
            }
        }
    }

    /// AVX2 cell scan for the packed (row publisher) read: returns the
    /// x0/x1 f32 pair per candidate via the blocked-SoA accessor is scalar
    /// anyway (publish is not the hot path) — nothing vectorized here.
    #[allow(dead_code)]
    pub(super) const _PREFILTER_ONLY: () = ();
}

/// Safe dispatch used by `EntityIndex::query`: runtime-detects AVX2 and runs
/// the prefilter+exact pipeline, or the scalar exact scan. All `unsafe`
/// stays in this module — the query path itself is safe code.
#[inline]
pub(crate) fn scan_cells(
    bx: &[f32],
    ids: &[i64],
    n: usize,
    ox: f64,
    oz: f64,
    qx0: f64,
    qx1: f64,
    qz0: f64,
    qz1: f64,
    my0: f32,
    my1: f32,
    out: &mut Vec<i64>,
) {
    // EPSF-expanded f32 prefilter bounds (superset proof at the top of the
    // file). Cell-relative XZ: only cells the expanded box touches are
    // scanned, so |qx? - origin| <= CELL + ex — comfortably f32-normal.
    const EPSF: f32 = 1.0e-2;

    #[cfg(target_arch = "x86_64")]
    {
        if use_avx2() {
            let px0 = ((qx0 - ox) as f32) - EPSF;
            let px1 = ((qx1 - ox) as f32) + EPSF;
            let pz0 = ((qz0 - oz) as f32) - EPSF;
            let pz1 = ((qz1 - oz) as f32) + EPSF;
            let py0 = my0 - EPSF;
            let py1 = my1 + EPSF;
            let n_blocks = (n + 7) >> 3;
            let rem = n - ((n_blocks - 1) << 3);
            let lane_mask: u32 = if rem >= 8 {
                u32::MAX
            } else {
                (1u32 << rem) - 1
            };
            // SAFETY: AVX2 was just detected; bx always holds whole blocks
            // (cell_add extends the 48-f32 block before its first write),
            // so every load stays in-bounds; lanes past n are masked out of
            // the movemask, and padded lanes reject themselves.
            unsafe {
                avx2::scan_blocks(
                    bx, ids, n_blocks, lane_mask, px0, px1, pz0, pz1, py0, py1, ox, oz, qx0, qx1,
                    qz0, qz1, my0, my1, out,
                );
            }
            return;
        }
    }
    scan_scalar(bx, ids, n, ox, oz, qx0, qx1, qz0, qz1, my0, my1, out);
}
