// quick probe: reproduce the panic on real classes
use centities_module::patch::insert_static_call_at_start;

const ENTITY: &[u8] = include_bytes!("../tests/fixtures/Entity.class");
const LOOKUP: &[u8] = include_bytes!("../tests/fixtures/EntityLookup.class");

fn report(name: &str, r: Result<Result<Vec<u8>, String>, Box<dyn std::any::Any + Send>>) {
    match r {
        Err(_) => println!("{name}: PANICKED"),
        Ok(Err(e)) => println!("{name}: refused: {e}"),
        Ok(Ok(b)) => println!("{name}: ok, {} bytes", b.len()),
    }
}

fn main() {
    report("setPos", std::panic::catch_unwind(|| {
        insert_static_call_at_start(
            ENTITY.to_vec(), "setPos", "(DDD)V",
            "dev/planeta9091/centities/CeHooks", "probePos",
            "(Lnet/minecraft/world/entity/Entity;DDD)V",
            &[(0, b'L'), (1, b'D'), (3, b'D'), (5, b'D')],
        )
    }));
    report("setBoundingBox", std::panic::catch_unwind(|| {
        insert_static_call_at_start(
            ENTITY.to_vec(), "setBoundingBox", "(Lnet/minecraft/world/phys/AABB;)V",
            "dev/planeta9091/centities/CeHooks", "probeBox",
            "(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/phys/AABB;)V",
            &[(0, b'L'), (1, b'L')],
        )
    }));
    report("lookup add+remove", std::panic::catch_unwind(|| {
        let b = insert_static_call_at_start(
            LOOKUP.to_vec(), "addEntity", "(Lnet/minecraft/world/entity/Entity;ZZ)Z",
            "dev/planeta9091/centities/CeHooks", "probeAdd",
            "(Lnet/minecraft/world/entity/Entity;)V", &[(1, b'L')],
        )?;
        insert_static_call_at_start(
            b, "removeEntity", "(Lnet/minecraft/world/entity/Entity;)V",
            "dev/planeta9091/centities/CeHooks", "probeRemove",
            "(Lnet/minecraft/world/entity/Entity;)V", &[(1, b'L')],
        )
    }));
}
