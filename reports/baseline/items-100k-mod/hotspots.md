# JVM hotspots (JFR ExecutionSample)

Samples: 5543

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 10.14% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7989` |
| 2 | 7.99% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4360` |
| 3 | 6.58% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 4 | 5.50% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8042` |
| 5 | 3.81% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 6 | 3.55% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5080` |
| 7 | 3.19% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 2.87% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3055` |
| 9 | 2.83% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 10 | 2.06% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 11 | 1.17% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 12 | 1.01% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 13 | 1.01% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 14 | 0.99% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2947` |
| 15 | 0.85% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5078` |
| 16 | 0.79% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4397` |
| 17 | 0.67% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 18 | 0.63% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity) line: 906` |
| 19 | 0.61% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 20 | 0.60% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2940` |
| 21 | 0.60% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3008` |
| 22 | 0.58% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7685` |
| 23 | 0.58% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4366` |
| 24 | 0.52% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2959` |
| 25 | 0.52% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2195` |
| 26 | 0.51% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7683` |
| 27 | 0.43% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7993` |
| 28 | 0.36% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 29 | 0.34% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 272` |
| 30 | 0.31% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2176` |
| 31 | 0.31% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 32 | 0.29% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 33 | 0.29% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2852` |
| 34 | 0.29% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 35 | 0.27% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1868` |
| 36 | 0.27% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 37 | 0.25% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupBlock() line: 824` |
| 38 | 0.23% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 39 | 0.23% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8003` |
| 40 | 0.22% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 41 | 0.22% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 42 | 0.22% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4220` |
| 43 | 0.20% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 44 | 0.20% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 45 | 0.20% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 46 | 0.20% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 47 | 0.20% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 48 | 0.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4429` |
| 49 | 0.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4221` |
| 50 | 0.18% | `net.minecraft.server.level.WorldGenRegion.lambda$new$2(ChunkAccess, ChunkStep, GenerationChunkHolder, int, int) line: 124` |
| 51 | 0.18% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2179` |
| 52 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4405` |
| 53 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4371` |
| 54 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4310` |
| 55 | 0.16% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 56 | 0.16% | `net.minecraft.world.level.Level.isClientSide() line: 924` |
| 57 | 0.16% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 58 | 0.16% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 433` |
| 59 | 0.16% | `net.minecraft.world.phys.Vec3.horizontalDistanceSqr() line: 199` |
| 60 | 0.14% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 12.36% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 11.26% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8038` |
| 3 | 6.66% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4397` |
| 4 | 5.85% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 4.55% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 6 | 3.82% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 7 | 3.36% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5080` |
| 8 | 3.25% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2968` |
| 9 | 2.98% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2850` |
| 10 | 2.06% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 11 | 1.80% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 12 | 1.77% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8030` |
| 13 | 1.68% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 14 | 1.23% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1709` |
| 15 | 1.17% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 16 | 1.10% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 17 | 0.94% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2967` |
| 18 | 0.90% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7996` |
| 19 | 0.49% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2947` |
| 20 | 0.47% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7988` |
| 21 | 0.45% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 548` |
| 22 | 0.43% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 23 | 0.38% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2180` |
| 24 | 0.36% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 25 | 0.36% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 26 | 0.34% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 526` |
| 27 | 0.32% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 28 | 0.31% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2948` |
| 29 | 0.29% | `net.minecraft.util.StaticCache2D.lambda$map$0(StaticCache2D$MappingFunction, int, int) line: 36` |
| 30 | 0.29% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8000` |

GC pauses: 0, total 0 ms

