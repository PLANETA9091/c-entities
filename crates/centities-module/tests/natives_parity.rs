//! Native descriptor parity (doc 21 regression guard).
//!
//! The e2e caught it the hard way: RegisterNatives is all-or-nothing, so a
//! single mismatched descriptor (queryNative2 was registered with a stale
//! `[J` out-param) fails the WHOLE arm sequence -> the module silently
//! stays vanilla. This test parses the compiled CeHooks.class and asserts
//! the native-method set equals the Rust-registered table EXACTLY — a
//! mismatch now fails in `cargo test` instead of in CI boot.

use std::collections::BTreeSet;

/// The Rust side of the contract: (name, descriptor) pairs passed to
/// RegisterNatives in crates/centities-module/src/lib.rs::register_natives.
/// KEEP IN SYNC — the test fails on any drift from either direction.
fn rust_registered() -> BTreeSet<(&'static str, &'static str)> {
    let mut s = BTreeSet::new();
    s.insert(("queryNative", "(IDDDDDD[JI[J[D)I"));
    s.insert(("queryNative2", "(IDDDDDDI[J[D)I"));
    s.insert(("nativeSetOutBuffer", "(Ljava/nio/ByteBuffer;)I"));
    s.insert(("nativeFlush", "(I[J[D)V"));
    s.insert(("nativeBoxDirect", "(JIDDDDDD)V"));
    s.insert(("nativeRemoveDirect", "(J)V"));
    s.insert(("nativeReset", "()V"));
    s.insert(("nativeItemTick", "(II[J[I[DI[I[D[I[D[I)I"));
    s.insert(("nativeItemInvalidate", "(II[I[I[I)V"));
    s
}

fn u16at(b: &[u8], i: usize) -> u16 {
    u16::from_be_bytes([b[i], b[i + 1]])
}
fn u32at(b: &[u8], i: usize) -> u32 {
    u32::from_be_bytes([b[i], b[i + 1], b[i + 2], b[i + 3]])
}

/// Minimal classfile walk: returns (name, descriptor) of every ACC_NATIVE
/// method. Only the pieces needed for the methods table are parsed.
fn native_methods(class: &[u8]) -> Result<BTreeSet<(String, String)>, String> {
    if class.len() < 10 || u32at(class, 0) != 0xCAFE_BABE {
        return Err("not a classfile".into());
    }
    let mut i = 8usize;
    let cp_count = u16at(class, i) as usize;
    i += 2;
    // constant pool: idx 1..cp_count; keep only Utf8 (tag 1) strings
    let mut utf8: std::collections::HashMap<u16, String> = std::collections::HashMap::new();
    let mut idx = 1usize;
    while idx < cp_count {
        let tag = class[i];
        i += 1;
        match tag {
            1 => {
                let len = u16at(class, i) as usize;
                i += 2;
                utf8.insert(
                    idx as u16,
                    String::from_utf8_lossy(&class[i..i + len]).into_owned(),
                );
                i += len;
            }
            7 | 8 | 16 | 19 | 20 => i += 2,
            15 => i += 3,
            5 | 6 => {
                i += 8;
                idx += 1; // long/double occupy two slots
            }
            3 | 4 | 9 | 10 | 11 | 12 | 17 | 18 => i += 4,
            other => return Err(format!("bad cp tag {other}")),
        }
        idx += 1;
    }
    let _access = u16at(class, i);
    i += 2;
    let _this = u16at(class, i);
    i += 2;
    let _super = u16at(class, i);
    i += 2;
    let ifc = u16at(class, i) as usize;
    i += 2 + ifc * 2;
    // fields (skip)
    let fcount = u16at(class, i) as usize;
    i += 2;
    for _ in 0..fcount {
        i += 6; // access, name, desc
        let attrs = u16at(class, i) as usize;
        i += 2;
        for _ in 0..attrs {
            let alen = u32at(class, i + 2) as usize; // length sits after name_index
            i += 6 + alen;
        }
    }
    // methods
    let mcount = u16at(class, i) as usize;
    i += 2;
    let ACC_NATIVE = 0x0100u16;
    let mut out = BTreeSet::new();
    for _ in 0..mcount {
        let access = u16at(class, i);
        let name = u16at(class, i + 2);
        let desc = u16at(class, i + 4);
        i += 6;
        let attrs = u16at(class, i) as usize;
        i += 2;
        for _ in 0..attrs {
            let alen = u32at(class, i + 2) as usize; // length sits after name_index
            i += 6 + alen;
        }
        if access & ACC_NATIVE != 0 {
            out.insert((
                utf8.get(&name).cloned().ok_or("bad name idx")?,
                utf8.get(&desc).cloned().ok_or("bad desc idx")?,
            ));
        }
    }
    Ok(out)
}

#[test]
fn registered_natives_match_compiled_ce_hooks() {
    let path = env!("CE_HOOKS_CLASS");
    let bytes = std::fs::read(path)
        .unwrap_or_else(|e| panic!("cannot read compiled CeHooks.class at {path}: {e}"));
    let java_side = native_methods(&bytes).expect("parse CeHooks.class");
    let rust_side = rust_registered();

    let rust_side_strings: BTreeSet<(String, String)> = rust_registered()
        .into_iter()
        .map(|(n, d)| (n.to_string(), d.to_string()))
        .collect();
    let java_missing_in_rust: Vec<_> = java_side.difference(&rust_side_strings).collect();
    let rust_missing_in_java: Vec<_> = rust_side_strings.difference(&java_side).collect();
    assert!(
        java_missing_in_rust.is_empty() && rust_missing_in_java.is_empty(),
        "native descriptor drift — RegisterNatives would fail the whole arm:\n  java-only: {java_missing_in_rust:?}\n  rust-only: {rust_missing_in_java:?}"
    );
}
