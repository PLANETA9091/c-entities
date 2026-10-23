//! Local reproduction of the CI 100k-fill native explosion: run the item
//! pipeline the way the module flush does — a growing batch of 100k items,
//! one flush per tick, 600+ ticks — against an in-memory flat-world section
//! provider, and print the process RSS trajectory. If Rust-side code
//! allocates-and-retains at ~130KB/item, RSS explodes here too (no JVM).

use centities_core::sections::{SectionData, SectionProvider};
use centities_core::tick::{tick_item, ItemState, Scratch, TickEnv};
use std::cell::Cell;
use std::rc::Rc;

fn rss_mb() -> f64 {
    let s = std::fs::read_to_string("/proc/self/statm").unwrap_or_default();
    let mut it = s.split_whitespace();
    let _total = it.next();
    it.next()
        .and_then(|v| v.parse::<f64>().ok())
        .map(|p| p * 4.0 / 1024.0)
        .unwrap_or(0.0)
}

fn main() {
    let n: usize = std::env::args()
        .nth(1)
        .and_then(|v| v.parse().ok())
        .unwrap_or(100_000);
    let ticks: usize = std::env::args()
        .nth(2)
        .and_then(|v| v.parse().ok())
        .unwrap_or(600);
    println!(
        "repro: {n} items x {ticks} ticks, start rss {:.0}MB",
        rss_mb()
    );

    let tenv = TickEnv {
        min_section_y: -4,
        max_section_y: 19,
        void_damage: false,
        void_threshold: -64.0,
        border_min_x: -29_999_984.0,
        border_max_x: 29_999_984.0,
        border_min_z: -29_999_984.0,
        border_max_z: 29_999_984.0,
        border_margin: 2.0,
    };

    // items in a 150x150 block area, falling from y=50 onto a floor at y=0
    let mut states: Vec<ItemState> = Vec::with_capacity(n);
    for i in 0..n {
        let fx = (i % 150) as f64;
        let fz = (i / 150) as f64;
        let y = 50.0 - ((i * 7) % 110) as f64 * 0.1;
        states.push(ItemState {
            x: fx,
            y,
            z: fz,
            dx: 0.0,
            dy: 0.0,
            dz: 0.0,
            xo: fx,
            yo: y,
            zo: fz,
            on_ground: false,
            fall_dist: 0.0,
            tick_count: (i as i32) % 4000,
            age: 0,
            pickup_delay: if i % 3 == 0 { 10 } else { 0 },
            despawn_rate: 6000,
            has_impulse: false,
            stuck: (0.0, 0.0, 0.0),
            sup_x: 0,
            sup_y: 0,
            sup_z: 0,
            sup_present: false,
            on_ground_no_blocks: false,
            fr_pos: (i32::MIN, i32::MIN, i32::MIN),
            friction: 0.6,
            speed: 1.0,
            fr_step_special: false,
            friction_disabled: false,
            land_fall: 0.0,
        });
    }

    // one ItemWorld-equivalent: the module's store grows lazily per entry —
    // emulate with a Vec of fresh entries keyed by id slot (the real one is
    // 1<<17 slots of ~137B; preallocating here matches the module).
    struct StoredEntry {
        tick_count: i32,
        despawn_rate: i32,
    }
    let mut store: Vec<StoredEntry> = (0..(1 << 17))
        .map(|_| StoredEntry {
            tick_count: 0,
            despawn_rate: 6000,
        })
        .collect();

    let mut grid = centities_core::sections::SectionGrid::new();
    let mut scratch = Scratch::new();
    let provider_calls = Cell::new(0u64);
    let tick_counter = Cell::new(0u32);
    let TICK = tick_counter.clone();
    let cells_all = (
        Rc::new(Cell::new(i32::MAX)),
        Rc::new(Cell::new(i32::MIN)),
        Rc::new(Cell::new(i32::MAX)),
        Rc::new(Cell::new(i32::MIN)),
        Rc::new(Cell::new(i32::MAX)),
        Rc::new(Cell::new(i32::MIN)),
    );
    let (MINX, MAXX, MINY, MAXY, MINZ, MAXZ) = (
        cells_all.0.clone(),
        cells_all.1.clone(),
        cells_all.2.clone(),
        cells_all.3.clone(),
        cells_all.4.clone(),
        cells_all.5.clone(),
    );

    {
        let pc = &provider_calls;
        // A solid floor at the section's lowest y layer: exercises the
        // collide + landing paths (the all-air repro never collided and
        // missed the CI explosion — the bug lives in the collide/landing
        // neighborhood).
        let tick_cell = TICK.clone();
        let mut provider =
            move |cx: i32, cy: i32, _cz: i32, _load: bool| -> Result<SectionData, ()> {
                pc.set(pc.get() + 1);
                // per-tick coordinate extremes (same cells the logger reads)
                MINX.set(MINX.get().min(cx));
                MAXX.set(MAXX.get().max(cx));
                MINY.set(MINY.get().min(cy));
                MAXY.set(MAXY.get().max(cy));
                MINZ.set(MINZ.get().min(_cz));
                MAXZ.set(MAXZ.get().max(_cz));
                let _ = tick_cell;
                let _ = (&MINX, &MAXX, &MINY, &MAXY, &MINZ, &MAXZ);
                if cy != 0 {
                    return Ok(SectionData::all_air());
                }
                // one palette entry: a full-cube collider, friction 0.6, speed 1.0
                let floor = centities_core::sections::PaletteEntry {
                    colliders: vec![centities_core::itemtick::Aabb6::new(
                        0.0, 0.0, 0.0, 1.0, 1.0, 1.0,
                    )],
                    friction: 0.6,
                    speed: 1.0,
                    is_water: false,
                    has_fluid: false,
                    cobweb: false,
                    entity_inside_special: false,
                    step_on_special: false,
                    fall_on_special: false,
                };
                let mut sd = SectionData::all_air();
                use centities_core::sections::PaletteEntry;
                sd.entries = vec![PaletteEntry::air(), floor];
                sd.blocks = vec![centities_core::sections::AIR_IDX; 4096];
                // fill y=0 layer (local y 0) with palette 1
                for lz in 0..16u16 {
                    for lx in 0..16u16 {
                        sd.blocks[(lx | (lz << 4)) as usize] = 1;
                    }
                }
                let _ = cx;
                Ok(sd)
            };
        let prov: SectionProvider = &mut provider;

        let t0 = std::time::Instant::now();
        for t in 0..ticks {
            tick_counter.set(t as u32);
            cells_all.0.set(i32::MAX);
            cells_all.1.set(i32::MIN);
            cells_all.2.set(i32::MAX);
            cells_all.3.set(i32::MIN);
            cells_all.4.set(i32::MAX);
            cells_all.5.set(i32::MIN);
            // emulate the flush: batch of items accumulated this tick
            for (k, st) in states.iter_mut().enumerate() {
                st.tick_count = st.tick_count.wrapping_add(1);
                let entry = &mut store[k & ((1 << 17) - 1)];
                entry.tick_count = st.tick_count;
                let _outcome = tick_item(
                    st,
                    (k as i64) | (3 << 17),
                    &tenv,
                    &mut grid,
                    &mut scratch,
                    prov,
                );
                // outcome ignored: the module commits + writebacks; the memory
                // story is identical (commit = field stores, no allocation).
            }
            if t % 10 == 0 || t == ticks - 1 {
                let el = t0.elapsed().as_millis() as f64 / 1000.0;
                let (mut md, mut mp) = (0.0f64, 0.0f64);
                for st in &states {
                    let ad = st.dx.abs().max(st.dy.abs()).max(st.dz.abs());
                    if ad.is_finite() {
                        md = md.max(ad);
                    }
                    let ap = st.x.abs().max(st.y.abs()).max(st.z.abs());
                    if ap.is_finite() {
                        mp = mp.max(ap);
                    }
                }
                println!(
                "tick {t:4} rss {:.0}MB grid={} calls={} elapsed {:.1}s maxD={md:.3} maxP={mp:.3} secX[{}..{}] secY[{}..{}] secZ[{}..{}]",
                rss_mb(),
                grid.populated.load(std::sync::atomic::Ordering::Relaxed),
                provider_calls.get(),
                el,
                cells_all.0.get(), cells_all.1.get(),
                cells_all.2.get(), cells_all.3.get(),
                cells_all.4.get(), cells_all.5.get(),
            );
            }
        }
        println!(
            "DONE rss {:.0}MB, ticks={}, provider_calls={} (up-calls)",
            rss_mb(),
            ticks,
            provider_calls.get()
        );
    } // provider scope
}
