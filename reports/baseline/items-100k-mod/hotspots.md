# JVM hotspots (JFR ExecutionSample)

Samples: 6525

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 7.98% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 2 | 5.93% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7120` |
| 3 | 4.98% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10983` |
| 4 | 3.65% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 5 | 3.37% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 3.10% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6357` |
| 7 | 2.90% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 2.77% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 9 | 2.61% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4777` |
| 10 | 2.53% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 11 | 2.44% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7130` |
| 12 | 2.15% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 13 | 1.61% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 10833` |
| 14 | 1.55% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6284` |
| 15 | 1.32% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 16 | 1.07% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4645` |
| 17 | 0.97% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6428` |
| 18 | 0.87% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10845` |
| 19 | 0.84% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 20 | 0.80% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6461` |
| 21 | 0.64% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3414` |
| 22 | 0.61% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 23 | 0.55% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 24 | 0.54% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4725` |
| 25 | 0.52% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 294` |
| 26 | 0.35% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 27 | 0.35% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10849` |
| 28 | 0.34% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4638` |
| 29 | 0.32% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6414` |
| 30 | 0.32% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 31 | 0.32% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10884` |
| 32 | 0.28% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 33 | 0.26% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 34 | 0.25% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 35 | 0.23% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 36 | 0.23% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3388` |
| 37 | 0.23% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 38 | 0.23% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4776` |
| 39 | 0.21% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 40 | 0.21% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 41 | 0.21% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4549` |
| 42 | 0.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6427` |
| 43 | 0.20% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 44 | 0.20% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 45 | 0.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6307` |
| 46 | 0.18% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 47 | 0.18% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 30` |
| 48 | 0.18% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4657` |
| 49 | 0.18% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.remove(Object) line: 430` |
| 50 | 0.18% | `net.minecraft.world.phys.Vec3.horizontalDistanceSqr() line: 199` |
| 51 | 0.17% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 52 | 0.17% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupBlock() line: 824` |
| 53 | 0.17% | `java.util.ArrayList$Itr.next() line: 1058` |
| 54 | 0.17% | `dev.planeta9091.centities.CeHooks.vtabHash(int) line: 393` |
| 55 | 0.17% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3358` |
| 56 | 0.15% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 184` |
| 57 | 0.15% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4720` |
| 58 | 0.14% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 59 | 0.14% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 60 | 0.14% | `net.minecraft.util.StaticCache2D.get(int, int) line: 49` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 9.16% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 8.41% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 7.42% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6275` |
| 4 | 5.12% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 3.86% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 6 | 3.65% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4997` |
| 7 | 3.37% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 2.96% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4678` |
| 9 | 2.77% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 10 | 2.57% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4542` |
| 11 | 2.28% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7130` |
| 12 | 2.07% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2651` |
| 13 | 1.73% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 11065` |
| 14 | 1.72% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 11078` |
| 15 | 1.47% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 16 | 1.20% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 17 | 1.06% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4676` |
| 18 | 0.83% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3393` |
| 19 | 0.57% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6414` |
| 20 | 0.55% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 21 | 0.52% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 728` |
| 22 | 0.46% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4645` |
| 23 | 0.40% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 24 | 0.40% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 25 | 0.35% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6432` |
| 26 | 0.34% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 168` |
| 27 | 0.32% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 442` |
| 28 | 0.32% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4646` |
| 29 | 0.29% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 30 | 0.26% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |

GC pauses: 0, total 0 ms

