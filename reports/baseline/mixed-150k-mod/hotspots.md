# JVM hotspots (JFR ExecutionSample)

Samples: 9568

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 12.37% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10555` |
| 2 | 6.93% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15846` |
| 3 | 6.46% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 5.38% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9255` |
| 5 | 3.60% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 2.58% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14966` |
| 7 | 1.70% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 8 | 1.63% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 9 | 1.45% | `sun.misc.Unsafe.getBoolean(Object, long) line: 252` |
| 10 | 1.41% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5938` |
| 11 | 1.28% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 12 | 1.09% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 13 | 0.92% | `dev.planeta9091.centities.CeHooks.ubsEmpty(Entity) line: 15189` |
| 14 | 0.90% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 15 | 0.89% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 16 | 0.86% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3173` |
| 17 | 0.86% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 18 | 0.80% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 19 | 0.80% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15867` |
| 20 | 0.78% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 21 | 0.74% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 22 | 0.70% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3177` |
| 23 | 0.69% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5942` |
| 24 | 0.67% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1030` |
| 25 | 0.64% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1044` |
| 26 | 0.63% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1041` |
| 27 | 0.61% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 28 | 0.59% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9016` |
| 29 | 0.55% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6632` |
| 30 | 0.53% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 31 | 0.52% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 32 | 0.50% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 33 | 0.47% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10568` |
| 34 | 0.47% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 35 | 0.47% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1029` |
| 36 | 0.45% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 37 | 0.44% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 38 | 0.43% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 39 | 0.42% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 40 | 0.38% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3172` |
| 41 | 0.33% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2613` |
| 42 | 0.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 43 | 0.27% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1039` |
| 44 | 0.25% | `java.util.ArrayList$Itr.next() line: 1058` |
| 45 | 0.23% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 46 | 0.21% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1046` |
| 47 | 0.20% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 48 | 0.19% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10470` |
| 49 | 0.19% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2641` |
| 50 | 0.19% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 51 | 0.18% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5173` |
| 52 | 0.18% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3182` |
| 53 | 0.18% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3952` |
| 54 | 0.17% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 55 | 0.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9435` |
| 56 | 0.17% | `dev.planeta9091.centities.CeHooks.batchUpsert(int, int, Entity, AABB, boolean, int) line: 2774` |
| 57 | 0.17% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 58 | 0.17% | `dev.planeta9091.centities.CeHooks.stackFast(ItemEntity) line: 1179` |
| 59 | 0.17% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 60 | 0.16% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 13.71% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 2 | 8.06% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 16126` |
| 3 | 7.56% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 4 | 4.07% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5939` |
| 5 | 3.60% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 3.29% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10555` |
| 7 | 2.95% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 8 | 2.32% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6140` |
| 9 | 1.80% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13538` |
| 10 | 1.67% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13605` |
| 11 | 1.63% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14974` |
| 12 | 1.37% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8990` |
| 13 | 1.11% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 14 | 1.08% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 15 | 1.00% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 16 | 0.99% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 17 | 0.98% | `dev.planeta9091.centities.CeHooks.syncVelRead(Entity) line: 15758` |
| 18 | 0.97% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean, int) line: 6367` |
| 19 | 0.97% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10598` |
| 20 | 0.92% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 16186` |
| 21 | 0.88% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1048` |
| 22 | 0.77% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14983` |
| 23 | 0.77% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 24 | 0.73% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 25 | 0.73% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15852` |
| 26 | 0.72% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 27 | 0.65% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8994` |
| 28 | 0.60% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean) line: 6182` |
| 29 | 0.54% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 896` |
| 30 | 0.52% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5940` |

GC pauses: 0, total 0 ms

