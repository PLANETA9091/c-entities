# Baseline run: `items-100k-mod`

Server: `26.3-222-e3ba7b1 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 100000 / stands 0 / zombies 0) (merge shrink: 54841 -> 54841) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 16.509 / 19.805 / 55.174 / 108.792 |
| MSPT mean, ms | 18.220 |
| TPS estimate (1000/p50) | 20.0 |
| heap used / max, MB | 1113.7 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 10.22% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 9.73% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3467` |
| 3 | 8.37% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 175` |
| 4 | 6.82% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 4363` |
| 5 | 3.37% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2059` |
| 6 | 3.15% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3613` |
| 7 | 2.73% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 8 | 2.51% | `net.minecraft.world.entity.Entity.level() line: 5415` |
| 9 | 2.04% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2147` |
| 10 | 1.82% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 11 | 1.82% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2021` |
| 12 | 1.35% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 13 | 1.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 14 | 1.28% | `net.minecraft.world.entity.Entity.isRemoved() line: 5328` |
| 15 | 1.11% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2085` |

