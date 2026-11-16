# JVM hotspots (JFR ExecutionSample)

Samples: 2601

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.61% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 2 | 4.84% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13865` |
| 3 | 2.73% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 4 | 2.58% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 1.88% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 6 | 1.61% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 7 | 1.50% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13212` |
| 8 | 1.46% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 9 | 1.46% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 10 | 1.42% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.31% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 12 | 1.27% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8945` |
| 13 | 1.15% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 14 | 0.92% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13884` |
| 15 | 0.77% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 16 | 0.73% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 17 | 0.65% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 18 | 0.58% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 19 | 0.54% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 20 | 0.50% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 21 | 0.50% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 22 | 0.46% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 23 | 0.46% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 24 | 0.46% | `java.util.HashMap.getNode(Object) line: 586` |
| 25 | 0.46% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 26 | 0.46% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11837` |
| 27 | 0.46% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 995` |
| 28 | 0.42% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 29 | 0.38% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 30 | 0.38% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 31 | 0.38% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 32 | 0.38% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 33 | 0.38% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 237` |
| 34 | 0.35% | `net.minecraft.server.level.WorldGenRegion.lambda$new$2(ChunkAccess, ChunkStep, GenerationChunkHolder, int, int) line: 124` |
| 35 | 0.35% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |
| 36 | 0.31% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 37 | 0.31% | `java.util.zip.ZipFile$Source.getEntryPos(String, boolean, ZipCoder) line: 1879` |
| 38 | 0.31% | `jdk.internal.classfile.impl.EntryMap.firstToken(int) line: 92` |
| 39 | 0.31% | `net.minecraft.world.level.chunk.ProtoChunk.moonrise$getBlock(int, int, int) line: 67` |
| 40 | 0.31% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 41 | 0.31% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 179` |
| 42 | 0.31% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2103` |
| 43 | 0.31% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 44 | 0.27% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 45 | 0.27% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 874` |
| 46 | 0.27% | `java.math.MutableBigInteger.mulsub(int[], int[], int, int, int) line: 675` |
| 47 | 0.27% | `net.minecraft.world.level.levelgen.Heightmap.update(int, int, int, BlockState) line: 101` |
| 48 | 0.27% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 49 | 0.27% | `sun.misc.Unsafe.getFloat(Object, long) line: 383` |
| 50 | 0.27% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3435` |
| 51 | 0.23% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 52 | 0.23% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 53 | 0.23% | `it.unimi.dsi.fastutil.ints.Int2IntOpenHashMap.find(int) line: 238` |
| 54 | 0.23% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 184` |
| 55 | 0.23% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 56 | 0.23% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 57 | 0.19% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 58 | 0.19% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 738` |
| 59 | 0.19% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1475` |
| 60 | 0.19% | `net.minecraft.world.phys.shapes.VoxelShape.toAabbs() line: 713` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.11% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14143` |
| 2 | 5.19% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11854` |
| 3 | 3.23% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11787` |
| 4 | 2.73% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 2.61% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 6 | 2.58% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 2.04% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 8 | 1.88% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 9 | 1.58% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 10 | 1.50% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7299` |
| 11 | 1.46% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 12 | 1.23% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8971` |
| 13 | 0.96% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 14 | 0.81% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 15 | 0.81% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 16 | 0.77% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 17 | 0.77% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 18 | 0.73% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 19 | 0.69% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 6115` |
| 20 | 0.69% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11807` |
| 21 | 0.65% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 22 | 0.65% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13220` |
| 23 | 0.58% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 168` |
| 24 | 0.58% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 25 | 0.54% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 26 | 0.54% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 27 | 0.50% | `org.objectweb.asm.ClassReader.readMethod(ClassVisitor, Context, int) line: 1512` |
| 28 | 0.50% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 29 | 0.46% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 30 | 0.46% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13892` |

GC pauses: 0, total 0 ms

