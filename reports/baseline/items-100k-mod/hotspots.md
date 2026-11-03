# JVM hotspots (JFR ExecutionSample)

Samples: 3264

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.24% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 2 | 3.77% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 3 | 3.43% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4072` |
| 4 | 3.31% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4152` |
| 5 | 3.28% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6483` |
| 6 | 3.16% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4204` |
| 7 | 2.82% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9777` |
| 8 | 2.18% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2835` |
| 9 | 1.84% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 10 | 1.81% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 11 | 1.81% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 12 | 1.56% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2873` |
| 13 | 1.56% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5684` |
| 14 | 1.47% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5710` |
| 15 | 1.44% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5781` |
| 16 | 1.41% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 17 | 1.35% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 18 | 1.26% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9730` |
| 19 | 1.23% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 20 | 1.10% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 21 | 1.07% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5675` |
| 22 | 1.04% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9733` |
| 23 | 0.77% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9770` |
| 24 | 0.67% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 25 | 0.61% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5814` |
| 26 | 0.58% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 27 | 0.55% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 28 | 0.55% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 29 | 0.52% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6473` |
| 30 | 0.46% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 31 | 0.43% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 32 | 0.40% | `java.lang.Double.isFinite(double) line: 1023` |
| 33 | 0.37% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5767` |
| 34 | 0.37% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9953` |
| 35 | 0.34% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 36 | 0.34% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 37 | 0.34% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 38 | 0.34% | `net.minecraft.world.phys.Vec3.horizontalDistanceSqr() line: 199` |
| 39 | 0.31% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 40 | 0.31% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5674` |
| 41 | 0.31% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 42 | 0.31% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4065` |
| 43 | 0.28% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 44 | 0.28% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4203` |
| 45 | 0.28% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3976` |
| 46 | 0.25% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 47 | 0.25% | `net.minecraft.server.level.WorldGenRegion.lambda$new$2(ChunkAccess, ChunkStep, GenerationChunkHolder, int, int) line: 124` |
| 48 | 0.25% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5185` |
| 49 | 0.25% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5623` |
| 50 | 0.21% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 51 | 0.21% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 52 | 0.21% | `com.google.common.collect.RegularImmutableList.get(int) line: 80` |
| 53 | 0.21% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4084` |
| 54 | 0.21% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 392` |
| 55 | 0.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5709` |
| 56 | 0.21% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9738` |
| 57 | 0.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5624` |
| 58 | 0.21% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 59 | 0.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5763` |
| 60 | 0.21% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 10.75% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 6.37% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9907` |
| 3 | 4.63% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2217` |
| 4 | 4.60% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6483` |
| 5 | 4.20% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 6 | 4.07% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5675` |
| 7 | 4.01% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4103` |
| 8 | 3.80% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 9 | 3.68% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4105` |
| 10 | 2.08% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 11 | 1.84% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 12 | 1.84% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5767` |
| 13 | 1.78% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3969` |
| 14 | 1.41% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4073` |
| 15 | 1.38% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 16 | 1.23% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4424` |
| 17 | 0.98% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4072` |
| 18 | 0.77% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 19 | 0.77% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 20 | 0.74% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 21 | 0.58% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 22 | 0.58% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 23 | 0.55% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |
| 24 | 0.49% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 25 | 0.40% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 26 | 0.40% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5765` |
| 27 | 0.37% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 28 | 0.34% | `dev.planeta9091.centities.CeHooks.itemImmunities(ItemEntity) line: 4180` |
| 29 | 0.31% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 30 | 0.31% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 1025` |

GC pauses: 0, total 0 ms

