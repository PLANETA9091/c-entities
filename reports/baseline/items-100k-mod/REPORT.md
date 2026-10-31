# Baseline run: `items-100k-mod`

Server: `26.3-231-8a170a3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62932 -> 62932) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 32.884 / 92.840 / 140.592 / 208.033 |
| MSPT mean, ms | 44.788 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1522.2 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 13.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5033` |
| 2 | 11.72% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8788` |
| 3 | 4.75% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 4.24% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 3.62% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3734` |
| 6 | 3.14% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 389` |
| 7 | 3.08% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8735` |
| 8 | 2.17% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 9 | 1.46% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 10 | 1.34% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5824` |
| 11 | 1.34% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2661` |
| 12 | 1.22% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3607` |
| 13 | 1.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5097` |
| 14 | 0.98% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 15 | 0.89% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |

