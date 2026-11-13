//! L8 flush plumbing test: the rayon chunk-zip must pair each item's state
//! with the right od/oi/recs slots, and every tickable item must produce a
//! commit record. Under the L119 vanilla %4 move gate (javap ItemEntity.tick
//! 26.3-249 offsets 226-265) a grounded small-horiz item carries the skip
//! tick's gravity (dy = -0.04 when (tc+id)%4 != 0) and zeroes it on the
//! gate tick — the per-item phase is computable exactly, so the whole
//! 5000-item pattern is asserted.

use centities_core::itemtick::Aabb6;
use centities_core::sections::{PaletteEntry, SectionCache, SectionData, SectionGrid};
use centities_core::tick::{tick_item, ItemState, Scratch, TickEnv, TickOutcome};

fn flat_world() -> impl FnMut(i32, i32, i32, bool) -> Result<SectionData, ()> {
    let ground = PaletteEntry {
        colliders: vec![Aabb6::new(0.0, 0.0, 0.0, 1.0, 1.0, 1.0)],
        friction: 0.6,
        speed: 1.0,
        is_water: false,
        has_fluid: false,
        cobweb: false,
        entity_inside_special: false,
        step_on_special: false,
        fall_on_special: false,
    };
    move |_cx, cy, _cz, _load| {
        if cy != -4 {
            return Err(()); // unloaded above/below: same shape as the bench farm
        }
        let ground = ground.clone();
        Ok(SectionData {
            entries: vec![PaletteEntry::air(), ground],
            blocks: {
                let mut b = vec![0u16; 4096]; // 0 = the air palette entry
                for y in 0..4 {
                    for z in 0..16 {
                        for x in 0..16 {
                            b[x | (z << 4) | (y << 8)] = 1; // ground at the bottom 4 layers
                        }
                    }
                }
                b
            },
            complex: false,
            any_fluid: false,
            any_inside_special: false,
        })
    }
}

fn env() -> TickEnv {
    TickEnv {
        min_section_y: -4,
        max_section_y: 7,
        void_damage: false,
        void_threshold: -64.0,
        border_min_x: -29_999_984.0,
        border_max_x: 29_999_984.0,
        border_min_z: -29_999_984.0,
        border_max_z: 29_999_984.0,
        border_margin: 2.0,
            cur_tick: 0,
            verdict_owned: false,
    }
}

#[test]
fn parallel_chunk_zip_commits_every_tickable_item() {
    const N: usize = 5000; // > 512: several chunks; not a multiple of 512
    let tenv = env();
    let grid = SectionGrid::new();

    // 5000 resting items on the flat ground (y = 60.0 = -64 + 4*... section
    // -4 covers blocks -64..-49; ground top at -64+4 = -60... use y=-60.0).
    let mut sts: Vec<ItemState> = (0..N)
        .map(|i| ItemState {
            x: (i % 100) as f64 * 0.5,
            y: -60.0,
            z: (i / 100) as f64 * 0.5,
            dx: 0.0,
            dy: 0.0,
            dz: 0.0,
            xo: (i % 100) as f64 * 0.5,
            yo: -60.0,
            zo: (i / 100) as f64 * 0.5,
            on_ground: true,
            tick_count: (i % 37) as i32,
            age: 100,
            pickup_delay: 0,
            despawn_rate: 6000,
            ..ItemState::new()
        })
        .collect();
    let ids: Vec<i64> = (0..N as i64).collect();
    let flags: Vec<i32> = vec![0; N]; // active
    let tickable: Vec<bool> = vec![true; N];
    let mut od = vec![0.0f64; N * 8];
    let mut oi = vec![0i32; N * 4];
    let mut recs: Vec<Option<u32>> = (0..N).map(|_| None).collect(); // outcome bits

    {
        let grid_ref = &grid;
        use rayon::prelude::*;
        sts.par_chunks_mut(512)
            .zip(od.par_chunks_mut(512 * 8))
            .zip(oi.par_chunks_mut(512 * 4))
            .zip(recs.par_chunks_mut(512))
            .zip(tickable.par_chunks(512))
            .zip(ids.par_chunks(512))
            .zip(flags.par_chunks(512))
            .for_each(|((((((st_c, od_c), oi_c), rec_c), tk_c), id_c), f_c)| {
                let mut scratch = Scratch::new();
                let mut provider = flat_world();
                let mut cache = SectionCache::new(grid_ref, &mut provider);
                for j in 0..st_c.len() {
                    if !tk_c[j] {
                        continue;
                    }
                    let outcome = tick_item(&mut st_c[j], id_c[j], &tenv, &mut cache, &mut scratch);
                    let mut ob = 0i32;
                    if outcome.has(TickOutcome::MOVED) {
                        ob |= 1;
                    }
                    if outcome.has(TickOutcome::ON_GROUND) {
                        ob |= 2;
                    }
                    od_c[j * 8] = st_c[j].dx;
                    od_c[j * 8 + 1] = st_c[j].dy;
                    od_c[j * 8 + 4] = st_c[j].y;
                    oi_c[j * 4] = st_c[j].age;
                    oi_c[j * 4 + 2] = ob;
                    rec_c[j] = Some(ob as u32);
                }
            });
    }

    let committed = recs.iter().filter(|r| r.is_some()).count();
    assert_eq!(
        committed, N,
        "every tickable item must produce a commit record"
    );
    // L119 the vanilla %4 move gate, UNIVERSAL over 5000 items: the gate
    // fires when ((tc + id) % 4 == 0) — the move's distance-zero sweep
    // zeroes dy; the skip ticks carry gravity (dy = -0.04). The position
    // is untouched, ON_GROUND is set either way, age 100 -> 101 proves the
    // body ran (both paths reach the tail), and the rec bits are
    // ON_GROUND only (the distance-zero sweep sets no MOVED).
    for k in 0..N {
        let tc = (k % 37) as i32;
        let phase = (tc + k as i32) % 4;
        assert_eq!(
            od[k * 8 + 1],
            if phase == 0 { 0.0 } else { -0.04 },
            "item {k}: the %4 dy pattern must hold"
        );
        assert_eq!(od[k * 8], 0.0, "item {k}: dx stays zero");
        assert_eq!(od[k * 8 + 4], -60.0, "item {k}: y untouched");
        assert_eq!(oi[k * 4], 101, "item {k}: age incremented");
        assert_eq!(
            recs[k],
            Some(2u32),
            "item {k}: ON_GROUND only — no MOVED, no flags"
        );
    }
    // Slots must not cross: item k's rec must equal its own oi bits.
    for k in (0..N).step_by(977) {
        assert_eq!(
            recs[k],
            Some(oi[k * 4 + 2] as u32),
            "rec/oi slot mismatch at {k}"
        );
    }
}
