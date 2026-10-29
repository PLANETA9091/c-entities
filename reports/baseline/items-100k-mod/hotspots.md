# JVM hotspots (JFR ExecutionSample)

Samples: 4505

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 8.30% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 2 | 8.30% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8003` |
| 3 | 5.02% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8056` |
| 4 | 4.46% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 3.66% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2961` |
| 6 | 2.95% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3022` |
| 7 | 2.62% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5094` |
| 8 | 2.33% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4374` |
| 9 | 1.89% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3069` |
| 10 | 1.73% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2190` |
| 11 | 1.55% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 12 | 1.47% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 13 | 1.33% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 14 | 1.31% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 15 | 1.27% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7699` |
| 16 | 1.15% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 17 | 1.09% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 18 | 1.07% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2209` |
| 19 | 0.95% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4380` |
| 20 | 0.91% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8007` |
| 21 | 0.89% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 22 | 0.78% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4411` |
| 23 | 0.75% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5092` |
| 24 | 0.73% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 25 | 0.69% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 26 | 0.55% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 27 | 0.49% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 28 | 0.47% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 29 | 0.44% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 30 | 0.38% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 31 | 0.38% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7697` |
| 32 | 0.36% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 33 | 0.33% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1868` |
| 34 | 0.33% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2954` |
| 35 | 0.33% | `net.minecraft.world.phys.Vec3.horizontalDistanceSqr() line: 199` |
| 36 | 0.31% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4235` |
| 37 | 0.31% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4419` |
| 38 | 0.29% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 39 | 0.27% | `java.util.ArrayList.<init>(int) line: 157` |
| 40 | 0.24% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 41 | 0.24% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 272` |
| 42 | 0.22% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 43 | 0.22% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 44 | 0.22% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 45 | 0.22% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 46 | 0.22% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1031` |
| 47 | 0.22% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 48 | 0.22% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2973` |
| 49 | 0.20% | `ca.spottedleaf.concurrentutil.collection.MultiThreadedQueue.getTailOpaque() line: 80` |
| 50 | 0.20% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2187` |
| 51 | 0.20% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2859` |
| 52 | 0.20% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3024` |
| 53 | 0.18% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 54 | 0.18% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 55 | 0.18% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 56 | 0.18% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3066` |
| 57 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4324` |
| 58 | 0.16% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 59 | 0.16% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1475` |
| 60 | 0.16% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 9.41% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8052` |
| 2 | 8.57% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4411` |
| 3 | 6.53% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 4 | 5.17% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 5.06% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5094` |
| 6 | 3.91% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1709` |
| 7 | 3.60% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2981` |
| 8 | 3.57% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 9 | 2.33% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2982` |
| 10 | 2.13% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8010` |
| 11 | 1.73% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 12 | 1.64% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 13 | 1.58% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 14 | 1.47% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 15 | 1.38% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2864` |
| 16 | 1.33% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 17 | 1.31% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8044` |
| 18 | 1.13% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8003` |
| 19 | 0.93% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8002` |
| 20 | 0.82% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 21 | 0.73% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 22 | 0.73% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 23 | 0.73% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2961` |
| 24 | 0.69% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2962` |
| 25 | 0.47% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 26 | 0.47% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |
| 27 | 0.42% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 28 | 0.38% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 29 | 0.38% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 548` |
| 30 | 0.36% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |

GC pauses: 0, total 0 ms

