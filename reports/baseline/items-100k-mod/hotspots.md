# JVM hotspots (JFR ExecutionSample)

Samples: 5182

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 15.55% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7936` |
| 2 | 4.90% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 4.86% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5374` |
| 4 | 3.74% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 3.45% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8698` |
| 6 | 2.57% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 7 | 1.99% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 8 | 1.85% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 13470` |
| 9 | 1.64% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5378` |
| 10 | 1.20% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 12962` |
| 11 | 1.14% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 12 | 1.06% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 13 | 1.02% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5380` |
| 14 | 0.96% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 15 | 0.93% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7696` |
| 16 | 0.85% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13892` |
| 17 | 0.83% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7562` |
| 18 | 0.71% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 19 | 0.71% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7982` |
| 20 | 0.68% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13618` |
| 21 | 0.60% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 22 | 0.46% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 161` |
| 23 | 0.44% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13606` |
| 24 | 0.39% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2513` |
| 25 | 0.35% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7490` |
| 26 | 0.35% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13953` |
| 27 | 0.33% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 28 | 0.33% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 29 | 0.33% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 179` |
| 30 | 0.33% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 5652` |
| 31 | 0.31% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 32 | 0.31% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 604` |
| 33 | 0.29% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 34 | 0.29% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5382` |
| 35 | 0.27% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2503` |
| 36 | 0.27% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5341` |
| 37 | 0.27% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7766` |
| 38 | 0.25% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 39 | 0.25% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 40 | 0.23% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 41 | 0.23% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 42 | 0.21% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 43 | 0.21% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 44 | 0.21% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 45 | 0.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7152` |
| 46 | 0.19% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 47 | 0.19% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 48 | 0.19% | `java.util.ArrayList.grow(int) line: 238` |
| 49 | 0.19% | `org.objectweb.asm.ClassReader.readCode(MethodVisitor, Context, int) line: 2425` |
| 50 | 0.19% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 51 | 0.19% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 52 | 0.19% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 184` |
| 53 | 0.19% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13609` |
| 54 | 0.17% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 55 | 0.17% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 56 | 0.17% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8708` |
| 57 | 0.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7672` |
| 58 | 0.15% | `com.mojang.serialization.Encoder$1.encode(Object, DynamicOps, Object) line: 25` |
| 59 | 0.15% | `java.util.ArrayList$Itr.next() line: 1058` |
| 60 | 0.15% | `java.lang.Class.cast(Object) line: 3491` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 20.92% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 9.11% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8708` |
| 3 | 4.90% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 3.74% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 3.30% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5262` |
| 6 | 2.57% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5923` |
| 7 | 2.10% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 8 | 1.99% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7682` |
| 9 | 1.93% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13883` |
| 10 | 1.68% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 11 | 1.43% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13896` |
| 12 | 1.27% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 13 | 1.12% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 14 | 1.06% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 12954` |
| 15 | 0.96% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 16 | 0.85% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7678` |
| 17 | 0.68% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 18 | 0.58% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 168` |
| 19 | 0.42% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 20 | 0.41% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 21 | 0.41% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7766` |
| 22 | 0.39% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 23 | 0.37% | `org.objectweb.asm.ClassReader.readMethod(ClassVisitor, Context, int) line: 1512` |
| 24 | 0.35% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 25 | 0.33% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder.getChunkIfPresentUnchecked(ChunkStatus) line: 475` |
| 26 | 0.33% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5389` |
| 27 | 0.29% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 1026` |
| 28 | 0.27% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 29 | 0.25% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 30 | 0.23% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |

GC pauses: 0, total 0 ms

