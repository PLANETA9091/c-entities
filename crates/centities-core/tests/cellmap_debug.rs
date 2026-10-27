//! Differential test that dumps the exact op sequence breaking CellMap.

use centities_core::CellMap;
use std::collections::HashMap;

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

#[test]
fn cellmap_minimal_break() {
    let mut rng = Rng(42);
    let mut m = CellMap::new(64);
    let mut truth: HashMap<u64, u32> = HashMap::new();
    let mut log: Vec<String> = Vec::new();

    for step in 0..200_000u64 {
        let k = 1 + rng.next() % 20_000;
        let op = rng.next() % 100;
        if op < 60 || truth.is_empty() {
            let v = ((step % 1000) + 1) as u32;
            m.insert(k, v);
            truth.insert(k, v);
            log.push(format!("ins {k} {v}"));
        } else {
            m.remove(k);
            truth.remove(&k);
            log.push(format!("rem {k}"));
        }
        if m.keys.iter().filter(|k| **k != 0).count() != truth.len() {
            eprintln!(
                "BREAK at step {step}: table={} truth={}",
                m.keys.iter().filter(|k| **k != 0).count(),
                truth.len()
            );
            eprintln!("last 40 ops: {:?}", &log[log.len().saturating_sub(40)..]);
            eprintln!(
                "missing keys: {:?}",
                truth
                    .keys()
                    .filter(|k| m.find(**k).is_none())
                    .take(10)
                    .collect::<Vec<_>>()
            );
            panic!("count mismatch");
        }
    }
}
