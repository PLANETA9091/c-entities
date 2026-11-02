# Baseline run: `items-100k`

Server: `26.3-237-7e88a0d (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 63017 -> 63017) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 76.125 / 91.105 / 168.718 / 217.895 |
| MSPT mean, ms | 82.660 |
| TPS estimate (1000/p50) | 13.1 |
| heap used / max, MB | 2169.0 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 9.42% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.48% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 3 | 3.99% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 4 | 2.87% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 5 | 2.70% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 6 | 2.62% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 2.52% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 8 | 1.92% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 9 | 1.90% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 10 | 1.86% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 11 | 1.84% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 12 | 1.66% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 1.54% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 14 | 1.52% | `java.util.EnumMap.get(Object) line: 248` |
| 15 | 1.44% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |

