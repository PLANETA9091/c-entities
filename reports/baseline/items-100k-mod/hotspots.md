# JVM hotspots (JFR ExecutionSample)

Samples: 5660

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 11.78% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9568` |
| 2 | 4.89% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 4.75% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8725` |
| 4 | 4.40% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 984` |
| 5 | 4.20% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 1.63% | `dev.planeta9091.centities.CeHooks.stackFast(ItemEntity) line: 1033` |
| 7 | 1.59% | `dev.planeta9091.centities.CeHooks.ubsEmpty(Entity) line: 14039` |
| 8 | 1.50% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 9 | 1.50% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 985` |
| 10 | 1.36% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6042` |
| 11 | 1.29% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8511` |
| 12 | 1.10% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13846` |
| 13 | 0.88% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14614` |
| 14 | 0.83% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 15 | 0.81% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 16 | 0.80% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 17 | 0.74% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 18 | 0.69% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6038` |
| 19 | 0.62% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 20 | 0.57% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5996` |
| 21 | 0.55% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8507` |
| 22 | 0.46% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14908` |
| 23 | 0.46% | `dev.planeta9091.centities.CeHooks.dispatchQuietSkip(ItemEntity, int) line: 968` |
| 24 | 0.44% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3007` |
| 25 | 0.44% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 26 | 0.44% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 982` |
| 27 | 0.42% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 28 | 0.39% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 29 | 0.37% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3002` |
| 30 | 0.35% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 31 | 0.34% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6316` |
| 32 | 0.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 161` |
| 33 | 0.30% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 34 | 0.30% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8489` |
| 35 | 0.30% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5932` |
| 36 | 0.28% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 37 | 0.28% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 38 | 0.28% | `java.util.ArrayList$Itr.next() line: 1058` |
| 39 | 0.28% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9578` |
| 40 | 0.28% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3006` |
| 41 | 0.28% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3012` |
| 42 | 0.28% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 994` |
| 43 | 0.27% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 44 | 0.25% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 45 | 0.25% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 46 | 0.23% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 47 | 0.23% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 48 | 0.21% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 49 | 0.21% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 50 | 0.21% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 51 | 0.21% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 874` |
| 52 | 0.21% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3003` |
| 53 | 0.21% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 990` |
| 54 | 0.19% | `com.mojang.serialization.Encoder$1.encode(Object, DynamicOps, Object) line: 25` |
| 55 | 0.19% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 56 | 0.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8242` |
| 57 | 0.19% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5848` |
| 58 | 0.19% | `dev.planeta9091.centities.CeHooks.stackFast(ItemEntity) line: 1028` |
| 59 | 0.19% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 981` |
| 60 | 0.18% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 12.35% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 2 | 8.96% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 3 | 7.56% | `dev.planeta9091.centities.CeHooks.dispatchQuietSkip(ItemEntity, int) line: 968` |
| 4 | 4.20% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 3.90% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9578` |
| 6 | 2.00% | `dev.planeta9091.centities.CeHooks.fullStackQuiet(ItemEntity) line: 931` |
| 7 | 1.86% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 8 | 1.73% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5944` |
| 9 | 1.73% | `dev.planeta9091.centities.CeHooks.stackFast(ItemEntity) line: 1028` |
| 10 | 1.59% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14962` |
| 11 | 1.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 12 | 1.17% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 13 | 1.06% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8489` |
| 14 | 1.01% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14924` |
| 15 | 0.95% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 16 | 0.83% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 17 | 0.81% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8493` |
| 18 | 0.76% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 994` |
| 19 | 0.76% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13838` |
| 20 | 0.67% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5973` |
| 21 | 0.65% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1000` |
| 22 | 0.60% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 23 | 0.42% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 24 | 0.42% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 25 | 0.41% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 26 | 0.37% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 27 | 0.37% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 28 | 0.35% | `net.minecraft.util.StaticCache2D.map(StaticCache2D$MappingFunction) line: 36` |
| 29 | 0.35% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6053` |
| 30 | 0.28% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |

GC pauses: 0, total 0 ms

