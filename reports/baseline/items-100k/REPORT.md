# Baseline run: `items-100k`

Server: `26.3-227-7f9c797 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62992 -> 62992) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 68.193 / 86.743 / 160.286 / 223.616 |
| MSPT mean, ms | 75.053 |
| TPS estimate (1000/p50) | 14.7 |
| heap used / max, MB | 2103.2 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 7.90% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.15% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 3.80% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 4 | 3.25% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 5 | 3.17% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 6 | 2.55% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 7 | 2.36% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 8 | 2.04% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 9 | 1.97% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 10 | 1.82% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 11 | 1.79% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.37% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 13 | 1.37% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 14 | 1.16% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 15 | 1.11% | `java.util.EnumMap.get(Object) line: 248` |

