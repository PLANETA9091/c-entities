# Baseline run: `spread-100k-mod`

Server: `26.3-230-804dd0c (MC: 26.3)`

## Results

| metric | value |
|---|---|
| entities spawned | 100000 (items 59941 / stands 20059 / zombies 20000) (merge shrink: 97391 -> 97391) |
| block entities | 0 (hoppers 0 / furnaces 0) |
| MSPT p50 / p90 / p99 / max, ms | 548.702 / 664.002 / 943.776 / 1207.981 |
| MSPT mean, ms | 577.676 |
| TPS estimate (1000/p50) | 1.8 |
| heap used / max, MB | 2215.2 / 4096.0 |
| measured ticks | 1200 |

## What burns the time (JFR, top-15 self-time)

| # | % | method |
|---:|---:|---|
| 1 | 19.01% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.64% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 3 | 2.29% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5274` |
| 4 | 2.08% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 5 | 1.76% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 1.64% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 1.59% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 8 | 1.48% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 9 | 1.42% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 10 | 1.39% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8378` |
| 11 | 1.19% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8237` |
| 12 | 1.08% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 13 | 1.05% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 14 | 1.05% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 15 | 1.00% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |

