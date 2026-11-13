# JVM hotspots (JFR ExecutionSample)

Samples: 2957

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.78% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 11919` |
| 2 | 5.11% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 3 | 3.58% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 4 | 3.04% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7416` |
| 5 | 3.01% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 6 | 2.00% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 7 | 1.89% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 1.62% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 9 | 1.59% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.59% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 11 | 1.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 12 | 1.12% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 13 | 1.08% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 14 | 0.95% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 15 | 0.74% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 16 | 0.71% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 17 | 0.68% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 11938` |
| 18 | 0.64% | `java.util.HashMap.getNode(Object) line: 586` |
| 19 | 0.61% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 20 | 0.57% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 21 | 0.54% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 22 | 0.54% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 23 | 0.54% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 24 | 0.41% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 25 | 0.41% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 26 | 0.41% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 11956` |
| 27 | 0.37% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 28 | 0.37% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 29 | 0.37% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 30 | 0.37% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 31 | 0.37% | `java.util.ArrayList$Itr.next() line: 1058` |
| 32 | 0.34% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 33 | 0.34% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 34 | 0.34% | `sun.misc.Unsafe.beforeMemoryAccess() line: 1777` |
| 35 | 0.34% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1910` |
| 36 | 0.34% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 37 | 0.34% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12143` |
| 38 | 0.30% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 39 | 0.30% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12048` |
| 40 | 0.30% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 41 | 0.30% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 42 | 0.27% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 43 | 0.27% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 44 | 0.27% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 45 | 0.27% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 46 | 0.27% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 47 | 0.27% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 48 | 0.27% | `net.minecraft.core.SectionPos.blockToSectionCoord(int) line: 90` |
| 49 | 0.27% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 50 | 0.27% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5173` |
| 51 | 0.24% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1475` |
| 52 | 0.24% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 53 | 0.20% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 54 | 0.20% | `java.util.zip.ZipFile$Source.getEntryPos(String, boolean, ZipCoder) line: 1879` |
| 55 | 0.20% | `jdk.internal.classfile.impl.EntryMap.firstToken(int) line: 92` |
| 56 | 0.20% | `net.minecraft.world.level.chunk.ProtoChunk.moonrise$getBlock(int, int, int) line: 67` |
| 57 | 0.20% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 58 | 0.20% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7442` |
| 59 | 0.20% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 92` |
| 60 | 0.20% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5200` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.20% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12143` |
| 2 | 4.97% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10325` |
| 3 | 4.13% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10258` |
| 4 | 3.38% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 5 | 3.31% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 6 | 2.67% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 7 | 2.40% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 8 | 1.89% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 1.62% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 10 | 1.59% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 11 | 1.52% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5152` |
| 12 | 1.15% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7442` |
| 13 | 0.95% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 14 | 0.91% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 15 | 0.88% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6223` |
| 16 | 0.74% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 17 | 0.74% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 18 | 0.71% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 19 | 0.68% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 20 | 0.61% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 21 | 0.61% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 22 | 0.57% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 23 | 0.57% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 24 | 0.54% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 25 | 0.51% | `java.util.HashMap.containsKey(Object) line: 603` |
| 26 | 0.47% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10278` |
| 27 | 0.41% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |
| 28 | 0.37% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 29 | 0.37% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 30 | 0.37% | `net.minecraft.server.level.ServerLevel.isPositionEntityTicking(BlockPos) line: 2956` |

GC pauses: 0, total 0 ms

