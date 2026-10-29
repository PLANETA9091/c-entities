# JVM hotspots (JFR ExecutionSample)

Samples: 4872

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 11.86% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7836` |
| 2 | 7.90% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4178` |
| 3 | 6.03% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 5.75% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 5 | 5.67% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7783` |
| 6 | 4.62% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 3.78% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2873` |
| 8 | 2.67% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 9 | 2.34% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2765` |
| 10 | 2.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 11 | 1.81% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 12 | 1.21% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4896` |
| 13 | 1.07% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4184` |
| 14 | 0.88% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 2826` |
| 15 | 0.86% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 16 | 0.80% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 17 | 0.74% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 18 | 0.72% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 19 | 0.70% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 20 | 0.68% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4215` |
| 21 | 0.66% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7787` |
| 22 | 0.66% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7797` |
| 23 | 0.62% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2018` |
| 24 | 0.57% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4898` |
| 25 | 0.45% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 26 | 0.43% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4223` |
| 27 | 0.35% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 28 | 0.35% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 29 | 0.35% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 30 | 0.33% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7477` |
| 31 | 0.29% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7479` |
| 32 | 0.27% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 33 | 0.25% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 34 | 0.25% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 272` |
| 35 | 0.25% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4039` |
| 36 | 0.23% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 37 | 0.23% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 38 | 0.21% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 39 | 0.21% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 40 | 0.18% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 41 | 0.18% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 42 | 0.18% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2002` |
| 43 | 0.16% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 44 | 0.16% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1999` |
| 45 | 0.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4177` |
| 46 | 0.16% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity) line: 774` |
| 47 | 0.16% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7794` |
| 48 | 0.16% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 49 | 0.14% | `net.minecraft.world.level.chunk.ProtoChunk.moonrise$getBlock(int, int, int) line: 67` |
| 50 | 0.14% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7791` |
| 51 | 0.14% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 52 | 0.14% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2663` |
| 53 | 0.14% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2771` |
| 54 | 0.14% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4128` |
| 55 | 0.14% | `net.minecraft.world.phys.Vec3.horizontalDistanceSqr() line: 199` |
| 56 | 0.14% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2670` |
| 57 | 0.14% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2664` |
| 58 | 0.14% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4125` |
| 59 | 0.12% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 60 | 0.12% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 12.48% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 11.93% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 7.47% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7832` |
| 4 | 6.10% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4215` |
| 5 | 6.08% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 6 | 3.94% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2786` |
| 7 | 3.35% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4898` |
| 8 | 3.33% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7824` |
| 9 | 2.75% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2668` |
| 10 | 2.30% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 11 | 1.91% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 12 | 1.62% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2765` |
| 13 | 1.33% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 14 | 1.15% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2785` |
| 15 | 1.07% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1541` |
| 16 | 0.88% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 17 | 0.80% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 18 | 0.80% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7790` |
| 19 | 0.72% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 20 | 0.64% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2003` |
| 21 | 0.51% | `dev.planeta9091.centities.CeHooks.isMergableMirror(ItemEntity) line: 400` |
| 22 | 0.45% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2766` |
| 23 | 0.43% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7782` |
| 24 | 0.39% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 25 | 0.35% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 26 | 0.33% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 27 | 0.31% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |
| 28 | 0.31% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 430` |
| 29 | 0.31% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7783` |
| 30 | 0.27% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |

GC pauses: 0, total 0 ms

