# Baseline run: `spread-100k-mod`

Server: `26.3-244-35a2d15 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97379 -> 97379) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 610.972 / 753.948 / 945.472 / 1169.952 |
| MSPT mean, ms | 637.121 |
| TPS estimate (1000/p50) | 1.6 |
| heap used / max, MB | 2185.3 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.10% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1326` |
| 2 | 3.25% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 3 | 2.59% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3396` |
| 4 | 2.47% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3340` |
| 5 | 1.92% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7102` |
| 6 | 1.82% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2816` |
| 7 | 1.81% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10965` |
| 8 | 1.70% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 9 | 1.51% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 1.47% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 11 | 1.37% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3370` |
| 12 | 1.16% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 13 | 1.16% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 14 | 1.14% | `java.util.ArrayDeque.addFirst(Object) line: 286` |
| 15 | 1.13% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |

