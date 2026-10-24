use centities_module::patch::insert_static_call_at_start;
use std::fs;

const ENTITY: &[u8] = include_bytes!("../tests/fixtures/Entity.class");
const LOOKUP: &[u8] = include_bytes!("../tests/fixtures/EntityLookup.class");

fn main() {
    fs::create_dir_all("/tmp/patched/net/minecraft/world/entity").unwrap();
    fs::create_dir_all("/tmp/patched/ca/spottedleaf/moonrise/patches/chunk_system/level/entity").unwrap();
    let e = insert_static_call_at_start(
        ENTITY.to_vec(), "setPos", "(DDD)V",
        "dev/planeta9091/centities/CeHooks", "probePos",
        "(Lnet/minecraft/world/entity/Entity;DDD)V",
        &[(0, b'L'), (1, b'D'), (3, b'D'), (5, b'D')],
    ).unwrap();
    fs::write("/tmp/patched/net/minecraft/world/entity/Entity.class", e).unwrap();
    let l = insert_static_call_at_start(
        LOOKUP.to_vec(), "addEntity", "(Lnet/minecraft/world/entity/Entity;ZZ)Z",
        "dev/planeta9091/centities/CeHooks", "probeAdd",
        "(Lnet/minecraft/world/entity/Entity;)V", &[(1, b'L')],
    ).unwrap();
    let l = insert_static_call_at_start(
        l, "removeEntity", "(Lnet/minecraft/world/entity/Entity;)V",
        "dev/planeta9091/centities/CeHooks", "probeRemove",
        "(Lnet/minecraft/world/entity/Entity;)V", &[(1, b'L')],
    ).unwrap();
    fs::write("/tmp/patched/ca/spottedleaf/moonrise/patches/chunk_system/level/entity/EntityLookup.class", l).unwrap();
    println!("dumped");
}
