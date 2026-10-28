# JVM hotspots (JFR ExecutionSample)

Samples: 4867

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 13.29% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7773` |
| 2 | 7.36% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4115` |
| 3 | 7.03% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 5.14% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 5 | 4.71% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 4.52% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7720` |
| 7 | 4.01% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2811` |
| 8 | 2.98% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 9 | 1.66% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2713` |
| 10 | 1.56% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 11 | 1.48% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 12 | 1.23% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4833` |
| 13 | 0.95% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2764` |
| 14 | 0.88% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4121` |
| 15 | 0.86% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 16 | 0.86% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 17 | 0.80% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 18 | 0.72% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7724` |
| 19 | 0.72% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7734` |
| 20 | 0.64% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4152` |
| 21 | 0.64% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 22 | 0.60% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 23 | 0.47% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4835` |
| 24 | 0.43% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 25 | 0.39% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4160` |
| 26 | 0.39% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7416` |
| 27 | 0.37% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 272` |
| 28 | 0.35% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2005` |
| 29 | 0.33% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 30 | 0.33% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 31 | 0.31% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 32 | 0.27% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 33 | 0.27% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 34 | 0.27% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 35 | 0.25% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 36 | 0.25% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7731` |
| 37 | 0.25% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 38 | 0.25% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2618` |
| 39 | 0.25% | `net.minecraft.world.level.Level.isClientSide() line: 924` |
| 40 | 0.23% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 41 | 0.23% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 42 | 0.23% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 43 | 0.23% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1475` |
| 44 | 0.21% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 45 | 0.21% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 46 | 0.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3975` |
| 47 | 0.21% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7414` |
| 48 | 0.21% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2706` |
| 49 | 0.21% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2766` |
| 50 | 0.18% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 51 | 0.18% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 52 | 0.16% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 53 | 0.16% | `net.minecraft.world.level.levelgen.Heightmap.update(int, int, int, BlockState) line: 101` |
| 54 | 0.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4062` |
| 55 | 0.16% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 850` |
| 56 | 0.16% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2760` |
| 57 | 0.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3976` |
| 58 | 0.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4064` |
| 59 | 0.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4065` |
| 60 | 0.14% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 13.54% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 2 | 11.55% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 3 | 7.09% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 4 | 6.39% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7769` |
| 5 | 5.42% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4152` |
| 6 | 4.25% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2734` |
| 7 | 3.33% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7761` |
| 8 | 3.23% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2616` |
| 9 | 2.92% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4835` |
| 10 | 1.83% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 11 | 1.58% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 12 | 1.46% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2733` |
| 13 | 1.38% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 14 | 1.23% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2713` |
| 15 | 0.97% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7727` |
| 16 | 0.86% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 17 | 0.86% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 18 | 0.74% | `dev.planeta9091.centities.CeHooks.isMergableMirror(ItemEntity) line: 400` |
| 19 | 0.72% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 20 | 0.68% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1528` |
| 21 | 0.49% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1990` |
| 22 | 0.43% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 23 | 0.43% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2714` |
| 24 | 0.39% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7719` |
| 25 | 0.37% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 417` |
| 26 | 0.33% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 27 | 0.33% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 1025` |
| 28 | 0.31% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 29 | 0.31% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 30 | 0.29% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |

GC pauses: 0, total 0 ms

