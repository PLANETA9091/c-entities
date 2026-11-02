# JVM hotspots (JFR ExecutionSample)

Samples: 3533

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 10.70% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5461` |
| 2 | 5.80% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4041` |
| 3 | 5.09% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9374` |
| 4 | 4.84% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 5 | 3.31% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3914` |
| 6 | 3.20% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3994` |
| 7 | 2.94% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 8 | 2.38% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 9 | 2.26% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6240` |
| 10 | 2.15% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 11 | 2.07% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 12 | 2.04% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 13 | 1.87% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2720` |
| 14 | 1.70% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5467` |
| 15 | 1.44% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 16 | 1.39% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 17 | 1.25% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2758` |
| 18 | 1.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5571` |
| 19 | 1.10% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 20 | 1.08% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 21 | 0.85% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 22 | 0.62% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6230` |
| 23 | 0.57% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5538` |
| 24 | 0.48% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 25 | 0.45% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 26 | 0.45% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9366` |
| 27 | 0.45% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 277` |
| 28 | 0.40% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9406` |
| 29 | 0.40% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 30 | 0.34% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 31 | 0.31% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5523` |
| 32 | 0.31% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9369` |
| 33 | 0.28% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3989` |
| 34 | 0.28% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 35 | 0.28% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5276` |
| 36 | 0.25% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 37 | 0.25% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 38 | 0.25% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 39 | 0.25% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3813` |
| 40 | 0.25% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5519` |
| 41 | 0.25% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3996` |
| 42 | 0.25% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 43 | 0.23% | `com.google.common.collect.RegularImmutableList.get(int) line: 80` |
| 44 | 0.23% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 45 | 0.20% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 46 | 0.20% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 47 | 0.20% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3926` |
| 48 | 0.20% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3990` |
| 49 | 0.17% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 50 | 0.17% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(int, int) line: 929` |
| 51 | 0.17% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 52 | 0.17% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2710` |
| 53 | 0.17% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 54 | 0.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5366` |
| 55 | 0.17% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4036` |
| 56 | 0.17% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6231` |
| 57 | 0.17% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9373` |
| 58 | 0.14% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 59 | 0.14% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 60 | 0.14% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 17.32% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 6.54% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9483` |
| 3 | 6.20% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3947` |
| 4 | 4.81% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3816` |
| 5 | 4.76% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6240` |
| 6 | 4.39% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2102` |
| 7 | 4.05% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3945` |
| 8 | 3.20% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 9 | 2.77% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3914` |
| 10 | 2.32% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9473` |
| 11 | 2.15% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 12 | 2.07% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 13 | 2.04% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5523` |
| 14 | 1.50% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 15 | 1.50% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 16 | 1.08% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3915` |
| 17 | 0.91% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2733` |
| 18 | 0.85% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 19 | 0.48% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 20 | 0.45% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 21 | 0.45% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |
| 22 | 0.45% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 711` |
| 23 | 0.40% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2737` |
| 24 | 0.37% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 25 | 0.37% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9482` |
| 26 | 0.34% | `net.minecraft.nbt.ListTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 74` |
| 27 | 0.28% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 28 | 0.28% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 29 | 0.28% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5542` |
| 30 | 0.25% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |

GC pauses: 0, total 0 ms

