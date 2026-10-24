# Baseline run: `spread-100k-mod`

Server: `26.3-222-e3ba7b1 (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97362 -> 97362) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 424.737 / 490.639 / 512.002 / 571.339 |
| MSPT mean, ms | 406.947 |
| TPS estimate (1000/p50) | 2.4 |
| heap used / max, MB | 1918.8 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 3.20% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 2 | 2.96% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3907` |
| 3 | 2.95% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 4 | 2.63% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 2.00% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 6 | 1.91% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 1.77% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 8 | 1.38% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 9 | 1.21% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 10 | 1.16% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 11 | 1.14% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 12 | 1.01% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 13 | 0.95% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1420` |
| 14 | 0.90% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 15 | 0.88% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |

