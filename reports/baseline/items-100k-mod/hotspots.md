# JVM hotspots (JFR ExecutionSample)

Samples: 5808

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 11.97% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7773` |
| 2 | 10.45% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4115` |
| 3 | 5.82% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 5.06% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 5 | 5.01% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 3.70% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7720` |
| 7 | 3.50% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2811` |
| 8 | 2.55% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 9 | 1.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 10 | 1.34% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2713` |
| 11 | 0.93% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 12 | 0.91% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4833` |
| 13 | 0.90% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4121` |
| 14 | 0.71% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 15 | 0.69% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4835` |
| 16 | 0.65% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 17 | 0.65% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4152` |
| 18 | 0.64% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2764` |
| 19 | 0.62% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7734` |
| 20 | 0.60% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 21 | 0.59% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 22 | 0.43% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2005` |
| 23 | 0.43% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 24 | 0.43% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1678` |
| 25 | 0.40% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7724` |
| 26 | 0.38% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 272` |
| 27 | 0.34% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 28 | 0.34% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4160` |
| 29 | 0.33% | `net.minecraft.world.level.levelgen.Heightmap.update(int, int, int, BlockState) line: 101` |
| 30 | 0.31% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7416` |
| 31 | 0.31% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 32 | 0.29% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 33 | 0.29% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 34 | 0.29% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 35 | 0.28% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 36 | 0.28% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2618` |
| 37 | 0.24% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 38 | 0.24% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 39 | 0.24% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 40 | 0.24% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 41 | 0.21% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 42 | 0.21% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1410` |
| 43 | 0.21% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2760` |
| 44 | 0.21% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2725` |
| 45 | 0.19% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 46 | 0.19% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 47 | 0.19% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 48 | 0.19% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 49 | 0.17% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 50 | 0.17% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1986` |
| 51 | 0.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4184` |
| 52 | 0.17% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2616` |
| 53 | 0.17% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7414` |
| 54 | 0.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3976` |
| 55 | 0.15% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 56 | 0.15% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 57 | 0.15% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1989` |
| 58 | 0.15% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2719` |
| 59 | 0.15% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7703` |
| 60 | 0.15% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2706` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 14.55% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 12.10% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 5.84% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 4 | 5.41% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4152` |
| 5 | 5.13% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7769` |
| 6 | 4.22% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7761` |
| 7 | 3.77% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2734` |
| 8 | 2.74% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4835` |
| 9 | 2.65% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2616` |
| 10 | 1.74% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 11 | 1.65% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 12 | 1.39% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 13 | 1.12% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2733` |
| 14 | 0.95% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1528` |
| 15 | 0.71% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 16 | 0.64% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7727` |
| 17 | 0.62% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 18 | 0.60% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 19 | 0.55% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2713` |
| 20 | 0.53% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 430` |
| 21 | 0.48% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1990` |
| 22 | 0.43% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2714` |
| 23 | 0.38% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 24 | 0.38% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 417` |
| 25 | 0.36% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 26 | 0.33% | `net.minecraft.world.level.chunk.ProtoChunk.setBlockState(BlockPos, BlockState, int) line: 213` |
| 27 | 0.33% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 28 | 0.29% | `com.sun.crypto.provider.CounterMode.crypt(byte[], int, int, byte[], int) line: 195` |
| 29 | 0.29% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 30 | 0.28% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |

GC pauses: 0, total 0 ms

