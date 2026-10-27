# Baseline run: `items-100k`

Server: `26.3-226-4f49049 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62948 -> 62948) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 57.546 / 69.278 / 126.068 / 167.667 |
| MSPT mean, ms | 62.826 |
| TPS estimate (1000/p50) | 17.4 |
| heap used / max, MB | 2196.9 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 7.34% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.25% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 3 | 4.03% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 4 | 3.80% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 5 | 3.35% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 6 | 3.06% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 2.07% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 8 | 2.07% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.69% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 10 | 1.57% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 11 | 1.53% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 12 | 1.40% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 13 | 1.16% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 14 | 1.14% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 15 | 1.04% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |

