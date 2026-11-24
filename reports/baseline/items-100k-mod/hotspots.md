# JVM hotspots (JFR ExecutionSample)

Samples: 3990

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 8.85% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9497` |
| 2 | 7.82% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10798` |
| 3 | 5.06% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 2.83% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.58% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1030` |
| 6 | 2.28% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1029` |
| 7 | 1.95% | `dev.planeta9091.centities.CeHooks.ubsEmpty(Entity) line: 15432` |
| 8 | 1.80% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6766` |
| 9 | 1.48% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 10 | 1.35% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 11 | 1.33% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15206` |
| 12 | 1.00% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 13 | 0.98% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1039` |
| 14 | 0.95% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 15 | 0.85% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 16 | 0.73% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 161` |
| 17 | 0.68% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 18 | 0.65% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 19 | 0.55% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3231` |
| 20 | 0.53% | `java.util.ArrayList$Itr.next() line: 1058` |
| 21 | 0.53% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9678` |
| 22 | 0.50% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 4084` |
| 23 | 0.45% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 24 | 0.43% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 25 | 0.43% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3236` |
| 26 | 0.43% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6775` |
| 27 | 0.43% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 16505` |
| 28 | 0.40% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 237` |
| 29 | 0.38% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10811` |
| 30 | 0.38% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9215` |
| 31 | 0.38% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9243` |
| 32 | 0.38% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 16503` |
| 33 | 0.38% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1044` |
| 34 | 0.38% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1027` |
| 35 | 0.33% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 36 | 0.33% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 37 | 0.30% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 38 | 0.30% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 39 | 0.30% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6778` |
| 40 | 0.30% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8932` |
| 41 | 0.30% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean, int) line: 6388` |
| 42 | 0.28% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 43 | 0.28% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 44 | 0.28% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 45 | 0.28% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10714` |
| 46 | 0.28% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 47 | 0.28% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1048` |
| 48 | 0.25% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3232` |
| 49 | 0.25% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1430` |
| 50 | 0.23% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 51 | 0.23% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 52 | 0.23% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 53 | 0.23% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6765` |
| 54 | 0.23% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3241` |
| 55 | 0.23% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1046` |
| 56 | 0.23% | `dev.planeta9091.centities.CeHooks.stackFast(ItemEntity) line: 1223` |
| 57 | 0.20% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 58 | 0.20% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 59 | 0.20% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 60 | 0.20% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 12.31% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 9.22% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 7.29% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10798` |
| 4 | 5.06% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 2.78% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean, int) line: 6501` |
| 6 | 1.95% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 16581` |
| 7 | 1.88% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1043` |
| 8 | 1.85% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 9 | 1.45% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9219` |
| 10 | 1.40% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 11 | 1.38% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15198` |
| 12 | 1.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 13 | 1.13% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9215` |
| 14 | 1.05% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 15 | 0.95% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 16 | 0.95% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 17 | 0.93% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean) line: 6269` |
| 18 | 0.90% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 19 | 0.68% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 20 | 0.63% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1048` |
| 21 | 0.60% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3721` |
| 22 | 0.50% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 163` |
| 23 | 0.48% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 24 | 0.45% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 25 | 0.45% | `java.math.BigInteger.modPow(BigInteger, BigInteger) line: 2924` |
| 26 | 0.40% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 27 | 0.38% | `net.minecraft.world.level.gameevent.GameEventDispatcher.post(Holder, Vec3, GameEvent$Context) line: 55` |
| 28 | 0.35% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 29 | 0.35% | `dev.planeta9091.centities.CeHooks.fullStackQuiet(ItemEntity) line: 965` |
| 30 | 0.33% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |

GC pauses: 0, total 0 ms

