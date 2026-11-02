# JVM hotspots (JFR ExecutionSample)

Samples: 5918

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 17.35% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 9.97% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5435` |
| 3 | 4.26% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 3.84% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4028` |
| 5 | 3.21% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 6 | 2.13% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9322` |
| 7 | 1.67% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3901` |
| 8 | 1.55% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9317` |
| 9 | 1.33% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 10 | 1.28% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5441` |
| 11 | 1.25% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 12 | 1.18% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 13 | 1.08% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 14 | 1.08% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 15 | 1.01% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9316` |
| 16 | 0.95% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6203` |
| 17 | 0.81% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 18 | 0.81% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2750` |
| 19 | 0.81% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2712` |
| 20 | 0.73% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5512` |
| 21 | 0.69% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3981` |
| 22 | 0.68% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 23 | 0.64% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6213` |
| 24 | 0.59% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 277` |
| 25 | 0.47% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5497` |
| 26 | 0.35% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 27 | 0.32% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 28 | 0.30% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 29 | 0.30% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5254` |
| 30 | 0.30% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3913` |
| 31 | 0.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 32 | 0.27% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2703` |
| 33 | 0.25% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 34 | 0.25% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2702` |
| 35 | 0.25% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5340` |
| 36 | 0.24% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 37 | 0.24% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 38 | 0.24% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3805` |
| 39 | 0.22% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 40 | 0.22% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 41 | 0.22% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 42 | 0.22% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 43 | 0.22% | `net.minecraft.world.level.Level.isClientSide() line: 924` |
| 44 | 0.20% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 45 | 0.20% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 46 | 0.20% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2709` |
| 47 | 0.20% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2724` |
| 48 | 0.19% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3894` |
| 49 | 0.17% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 50 | 0.17% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 51 | 0.17% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 52 | 0.17% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 53 | 0.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5253` |
| 54 | 0.17% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4025` |
| 55 | 0.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5382` |
| 56 | 0.17% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3907` |
| 57 | 0.15% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5185` |
| 58 | 0.15% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5200` |
| 59 | 0.15% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3798` |
| 60 | 0.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5383` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 16.19% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 13.70% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9321` |
| 3 | 4.87% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9424` |
| 4 | 4.28% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 5 | 4.11% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3934` |
| 6 | 3.67% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9414` |
| 7 | 3.40% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3803` |
| 8 | 3.38% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6213` |
| 9 | 3.36% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2094` |
| 10 | 1.98% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 11 | 1.94% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 12 | 1.25% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 13 | 1.10% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5497` |
| 14 | 1.10% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3932` |
| 15 | 0.98% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3901` |
| 16 | 0.81% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 17 | 0.71% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 18 | 0.61% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9423` |
| 19 | 0.59% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 711` |
| 20 | 0.57% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2725` |
| 21 | 0.56% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 22 | 0.44% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 23 | 0.37% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 24 | 0.35% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 25 | 0.32% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3902` |
| 26 | 0.27% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 425` |
| 27 | 0.24% | `com.sun.crypto.provider.CounterMode.crypt(byte[], int, int, byte[], int) line: 195` |
| 28 | 0.22% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 29 | 0.22% | `net.minecraft.world.level.chunk.status.ChunkStatusTasks.lambda$buildTerrain$0(ChunkAccess) line: 130` |
| 30 | 0.22% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |

GC pauses: 0, total 0 ms

