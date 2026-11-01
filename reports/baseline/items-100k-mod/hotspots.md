# JVM hotspots (JFR ExecutionSample)

Samples: 4751

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 10.17% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8920` |
| 2 | 7.62% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5207` |
| 3 | 6.08% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 4 | 5.60% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3800` |
| 5 | 4.74% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8867` |
| 6 | 4.71% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 4.13% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 8 | 2.36% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3673` |
| 9 | 2.27% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 10 | 1.79% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 11 | 1.75% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3753` |
| 12 | 1.56% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5213` |
| 13 | 1.45% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5953` |
| 14 | 1.43% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2697` |
| 15 | 1.33% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 16 | 1.26% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5943` |
| 17 | 1.05% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 18 | 0.97% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 19 | 0.93% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 20 | 0.91% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8881` |
| 21 | 0.88% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2735` |
| 22 | 0.76% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 23 | 0.74% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5244` |
| 24 | 0.72% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 8561` |
| 25 | 0.61% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8871` |
| 26 | 0.57% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 27 | 0.55% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5252` |
| 28 | 0.51% | `dev.planeta9091.centities.CeHooks.probeDeltaCommon(Entity, double, double, double) line: 1212` |
| 29 | 0.48% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 8563` |
| 30 | 0.38% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 277` |
| 31 | 0.34% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 32 | 0.34% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8878` |
| 33 | 0.34% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 34 | 0.32% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5081` |
| 35 | 0.29% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 36 | 0.29% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3666` |
| 37 | 0.25% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 38 | 0.25% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 39 | 0.25% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5285` |
| 40 | 0.25% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3755` |
| 41 | 0.23% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 874` |
| 42 | 0.23% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5026` |
| 43 | 0.21% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 44 | 0.21% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 45 | 0.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5206` |
| 46 | 0.19% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3577` |
| 47 | 0.17% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 48 | 0.17% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 49 | 0.17% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 50 | 0.17% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 51 | 0.17% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 52 | 0.17% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2687` |
| 53 | 0.17% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 54 | 0.17% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2267` |
| 55 | 0.17% | `org.bukkit.craftbukkit.entity.CraftEntity.isValid() line: 508` |
| 56 | 0.15% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupBlock() line: 824` |
| 57 | 0.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5156` |
| 58 | 0.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5153` |
| 59 | 0.15% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8875` |
| 60 | 0.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5155` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 13.07% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 10.36% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 6.78% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8916` |
| 4 | 6.48% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5244` |
| 5 | 5.73% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3706` |
| 6 | 4.15% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 7 | 3.60% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5953` |
| 8 | 3.22% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2079` |
| 9 | 2.90% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 10 | 2.29% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3575` |
| 11 | 2.27% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8908` |
| 12 | 2.17% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3704` |
| 13 | 1.81% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 14 | 1.73% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8874` |
| 15 | 1.20% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3673` |
| 16 | 1.20% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 17 | 1.16% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 18 | 0.97% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 19 | 0.95% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 20 | 0.88% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2710` |
| 21 | 0.67% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8866` |
| 22 | 0.53% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity, Vec3) line: 1166` |
| 23 | 0.51% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8867` |
| 24 | 0.38% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 711` |
| 25 | 0.34% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 26 | 0.34% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3674` |
| 27 | 0.27% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 28 | 0.25% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 29 | 0.25% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 30 | 0.23% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |

GC pauses: 0, total 0 ms

