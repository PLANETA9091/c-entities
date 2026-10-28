# JVM hotspots (JFR ExecutionSample)

Samples: 5230

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 13.12% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7669` |
| 2 | 7.84% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 3 | 5.91% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7722` |
| 4 | 4.59% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 3.71% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2722` |
| 6 | 3.56% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4073` |
| 7 | 3.29% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2671` |
| 8 | 2.64% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4784` |
| 9 | 2.58% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2769` |
| 10 | 1.45% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 11 | 1.41% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 12 | 1.19% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 13 | 1.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4079` |
| 14 | 1.13% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 15 | 1.01% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 16 | 0.99% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4110` |
| 17 | 0.99% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 18 | 0.99% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 19 | 0.86% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7365` |
| 20 | 0.80% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1963` |
| 21 | 0.80% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 22 | 0.78% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 23 | 0.63% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7673` |
| 24 | 0.54% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity) line: 719` |
| 25 | 0.44% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 270` |
| 26 | 0.40% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4118` |
| 27 | 0.40% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4782` |
| 28 | 0.38% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 29 | 0.36% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 30 | 0.36% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5187` |
| 31 | 0.36% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 32 | 0.34% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1944` |
| 33 | 0.33% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 34 | 0.31% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 35 | 0.29% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 36 | 0.29% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2718` |
| 37 | 0.29% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 38 | 0.27% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 39 | 0.27% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7363` |
| 40 | 0.25% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 41 | 0.25% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4135` |
| 42 | 0.23% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 43 | 0.23% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 44 | 0.23% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1636` |
| 45 | 0.21% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 46 | 0.21% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 47 | 0.21% | `net.minecraft.world.phys.Vec3.horizontalDistanceSqr() line: 199` |
| 48 | 0.19% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2683` |
| 49 | 0.17% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 50 | 0.17% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 51 | 0.17% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 52 | 0.17% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2766` |
| 53 | 0.17% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2571` |
| 54 | 0.17% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7706` |
| 55 | 0.15% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupBlock() line: 824` |
| 56 | 0.15% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 57 | 0.15% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2664` |
| 58 | 0.15% | `it.unimi.dsi.fastutil.objects.Reference2IntOpenHashMap.find(Object) line: 241` |
| 59 | 0.15% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2569` |
| 60 | 0.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3934` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 14.11% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7718` |
| 2 | 8.22% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 3 | 8.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4110` |
| 4 | 6.14% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 4.53% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4784` |
| 6 | 4.21% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2691` |
| 7 | 3.17% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 8 | 2.93% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2692` |
| 9 | 2.20% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7676` |
| 10 | 1.72% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 11 | 1.47% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2574` |
| 12 | 1.45% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1486` |
| 13 | 1.43% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 14 | 1.32% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7710` |
| 15 | 1.19% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 16 | 1.13% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 17 | 1.11% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2671` |
| 18 | 1.03% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7668` |
| 19 | 0.99% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2672` |
| 20 | 0.99% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 21 | 0.86% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 22 | 0.71% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7669` |
| 23 | 0.55% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 24 | 0.54% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 25 | 0.52% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 26 | 0.44% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 375` |
| 27 | 0.34% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 28 | 0.34% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |
| 29 | 0.34% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 388` |
| 30 | 0.31% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |

GC pauses: 0, total 0 ms

