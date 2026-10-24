# Baseline run: `items-100k`

Server: `26.3-222-e3ba7b1 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62917 -> 62917) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 59.669 / 71.646 / 130.459 / 158.695 |
| MSPT mean, ms | 64.624 |
| TPS estimate (1000/p50) | 16.8 |
| heap used / max, MB | 1006.4 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 7.48% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 5.40% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 4.92% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2403` |
| 4 | 3.79% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 5 | 3.06% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 2.86% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.76% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 8 | 2.33% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 1.93% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 10 | 1.91% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 11 | 1.88% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 12 | 1.75% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 13 | 1.55% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3868` |
| 14 | 1.53% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 15 | 1.48% | `net.minecraft.world.entity.Entity.isRemoved() line: 5328` |

