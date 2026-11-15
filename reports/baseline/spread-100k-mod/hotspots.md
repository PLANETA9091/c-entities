# JVM hotspots (JFR ExecutionSample)

Samples: 10140

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 9.21% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8335` |
| 2 | 6.52% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13141` |
| 3 | 5.73% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7583` |
| 4 | 4.22% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 5 | 3.44% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 6 | 3.33% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.47% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 2.18% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 9 | 2.06% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 10 | 1.71% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5003` |
| 11 | 1.65% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 12 | 1.59% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5230` |
| 13 | 1.54% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 14 | 1.27% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7454` |
| 15 | 1.08% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 16 | 1.08% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 17 | 1.07% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 18 | 0.98% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 19 | 0.90% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 20 | 0.87% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 21 | 0.60% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 22 | 0.59% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 243` |
| 23 | 0.49% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 24 | 0.47% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 25 | 0.45% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 26 | 0.45% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5234` |
| 27 | 0.41% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 9931` |
| 28 | 0.38% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 29 | 0.37% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 30 | 0.35% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 31 | 0.34% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 12855` |
| 32 | 0.34% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 33 | 0.33% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 34 | 0.32% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 35 | 0.31% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 36 | 0.29% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(int, int) line: 929` |
| 37 | 0.26% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 38 | 0.26% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13040` |
| 39 | 0.26% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5147` |
| 40 | 0.26% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 41 | 0.25% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.lambda$schedule$0(int, int, NewChunkHolder, List, int, int) line: 762` |
| 42 | 0.25% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7406` |
| 43 | 0.24% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7382` |
| 44 | 0.24% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2464` |
| 45 | 0.23% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 46 | 0.22% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 47 | 0.20% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12994` |
| 48 | 0.19% | `ca.spottedleaf.concurrentutil.lock.ReentrantAreaLock.unlock(ReentrantAreaLock$Node) line: 266` |
| 49 | 0.19% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11271` |
| 50 | 0.18% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1156` |
| 51 | 0.18% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 5508` |
| 52 | 0.18% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 53 | 0.18% | `dev.planeta9091.centities.CeHooks.nativeMobTick(Mob) line: 11059` |
| 54 | 0.17% | `java.util.Collection.stream() line: 748` |
| 55 | 0.17% | `net.minecraft.server.level.WorldGenRegion.lambda$new$2(ChunkAccess, ChunkStep, GenerationChunkHolder, int, int) line: 124` |
| 56 | 0.17% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 57 | 0.16% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 58 | 0.16% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 59 | 0.16% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 60 | 0.16% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 9.92% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 2 | 8.71% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 3 | 6.73% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 5.80% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 4.19% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5771` |
| 6 | 3.64% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 7 | 3.07% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8345` |
| 8 | 1.75% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5101` |
| 9 | 1.41% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7392` |
| 10 | 1.20% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11177` |
| 11 | 1.15% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5159` |
| 12 | 1.08% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 13 | 0.96% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 14 | 0.96% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 15 | 0.94% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11244` |
| 16 | 0.92% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8365` |
| 17 | 0.90% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 18 | 0.79% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 19 | 0.64% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13245` |
| 20 | 0.61% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 21 | 0.51% | `dev.planeta9091.centities.CeHooks$NmMap.get(int) line: 9938` |
| 22 | 0.49% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |
| 23 | 0.46% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8361` |
| 24 | 0.45% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 25 | 0.45% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 848` |
| 26 | 0.43% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 27 | 0.39% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13232` |
| 28 | 0.37% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 29 | 0.37% | `it.unimi.dsi.fastutil.objects.ReferenceCollections$UnmodifiableCollection.contains(Object) line: 344` |
| 30 | 0.37% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 1025` |

GC pauses: 0, total 0 ms

