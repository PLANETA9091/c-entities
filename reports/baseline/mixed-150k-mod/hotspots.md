# JVM hotspots (JFR ExecutionSample)

Samples: 9424

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 9.65% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10799` |
| 2 | 6.04% | `dev.planeta9091.centities.CeHooks.sitStepsEmpty(Object) line: 15343` |
| 3 | 4.74% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 4.07% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 2.80% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15210` |
| 6 | 2.73% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9498` |
| 7 | 1.54% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 8 | 1.50% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 9 | 1.49% | `dev.planeta9091.centities.CeHooks.ubsEmpty(Entity) line: 15459` |
| 10 | 1.24% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 11 | 1.17% | `sun.misc.Unsafe.getBoolean(Object, long) line: 252` |
| 12 | 1.02% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 13 | 1.02% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9244` |
| 14 | 1.01% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6767` |
| 15 | 1.00% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16310` |
| 16 | 0.90% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 17 | 0.89% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 18 | 0.89% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 19 | 0.89% | `java.util.HashMap.getNode(Object) line: 586` |
| 20 | 0.76% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3233` |
| 21 | 0.71% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 22 | 0.71% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 1000` |
| 23 | 0.65% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3237` |
| 24 | 0.58% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 25 | 0.56% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 6026` |
| 26 | 0.52% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1029` |
| 27 | 0.50% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1030` |
| 28 | 0.50% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1044` |
| 29 | 0.48% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 30 | 0.47% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 31 | 0.46% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 32 | 0.45% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1039` |
| 33 | 0.44% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9548` |
| 34 | 0.41% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3232` |
| 35 | 0.39% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 36 | 0.34% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10812` |
| 37 | 0.34% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean, int) line: 6389` |
| 38 | 0.34% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean, int) line: 6295` |
| 39 | 0.33% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 6030` |
| 40 | 0.33% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1041` |
| 41 | 0.32% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 42 | 0.31% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 43 | 0.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 44 | 0.28% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 45 | 0.28% | `net.minecraft.world.entity.EntitySelector.lambda$pushableBy$0(Entity, Team, Team$CollisionRule, Entity) line: 52` |
| 46 | 0.27% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9216` |
| 47 | 0.27% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2673` |
| 48 | 0.27% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1518` |
| 49 | 0.27% | `dev.planeta9091.centities.CeHooks.standQuietStamp(ArmorStand, int) line: 5891` |
| 50 | 0.25% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10714` |
| 51 | 0.25% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 16553` |
| 52 | 0.24% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 53 | 0.23% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 54 | 0.23% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 55 | 0.22% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2701` |
| 56 | 0.21% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6766` |
| 57 | 0.21% | `dev.planeta9091.centities.CeHooks.batchUpsert(int, int, Entity, AABB, boolean, int) line: 2834` |
| 58 | 0.20% | `java.lang.Double.isFinite(double) line: 1023` |
| 59 | 0.20% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1048` |
| 60 | 0.19% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 11.10% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 2 | 6.04% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16288` |
| 3 | 5.79% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 4 | 4.07% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 3.41% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 6 | 2.94% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10799` |
| 7 | 2.10% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 6027` |
| 8 | 1.79% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 16569` |
| 9 | 1.68% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9216` |
| 10 | 1.60% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13782` |
| 11 | 1.54% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15218` |
| 12 | 1.49% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 16629` |
| 13 | 1.47% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13849` |
| 14 | 1.44% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6228` |
| 15 | 1.44% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean, int) line: 6502` |
| 16 | 1.32% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10842` |
| 17 | 1.27% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean) line: 6270` |
| 18 | 1.10% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 19 | 1.06% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 20 | 1.06% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 21 | 1.00% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 22 | 0.91% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15227` |
| 23 | 0.83% | `java.util.HashMap.containsKey(Object) line: 603` |
| 24 | 0.81% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 6028` |
| 25 | 0.74% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 896` |
| 26 | 0.71% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 27 | 0.68% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16295` |
| 28 | 0.64% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1048` |
| 29 | 0.54% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 30 | 0.48% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 6029` |

GC pauses: 0, total 0 ms

