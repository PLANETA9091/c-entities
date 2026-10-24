use centities_module::patch::replace_body_with_static;
use std::fs;
const LEVEL: &[u8] = include_bytes!("../tests/fixtures/Level.class");
fn main() {
    let once = replace_body_with_static(
        LEVEL.to_vec(),
        "getEntities",
        "(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/phys/AABB;Ljava/util/function/Predicate;)Ljava/util/List;",
        "dev/planeta9091/centities/CeHooks", "getEntities",
        "(Lnet/minecraft/world/level/Level;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/phys/AABB;Ljava/util/function/Predicate;)Ljava/util/List;",
        &[0, 1, 2, 3]).unwrap();
    let twice = replace_body_with_static(
        once,
        "getEntitiesOfClass",
        "(Ljava/lang/Class;Lnet/minecraft/world/phys/AABB;Ljava/util/function/Predicate;)Ljava/util/List;",
        "dev/planeta9091/centities/CeHooks", "getEntitiesOfClass",
        "(Lnet/minecraft/world/level/Level;Ljava/lang/Class;Lnet/minecraft/world/phys/AABB;Ljava/util/function/Predicate;)Ljava/util/List;",
        &[0, 1, 2, 3]).unwrap();
    fs::create_dir_all("/tmp/patched/net/minecraft/world/level").unwrap();
    fs::write("/tmp/patched/net/minecraft/world/level/Level.class", twice).unwrap();
    println!("dumped");
}
