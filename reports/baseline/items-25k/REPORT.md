# Baseline run: `items-25k`

Server: `1.21.8-60-29c8822 (MC: 1.21.8)`

## Results

| metric | value |
|---|---|
| entities spawned | 25000 (items 25000 / stands 0 / zombies 0) (merge shrink: 19920 -> 19920) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 18.857 / 22.850 / 33.527 / 43.139 |
| MSPT mean, ms | 19.863 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1351.7 / 8192.0 |
| measured ticks | 600 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.25% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 2 | 2.94% | `java.util.HashMap.getNode(Object) line: 585` |
| 3 | 2.90% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 4 | 2.67% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 5 | 2.47% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 6 | 2.27% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 7 | 2.08% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 8 | 1.92% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 9 | 1.88% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1302` |
| 10 | 1.84% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.80% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 12 | 1.80% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 13 | 1.80% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 69` |
| 14 | 1.61% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 15 | 1.41% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |

