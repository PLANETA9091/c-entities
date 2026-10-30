# Baseline run: `items-100k`

Server: `26.3-231-8a170a3 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 62940 -> 62940) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 68.710 / 84.566 / 154.829 / 212.308 |
| MSPT mean, ms | 75.499 |
| TPS estimate (1000/p50) | 14.6 |
| heap used / max, MB | 1862.4 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 9.74% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 5.41% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 3 | 5.02% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 4 | 3.94% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 5 | 3.72% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 6 | 2.78% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 2.41% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 2.10% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 9 | 2.00% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 10 | 1.67% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 11 | 1.67% | `java.util.EnumMap.get(Object) line: 248` |
| 12 | 1.66% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 13 | 1.63% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 14 | 1.62% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 15 | 1.32% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |

