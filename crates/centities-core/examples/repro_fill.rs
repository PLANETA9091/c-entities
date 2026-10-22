//! Local reproduction of the CI 100k-fill native explosion: run the item
//! pipeline the way the module flush does — a growing batch of 100k items,
//! one flush per tick, 600+ ticks — against an in-memory flat-world section
//! provider, and print the process RSS trajectory. If Rust-side code
//! allocates-and-retains at ~130KB/item, RSS explodes here too (no JVM).

use centities_core::sections::{SectionData, SectionProvider};
use centities_core::tick::{tick_item, ItemState, Scratch, TickEnv};
use std::cell::Cell;

fn rss_mb() -> f64 {
    let s = std::fs::read_to_string("/proc/self/statm").unwrap_or_default();
    let mut it = s.split_whitespace();
    let _total = it.next();
    it.next().and_then(|v| v.parse::<f64>().ok()).map(|p| p * 4.0 / 1024.0).unwrap_or(0.0)
}

fn main() {
    let n: usize = std::env::args().nth(1).and_then(|v| v.parse().ok()).unwrap_or(100_000);
    let ticks: usize = std::env::args().nth(2).and_then(|v| v.parse().ok()).unwrap_or(600);
    println!("repro: {n} items x {ticks} ticks, start rss {:.0}MB", rss_mb());

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
            x: fx, y, z: fz,
            dx: 0.0, dy: 0.0, dz: 0.0,
            xo: fx, yo: y, zo: fz,
            on_ground: false,
            fall_dist: 0.0,
            tick_count: (i as i32) % 4000,
            age: 0,
            pickup_delay: if i % 3 == 0 { 10 } else { 0 },
            despawn_rate: 6000,
            has_impulse: false,
            stuck: (0.0, 0.0, 0.0),
            sup_x: 0, sup_y: 0, sup_z: 0,
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
        .map(|_| StoredEntry { tick_count: 0, despawn_rate: 6000 })
        .collect();

    let mut grid = centities_core::sections::SectionGrid::new();
    let mut scratch = Scratch::new();
    let provider_calls = Cell::new(0u64);

    {
        let pc = &provider_calls;
        let mut provider = move |_cx: i32, _cy: i32, _cz: i32, _load: bool| -> Result<SectionData, ()> {
            pc.set(pc.get() + 1);
            Ok(SectionData::all_air())
        };
        let prov: SectionProvider = &mut provider;

    let t0 = std::time::Instant::now();
    for t in 0..ticks {
        // emulate the flush: batch of items accumulated this tick
        for (k, st) in states.iter_mut().enumerate() {
            st.tick_count = st.tick_count.wrapping_add(1);
            let entry = &mut store[k & ((1 << 17) - 1)];
            entry.tick_count = st.tick_count;
            let _outcome = tick_item(st, (k as i64) | (3 << 17), &tenv, &mut grid, &mut scratch, prov);
            // outcome ignored: the module commits + writebacks; the memory
            // story is identical (commit = field stores, no allocation).
        }
        if t % 50 == 0 || t == ticks - 1 {
            let el = t0.elapsed().as_millis() as f64 / 1000.0;
            println!(
                "tick {t:4} rss {:.0}MB grid={} calls={} elapsed {:.1}s",
                rss_mb(),
                grid.populated,
                provider_calls.get(),
                el
            );
        }
    }
    println!(
        "DONE rss {:.0}MB, ticks={}, provider_calls={} (up-calls)",
        rss_mb(), ticks, provider_calls.get()
    );
    } // provider scope
}
