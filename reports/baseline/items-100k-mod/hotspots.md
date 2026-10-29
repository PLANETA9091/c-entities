# JVM hotspots (JFR ExecutionSample)

Samples: 4490

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 10.13% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8056` |
| 2 | 7.64% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4374` |
| 3 | 5.86% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 4 | 4.97% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3069` |
| 5 | 4.34% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 4.25% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8003` |
| 7 | 3.79% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 8 | 2.74% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2961` |
| 9 | 2.32% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 10 | 2.16% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 11 | 1.83% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3022` |
| 12 | 1.65% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 13 | 1.60% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2190` |
| 14 | 1.54% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5094` |
| 15 | 1.16% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 16 | 1.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4380` |
| 17 | 1.09% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2209` |
| 18 | 0.98% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4411` |
| 19 | 0.85% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 20 | 0.73% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 21 | 0.73% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8017` |
| 22 | 0.69% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5092` |
| 23 | 0.62% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 24 | 0.58% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8007` |
| 25 | 0.58% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7697` |
| 26 | 0.56% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 27 | 0.56% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7699` |
| 28 | 0.49% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 29 | 0.40% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1868` |
| 30 | 0.38% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 31 | 0.38% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 272` |
| 32 | 0.36% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4443` |
| 33 | 0.36% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8014` |
| 34 | 0.36% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity) line: 906` |
| 35 | 0.33% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 36 | 0.31% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4419` |
| 37 | 0.29% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 38 | 0.29% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 39 | 0.27% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 40 | 0.27% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 41 | 0.24% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 42 | 0.22% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 43 | 0.22% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2954` |
| 44 | 0.18% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 45 | 0.16% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 46 | 0.16% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1410` |
| 47 | 0.16% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 48 | 0.16% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 49 | 0.16% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 50 | 0.16% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 51 | 0.16% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 52 | 0.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4322` |
| 53 | 0.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4234` |
| 54 | 0.16% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 55 | 0.16% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3024` |
| 56 | 0.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4235` |
| 57 | 0.16% | `net.minecraft.world.phys.Vec3.lambda$static$2(Vec3) line: 23` |
| 58 | 0.13% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 59 | 0.13% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 60 | 0.13% | `com.google.common.collect.RegularImmutableList.get(int) line: 80` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 12.34% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 10.22% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 6.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4411` |
| 4 | 6.15% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8052` |
| 5 | 5.12% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2982` |
| 6 | 3.88% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5094` |
| 7 | 3.81% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 8 | 3.36% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1709` |
| 9 | 2.38% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2864` |
| 10 | 2.36% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 11 | 2.23% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8044` |
| 12 | 2.23% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2981` |
| 13 | 2.18% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 14 | 1.56% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 15 | 1.38% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2961` |
| 16 | 1.27% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8010` |
| 17 | 1.22% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 18 | 1.18% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 19 | 0.76% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8002` |
| 20 | 0.62% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 548` |
| 21 | 0.62% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2962` |
| 22 | 0.56% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 23 | 0.56% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 24 | 0.51% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2194` |
| 25 | 0.49% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8003` |
| 26 | 0.42% | `java.math.BigInteger.modPow(BigInteger, BigInteger) line: 2924` |
| 27 | 0.38% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 526` |
| 28 | 0.36% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 29 | 0.31% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1842` |
| 30 | 0.29% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |

GC pauses: 0, total 0 ms

