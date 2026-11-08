//! N1 — the REAL block facts: `SectionFacts` implements the `BlockFacts`
//! seam over the item pipeline's thread-safe section cache
//! (`centities_core::sections`). The Java bridge already gathers sections
//! for the native item tick; the mob tick reuses the same `SectionGrid`
//! and answers path-safety in Rust — no JNI in the loop (the Horizon O(1)
//! row contract at our palette level).
//!
//! `can_stand` = the path-type-0 standable contract, conservatively exact:
//!   * ground (y-1) has a collider whose top reaches the standing plane;
//!   * the feet (y) and head (y+1) blocks carry NO collider at all
//!     (partial-collider blocks — slabs, stairs — are path-type != 0, so
//!     refusing them is the conservative direction);
//!   * any unloaded-section visit refuses (walking into unloaded chunks is
//!     never better than refusing the stroll target).
use std::cell::RefCell;

use centities_core::sections::{SectionCache, SectionData, SectionGrid};

use crate::world::BlockFacts;

pub struct SectionFacts<'a> {
    cache: RefCell<SectionCache<'a>>,
    pub min_section_y: i32,
    pub max_section_y: i32,
}

impl<'a> SectionFacts<'a> {
    /// Default facts: every section reads as all-air (the constructor is
    /// for tests / open-world stubs — production uses new_with_provider).
    pub fn new(grid: &'a SectionGrid, min_section_y: i32, max_section_y: i32) -> Self {
        // the tiny closure leaks once per SectionFacts construction —
        // SectionProvider is a &'a mut dyn FnMut and must outlive 'a
        let p: Box<dyn FnMut(i32, i32, i32, bool) -> Result<SectionData, ()>> =
            Box::new(|_, _, _, _| Ok(SectionData::all_air()));
        let leaked: &'static mut dyn FnMut(i32, i32, i32, bool) -> Result<SectionData, ()> =
            Box::leak(p);
        SectionFacts {
            cache: RefCell::new(SectionCache::new(grid, leaked)),
            min_section_y,
            max_section_y,
        }
    }

    pub fn new_with_provider(
        grid: &'a SectionGrid,
        provider: centities_core::sections::SectionProvider<'a>,
        min_section_y: i32,
        max_section_y: i32,
    ) -> Self {
        SectionFacts {
            cache: RefCell::new(SectionCache::new(grid, provider)),
            min_section_y,
            max_section_y,
        }
    }
}

impl BlockFacts for SectionFacts<'_> {
    fn can_stand(&self, x: f64, y: f64, z: f64) -> bool {
        let (bx, by, bz) = (x.floor() as i32, y.floor() as i32, z.floor() as i32);
        let (mn, mx) = (self.min_section_y, self.max_section_y);
        let mut c = self.cache.borrow_mut();
        // ground: a collider must span the standing plane
        let (ground, un) = c.block_box_at(bx, by - 1, bz, mn, mx);
        if un {
            return false;
        }
        match ground {
            // colliders are block-RELATIVE (0..=1 within the block, doc'd in
            // PaletteEntry): the absolute top = block base + relative top
            Some(b) if (by - 1) as f64 + b.y1 >= y => {}
            _ => return false,
        }
        // feet + head: fully open
        let (feet, un1) = c.block_box_at(bx, by, bz, mn, mx);
        if un1 || feet.is_some() {
            return false;
        }
        let (head, un2) = c.block_box_at(bx, by + 1, bz, mn, mx);
        if un2 {
            return false;
        }
        head.is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use centities_core::itemtick::Aabb6;
    use centities_core::sections::{PaletteEntry, SectionData, SEC_BLOCKS};

    /// A synthetic flat world: stone at y = 63 (section cy 3), air above.
    /// An optional wall column at (5, *, 0) rises to y=65.
    fn flat_provider(cx: i32, cy: i32, cz: i32, _load: bool) -> Result<SectionData, ()> {
        if cy != 3 && cy != 4 {
            return Ok(SectionData::all_air());
        }
        let mut stone = PaletteEntry::air();
        stone.colliders = vec![Aabb6 {
            x0: 0.0,
            y0: 0.0,
            z0: 0.0,
            x1: 1.0,
            y1: 1.0,
            z1: 1.0,
        }];
        let mut data = SectionData {
            entries: vec![PaletteEntry::air(), stone],
            blocks: vec![0; SEC_BLOCKS], // 0 = air palette slot
            complex: false,
            any_fluid: false,
            any_inside_special: false,
        };
        for lx in 0..16u16 {
            for lz in 0..16u16 {
                for ly in 0..16u16 {
                    let wy = cy * 16 + ly as i32;
                    let wx = cx * 16 + lx as i32;
                    let wz = cz * 16 + lz as i32;
                    let stone_here = wy == 63
                        || (wx == 5 && wz == 0 && (63..=65).contains(&wy));
                    if stone_here {
                        let local = (lx | (lz << 4) | (ly << 8)) as usize;
                        data.blocks[local] = 1;
                    }
                }
            }
        }
        Ok(data)
    }

    #[test]
    fn flat_ground_and_wall() {
        let grid: &'static SectionGrid = Box::leak(Box::new(SectionGrid::new()));
        let provider: &'static mut dyn FnMut(i32, i32, i32, bool) -> Result<SectionData, ()> =
            Box::leak(Box::new(flat_provider));
        let f = SectionFacts::new_with_provider(grid, provider, 0, 8);
        // standing plane y=64 on the stone floor
        assert!(f.can_stand(0.5, 64.0, 0.5));
        assert!(f.can_stand(4.5, 64.0, 0.5));
        // the wall column blocks (5, 64, 0): feet inside stone
        assert!(!f.can_stand(5.0, 64.0, 0.0), "feet inside the wall");
        // (5, 65, 0): feet still inside stone (the wall spans 63..=65)
        assert!(!f.can_stand(5.0, 65.0, 0.0));
        // on top of the wall IS standable: ground (y=65) top y1=66 reaches
        // the plane, feet (66) and head (67) are open
        assert!(f.can_stand(5.0, 66.0, 0.0));
        // outside the wall x: fine
        assert!(f.can_stand(6.5, 64.0, 0.5));
        // above the world (no ground)
        assert!(!f.can_stand(0.5, 200.0, 0.5));
    }
}
