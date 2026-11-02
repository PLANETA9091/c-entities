# Baseline run: `mixed-150k-mod`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 150000 (items 105073 / stands 30072 / zombies 14855) (merge shrink: 109652 -> 109652) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 221.484 / 335.668 / 478.360 / 604.038 |
| MSPT mean, ms | 244.956 |
| TPS estimate (1000/p50) | 4.5 |
| heap used / max, MB | 2412.9 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.53% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.67% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6230` |
| 3 | 3.37% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9374` |
| 4 | 3.31% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 2.89% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 2.84% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 7 | 2.54% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 2.19% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9877` |
| 9 | 2.19% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 10 | 1.73% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 11 | 1.62% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 12 | 1.49% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 13 | 1.46% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 14 | 1.44% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 15 | 1.33% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |

