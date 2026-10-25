# Baseline run: `items-100k`

Server: `26.3-223-e415f5e (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62926 -> 62926) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 64.074 / 78.689 / 146.515 / 188.831 |
| MSPT mean, ms | 70.055 |
| TPS estimate (1000/p50) | 15.6 |
| heap used / max, MB | 1125.4 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 7.55% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.49% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 3 | 3.64% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 3.00% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 5 | 2.91% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 6 | 2.28% | `net.minecraft.util.KeyframeTrackSampler.sample(long) line: 67` |
| 7 | 2.26% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 2.11% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 9 | 2.03% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 10 | 1.92% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 11 | 1.79% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 12 | 1.78% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 13 | 1.76% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 14 | 1.42% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 15 | 1.29% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |

