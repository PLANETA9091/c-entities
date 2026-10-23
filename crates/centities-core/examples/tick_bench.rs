//! L6 core bench: where does the per-item tick time go?
use centities_core::itemtick::Aabb6;
use centities_core::sections::{PaletteEntry, SectionData, SectionGrid, AIR_IDX};
use centities_core::tick::{tick_item, ItemState, Scratch, TickEnv};
use std::time::Instant;

const MIN_SEC: i32 = 0;
const MAX_SEC: i32 = 23;

fn flat_env() -> TickEnv {
    TickEnv {
        min_section_y: MIN_SEC,
        max_section_y: MAX_SEC,
        void_damage: false,
        void_threshold: -64.0,
        border_min_x: -29_999_984.0,
        border_max_x: 29_999_984.0,
        border_min_z: -29_999_984.0,
        border_max_z: 29_999_984.0,
        border_margin: 2.0,
    }
}

fn main() {
    let env = flat_env();
    let mut sg = SectionGrid::new();
    let mut scratch = Scratch::new();
    // flat world provider (stone ground at y=63, air above), chunk (0,0) only
    let mut prov = move |cx: i32, cy: i32, cz: i32, _load: bool| -> Result<SectionData, ()> {
        if cx != 0 || cz != 0 || cy < MIN_SEC || cy > MAX_SEC {
            return Err(());
        }
        if cy == 3 {
            let mut data = SectionData::all_air();
            data.entries = vec![
                PaletteEntry {
                    collider: None,
                    friction: 0.6,
                    speed: 1.0,
                    is_water: false,
                    has_fluid: false,
                    cobweb: false,
                    entity_inside_special: false,
                    step_on_special: false,
                    fall_on_special: false,
                },
                PaletteEntry {
                    collider: Some(Aabb6::new(0.0, 0.0, 0.0, 1.0, 1.0, 1.0)),
                    friction: 0.6,
                    speed: 1.0,
                    is_water: false,
                    has_fluid: false,
                    cobweb: false,
                    entity_inside_special: false,
                    step_on_special: false,
                    fall_on_special: false,
                },
            ];
            data.blocks = vec![1u16; 4096];
            return Ok(data);
        }
        Ok(SectionData::all_air())
    };

    // 100k resting items in one cell area
    let n = 100_000usize;
    let mut states = Vec::with_capacity(n);
    for i in 0..n {
        let x = ((i % 316) as f64) * 0.5 + 0.25;
        let z = ((i / 316) as f64) * 0.5 + 0.25;
        states.push(ItemState {
            x,
            y: 64.0,
            z,
            dx: 0.0,
            dy: -0.04,
            dz: 0.0,
            xo: x,
            yo: 64.0,
            zo: z,
            on_ground: true,
            tick_count: 4,
            age: 100,
            despawn_rate: 6000,
            ..ItemState::new()
        });
    }

    // warmup
    for (k, st) in states.iter_mut().enumerate() {
        st.tick_count = (k % 4 + 1) as i32;
        let _ = tick_item(st, k as i64, &env, &mut sg, &mut scratch, &mut prov);
    }
    let t = Instant::now();
    let mut moved = 0u32;
    for (k, st) in states.iter_mut().enumerate() {
        st.tick_count = (k % 4 + 1) as i32;
        let o = tick_item(st, k as i64, &env, &mut sg, &mut scratch, &mut prov);
        moved += o.0 as u32 & 1;
    }
    let dt = t.elapsed();
    println!(
        "tick_item x{}: {:.1}ns/item (moved {}), grid: {} lookups, {} populated",
        n,
        dt.as_nanos() as f64 / n as f64,
        moved,
        sg.lookups.load(std::sync::atomic::Ordering::Relaxed),
        sg.populated.load(std::sync::atomic::Ordering::Relaxed)
    );
}
