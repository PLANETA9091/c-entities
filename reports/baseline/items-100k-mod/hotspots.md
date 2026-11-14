# JVM hotspots (JFR ExecutionSample)

Samples: 3388

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 12.34% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7255` |
| 2 | 3.98% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8018` |
| 3 | 3.10% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8008` |
| 4 | 2.89% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7195` |
| 5 | 2.69% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12640` |
| 6 | 2.21% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 7 | 2.18% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 2.04% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 1.65% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 10 | 1.62% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 5320` |
| 11 | 1.53% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 12 | 1.42% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 5329` |
| 13 | 1.12% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5070` |
| 14 | 1.09% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 15 | 1.06% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 16 | 1.00% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 12490` |
| 17 | 0.86% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 18 | 0.71% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 19 | 0.59% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 20 | 0.53% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 21 | 0.53% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2447` |
| 22 | 0.50% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 23 | 0.41% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 24 | 0.41% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 25 | 0.41% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 26 | 0.38% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 27 | 0.38% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 28 | 0.35% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 29 | 0.35% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 30 | 0.35% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.remove(Object) line: 430` |
| 31 | 0.32% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 32 | 0.32% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 33 | 0.30% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 34 | 0.30% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 35 | 0.30% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 36 | 0.30% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7133` |
| 37 | 0.30% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 38 | 0.27% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 39 | 0.27% | `net.minecraft.world.level.ChunkPos.z() line: 20` |
| 40 | 0.27% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 5331` |
| 41 | 0.27% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4962` |
| 42 | 0.24% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 43 | 0.24% | `java.lang.String.hashCode() line: 2433` |
| 44 | 0.24% | `com.google.common.collect.RegularImmutableList.get(int) line: 80` |
| 45 | 0.24% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 46 | 0.24% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 5319` |
| 47 | 0.24% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5078` |
| 48 | 0.21% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 49 | 0.21% | `java.lang.Class.cast(Object) line: 3491` |
| 50 | 0.21% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 51 | 0.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6936` |
| 52 | 0.21% | `net.minecraft.server.level.ChunkMap$TrackedEntity.removePlayer(ServerPlayer) line: 1400` |
| 53 | 0.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6989` |
| 54 | 0.18% | `com.viaversion.viaversion.util.MinecraftKey.isValidPath(String) line: 92` |
| 55 | 0.18% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 56 | 0.18% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5188` |
| 57 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7151` |
| 58 | 0.18% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 5333` |
| 59 | 0.18% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5029` |
| 60 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6961` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 17.98% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 7.38% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 4.10% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5078` |
| 4 | 2.83% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 2.27% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 6 | 2.21% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5569` |
| 7 | 2.15% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8018` |
| 8 | 2.10% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 9 | 2.04% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 10 | 1.83% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5039` |
| 11 | 1.65% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 12 | 1.21% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12722` |
| 13 | 1.09% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7137` |
| 14 | 0.80% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5073` |
| 15 | 0.77% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7133` |
| 16 | 0.77% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 17 | 0.74% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 18 | 0.71% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 19 | 0.53% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 20 | 0.41% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 21 | 0.38% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 22 | 0.38% | `net.minecraft.server.level.ChunkMap$TrackedEntity.removePlayer(ServerPlayer) line: 1400` |
| 23 | 0.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 24 | 0.35% | `org.objectweb.asm.ClassReader.readMethod(ClassVisitor, Context, int) line: 1512` |
| 25 | 0.35% | `java.util.ArrayList.grow(int) line: 238` |
| 26 | 0.35% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 27 | 0.32% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 28 | 0.32% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.schedule(int, int, ChunkStatus, NewChunkHolder, List) line: 673` |
| 29 | 0.32% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7209` |
| 30 | 0.32% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |

GC pauses: 0, total 0 ms

