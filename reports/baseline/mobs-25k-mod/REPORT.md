# Baseline run: `mobs-25k-mod`

Server: `26.3-241-5050b0f (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25070 -> 25070) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 300.212 / 504.147 / 609.247 / 682.380 |
| MSPT mean, ms | 341.071 |
| TPS estimate (1000/p50) | 3.3 |
| heap used / max, MB | 2056.9 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 11.23% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2669` |
| 2 | 7.36% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 3 | 5.06% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2400` |
| 4 | 1.76% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 5 | 1.69% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 6 | 1.58% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 1.35% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6742` |
| 8 | 1.28% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 9 | 1.20% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 10 | 1.11% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 11 | 1.05% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10100` |
| 12 | 1.02% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10723` |
| 13 | 0.99% | `dev.planeta9091.centities.CeHooks.ordSortVanilla(ArrayList) line: 2398` |
| 14 | 0.97% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 15 | 0.96% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |

