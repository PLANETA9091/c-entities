//! Repro: parse REAL Paper classes with the prefix patcher.
//! Locally: PAPER_JAR=/path/to/paper-1.21.8.jar cargo test -p centities-module --test patch_entity_repro
//! (extracts Entity.class + EntityLookup.class from the jar at runtime).

use centities_module::patch::insert_static_call_at_start;

fn extract(jar: &str, entry: &str) -> Option<Vec<u8>> {
    let f = std::fs::File::open(jar).ok()?;
    let mut z = zip::ZipArchive::new(f).ok()?;
    let mut e = z.by_name(entry).ok()?;
    let mut out = Vec::new();
    std::io::Read::read_to_end(&mut e, &mut out).ok()?;
    Some(out)
}

#[test]
fn patch_real_entity_and_lookup() {
    let jar = match std::env::var("PAPER_JAR") {
        Ok(j) if std::path::Path::new(&j).exists() => j,
        _ => {
            eprintln!("skip: PAPER_JAR not set or missing");
            return;
        }
    };
    let entity = extract(&jar, "net/minecraft/world/entity/Entity.class").expect("Entity.class");
    println!("Entity.class: {} bytes", entity.len());
    let lookup = extract(
        &jar,
        "ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.class",
    )
    .expect("EntityLookup.class");
    println!("EntityLookup.class: {} bytes", lookup.len());

    // Entity.setPos(DDD)V — no exc table, LNT+LVT only
    let r1 = insert_static_call_at_start(
        entity.clone(),
        "setPos",
        "(DDD)V",
        "dev/planeta9091/centities/CeHooks",
        "probePos",
        "(Lnet/minecraft/world/entity/Entity;DDD)V",
        &[(0, b'L'), (1, b'D'), (3, b'D'), (5, b'D')],
    );
    println!(
        "setPos: {:?}",
        r1.as_ref().map(|b| b.len()).map_err(|e| e.clone())
    );
    assert!(r1.is_ok(), "setPos patch failed: {:?}", r1.err());

    // Entity.setBoundingBox(AABB)V — has StackMapTable
    let r2 = insert_static_call_at_start(
        entity,
        "setBoundingBox",
        "(Lnet/minecraft/world/phys/AABB;)V",
        "dev/planeta9091/centities/CeHooks",
        "probeBox",
        "(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/phys/AABB;)V",
        &[(0, b'L'), (1, b'L')],
    );
    println!(
        "setBoundingBox: {:?}",
        r2.as_ref().map(|b| b.len()).map_err(|e| e.clone())
    );
    assert!(r2.is_ok(), "setBoundingBox patch failed: {:?}", r2.err());

    // EntityLookup.addEntity(Entity,boolean,boolean)Z — full_frame SMT with
    // Double/Object locals (regression: skip_verification_info tag map)
    let r3 = insert_static_call_at_start(
        lookup.clone(),
        "addEntity",
        "(Lnet/minecraft/world/entity/Entity;ZZ)Z",
        "dev/planeta9091/centities/CeHooks",
        "probeAdd",
        "(Lnet/minecraft/world/entity/Entity;)V",
        &[(1, b'L')],
    );
    println!(
        "addEntity: {:?}",
        r3.as_ref().map(|b| b.len()).map_err(|e| e.clone())
    );
    assert!(r3.is_ok(), "addEntity patch failed: {:?}", r3.err());
    // probe target must be void; the METHOD may return Z (we pop nothing —
    // wait, probeAdd returns V and is inserted at start: stack-neutral ok).

    // EntityLookup.removeEntity(Entity)V — append_frame SMT
    let r4 = insert_static_call_at_start(
        lookup,
        "removeEntity",
        "(Lnet/minecraft/world/entity/Entity;)V",
        "dev/planeta9091/centities/CeHooks",
        "probeRemove",
        "(Lnet/minecraft/world/entity/Entity;)V",
        &[(1, b'L')],
    );
    println!(
        "removeEntity: {:?}",
        r4.as_ref().map(|b| b.len()).map_err(|e| e.clone())
    );
    assert!(r4.is_ok(), "removeEntity patch failed: {:?}", r4.err());
}
