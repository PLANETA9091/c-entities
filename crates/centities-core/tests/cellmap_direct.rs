//! Direct stress test for the open-addressed CellMap.

use centities_core::CellMap;

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }
}

fn refcheck(m: &CellMap, truth: &std::collections::HashMap<u64, u32>) {
    assert_eq!(m.keys.iter().filter(|k| **k != 0).count(), truth.len(),
        "table live count != truth len");
    for (k, v) in truth {
        assert_eq!(m.find(*k), Some(*v), "find({k}) failed");
    }
    // every table entry must exist in truth
    for i in 0..m.keys.len() {
        if m.keys[i] != 0 {
            assert_eq!(truth.get(&m.keys[i]), Some(&m.vals[i]), "table slot {i} stale");
        }
    }
}

#[test]
fn cellmap_insert_remove_storm() {
    let mut rng = Rng(0x5eed_1111_2222_3333);
    let mut m = CellMap::new(64);
    let mut truth: std::collections::HashMap<u64, u32> = std::collections::HashMap::new();

    for step in 0..200_000u64 {
        let k = 1 + rng.next() % 20_000;
        let op = rng.next() % 100;
        if op < 60 || truth.is_empty() {
            let v = ((step % 1000) + 1) as u32;
            m.insert(k, v);
            truth.insert(k, v);
        } else {
            m.remove(k);
            truth.remove(&k);
        }
        if step % 10_000 == 0 {
            refcheck(&m, &truth);
        }
    }
    refcheck(&m, &truth);
}
