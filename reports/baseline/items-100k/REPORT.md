# Baseline run: `items-100k`

Server: `26.3-244-35a2d15 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62947 -> 62947) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 72.536 / 89.776 / 168.767 / 225.481 |
| MSPT mean, ms | 79.581 |
| TPS estimate (1000/p50) | 13.8 |
| heap used / max, MB | 1858.9 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 6.47% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 6.33% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 3.38% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 4 | 3.22% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 5 | 2.74% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 6 | 2.55% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 7 | 2.48% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 8 | 2.12% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 9 | 1.89% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 10 | 1.62% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 11 | 1.56% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 12 | 1.48% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 13 | 1.46% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 14 | 1.40% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 15 | 1.34% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |

