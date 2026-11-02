# JVM hotspots (JFR ExecutionSample)

Samples: 4681

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 9.38% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4077` |
| 2 | 8.35% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5546` |
| 3 | 7.35% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 6.67% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9618` |
| 5 | 3.16% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 6 | 2.46% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.35% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3950` |
| 8 | 2.03% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9574` |
| 9 | 2.01% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9571` |
| 10 | 1.94% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9611` |
| 11 | 1.64% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5622` |
| 12 | 1.54% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 13 | 1.32% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5551` |
| 14 | 1.28% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 15 | 1.24% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4030` |
| 16 | 1.09% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 17 | 1.05% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2777` |
| 18 | 0.94% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 19 | 0.94% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 20 | 0.90% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 21 | 0.73% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2739` |
| 22 | 0.68% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6324` |
| 23 | 0.66% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6314` |
| 24 | 0.64% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9579` |
| 25 | 0.43% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9794` |
| 26 | 0.34% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3943` |
| 27 | 0.32% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 28 | 0.32% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 277` |
| 29 | 0.30% | `ca.spottedleaf.concurrentutil.collection.MultiThreadedQueue$LinkedNode.<init>(Object, MultiThreadedQueue$LinkedNode) line: 1250` |
| 30 | 0.30% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 392` |
| 31 | 0.30% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5363` |
| 32 | 0.30% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 33 | 0.26% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 34 | 0.26% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 35 | 0.26% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 36 | 0.26% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 37 | 0.26% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2736` |
| 38 | 0.26% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 39 | 0.26% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5362` |
| 40 | 0.26% | `net.minecraft.world.phys.Vec3.horizontalDistanceSqr() line: 199` |
| 41 | 0.23% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 42 | 0.23% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5493` |
| 43 | 0.23% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5607` |
| 44 | 0.21% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 45 | 0.21% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 46 | 0.21% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2730` |
| 47 | 0.19% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 48 | 0.19% | `com.google.common.collect.RegularImmutableList.get(int) line: 80` |
| 49 | 0.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5603` |
| 50 | 0.19% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2729` |
| 51 | 0.19% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 52 | 0.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5495` |
| 53 | 0.19% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4074` |
| 54 | 0.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5522` |
| 55 | 0.17% | `dev.planeta9091.centities.CeHooks.vtabHash(int) line: 376` |
| 56 | 0.17% | `org.bukkit.craftbukkit.entity.CraftEntity.isValid() line: 508` |
| 57 | 0.15% | `com.sun.crypto.provider.CounterMode.increment(byte[]) line: 169` |
| 58 | 0.15% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 59 | 0.15% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 60 | 0.15% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3962` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 14.98% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 13.37% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9748` |
| 3 | 9.66% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3983` |
| 4 | 7.35% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 5 | 3.29% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6324` |
| 6 | 3.16% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 7 | 3.03% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2121` |
| 8 | 2.44% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3847` |
| 9 | 1.71% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 10 | 1.58% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3981` |
| 11 | 1.43% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 12 | 1.37% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3950` |
| 13 | 1.32% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5607` |
| 14 | 1.09% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4298` |
| 15 | 0.94% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 16 | 0.90% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 17 | 0.62% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 18 | 0.47% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 19 | 0.38% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 20 | 0.38% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 425` |
| 21 | 0.32% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 711` |
| 22 | 0.30% | `ca.spottedleaf.concurrentutil.collection.MultiThreadedQueue.<init>() line: 90` |
| 23 | 0.30% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 24 | 0.30% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3951` |
| 25 | 0.30% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 1025` |
| 26 | 0.28% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 27 | 0.26% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 28 | 0.26% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |
| 29 | 0.26% | `dev.planeta9091.centities.CeHooks.itemImmunities(ItemEntity) line: 4058` |
| 30 | 0.23% | `com.sun.crypto.provider.CounterMode.crypt(byte[], int, int, byte[], int) line: 195` |

GC pauses: 0, total 0 ms

