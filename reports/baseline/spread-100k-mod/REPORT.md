# Baseline run: `spread-100k-mod`

Server: `26.3-249-1a995d6 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 88784 -> 88661) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 163.572 / 263.700 / 414.798 / 498.042 |
| MSPT mean, ms | 186.494 |
| TPS estimate (1000/p50) | 6.1 |
| heap used / max, MB | 2439.5 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 6.61% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 2 | 5.37% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3698` |
| 3 | 3.20% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7808` |
| 4 | 2.89% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3642` |
| 5 | 2.78% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7056` |
| 6 | 2.74% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12440` |
| 7 | 2.48% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3656` |
| 8 | 2.16% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3659` |
| 9 | 2.15% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3671` |
| 10 | 1.95% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 12988` |
| 11 | 1.70% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 12 | 1.68% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 13 | 1.49% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3722` |
| 14 | 1.45% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3672` |
| 15 | 1.43% | `java.util.ArrayList.<init>(int) line: 157` |

