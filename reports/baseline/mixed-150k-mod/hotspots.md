# JVM hotspots (JFR ExecutionSample)

Samples: 10240

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 11.06% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10834` |
| 2 | 6.49% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16571` |
| 3 | 5.47% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 3.55% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9533` |
| 5 | 3.54% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 2.68% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15291` |
| 7 | 2.48% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13915` |
| 8 | 1.70% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 9 | 1.35% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9279` |
| 10 | 1.29% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 11 | 1.18% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6775` |
| 12 | 1.15% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 13 | 1.12% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 14 | 0.96% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 6034` |
| 15 | 0.94% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16590` |
| 16 | 0.92% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3245` |
| 17 | 0.83% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 1000` |
| 18 | 0.81% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 19 | 0.81% | `java.util.HashMap.getNode(Object) line: 586` |
| 20 | 0.80% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 21 | 0.68% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3241` |
| 22 | 0.59% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1029` |
| 23 | 0.57% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 24 | 0.53% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 25 | 0.51% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1044` |
| 26 | 0.47% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 27 | 0.42% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3240` |
| 28 | 0.41% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 29 | 0.41% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 6038` |
| 30 | 0.38% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10847` |
| 31 | 0.38% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 32 | 0.38% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 16833` |
| 33 | 0.37% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 34 | 0.35% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 35 | 0.34% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 36 | 0.32% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 37 | 0.31% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10749` |
| 38 | 0.30% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 39 | 0.30% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 40 | 0.30% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 41 | 0.30% | `net.minecraft.world.entity.EntitySelector.lambda$pushableBy$0(Entity, Team, Team$CollisionRule, Entity) line: 52` |
| 42 | 0.27% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 16831` |
| 43 | 0.26% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 17168` |
| 44 | 0.26% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1046` |
| 45 | 0.25% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2678` |
| 46 | 0.25% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1041` |
| 47 | 0.25% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1027` |
| 48 | 0.24% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 49 | 0.23% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean, int) line: 6397` |
| 50 | 0.22% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 51 | 0.22% | `dev.planeta9091.centities.CeHooks.syncVelReadWord(Entity, int) line: 16406` |
| 52 | 0.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8968` |
| 53 | 0.21% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean, int) line: 6303` |
| 54 | 0.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9251` |
| 55 | 0.19% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3250` |
| 56 | 0.19% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6784` |
| 57 | 0.19% | `dev.planeta9091.centities.CeHooks.standQuietStamp(ArmorStand, int) line: 5899` |
| 58 | 0.19% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 16845` |
| 59 | 0.18% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 60 | 0.18% | `dev.planeta9091.centities.CeHooks.cellOfBox(AABB) line: 2928` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 12.41% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 2 | 7.96% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 16849` |
| 3 | 6.72% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 4 | 3.54% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 3.07% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 6 | 3.07% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 6035` |
| 7 | 2.93% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8513` |
| 8 | 2.20% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10834` |
| 9 | 1.74% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6236` |
| 10 | 1.66% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 11 | 1.54% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9251` |
| 12 | 1.54% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean, int) line: 6510` |
| 13 | 1.49% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16575` |
| 14 | 1.44% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1048` |
| 15 | 1.29% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15299` |
| 16 | 1.19% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10877` |
| 17 | 1.00% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean) line: 6278` |
| 18 | 0.99% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 19 | 0.86% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 20 | 0.85% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 21 | 0.78% | `java.util.HashMap.containsKey(Object) line: 603` |
| 22 | 0.77% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15308` |
| 23 | 0.75% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 6036` |
| 24 | 0.74% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 25 | 0.69% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 26 | 0.62% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 27 | 0.47% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 28 | 0.47% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 896` |
| 29 | 0.43% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 6037` |
| 30 | 0.39% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |

GC pauses: 0, total 0 ms

