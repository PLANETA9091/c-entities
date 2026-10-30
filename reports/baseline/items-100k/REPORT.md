# Baseline run: `items-100k`

Server: `26.3-231-8a170a3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62919 -> 62919) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 68.383 / 87.379 / 154.214 / 226.062 |
| MSPT mean, ms | 75.177 |
| TPS estimate (1000/p50) | 14.6 |
| heap used / max, MB | 2096.6 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 8.98% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.65% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 3 | 4.30% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 4 | 4.09% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 3.63% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 6 | 3.22% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 7 | 2.06% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 8 | 1.81% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 9 | 1.63% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 10 | 1.54% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.41% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 12 | 1.35% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 13 | 1.27% | `java.util.EnumMap.get(Object) line: 248` |
| 14 | 1.25% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 15 | 1.18% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |

