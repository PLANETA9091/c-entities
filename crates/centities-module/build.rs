//! Compiles sources/java/CeHooks.java against the mojang-mapped Paper 1.21.8
//! jar and embeds the resulting class bytes (include_bytes! in lib.rs).
//!
//! PAPER_JAR env var overrides the default location
//! (<repo>/../paper-merged/paper-1.21.8.jar).

use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=sources/java/CeHooks.java");
    println!("cargo:rerun-if-env-changed=PAPER_JAR");

    let manifest = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let paper = env::var("PAPER_JAR").unwrap_or_else(|_| {
        let p = manifest
            .parent()
            .and_then(|p| p.parent())
            .and_then(|p| p.parent()) // repo root's parent (work dir)
            .map(|w| w.join("paper-merged").join("paper-1.21.8.jar"));
        // crates/centities-module -> crates -> repo -> work
        let alt = manifest
            .parent()
            .and_then(|p| p.parent())
            .map(|r| r.join("server").join("paper.jar"));
        match (p.filter(|p| p.exists()), alt.filter(|p| p.exists())) {
            (Some(p), _) => p.to_string_lossy().into_owned(),
            (None, Some(a)) => a.to_string_lossy().into_owned(),
            _ => panic!(
                "PAPER_JAR not found: pass PAPER_JAR=<mojang-mapped paper 1.21.8 jar> \
                 (run scripts/fetch-paper.sh + one paperclip boot, see README)"
            ),
        }
    });

    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    let classes = out.join("classes");
    let _ = std::fs::remove_dir_all(&classes);
    std::fs::create_dir_all(&classes).unwrap();

    let src = manifest.join("sources").join("java").join("CeHooks.java");
    let javac = find_javac();
    let st = Command::new(&javac)
        .args([
            "--release",
            "17",
            "-nowarn",
            "-cp",
            &paper,
            "-d",
            classes.to_str().unwrap(),
            src.to_str().unwrap(),
        ])
        .status()
        .expect("javac (JDK 17+) must be on PATH");
    assert!(st.success(), "javac failed for CeHooks.java");
    println!("cargo:rustc-env=CE_HOOKS_CLASS={}", classes.join("dev/planeta9091/centities/CeHooks.class").display());

    // Embed ALL classes (the bridge may declare nested/helper classes): a
    // simple length-prefixed blob — [u32 count][u16 nameLen][name][u32 len][bytes]...
    let mut names: Vec<PathBuf> = Vec::new();
    let pkg = classes.join("dev/planeta9091/centities");
    for e in std::fs::read_dir(&pkg).expect("compiled classes dir") {
        let p = e.expect("entry").path();
        if p.extension().and_then(|s| s.to_str()) == Some("class") {
            names.push(p);
        }
    }
    names.sort();
    assert!(!names.is_empty(), "no classes compiled");
    let mut blob: Vec<u8> = Vec::new();
    blob.extend_from_slice(&(names.len() as u32).to_be_bytes());
    for n in &names {
        let rel = n.strip_prefix(&classes).expect("rel").to_str().expect("utf8");
        let bytes = std::fs::read(n).expect("class bytes");
        blob.extend_from_slice(&(rel.len() as u16).to_be_bytes());
        blob.extend_from_slice(rel.as_bytes());
        blob.extend_from_slice(&(bytes.len() as u32).to_be_bytes());
        blob.extend_from_slice(&bytes);
    }
    let blob_path = out.join("hooks_classes.bin");
    std::fs::write(&blob_path, &blob).expect("write blob");
    println!("cargo:rustc-env=CE_HOOKS_BLOB={}", blob_path.display());
}

/// Locate javac: PATH, JAVA_HOME, common Linux install globs.
fn find_javac() -> String {
    if Command::new("javac").arg("-version").output().is_ok() {
        return "javac".into();
    }
    let mut candidates: Vec<PathBuf> = Vec::new();
    if let Ok(home) = env::var("JAVA_HOME") {
        candidates.push(PathBuf::from(home).join("bin").join("javac"));
    }
    if let Ok(home) = env::var("HOME") {
        let pat = PathBuf::from(home).join("tools");
        if let Ok(rd) = std::fs::read_dir(&pat) {
            for e in rd.flatten() {
                candidates.push(e.path().join("bin").join("javac"));
            }
        }
    }
    if let Ok(rd) = std::fs::read_dir("/usr/lib/jvm") {
        for e in rd.flatten() {
            candidates.push(e.path().join("bin").join("javac"));
        }
    }
    for c in candidates {
        if c.is_file() {
            return c.to_string_lossy().into_owned();
        }
    }
    panic!("javac not found: install JDK 17+ or set JAVA_HOME");
}
