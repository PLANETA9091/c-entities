# JVM hotspots (JFR ExecutionSample)

Samples: 9264

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 9.95% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8285` |
| 2 | 7.12% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7533` |
| 3 | 6.78% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13091` |
| 4 | 4.39% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 5 | 3.86% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 3.55% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 7 | 3.25% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 8 | 3.16% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 2.37% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5003` |
| 10 | 1.89% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5216` |
| 11 | 1.53% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 12 | 1.53% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 13 | 1.41% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 14 | 1.28% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 15 | 1.16% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 16 | 1.04% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 17 | 0.72% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7422` |
| 18 | 0.66% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 19 | 0.65% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 20 | 0.58% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5220` |
| 21 | 0.57% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 22 | 0.57% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 23 | 0.56% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 24 | 0.55% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 25 | 0.50% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5222` |
| 26 | 0.43% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 27 | 0.42% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 28 | 0.39% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7378` |
| 29 | 0.37% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 9881` |
| 30 | 0.32% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 31 | 0.32% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 32 | 0.31% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 33 | 0.31% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 34 | 0.29% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 35 | 0.29% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 12805` |
| 36 | 0.27% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 37 | 0.26% | `org.spigotmc.WatchdogThread.monotonicMillis() line: 36` |
| 38 | 0.25% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 39 | 0.25% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 40 | 0.24% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 41 | 0.24% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 42 | 0.23% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1907` |
| 43 | 0.22% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 5494` |
| 44 | 0.21% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 45 | 0.21% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12990` |
| 46 | 0.19% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 47 | 0.19% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8295` |
| 48 | 0.19% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 49 | 0.17% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 50 | 0.17% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2464` |
| 51 | 0.16% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 52 | 0.16% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 53 | 0.16% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 54 | 0.16% | `sun.misc.Unsafe.beforeMemoryAccess() line: 1777` |
| 55 | 0.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7354` |
| 56 | 0.16% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11221` |
| 57 | 0.16% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 58 | 0.15% | `ca.spottedleaf.moonrise.patches.starlight.light.SWMRNibbleArray.getUpdating(int) line: 393` |
| 59 | 0.15% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(int, int) line: 929` |
| 60 | 0.14% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 10.70% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 2 | 9.50% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 3 | 7.03% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 6.90% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 4.30% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5757` |
| 6 | 3.99% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8295` |
| 7 | 3.72% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 8 | 2.40% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5116` |
| 9 | 1.47% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11194` |
| 10 | 1.41% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 11 | 1.23% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5145` |
| 12 | 1.19% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11127` |
| 13 | 1.12% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7364` |
| 14 | 1.00% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 15 | 0.77% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 16 | 0.70% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8315` |
| 17 | 0.66% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 18 | 0.60% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 19 | 0.59% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 20 | 0.57% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 21 | 0.55% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 22 | 0.55% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 23 | 0.53% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13195` |
| 24 | 0.53% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 25 | 0.51% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 26 | 0.49% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 27 | 0.44% | `dev.planeta9091.centities.CeHooks$NmMap.get(int) line: 9888` |
| 28 | 0.42% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 29 | 0.41% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 30 | 0.32% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 848` |

GC pauses: 0, total 0 ms

