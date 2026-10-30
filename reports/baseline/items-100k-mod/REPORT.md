# Baseline run: `items-100k-mod`

Server: `26.3-231-8a170a3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62881 -> 62881) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 34.817 / 93.176 / 165.148 / 218.355 |
| MSPT mean, ms | 46.439 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1199.3 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 14.91% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4719` |
| 2 | 12.22% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8474` |
| 3 | 5.97% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 4.98% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 3.18% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 374` |
| 6 | 2.98% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8421` |
| 7 | 2.62% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3420` |
| 8 | 1.64% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 9 | 1.17% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5510` |
| 10 | 1.14% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3312` |
| 11 | 1.10% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 12 | 1.00% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 13 | 0.95% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4783` |
| 14 | 0.90% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 15 | 0.90% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2448` |

