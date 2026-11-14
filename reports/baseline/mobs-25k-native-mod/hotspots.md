# JVM hotspots (JFR ExecutionSample)

Samples: 2919

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.89% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12263` |
| 2 | 5.48% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 3 | 4.08% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 4 | 3.25% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 5 | 2.91% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7760` |
| 6 | 2.60% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 1.99% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 8 | 1.61% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.51% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 10 | 1.23% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 11 | 1.10% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 12 | 0.96% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 13 | 0.82% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 14 | 0.82% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 15 | 0.75% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12282` |
| 16 | 0.69% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 17 | 0.65% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 18 | 0.62% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 19 | 0.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 20 | 0.48% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 21 | 0.48% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 22 | 0.45% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 23 | 0.45% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12392` |
| 24 | 0.45% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12487` |
| 25 | 0.41% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 26 | 0.41% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 27 | 0.41% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 28 | 0.38% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 29 | 0.38% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |
| 30 | 0.38% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 31 | 0.34% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 32 | 0.34% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 33 | 0.34% | `sun.misc.Unsafe.getFloat(Object, long) line: 383` |
| 34 | 0.34% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 35 | 0.34% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12300` |
| 36 | 0.34% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 92` |
| 37 | 0.31% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 38 | 0.31% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 39 | 0.31% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 40 | 0.31% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 41 | 0.31% | `java.util.ArrayList$Itr.next() line: 1058` |
| 42 | 0.31% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 179` |
| 43 | 0.31% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1910` |
| 44 | 0.31% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10672` |
| 45 | 0.31% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 46 | 0.27% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 47 | 0.27% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 48 | 0.27% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 49 | 0.27% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 50 | 0.24% | `java.util.HashMap.getNode(Object) line: 588` |
| 51 | 0.24% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1475` |
| 52 | 0.24% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 53 | 0.24% | `sun.misc.Unsafe.beforeMemoryAccess() line: 1777` |
| 54 | 0.24% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7786` |
| 55 | 0.24% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 56 | 0.24% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10652` |
| 57 | 0.24% | `net.minecraft.world.entity.EntityDimensions.makeBoundingBox(double, double, double) line: 22` |
| 58 | 0.21% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 59 | 0.21% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 60 | 0.21% | `java.util.zip.ZipFile$Source.getEntryPos(String, boolean, ZipCoder) line: 1879` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.30% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12487` |
| 2 | 5.24% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10669` |
| 3 | 4.56% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 4 | 3.60% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10602` |
| 5 | 3.25% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 6 | 2.60% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 2.43% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 8 | 2.23% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 9 | 1.61% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5377` |
| 10 | 1.51% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 11 | 1.44% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6450` |
| 12 | 1.27% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 13 | 1.23% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7786` |
| 14 | 1.23% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 15 | 0.92% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 16 | 0.89% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 17 | 0.82% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 18 | 0.75% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 19 | 0.72% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 20 | 0.69% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 21 | 0.62% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 22 | 0.55% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 23 | 0.51% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 24 | 0.48% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 25 | 0.48% | `org.objectweb.asm.ClassReader.readMethod(ClassVisitor, Context, int) line: 1512` |
| 26 | 0.48% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12288` |
| 27 | 0.45% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.has(Entity) line: 473` |
| 28 | 0.41% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12290` |
| 29 | 0.41% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10630` |
| 30 | 0.38% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10622` |

GC pauses: 0, total 0 ms

