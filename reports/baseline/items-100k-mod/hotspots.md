# JVM hotspots (JFR ExecutionSample)

Samples: 6331

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 13.84% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4941` |
| 2 | 11.91% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8696` |
| 3 | 6.74% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 4.88% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 3.14% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3642` |
| 6 | 2.91% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8643` |
| 7 | 2.16% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 389` |
| 8 | 1.63% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 9 | 1.53% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 10 | 1.22% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2598` |
| 11 | 1.20% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 12 | 1.15% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5732` |
| 13 | 1.09% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5005` |
| 14 | 0.84% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 15 | 0.84% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2173` |
| 16 | 0.82% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8657` |
| 17 | 0.74% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 18 | 0.73% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 19 | 0.71% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5042` |
| 20 | 0.57% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3534` |
| 21 | 0.44% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5734` |
| 22 | 0.39% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 23 | 0.35% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 8337` |
| 24 | 0.33% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3595` |
| 25 | 0.32% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 26 | 0.32% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 277` |
| 27 | 0.30% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 28 | 0.27% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 29 | 0.25% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2588` |
| 30 | 0.25% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8647` |
| 31 | 0.25% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4824` |
| 32 | 0.24% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 33 | 0.24% | `java.lang.Integer.valueOf(int) line: 1006` |
| 34 | 0.22% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 35 | 0.22% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 36 | 0.22% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 37 | 0.22% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 38 | 0.21% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 39 | 0.21% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 40 | 0.21% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 41 | 0.21% | `java.util.HashMap.getNode(Object) line: 577` |
| 42 | 0.19% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 43 | 0.19% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 44 | 0.19% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3439` |
| 45 | 0.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5011` |
| 46 | 0.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4823` |
| 47 | 0.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5050` |
| 48 | 0.19% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 49 | 0.19% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3437` |
| 50 | 0.17% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 51 | 0.17% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2610` |
| 52 | 0.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4952` |
| 53 | 0.17% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3527` |
| 54 | 0.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4911` |
| 55 | 0.16% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 56 | 0.16% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5195` |
| 57 | 0.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4953` |
| 58 | 0.16% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5730` |
| 59 | 0.16% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 60 | 0.14% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 18.89% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 12.07% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 6.76% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 4 | 4.28% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8692` |
| 5 | 4.28% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8684` |
| 6 | 3.44% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3555` |
| 7 | 2.32% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 1985` |
| 8 | 2.21% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2594` |
| 9 | 2.15% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5734` |
| 10 | 1.85% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 11 | 1.72% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3437` |
| 12 | 1.63% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 13 | 1.53% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 14 | 1.11% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 688` |
| 15 | 0.98% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5042` |
| 16 | 0.79% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2611` |
| 17 | 0.73% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 18 | 0.60% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3554` |
| 19 | 0.55% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 20 | 0.49% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 21 | 0.46% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8650` |
| 22 | 0.38% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 23 | 0.38% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2142` |
| 24 | 0.35% | `java.util.HashMap.get(Object) line: 565` |
| 25 | 0.33% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 26 | 0.32% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 27 | 0.32% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 637` |
| 28 | 0.30% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3534` |
| 29 | 0.27% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |
| 30 | 0.24% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |

GC pauses: 0, total 0 ms

