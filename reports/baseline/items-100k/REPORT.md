# Baseline run: `items-100k`

Server: `26.3-226-4f49049 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62948 -> 62948) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 62.607 / 77.756 / 144.175 / 180.230 |
| MSPT mean, ms | 68.863 |
| TPS estimate (1000/p50) | 16.0 |
| heap used / max, MB | 1819.9 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 6.19% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 3.39% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 3.28% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 3.21% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 5 | 2.64% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 6 | 2.40% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 7 | 2.26% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 2.26% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 9 | 2.22% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 10 | 2.17% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 11 | 2.17% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 12 | 1.92% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 1.62% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 14 | 1.30% | `java.util.EnumMap.get(Object) line: 248` |
| 15 | 1.29% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |

