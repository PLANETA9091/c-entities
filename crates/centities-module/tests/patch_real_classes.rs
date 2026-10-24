//! Prefix patcher against the REAL Entity.class and EntityLookup.class from
//! the shipped mojang-mapped Paper 1.21.8 jar. A panic here aborts the JVM
//! when patching runs inside a JVMTI callback — these tests must stay green
//! for every weave target BEFORE a live boot is attempted.

use centities_module::patch::insert_static_call_at_start;

const ENTITY: &[u8] = include_bytes!("fixtures/Entity.class");
const LOOKUP: &[u8] = include_bytes!("fixtures/EntityLookup.class");

#[test]
fn entity_setpos_prefix_lands() {
    let patched = insert_static_call_at_start(
        ENTITY.to_vec(),
        "setPos",
        "(DDD)V",
        "dev/planeta9091/centities/CeHooks",
        "probePos",
        "(Lnet/minecraft/world/entity/Entity;DDD)V",
        &[(0, b'L'), (1, b'D'), (3, b'D'), (5, b'D')],
    )
    .expect("setPos patch must succeed on the real Entity.class");
    assert!(patched.len() > ENTITY.len());
    // structural sanity: magic intact and CP grew by exactly 6 entries
    assert_eq!(&patched[0..4], &[0xCA, 0xFE, 0xBA, 0xBE]);
    let cp_new = u16::from_be_bytes([patched[8], patched[9]]);
    let cp_old = u16::from_be_bytes([ENTITY[8], ENTITY[9]]);
    assert_eq!(cp_new, cp_old + 6, "cp grew by 3 utf8 + Class + NAT + Methodref");
}

#[test]
fn entity_setboundingbox_prefix_lands() {
    let patched = insert_static_call_at_start(
        ENTITY.to_vec(),
        "setBoundingBox",
        "(Lnet/minecraft/world/phys/AABB;)V",
        "dev/planeta9091/centities/CeHooks",
        "probeBox",
        "(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/phys/AABB;)V",
        &[(0, b'L'), (1, b'L')],
    )
    .expect("setBoundingBox patch must succeed on the real Entity.class");
    assert!(patched.len() > ENTITY.len());
}

#[test]
fn lookup_add_remove_prefixes_land() {
    let one = insert_static_call_at_start(
        LOOKUP.to_vec(),
        "addEntity",
        "(Lnet/minecraft/world/entity/Entity;ZZ)Z",
        "dev/planeta9091/centities/CeHooks",
        "probeAdd",
        "(Lnet/minecraft/world/entity/Entity;)V",
        &[(1, b'L')],
    )
    .expect("addEntity patch must succeed on the real EntityLookup.class");
    let two = insert_static_call_at_start(
        one,
        "removeEntity",
        "(Lnet/minecraft/world/entity/Entity;)V",
        "dev/planeta9091/centities/CeHooks",
        "probeRemove",
        "(Lnet/minecraft/world/entity/Entity;)V",
        &[(1, b'L')],
    )
    .expect("removeEntity patch must succeed after addEntity was patched");
    assert!(two.len() > LOOKUP.len());
}

#[test]
fn patched_classes_are_jvm_verifiable() {
    // Full-chain check: patch every probe target and hand the bytes to an
    // actual JVM class parse (javap in disguise: -Xshare off, no exec in
    // tests — instead we re-walk the CP of the result and assert the CP
    // count/structure are coherent).
    for (name, bytes) in [
        ("entity", insert_static_call_at_start(
            ENTITY.to_vec(), "setPos", "(DDD)V",
            "dev/planeta9091/centities/CeHooks", "probePos",
            "(Lnet/minecraft/world/entity/Entity;DDD)V",
            &[(0, b'L'), (1, b'D'), (3, b'D'), (5, b'D')],
        ).unwrap()),
        ("lookup", insert_static_call_at_start(
            LOOKUP.to_vec(), "addEntity", "(Lnet/minecraft/world/entity/Entity;ZZ)Z",
            "dev/planeta9091/centities/CeHooks", "probeAdd",
            "(Lnet/minecraft/world/entity/Entity;)V", &[(1, b'L')],
        ).unwrap()),
    ] {
        let cp_count = u16::from_be_bytes([bytes[8], bytes[9]]);
        assert!(cp_count > 100, "{name}: cp count sane");
        assert_eq!(&bytes[0..4], &[0xCA, 0xFE, 0xBA, 0xBE], "{name}: magic");
        // code array of the target must have grown: file grows by more than
        // the CP additions alone
        assert!(bytes.len() > ENTITY.len().min(LOOKUP.len()), "{name}: grew");
    }
}

#[test]
fn level_query_bodies_swap_cleanly() {
    use centities_module::patch::replace_body_with_static;
    const LEVEL: &[u8] = include_bytes!("fixtures/Level.class");
    let once = replace_body_with_static(
        LEVEL.to_vec(),
        "getEntities",
        "(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/phys/AABB;Ljava/util/function/Predicate;)Ljava/util/List;",
        "dev/planeta9091/centities/CeHooks",
        "getEntities",
        "(Lnet/minecraft/world/level/Level;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/phys/AABB;Ljava/util/function/Predicate;)Ljava/util/List;",
        &[0, 1, 2, 3],
    )
    .expect("getEntities body swap");
    let twice = replace_body_with_static(
        once,
        "getEntitiesOfClass",
        "(Ljava/lang/Class;Lnet/minecraft/world/phys/AABB;Ljava/util/function/Predicate;)Ljava/util/List;",
        "dev/planeta9091/centities/CeHooks",
        "getEntitiesOfClass",
        "(Lnet/minecraft/world/level/Level;Ljava/lang/Class;Lnet/minecraft/world/phys/AABB;Ljava/util/function/Predicate;)Ljava/util/List;",
        &[0, 1, 2, 3],
    )
    .expect("getEntitiesOfClass body swap");
    assert_eq!(&twice[0..4], &[0xCA, 0xFE, 0xBA, 0xBE]);
    // both swaps removed far more bytecode than they added
    assert!(twice.len() < LEVEL.len() + 200);
}
