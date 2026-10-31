# JVM hotspots (JFR ExecutionSample)

Samples: 6664

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 14.77% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5033` |
| 2 | 11.04% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8788` |
| 3 | 4.85% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 4.05% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 5 | 3.83% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8735` |
| 6 | 3.42% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3734` |
| 7 | 2.97% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 389` |
| 8 | 2.34% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 9 | 1.41% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 10 | 1.22% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5097` |
| 11 | 1.10% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3607` |
| 12 | 1.04% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5824` |
| 13 | 0.95% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 14 | 0.89% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2661` |
| 15 | 0.87% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 16 | 0.84% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 17 | 0.78% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 18 | 0.74% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 19 | 0.71% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8749` |
| 20 | 0.65% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 21 | 0.62% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5134` |
| 22 | 0.51% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3687` |
| 23 | 0.45% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5081` |
| 24 | 0.45% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 25 | 0.44% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5826` |
| 26 | 0.42% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8739` |
| 27 | 0.36% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4916` |
| 28 | 0.32% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 277` |
| 29 | 0.29% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 30 | 0.29% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 8429` |
| 31 | 0.27% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 32 | 0.27% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3511` |
| 33 | 0.26% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 34 | 0.26% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 35 | 0.24% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 36 | 0.24% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5142` |
| 37 | 0.23% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 38 | 0.23% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 39 | 0.21% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 40 | 0.21% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 41 | 0.21% | `java.lang.Integer.valueOf(int) line: 1006` |
| 42 | 0.20% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 43 | 0.20% | `java.util.HashMap.getNode(Object) line: 577` |
| 44 | 0.18% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 45 | 0.18% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 46 | 0.18% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3619` |
| 47 | 0.18% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8743` |
| 48 | 0.17% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 49 | 0.17% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 50 | 0.17% | `net.minecraft.world.level.levelgen.Heightmap.update(int, int, int, BlockState) line: 101` |
| 51 | 0.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5175` |
| 52 | 0.17% | `dev.planeta9091.centities.CeHooks.probeDeltaCommon(Entity, double, double, double) line: 1176` |
| 53 | 0.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5096` |
| 54 | 0.17% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8718` |
| 55 | 0.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5044` |
| 56 | 0.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5047` |
| 57 | 0.15% | `com.sun.crypto.provider.CounterMode.increment(byte[]) line: 169` |
| 58 | 0.15% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 59 | 0.15% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1475` |
| 60 | 0.15% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5192` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 19.93% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 11.16% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 5.46% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8784` |
| 4 | 4.10% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 5 | 3.86% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8776` |
| 6 | 3.56% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3640` |
| 7 | 2.43% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5826` |
| 8 | 2.43% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3509` |
| 9 | 2.22% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2657` |
| 10 | 1.71% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2043` |
| 11 | 1.64% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 12 | 1.62% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 13 | 1.41% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 14 | 1.40% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5134` |
| 15 | 0.84% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2674` |
| 16 | 0.84% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3638` |
| 17 | 0.77% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2228` |
| 18 | 0.74% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 19 | 0.74% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3607` |
| 20 | 0.69% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 21 | 0.68% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8742` |
| 22 | 0.47% | `java.util.HashMap.get(Object) line: 565` |
| 23 | 0.45% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3608` |
| 24 | 0.35% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 25 | 0.35% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 26 | 0.32% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 27 | 0.32% | `java.math.BigInteger.modPow(BigInteger, BigInteger) line: 2924` |
| 28 | 0.32% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 675` |
| 29 | 0.29% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 30 | 0.27% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8734` |

GC pauses: 0, total 0 ms

