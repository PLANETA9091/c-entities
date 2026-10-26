# Baseline run: `mobs-25k-mod`

Server: `26.3-224-eb9098a (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 0 / stands 0 / zombies 25000) (merge shrink: 25054 -> 25054) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 158.261 / 261.242 / 337.069 / 380.808 |
| MSPT mean, ms | 172.938 |
| TPS estimate (1000/p50) | 6.3 |
| heap used / max, MB | 1326.2 / 4096.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.30% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.22% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 1.97% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1406` |
| 4 | 1.55% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 5 | 1.30% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 6 | 1.25% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 7 | 1.18% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 8 | 1.17% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6420` |
| 9 | 1.15% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 10 | 1.15% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 11 | 1.13% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 12 | 1.13% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 13 | 1.11% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 14 | 1.06% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 4375` |
| 15 | 1.06% | `java.util.Optional.ifPresent(Consumer) line: 178` |

