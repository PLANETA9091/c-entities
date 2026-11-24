# Baseline run: `items-100k`

Server: `26.3-262-827eeeb (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62974 -> 62974) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 70.231 / 121.563 / 179.753 / 230.923 |
| MSPT mean, ms | 83.060 |
| TPS estimate (1000/p50) | 14.2 |
| heap used / max, MB | 1843.5 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 7.18% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 3.98% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 3 | 3.83% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 4 | 3.73% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 5 | 2.98% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 2.64% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 7 | 1.97% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 1.88% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.69% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 10 | 1.56% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 11 | 1.55% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 12 | 1.55% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 13 | 1.50% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 14 | 1.47% | `java.util.EnumMap.get(Object) line: 248` |
| 15 | 1.37% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |

