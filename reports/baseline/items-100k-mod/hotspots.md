# JVM hotspots (JFR ExecutionSample)

Samples: 4622

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 16.05% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7819` |
| 2 | 4.91% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 4.54% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5291` |
| 4 | 4.50% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8581` |
| 5 | 4.00% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 3.72% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 12838` |
| 7 | 3.01% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 8 | 2.68% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13598` |
| 9 | 1.97% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 10 | 1.95% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 13312` |
| 11 | 1.51% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5295` |
| 12 | 1.34% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5297` |
| 13 | 1.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7613` |
| 14 | 1.13% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 15 | 0.97% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 16 | 0.93% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7865` |
| 17 | 0.82% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 18 | 0.78% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 19 | 0.78% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7479` |
| 20 | 0.54% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 21 | 0.48% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 22 | 0.45% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2490` |
| 23 | 0.30% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 24 | 0.30% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 25 | 0.30% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7589` |
| 26 | 0.28% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 27 | 0.28% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7683` |
| 28 | 0.26% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 29 | 0.26% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 30 | 0.24% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 31 | 0.24% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 32 | 0.24% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 33 | 0.24% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7407` |
| 34 | 0.24% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8591` |
| 35 | 0.24% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 179` |
| 36 | 0.22% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 37 | 0.22% | `java.util.ArrayList$Itr.next() line: 1058` |
| 38 | 0.22% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.remove(Object) line: 430` |
| 39 | 0.19% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 40 | 0.19% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 41 | 0.17% | `java.util.HashMap.getNode(Object) line: 586` |
| 42 | 0.17% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 43 | 0.17% | `java.lang.Class.cast(Object) line: 3491` |
| 44 | 0.17% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 45 | 0.17% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 46 | 0.17% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 5569` |
| 47 | 0.15% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 48 | 0.15% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 49 | 0.15% | `java.util.zip.ZipFile$Source.getEntryPos(String, boolean, ZipCoder) line: 1879` |
| 50 | 0.15% | `it.unimi.dsi.fastutil.objects.ReferenceLinkedOpenHashSet.add(Object) line: 416` |
| 51 | 0.15% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 52 | 0.15% | `net.minecraft.server.level.WorldGenRegion.lambda$new$2(ChunkAccess, ChunkStep, GenerationChunkHolder, int, int) line: 124` |
| 53 | 0.15% | `ca.spottedleaf.moonrise.patches.starlight.light.SWMRNibbleArray.getUpdating(int) line: 393` |
| 54 | 0.15% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13451` |
| 55 | 0.15% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 184` |
| 56 | 0.13% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 57 | 0.13% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 874` |
| 58 | 0.13% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 59 | 0.13% | `com.google.common.collect.RegularImmutableList.get(int) line: 80` |
| 60 | 0.13% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 21.51% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 8.59% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8591` |
| 3 | 4.91% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 4.91% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 3.79% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 6 | 3.42% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5187` |
| 7 | 3.01% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5840` |
| 8 | 2.88% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 9 | 1.99% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13689` |
| 10 | 1.97% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7599` |
| 11 | 1.51% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 12 | 1.17% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 13 | 0.80% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 14 | 0.78% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 15 | 0.65% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7595` |
| 16 | 0.54% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7617` |
| 17 | 0.52% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 168` |
| 18 | 0.43% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 19 | 0.39% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 20 | 0.39% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 21 | 0.32% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7683` |
| 22 | 0.30% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 23 | 0.28% | `java.math.BigInteger.modPow(BigInteger, BigInteger) line: 2924` |
| 24 | 0.28% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 25 | 0.28% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 1025` |
| 26 | 0.26% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 27 | 0.26% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13702` |
| 28 | 0.24% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 29 | 0.24% | `net.minecraft.server.level.ChunkMap$TrackedEntity.removePlayer(ServerPlayer) line: 1400` |
| 30 | 0.24% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13708` |

GC pauses: 0, total 0 ms

