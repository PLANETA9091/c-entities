# JVM hotspots (JFR ExecutionSample)

Samples: 6491

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 16.55% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7407` |
| 2 | 5.70% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8160` |
| 3 | 5.42% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 4 | 4.30% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 4.24% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 5413` |
| 6 | 3.73% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 3.42% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 8 | 2.79% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7347` |
| 9 | 2.63% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 10 | 2.34% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 12680` |
| 11 | 2.31% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12966` |
| 12 | 1.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7303` |
| 13 | 1.02% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 14 | 0.94% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 15 | 0.92% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 16 | 0.80% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 17 | 0.68% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 5422` |
| 18 | 0.57% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 19 | 0.57% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8170` |
| 20 | 0.51% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7279` |
| 21 | 0.48% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2447` |
| 22 | 0.35% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 23 | 0.31% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 24 | 0.29% | `java.util.ArrayList$Itr.next() line: 1058` |
| 25 | 0.25% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 26 | 0.25% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 27 | 0.23% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 28 | 0.23% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 29 | 0.23% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 341` |
| 30 | 0.22% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 31 | 0.22% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 32 | 0.22% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 33 | 0.22% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.remove(Object) line: 430` |
| 34 | 0.22% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 35 | 0.20% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 36 | 0.18% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 37 | 0.18% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 38 | 0.17% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 39 | 0.17% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 40 | 0.15% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 41 | 0.15% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 42 | 0.15% | `java.lang.Class.cast(Object) line: 3491` |
| 43 | 0.15% | `ru.planeta9091.cebaseline.BaselinePlugin.onTickEnd(ServerTickEndEvent) line: 565` |
| 44 | 0.14% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 45 | 0.14% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 5426` |
| 46 | 0.14% | `java.lang.Iterable.forEach(Consumer) line: 75` |
| 47 | 0.12% | `com.sun.crypto.provider.CounterMode.increment(byte[]) line: 169` |
| 48 | 0.12% | `org.objectweb.asm.ClassReader.readCode(MethodVisitor, Context, int) line: 2425` |
| 49 | 0.12% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 50 | 0.12% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7142` |
| 51 | 0.12% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 179` |
| 52 | 0.12% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 53 | 0.12% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6938` |
| 54 | 0.12% | `org.bukkit.craftbukkit.entity.CraftEntity.isValid() line: 510` |
| 55 | 0.11% | `java.lang.String.hashCode() line: 2433` |
| 56 | 0.11% | `java.lang.String.length() line: 1583` |
| 57 | 0.11% | `com.google.common.collect.RegularImmutableList.get(int) line: 80` |
| 58 | 0.11% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 180` |
| 59 | 0.11% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 60 | 0.11% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7210` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 22.86% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 6.52% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 5.42% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5676` |
| 4 | 5.21% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5167` |
| 5 | 4.39% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 6 | 4.30% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 3.17% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5118` |
| 8 | 2.63% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7289` |
| 9 | 2.42% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 10 | 2.36% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13057` |
| 11 | 1.56% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 12 | 1.00% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 13 | 0.80% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 14 | 0.59% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7285` |
| 15 | 0.57% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8170` |
| 16 | 0.52% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 17 | 0.37% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 18 | 0.32% | `org.objectweb.asm.ClassReader.readMethod(ClassVisitor, Context, int) line: 1512` |
| 19 | 0.32% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 168` |
| 20 | 0.31% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 21 | 0.29% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 22 | 0.29% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13019` |
| 23 | 0.28% | `java.util.ArrayList.grow(int) line: 238` |
| 24 | 0.28% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 163` |
| 25 | 0.26% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 26 | 0.26% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7361` |
| 27 | 0.26% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 28 | 0.25% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 29 | 0.25% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13070` |
| 30 | 0.25% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7347` |

GC pauses: 0, total 0 ms

