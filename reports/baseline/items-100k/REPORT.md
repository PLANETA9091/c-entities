# Baseline run: `items-100k`

Server: `26.3-230-804dd0c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62993 -> 62993) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 74.210 / 92.783 / 174.445 / 218.151 |
| MSPT mean, ms | 81.428 |
| TPS estimate (1000/p50) | 13.5 |
| heap used / max, MB | 959.3 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 8.14% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 7.72% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 4.73% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 4 | 4.22% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 5 | 3.25% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 6 | 2.85% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 7 | 2.12% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 8 | 1.86% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 9 | 1.66% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 10 | 1.59% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 11 | 1.43% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 12 | 1.33% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 1.31% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 14 | 1.20% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 15 | 1.12% | `java.util.EnumMap.get(Object) line: 248` |

