# Baseline run: `items-100k`

Server: `26.3-257-d81ca50 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62985 -> 62985) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 70.126 / 124.599 / 187.684 / 249.861 |
| MSPT mean, ms | 83.803 |
| TPS estimate (1000/p50) | 14.3 |
| heap used / max, MB | 2336.9 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 5.10% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.02% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 3 | 3.41% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 3.16% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 5 | 2.55% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 6 | 1.84% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 7 | 1.78% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 8 | 1.69% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 9 | 1.67% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 10 | 1.46% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 11 | 1.43% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.41% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 13 | 1.40% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 14 | 1.33% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 15 | 1.23% | `net.minecraft.world.entity.Entity.getX() line: 5100` |

