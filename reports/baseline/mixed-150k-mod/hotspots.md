# JVM hotspots (JFR ExecutionSample)

Samples: 10443

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 10.94% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10831` |
| 2 | 6.00% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16558` |
| 3 | 5.61% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 3.53% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 3.53% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9530` |
| 6 | 2.77% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15288` |
| 7 | 2.34% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13912` |
| 8 | 1.42% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 9 | 1.38% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 10 | 1.17% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6772` |
| 11 | 1.13% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9276` |
| 12 | 1.09% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16577` |
| 13 | 0.95% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 14 | 0.93% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 15 | 0.84% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 16 | 0.83% | `java.util.HashMap.getNode(Object) line: 586` |
| 17 | 0.83% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3238` |
| 18 | 0.79% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10875` |
| 19 | 0.79% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 20 | 0.78% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 6031` |
| 21 | 0.60% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3242` |
| 22 | 0.57% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1030` |
| 23 | 0.55% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 6035` |
| 24 | 0.55% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 1000` |
| 25 | 0.53% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1039` |
| 26 | 0.51% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 27 | 0.50% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 28 | 0.49% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 29 | 0.47% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10844` |
| 30 | 0.45% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10747` |
| 31 | 0.43% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 32 | 0.43% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 33 | 0.43% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 34 | 0.43% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1029` |
| 35 | 0.38% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16526` |
| 36 | 0.35% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 37 | 0.35% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 38 | 0.35% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 39 | 0.35% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 40 | 0.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 41 | 0.32% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3237` |
| 42 | 0.31% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 16820` |
| 43 | 0.30% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 16818` |
| 44 | 0.30% | `net.minecraft.world.entity.EntitySelector.lambda$pushableBy$0(Entity, Team, Team$CollisionRule, Entity) line: 52` |
| 45 | 0.27% | `dev.planeta9091.centities.CeHooks.batchUpsert(int, int, Entity, AABB, boolean, int) line: 2839` |
| 46 | 0.27% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1041` |
| 47 | 0.26% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean, int) line: 6394` |
| 48 | 0.25% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 49 | 0.25% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 17147` |
| 50 | 0.24% | `java.util.ArrayList$Itr.next() line: 1058` |
| 51 | 0.24% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1044` |
| 52 | 0.23% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 53 | 0.23% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 54 | 0.22% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9710` |
| 55 | 0.22% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6771` |
| 56 | 0.22% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1048` |
| 57 | 0.22% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13900` |
| 58 | 0.21% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3247` |
| 59 | 0.20% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15305` |
| 60 | 0.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8965` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 13.78% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 2 | 8.19% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 16836` |
| 3 | 6.34% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 4 | 3.66% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 6032` |
| 5 | 3.53% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 3.35% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 7 | 2.82% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8510` |
| 8 | 2.56% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10831` |
| 9 | 1.74% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 10 | 1.72% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean, int) line: 6507` |
| 11 | 1.63% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6233` |
| 12 | 1.52% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9248` |
| 13 | 1.38% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15296` |
| 14 | 1.22% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16562` |
| 15 | 1.01% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 16 | 0.96% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 17 | 0.92% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15305` |
| 18 | 0.86% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 19 | 0.86% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean) line: 6275` |
| 20 | 0.78% | `java.util.HashMap.containsKey(Object) line: 603` |
| 21 | 0.70% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 22 | 0.70% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1048` |
| 23 | 0.65% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 896` |
| 24 | 0.58% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 25 | 0.56% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13823` |
| 26 | 0.55% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 27 | 0.53% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9252` |
| 28 | 0.45% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 29 | 0.43% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 30 | 0.43% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 848` |

GC pauses: 0, total 0 ms

