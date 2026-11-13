//! L119 merge-scan margin test: does the index's exact query find a
//! candidate whose box overlaps the scan box by a 0.006-tall y band?
//! (The t=159 vanilla event: 971's scan box [-59.243863, -58.993863] vs
//! 1827's spawn box [-59.0, -58.75] — the marginal [-59.0, -58.993863].)

use centities_core::EntityIndex;
use centities_core::Aabb;

#[test]
fn merge_scan_marginal_y_overlap_is_found() {
    let mut idx = EntityIndex::new();
    // 1827's spawn box: the item bb at (38.555956, -59.0, -63.578423),
    // dims 0.25x0.25x0.25 (x-0.125..x+0.125, y..y+0.25, z-0.125..z+0.125)
    let cand = Aabb::new(
        38.555956 - 0.125,
        -59.0,
        -63.578423 - 0.125,
        38.555956 + 0.125,
        -58.75,
        -63.578423 + 0.125,
    );
    idx.upsert_ord(1827, 0, &cand, -4, 7777);

    // 971's scan-time box: the bb at (39.011796, -59.243863, -64.217284)
    // inflated by (0.5, 0.0, 0.5)
    let qbox = Aabb::new(
        39.011796 - 0.125 - 0.5,
        -59.243863,
        -64.217284 - 0.125 - 0.5,
        39.011796 + 0.125 + 0.5,
        -59.243863 + 0.25,
        -64.217284 + 0.125 + 0.5,
    );
    let mut out = Vec::new();
    idx.query(0, &qbox, &mut out);
    assert!(
        out.contains(&1827),
        "the marginal [-59.0, -58.993863] overlap must be found; got {:?}",
        out
    );
}

#[test]
fn merge_scan_exact_zero_overlap_boundary() {
    let mut idx = EntityIndex::new();
    // the candidate sits ENTIRELY ABOVE the query's maxY: its minY EQUALS
    // the query's maxY exactly — vanilla AABB.intersects is strict
    // (other.minY < this.maxY is FALSE) so a touch is NOT an intersect and
    // the index must agree (the L119 view trim's boundary contract).
    let cand = Aabb::new(-0.125, -58.75, -0.125, 0.125, -58.5, 0.125);
    idx.upsert_ord(1, 0, &cand, -4, 1);
    let qbox = Aabb::new(-0.625, -59.25, -0.625, 0.625, -58.75, 0.625);
    let mut out = Vec::new();
    idx.query(0, &qbox, &mut out);
    // the raw query may keep the eps-slack superset; the VIEW trim (the
    // n_merge_scan's strict intersect on box_of) must reject it
    let kept: Vec<i64> = out
        .iter()
        .filter(|&&id| {
            match idx.box_of(0, id) {
                Some(b) => {
                    b[3] > qbox.x0 && b[0] < qbox.x1
                        && b[4] > qbox.y0 && b[1] < qbox.y1
                        && b[5] > qbox.z0 && b[2] < qbox.z1
                }
                None => false,
            }
        })
        .copied()
        .collect();
    assert!(
        !kept.contains(&1),
        "the exact-touch boundary must NOT survive the strict view trim"
    );
}
