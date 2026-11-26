# JVM hotspots (JFR ExecutionSample)

Samples: 8749

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 8.58% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10829` |
| 2 | 4.50% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9578` |
| 3 | 3.94% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 3.23% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 5 | 2.61% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 6 | 2.54% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 7 | 1.85% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15240` |
| 8 | 1.78% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6770` |
| 9 | 1.68% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16449` |
| 10 | 1.45% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 11 | 1.29% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9274` |
| 12 | 1.27% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 13 | 1.21% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 14 | 1.07% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 6029` |
| 15 | 1.05% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 16 | 0.90% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 17 | 0.70% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 18 | 0.70% | `sun.misc.Unsafe.getBoolean(Object, long) line: 252` |
| 19 | 0.66% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 20 | 0.62% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3240` |
| 21 | 0.58% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 22 | 0.56% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 243` |
| 23 | 0.54% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 24 | 0.54% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 25 | 0.51% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16468` |
| 26 | 0.50% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 27 | 0.47% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 28 | 0.46% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 29 | 0.45% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1030` |
| 30 | 0.42% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 31 | 0.40% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 32 | 0.40% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 6033` |
| 33 | 0.39% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 34 | 0.39% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1029` |
| 35 | 0.37% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 36 | 0.35% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(int, int) line: 930` |
| 37 | 0.35% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 38 | 0.35% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 39 | 0.34% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 40 | 0.33% | `java.util.Map.forEach(BiConsumer) line: 723` |
| 41 | 0.32% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9246` |
| 42 | 0.32% | `dev.planeta9091.centities.CeHooks.syncVelReadWord(Entity, int) line: 16292` |
| 43 | 0.31% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10842` |
| 44 | 0.31% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6769` |
| 45 | 0.30% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 16711` |
| 46 | 0.29% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 47 | 0.29% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3236` |
| 48 | 0.29% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1044` |
| 49 | 0.27% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1039` |
| 50 | 0.26% | `net.minecraft.world.level.chunk.ProtoChunk.setBlockState(BlockPos, BlockState, int) line: 215` |
| 51 | 0.25% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 52 | 0.25% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1041` |
| 53 | 0.24% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.lambda$schedule$0(int, int, NewChunkHolder, List, int, int) line: 762` |
| 54 | 0.24% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10744` |
| 55 | 0.23% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 56 | 0.22% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean) line: 6273` |
| 57 | 0.22% | `dev.planeta9091.centities.CeHooks.batchUpsert(int, int, Entity, AABB, boolean, int) line: 2837` |
| 58 | 0.22% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15247` |
| 59 | 0.22% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 60 | 0.19% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 9.84% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 2 | 7.61% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 3 | 3.14% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16453` |
| 4 | 3.04% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 6030` |
| 5 | 2.78% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 16727` |
| 6 | 2.33% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean, int) line: 6505` |
| 7 | 2.26% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 8 | 2.03% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10829` |
| 9 | 1.79% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6231` |
| 10 | 1.25% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9250` |
| 11 | 1.21% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 12 | 1.21% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13879` |
| 13 | 1.17% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 14 | 1.13% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13812` |
| 15 | 1.12% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 16 | 1.05% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15248` |
| 17 | 0.91% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9246` |
| 18 | 0.89% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15257` |
| 19 | 0.82% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 20 | 0.77% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10872` |
| 21 | 0.74% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean) line: 6273` |
| 22 | 0.72% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 23 | 0.63% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1048` |
| 24 | 0.59% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 896` |
| 25 | 0.55% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |
| 26 | 0.53% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 27 | 0.51% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 28 | 0.50% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 29 | 0.47% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 30 | 0.45% | `it.unimi.dsi.fastutil.objects.ReferenceCollections$UnmodifiableCollection.contains(Object) line: 344` |

GC pauses: 0, total 0 ms

