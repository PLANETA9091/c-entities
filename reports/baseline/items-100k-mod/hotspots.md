# JVM hotspots (JFR ExecutionSample)

Samples: 5656

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 11.53% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7913` |
| 2 | 8.35% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8724` |
| 3 | 5.16% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 4.14% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 5 | 3.82% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 3.29% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7733` |
| 7 | 3.08% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 8 | 2.14% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 13496` |
| 9 | 1.56% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5384` |
| 10 | 1.20% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 12988` |
| 11 | 1.13% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 12 | 1.01% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13918` |
| 13 | 0.87% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 14 | 0.80% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 15 | 0.76% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13644` |
| 16 | 0.72% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5388` |
| 17 | 0.67% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7717` |
| 18 | 0.65% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2513` |
| 19 | 0.58% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 20 | 0.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 161` |
| 21 | 0.51% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7583` |
| 22 | 0.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 23 | 0.42% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 24 | 0.39% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5390` |
| 25 | 0.37% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 604` |
| 26 | 0.35% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 27 | 0.32% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 28 | 0.30% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13632` |
| 29 | 0.27% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 30 | 0.27% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 31 | 0.25% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 32 | 0.25% | `java.util.ArrayList$Itr.next() line: 1058` |
| 33 | 0.25% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 34 | 0.25% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 35 | 0.23% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 36 | 0.23% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7511` |
| 37 | 0.23% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7693` |
| 38 | 0.21% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 39 | 0.21% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 40 | 0.21% | `java.util.zip.ZipFile$Source.getEntryPos(String, boolean, ZipCoder) line: 1879` |
| 41 | 0.21% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8734` |
| 42 | 0.21% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 5662` |
| 43 | 0.21% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13979` |
| 44 | 0.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8082` |
| 45 | 0.19% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 46 | 0.19% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.remove(Object) line: 430` |
| 47 | 0.16% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 48 | 0.16% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 49 | 0.16% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(int, int) line: 929` |
| 50 | 0.16% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 51 | 0.14% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 52 | 0.14% | `net.minecraft.world.level.biome.Climate$RTree.lambda$comparator$0(int, boolean, Climate$RTree$Node) line: 405` |
| 53 | 0.14% | `com.google.common.collect.RegularImmutableList.get(int) line: 80` |
| 54 | 0.14% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8723` |
| 55 | 0.14% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5932` |
| 56 | 0.14% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13635` |
| 57 | 0.12% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1475` |
| 58 | 0.12% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 59 | 0.12% | `org.objectweb.asm.ClassReader.readCode(MethodVisitor, Context, int) line: 2425` |
| 60 | 0.12% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1156` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 18.09% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 8.80% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 5.16% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 4.14% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5933` |
| 5 | 3.91% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8734` |
| 6 | 3.34% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5272` |
| 7 | 3.08% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7703` |
| 8 | 2.25% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13909` |
| 9 | 1.84% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 10 | 1.45% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 11 | 1.31% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13922` |
| 12 | 1.29% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 13 | 1.13% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 14 | 0.99% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 12980` |
| 15 | 0.87% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 16 | 0.85% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7699` |
| 17 | 0.57% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 18 | 0.41% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder.getChunkIfPresentUnchecked(ChunkStatus) line: 475` |
| 19 | 0.37% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 20 | 0.35% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 21 | 0.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 22 | 0.30% | `org.objectweb.asm.ClassReader.readMethod(ClassVisitor, Context, int) line: 1512` |
| 23 | 0.28% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 24 | 0.28% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 25 | 0.28% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7787` |
| 26 | 0.27% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 27 | 0.27% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 28 | 0.25% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 29 | 0.25% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5399` |
| 30 | 0.21% | `java.util.zip.ZipFile.getEntry(String) line: 294` |

GC pauses: 0, total 0 ms

