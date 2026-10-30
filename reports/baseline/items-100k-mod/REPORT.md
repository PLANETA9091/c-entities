# Baseline run: `items-100k-mod`

Server: `26.3-231-8a170a3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62907 -> 62907) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 29.127 / 74.785 / 154.514 / 214.606 |
| MSPT mean, ms | 38.680 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1590.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 12.05% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4761` |
| 2 | 11.69% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8516` |
| 3 | 6.03% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 4.37% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 3.43% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 374` |
| 6 | 3.39% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8463` |
| 7 | 3.10% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3462` |
| 8 | 2.34% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 9 | 2.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 10 | 2.12% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3354` |
| 11 | 1.49% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 12 | 1.23% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 13 | 1.13% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 14 | 1.01% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4825` |
| 15 | 0.99% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2474` |

