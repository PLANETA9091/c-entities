# JVM hotspots (JFR ExecutionSample)

Samples: 5459

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 12.53% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7773` |
| 2 | 8.08% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4115` |
| 3 | 5.73% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 4 | 4.47% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 5 | 4.16% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 3.96% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7720` |
| 7 | 2.93% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2811` |
| 8 | 2.69% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 9 | 2.07% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2713` |
| 10 | 1.43% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4833` |
| 11 | 1.37% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 12 | 1.30% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 13 | 1.04% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4121` |
| 14 | 0.92% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 15 | 0.88% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2764` |
| 16 | 0.79% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 17 | 0.71% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4835` |
| 18 | 0.71% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 19 | 0.68% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4152` |
| 20 | 0.62% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 21 | 0.59% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 22 | 0.55% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2005` |
| 23 | 0.51% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 24 | 0.48% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7416` |
| 25 | 0.40% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4160` |
| 26 | 0.40% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7724` |
| 27 | 0.38% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 28 | 0.37% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 29 | 0.35% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 30 | 0.35% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 31 | 0.35% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2760` |
| 32 | 0.35% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7734` |
| 33 | 0.33% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 272` |
| 34 | 0.31% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 35 | 0.29% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 36 | 0.29% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7414` |
| 37 | 0.27% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 38 | 0.26% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 39 | 0.26% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 40 | 0.24% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7728` |
| 41 | 0.22% | `net.minecraft.world.level.levelgen.Heightmap.update(int, int, int, BlockState) line: 101` |
| 42 | 0.22% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 43 | 0.22% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4114` |
| 44 | 0.22% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2618` |
| 45 | 0.20% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 46 | 0.20% | `net.minecraft.world.level.Level.isClientSide() line: 924` |
| 47 | 0.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4065` |
| 48 | 0.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3976` |
| 49 | 0.18% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 50 | 0.18% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 51 | 0.18% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 52 | 0.18% | `java.lang.Iterable.forEach(Consumer) line: 75` |
| 53 | 0.16% | `com.google.common.collect.RegularImmutableList.get(int) line: 80` |
| 54 | 0.16% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 55 | 0.16% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 56 | 0.16% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1989` |
| 57 | 0.16% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2761` |
| 58 | 0.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4126` |
| 59 | 0.15% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1475` |
| 60 | 0.15% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7731` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 12.84% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 12.71% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 5.99% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4152` |
| 4 | 5.20% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7769` |
| 5 | 4.51% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 6 | 3.21% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2734` |
| 7 | 3.02% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4835` |
| 8 | 2.89% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2616` |
| 9 | 2.67% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7761` |
| 10 | 2.36% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 11 | 2.03% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 12 | 1.54% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2733` |
| 13 | 1.37% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 14 | 1.12% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2713` |
| 15 | 1.01% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1528` |
| 16 | 0.99% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 17 | 0.90% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7727` |
| 18 | 0.79% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 19 | 0.62% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 20 | 0.55% | `dev.planeta9091.centities.CeHooks.isMergableMirror(ItemEntity) line: 400` |
| 21 | 0.53% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7719` |
| 22 | 0.53% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1990` |
| 23 | 0.51% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2714` |
| 24 | 0.49% | `java.math.BigInteger.modPow(BigInteger, BigInteger) line: 2924` |
| 25 | 0.48% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 26 | 0.46% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 27 | 0.37% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 28 | 0.33% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 417` |
| 29 | 0.31% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 30 | 0.29% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |

GC pauses: 0, total 0 ms

