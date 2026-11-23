# JVM hotspots (JFR ExecutionSample)

Samples: 10556

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 10.52% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10701` |
| 2 | 6.61% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15992` |
| 3 | 5.49% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 4.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9400` |
| 5 | 3.56% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 2.67% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15112` |
| 7 | 1.82% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 8 | 1.44% | `sun.misc.Unsafe.getBoolean(Object, long) line: 252` |
| 9 | 1.41% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 10 | 1.35% | `dev.planeta9091.centities.CeHooks.ubsEmpty(Entity) line: 15335` |
| 11 | 1.28% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 12 | 1.26% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6706` |
| 13 | 1.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9161` |
| 14 | 1.13% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 15 | 1.11% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 16 | 1.08% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5967` |
| 17 | 0.89% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 18 | 0.81% | `java.util.HashMap.getNode(Object) line: 586` |
| 19 | 0.81% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16013` |
| 20 | 0.74% | `net.minecraft.world.entity.EntitySelector.lambda$pushableBy$0(Entity, Team, Team$CollisionRule, Entity) line: 52` |
| 21 | 0.71% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3173` |
| 22 | 0.65% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5971` |
| 23 | 0.64% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1044` |
| 24 | 0.63% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 25 | 0.63% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 1000` |
| 26 | 0.62% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3177` |
| 27 | 0.61% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 28 | 0.60% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 29 | 0.57% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 30 | 0.54% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 31 | 0.46% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 32 | 0.44% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 33 | 0.44% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1039` |
| 34 | 0.43% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 35 | 0.40% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1030` |
| 36 | 0.39% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10714` |
| 37 | 0.39% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1041` |
| 38 | 0.34% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 39 | 0.31% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 40 | 0.31% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 41 | 0.30% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2613` |
| 42 | 0.29% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3172` |
| 43 | 0.28% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 44 | 0.26% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1029` |
| 45 | 0.24% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean, int) line: 6330` |
| 46 | 0.23% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 16256` |
| 47 | 0.22% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 48 | 0.22% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3182` |
| 49 | 0.22% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 50 | 0.21% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 51 | 0.21% | `java.lang.Double.isFinite(double) line: 1023` |
| 52 | 0.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9133` |
| 53 | 0.20% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 54 | 0.19% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 55 | 0.19% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6715` |
| 56 | 0.19% | `dev.planeta9091.centities.CeHooks.batchUpsert(int, int, Entity, AABB, boolean, int) line: 2774` |
| 57 | 0.17% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 58 | 0.17% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2641` |
| 59 | 0.17% | `java.util.ArrayList$Itr.next() line: 1058` |
| 60 | 0.16% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 11.60% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 2 | 7.85% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 16272` |
| 3 | 7.08% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 4 | 3.59% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5968` |
| 5 | 3.56% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 3.11% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 7 | 2.59% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10701` |
| 8 | 2.05% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6169` |
| 9 | 1.86% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13684` |
| 10 | 1.77% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13751` |
| 11 | 1.68% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean, int) line: 6441` |
| 12 | 1.44% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9133` |
| 13 | 1.41% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15120` |
| 14 | 1.35% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 16332` |
| 15 | 1.17% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 16 | 1.15% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 17 | 1.15% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10744` |
| 18 | 1.00% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 19 | 0.99% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 20 | 0.94% | `dev.planeta9091.centities.CeHooks.syncVelRead(Entity) line: 15904` |
| 21 | 0.90% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15129` |
| 22 | 0.77% | `java.util.HashMap.containsKey(Object) line: 603` |
| 23 | 0.75% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 24 | 0.73% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean) line: 6211` |
| 25 | 0.65% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 26 | 0.65% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1048` |
| 27 | 0.63% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 28 | 0.52% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 896` |
| 29 | 0.52% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15998` |
| 30 | 0.47% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |

GC pauses: 0, total 0 ms

