# JVM hotspots (JFR ExecutionSample)

Samples: 5297

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 17.71% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7533` |
| 2 | 6.17% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8285` |
| 3 | 4.59% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 4.21% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 5 | 3.76% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 6 | 3.59% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 3.51% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5216` |
| 8 | 3.44% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 9 | 2.19% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 12805` |
| 10 | 1.89% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13091` |
| 11 | 1.72% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5220` |
| 12 | 1.09% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 13 | 1.00% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7378` |
| 14 | 1.00% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5222` |
| 15 | 1.00% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 16 | 0.98% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7382` |
| 17 | 0.72% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 18 | 0.64% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 19 | 0.55% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 20 | 0.45% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7360` |
| 21 | 0.45% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2455` |
| 22 | 0.43% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 23 | 0.36% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2459` |
| 24 | 0.34% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 25 | 0.28% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 26 | 0.28% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7354` |
| 27 | 0.26% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 28 | 0.26% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 29 | 0.25% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 30 | 0.25% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 31 | 0.25% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 32 | 0.25% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7217` |
| 33 | 0.25% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2464` |
| 34 | 0.23% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 35 | 0.23% | `java.lang.Double.isFinite(double) line: 1023` |
| 36 | 0.21% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 37 | 0.21% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1475` |
| 38 | 0.21% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 39 | 0.19% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 40 | 0.19% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 41 | 0.19% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8295` |
| 42 | 0.19% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.remove(Object) line: 430` |
| 43 | 0.19% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2458` |
| 44 | 0.17% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 45 | 0.17% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 46 | 0.15% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 47 | 0.15% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 48 | 0.15% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 49 | 0.15% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5183` |
| 50 | 0.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6879` |
| 51 | 0.15% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1430` |
| 52 | 0.15% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5224` |
| 53 | 0.15% | `ru.planeta9091.cebaseline.BaselinePlugin.onTickEnd(ServerTickEndEvent) line: 565` |
| 54 | 0.13% | `java.util.HashMap.getNode(Object) line: 586` |
| 55 | 0.13% | `net.minecraft.nbt.ListTag.getIdToWrite(List, int) line: 37` |
| 56 | 0.13% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 57 | 0.13% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7216` |
| 58 | 0.13% | `java.util.ArrayList$Itr.next() line: 1058` |
| 59 | 0.11% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 874` |
| 60 | 0.11% | `java.util.BitSet.expandTo(int) line: 357` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 22.37% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 7.36% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8295` |
| 3 | 6.53% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 5.27% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 5 | 4.59% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 3.76% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5757` |
| 7 | 3.44% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7364` |
| 8 | 2.81% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5145` |
| 9 | 2.21% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13182` |
| 10 | 1.91% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 11 | 1.55% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7360` |
| 12 | 1.26% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 13 | 1.13% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 14 | 0.66% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 15 | 0.64% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 16 | 0.42% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 17 | 0.36% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 18 | 0.34% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 19 | 0.34% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7382` |
| 20 | 0.30% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 21 | 0.28% | `net.minecraft.util.StaticCache2D.map(StaticCache2D$MappingFunction) line: 36` |
| 22 | 0.28% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 23 | 0.28% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 24 | 0.26% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 25 | 0.26% | `java.math.BigInteger.modPow(BigInteger, BigInteger) line: 2924` |
| 26 | 0.25% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 27 | 0.25% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.schedule(int, int, ChunkStatus, NewChunkHolder, List) line: 673` |
| 28 | 0.25% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13201` |
| 29 | 0.21% | `org.objectweb.asm.ClassReader.readMethod(ClassVisitor, Context, int) line: 1512` |
| 30 | 0.21% | `net.minecraft.server.level.ChunkMap$TrackedEntity.removePlayer(ServerPlayer) line: 1400` |

GC pauses: 0, total 0 ms

