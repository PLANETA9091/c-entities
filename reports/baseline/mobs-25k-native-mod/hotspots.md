# JVM hotspots (JFR ExecutionSample)

Samples: 3592

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.01% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13618` |
| 2 | 4.37% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 3 | 4.12% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 4 | 3.12% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 12965` |
| 5 | 3.12% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8698` |
| 6 | 2.17% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 2.14% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 8 | 1.50% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 9 | 1.36% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 10 | 1.34% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 11 | 0.67% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 0.58% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13637` |
| 13 | 0.58% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 14 | 0.56% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 15 | 0.53% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 16 | 0.47% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 17 | 0.47% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 18 | 0.47% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 19 | 0.45% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 20 | 0.42% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 21 | 0.42% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 22 | 0.42% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 23 | 0.42% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 24 | 0.39% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 25 | 0.39% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 26 | 0.36% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 27 | 0.36% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 28 | 0.36% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 29 | 0.36% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 30 | 0.33% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 31 | 0.33% | `java.util.HashMap.getNode(Object) line: 586` |
| 32 | 0.33% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 33 | 0.33% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 34 | 0.33% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 35 | 0.31% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 36 | 0.31% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 37 | 0.31% | `net.minecraft.world.entity.EntityDimensions.makeBoundingBox(double, double, double) line: 22` |
| 38 | 0.31% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11610` |
| 39 | 0.31% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 40 | 0.28% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 41 | 0.28% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 42 | 0.25% | `net.minecraft.world.level.biome.Climate$RTree.buildParameterSpace(List) line: 455` |
| 43 | 0.25% | `jdk.internal.foreign.HeapMemorySegmentImpl$OfByte.unsafeGetBase() line: 114` |
| 44 | 0.25% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 45 | 0.22% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 46 | 0.22% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13896` |
| 47 | 0.22% | `java.util.ArrayList$Itr.next() line: 1058` |
| 48 | 0.19% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 49 | 0.19% | `java.util.zip.ZipFile$Source.getEntryPos(String, boolean, ZipCoder) line: 1879` |
| 50 | 0.19% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 51 | 0.19% | `com.google.common.collect.RegularImmutableList.get(int) line: 80` |
| 52 | 0.19% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 53 | 0.19% | `ca.spottedleaf.moonrise.patches.starlight.light.SWMRNibbleArray.getUpdating(int) line: 389` |
| 54 | 0.19% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 55 | 0.19% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11569` |
| 56 | 0.19% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8724` |
| 57 | 0.19% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1976` |
| 58 | 0.19% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap$MapIterator.nextEntry() line: 655` |
| 59 | 0.19% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5173` |
| 60 | 0.19% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 10294` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.88% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13896` |
| 2 | 4.51% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11540` |
| 3 | 4.23% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11607` |
| 4 | 3.42% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 3.29% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 6 | 2.90% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 7 | 2.17% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 1.89% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5923` |
| 9 | 1.48% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 10 | 1.36% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 11 | 1.25% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 12 | 1.14% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7103` |
| 13 | 1.11% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 14 | 0.81% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 15 | 0.72% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 16 | 0.64% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 17 | 0.61% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8724` |
| 18 | 0.53% | `net.minecraft.util.StaticCache2D.map(StaticCache2D$MappingFunction) line: 36` |
| 19 | 0.50% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 20 | 0.47% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 21 | 0.47% | `net.minecraft.server.level.ServerLevel.isPositionEntityTicking(BlockPos) line: 2957` |
| 22 | 0.47% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 23 | 0.45% | `it.unimi.dsi.fastutil.objects.ReferenceCollections$UnmodifiableCollection.contains(Object) line: 344` |
| 24 | 0.45% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 12973` |
| 25 | 0.36% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 26 | 0.36% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 27 | 0.33% | `com.sun.crypto.provider.CounterMode.crypt(byte[], int, int, byte[], int) line: 195` |
| 28 | 0.33% | `org.objectweb.asm.ClassReader.readMethod(ClassVisitor, Context, int) line: 1512` |
| 29 | 0.33% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.schedule(int, int, ChunkStatus, NewChunkHolder, List) line: 673` |
| 30 | 0.33% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |

GC pauses: 0, total 0 ms

