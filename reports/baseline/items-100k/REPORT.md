# Baseline run: `items-100k`

Server: `26.3-223-e415f5e (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62830 -> 62830) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 67.613 / 84.914 / 156.641 / 197.553 |
| MSPT mean, ms | 74.210 |
| TPS estimate (1000/p50) | 14.8 |
| heap used / max, MB | 1217.6 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 7.43% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 5.86% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 4.36% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 4 | 2.98% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 5 | 2.82% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 6 | 2.20% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 7 | 2.18% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 8 | 2.08% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 9 | 2.03% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 10 | 1.71% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 11 | 1.64% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 12 | 1.59% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 13 | 1.48% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 14 | 1.43% | `java.util.EnumMap.get(Object) line: 248` |
| 15 | 1.39% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |

