# JVM hotspots (JFR ExecutionSample)

Samples: 9379

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 10.07% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10829` |
| 2 | 6.74% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 6.16% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16449` |
| 4 | 4.93% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9528` |
| 5 | 4.12% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 2.36% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15240` |
| 7 | 1.79% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 8 | 1.49% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 9 | 1.45% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 10 | 1.45% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 6029` |
| 11 | 1.17% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 12 | 1.13% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 13 | 0.97% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 14 | 0.97% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3240` |
| 15 | 0.96% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6770` |
| 16 | 0.93% | `sun.misc.Unsafe.getBoolean(Object, long) line: 252` |
| 17 | 0.92% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 18 | 0.90% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 19 | 0.87% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1029` |
| 20 | 0.78% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1030` |
| 21 | 0.77% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1044` |
| 22 | 0.76% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16468` |
| 23 | 0.74% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 24 | 0.73% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 25 | 0.69% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9274` |
| 26 | 0.68% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 27 | 0.65% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 6033` |
| 28 | 0.64% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 29 | 0.63% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3236` |
| 30 | 0.60% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5185` |
| 31 | 0.50% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 32 | 0.44% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16417` |
| 33 | 0.43% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 34 | 0.43% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 35 | 0.42% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2676` |
| 36 | 0.42% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3235` |
| 37 | 0.36% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 38 | 0.35% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1039` |
| 39 | 0.35% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 40 | 0.34% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1518` |
| 41 | 0.33% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 42 | 0.32% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10842` |
| 43 | 0.32% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 44 | 0.31% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6779` |
| 45 | 0.31% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 46 | 0.29% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 47 | 0.28% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 48 | 0.28% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 49 | 0.27% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10744` |
| 50 | 0.27% | `java.util.ArrayList$Itr.next() line: 1058` |
| 51 | 0.27% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 52 | 0.27% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 17038` |
| 53 | 0.23% | `sun.misc.Unsafe.beforeMemoryAccess() line: 1777` |
| 54 | 0.23% | `dev.planeta9091.centities.CeHooks.standCheckActive(ArmorStand) line: 5939` |
| 55 | 0.20% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5173` |
| 56 | 0.20% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean, int) line: 6392` |
| 57 | 0.20% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2701` |
| 58 | 0.20% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6769` |
| 59 | 0.20% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1046` |
| 60 | 0.20% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1485` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 11.28% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 2 | 7.67% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 16727` |
| 3 | 7.27% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 4 | 4.12% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 3.87% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 6030` |
| 6 | 3.54% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10829` |
| 7 | 2.76% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 8 | 2.54% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6231` |
| 9 | 1.72% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13879` |
| 10 | 1.61% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean, int) line: 6505` |
| 11 | 1.45% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 12 | 1.45% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15248` |
| 13 | 1.40% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13812` |
| 14 | 1.38% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9246` |
| 15 | 1.27% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16453` |
| 16 | 1.26% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 17 | 1.17% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10872` |
| 18 | 1.09% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 19 | 1.06% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 896` |
| 20 | 1.03% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 21 | 1.02% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1048` |
| 22 | 1.01% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 23 | 0.94% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15257` |
| 24 | 0.85% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 25 | 0.78% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 26 | 0.66% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 27 | 0.66% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 6031` |
| 28 | 0.64% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean) line: 6273` |
| 29 | 0.64% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 30 | 0.58% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9250` |

GC pauses: 0, total 0 ms

