# Baseline run: `items-100k`

Server: `26.3-253-63e41db (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 63028 -> 63028) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 71.270 / 116.963 / 176.332 / 304.920 |
| MSPT mean, ms | 83.271 |
| TPS estimate (1000/p50) | 14.0 |
| heap used / max, MB | 2046.9 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 7.13% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 3.67% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 3 | 3.60% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 4 | 3.58% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 5 | 3.56% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 2.21% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 7 | 2.13% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 8 | 2.10% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 9 | 1.74% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.71% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 11 | 1.69% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 12 | 1.54% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 13 | 1.47% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 14 | 1.42% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 15 | 1.28% | `java.util.EnumMap.get(Object) line: 248` |

