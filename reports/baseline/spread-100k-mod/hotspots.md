# JVM hotspots (JFR ExecutionSample)

Samples: 9308

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 7.41% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10331` |
| 2 | 5.34% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9132` |
| 3 | 4.88% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15622` |
| 4 | 3.94% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 3.16% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 6 | 2.04% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 7 | 1.90% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14742` |
| 8 | 1.50% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 9 | 1.50% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6600` |
| 10 | 1.35% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 11 | 1.28% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8843` |
| 12 | 1.22% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 13 | 1.20% | `sun.misc.Unsafe.getBoolean(Object, long) line: 252` |
| 14 | 1.02% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 15 | 0.87% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 16 | 0.86% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 17 | 0.82% | `dev.planeta9091.centities.CeHooks.ubsEmpty(Entity) line: 14965` |
| 18 | 0.78% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5924` |
| 19 | 0.77% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 20 | 0.72% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10375` |
| 21 | 0.70% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 22 | 0.69% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 23 | 0.64% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 24 | 0.58% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 25 | 0.58% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5928` |
| 26 | 0.56% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 27 | 0.55% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 28 | 0.55% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 29 | 0.46% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 30 | 0.46% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 31 | 0.44% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10344` |
| 32 | 0.40% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15643` |
| 33 | 0.39% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 34 | 0.39% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1024` |
| 35 | 0.39% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1025` |
| 36 | 0.37% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 37 | 0.35% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(int, int) line: 929` |
| 38 | 0.35% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3163` |
| 39 | 0.35% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 40 | 0.34% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10275` |
| 41 | 0.34% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1034` |
| 42 | 0.33% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 43 | 0.30% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 44 | 0.29% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 45 | 0.28% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 46 | 0.27% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1036` |
| 47 | 0.26% | `net.minecraft.world.level.levelgen.Heightmap.update(int, int, int, BlockState) line: 101` |
| 48 | 0.26% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 49 | 0.25% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3158` |
| 50 | 0.25% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8817` |
| 51 | 0.24% | `org.spigotmc.WatchdogThread.run() line: 66` |
| 52 | 0.24% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13411` |
| 53 | 0.23% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2599` |
| 54 | 0.21% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.lambda$schedule$0(int, int, NewChunkHolder, List, int, int) line: 762` |
| 55 | 0.20% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6609` |
| 56 | 0.20% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6599` |
| 57 | 0.19% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 58 | 0.19% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3159` |
| 59 | 0.18% | `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock.unlock(ReentrantAreaLock$Node) line: 266` |
| 60 | 0.18% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 10.13% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 2 | 8.37% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 3 | 5.62% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 15902` |
| 4 | 2.86% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5925` |
| 5 | 2.33% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 6 | 2.03% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean) line: 6335` |
| 7 | 1.99% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10331` |
| 8 | 1.63% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13314` |
| 9 | 1.53% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6126` |
| 10 | 1.39% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8821` |
| 11 | 1.32% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13381` |
| 12 | 1.22% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14750` |
| 13 | 1.07% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 14 | 0.90% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14759` |
| 15 | 0.89% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8817` |
| 16 | 0.89% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 896` |
| 17 | 0.85% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 18 | 0.82% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 15962` |
| 19 | 0.78% | `dev.planeta9091.centities.CeHooks.itemDispatchWalk(ItemEntity, int) line: 6154` |
| 20 | 0.77% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 21 | 0.68% | `net.minecraft.util.StaticCache2D.map(StaticCache2D$MappingFunction) line: 36` |
| 22 | 0.64% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 23 | 0.63% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 24 | 0.62% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 25 | 0.60% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 26 | 0.55% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |
| 27 | 0.50% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 28 | 0.47% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 29 | 0.46% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 30 | 0.46% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 1026` |

GC pauses: 0, total 0 ms

