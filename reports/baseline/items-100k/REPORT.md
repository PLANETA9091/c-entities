# Baseline run: `items-100k`

Server: `26.3-249-1a995d6 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62981 -> 62981) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 67.996 / 113.929 / 174.321 / 304.210 |
| MSPT mean, ms | 80.638 |
| TPS estimate (1000/p50) | 14.7 |
| heap used / max, MB | 1804.7 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 7.30% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 5.43% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 3 | 3.64% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 4 | 3.44% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 5 | 3.09% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 2.18% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 7 | 2.16% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 2.06% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.74% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 10 | 1.73% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 11 | 1.72% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 12 | 1.61% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 13 | 1.59% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 14 | 1.56% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 15 | 1.35% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |

