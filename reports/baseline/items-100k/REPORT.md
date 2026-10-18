# Baseline run: `items-100k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62905 -> 62905) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 65.205 / 72.138 / 109.021 / 139.155 |
| MSPT mean, ms | 68.051 |
| TPS estimate (1000/p50) | 15.3 |
| heap used / max, MB | 1368.5 / 8192.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 7.54% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 5.64% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 3 | 5.41% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 4 | 3.52% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 5 | 3.14% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 6 | 2.56% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 2.46% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 8 | 1.81% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 9 | 1.79% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 10 | 1.76% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 69` |
| 11 | 1.72% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1215` |
| 12 | 1.72% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 13 | 1.58% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 14 | 1.56% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 15 | 1.36% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |

