# Baseline run: `mobs-25k-mod`

Server: `26.3-224-eb9098a (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25039 -> 25039) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 161.121 / 250.821 / 319.289 / 342.093 |
| MSPT mean, ms | 171.265 |
| TPS estimate (1000/p50) | 6.2 |
| heap used / max, MB | 1895.5 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 4.13% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.98% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 1.52% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 4 | 1.46% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 1.43% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 6 | 1.36% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 6507` |
| 7 | 1.33% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 6556` |
| 8 | 1.12% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 9 | 1.08% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1407` |
| 10 | 1.05% | `java.util.EnumMap.get(Object) line: 248` |
| 11 | 1.00% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 12 | 0.96% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 13 | 0.88% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 14 | 0.86% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 15 | 0.82% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |

