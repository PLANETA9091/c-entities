# Baseline run: `mobs-25k-mod`

Server: `26.3-252-5184e9c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 24804 -> 24804) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 2640.145 / 2840.175 / 3003.427 / 3120.819 |
| MSPT mean, ms | 2638.609 |
| TPS estimate (1000/p50) | 0.4 |
| heap used / max, MB | 1984.9 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 21.90% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1410` |
| 2 | 3.10% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3142` |
| 3 | 2.77% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3143` |
| 4 | 1.45% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 13672` |
| 5 | 1.08% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 6 | 0.93% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 0.90% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 8 | 0.84% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2906` |
| 9 | 0.83% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12967` |
| 10 | 0.82% | `java.util.ArrayList.add(Object, Object[], int) line: 485` |
| 11 | 0.78% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 12 | 0.76% | `java.util.ArrayList.add(Object) line: 497` |
| 13 | 0.61% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 14 | 0.59% | `java.util.HashMap.getNode(Object) line: 586` |
| 15 | 0.58% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |

