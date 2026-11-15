# JVM hotspots (JFR ExecutionSample)

Samples: 3238

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.52% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 2 | 5.78% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13644` |
| 3 | 3.95% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8724` |
| 4 | 2.90% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 5 | 2.90% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 2.66% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 12991` |
| 7 | 2.56% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 8 | 1.61% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 9 | 1.61% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 10 | 1.33% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 11 | 1.17% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.08% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13663` |
| 13 | 0.99% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 14 | 0.99% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 15 | 0.96% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 16 | 0.71% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 17 | 0.68% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 18 | 0.68% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 19 | 0.53% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 20 | 0.49% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 21 | 0.46% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 22 | 0.46% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 23 | 0.46% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 24 | 0.46% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11616` |
| 25 | 0.46% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13681` |
| 26 | 0.46% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 27 | 0.43% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 28 | 0.43% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 29 | 0.43% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 237` |
| 30 | 0.43% | `sun.misc.Unsafe.beforeMemoryAccess() line: 1777` |
| 31 | 0.40% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 32 | 0.37% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 33 | 0.37% | `net.minecraft.world.entity.EntityDimensions.makeBoundingBox(double, double, double) line: 22` |
| 34 | 0.34% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 35 | 0.34% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 36 | 0.34% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13922` |
| 37 | 0.31% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 38 | 0.31% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 39 | 0.31% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 40 | 0.31% | `java.util.zip.ZipFile$Source.getEntryPos(String, boolean, ZipCoder) line: 1879` |
| 41 | 0.28% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 42 | 0.28% | `java.util.Comparator.lambda$comparingLong$6043328a$1(ToLongFunction, Object, Object) line: 515` |
| 43 | 0.28% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 44 | 0.28% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1976` |
| 45 | 0.28% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 46 | 0.28% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 47 | 0.25% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 48 | 0.22% | `net.minecraft.world.level.levelgen.material.rule.ConditionRule.lambda$compile$0(ConditionEvaluator, RuleEvaluator, int, int, int) line: 27` |
| 49 | 0.22% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 50 | 0.22% | `net.minecraft.server.level.WorldGenRegion.getChunk(int, int) line: 175` |
| 51 | 0.22% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 52 | 0.22% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |
| 53 | 0.22% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13979` |
| 54 | 0.22% | `dev.planeta9091.centities.CeHooks.pushFlags(LivingEntity) line: 2173` |
| 55 | 0.22% | `java.util.ArrayList$Itr.next() line: 1058` |
| 56 | 0.22% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11636` |
| 57 | 0.22% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1956` |
| 58 | 0.19% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 59 | 0.19% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 874` |
| 60 | 0.19% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.idFor(Object, PaletteResize) line: 135` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.47% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13922` |
| 2 | 6.42% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11633` |
| 3 | 4.05% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 3.40% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11566` |
| 5 | 2.93% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 6 | 2.90% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 2.25% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 8 | 2.25% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5933` |
| 9 | 2.16% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 10 | 1.61% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 11 | 1.61% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 12 | 1.45% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7117` |
| 13 | 1.11% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 14 | 0.99% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 12999` |
| 15 | 0.96% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 16 | 0.71% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 17 | 0.71% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8750` |
| 18 | 0.68% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 19 | 0.65% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 20 | 0.62% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 21 | 0.59% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 22 | 0.53% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 23 | 0.53% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 24 | 0.46% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 25 | 0.46% | `net.minecraft.util.StaticCache2D.map(StaticCache2D$MappingFunction) line: 36` |
| 26 | 0.46% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.has(Entity) line: 473` |
| 27 | 0.46% | `net.minecraft.server.level.ServerLevel.isPositionEntityTicking(BlockPos) line: 2957` |
| 28 | 0.40% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 29 | 0.40% | `net.minecraft.world.entity.EntityDimensions.makeBoundingBox(Vec3) line: 16` |
| 30 | 0.34% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |

GC pauses: 0, total 0 ms

