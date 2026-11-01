# JVM hotspots (JFR ExecutionSample)

Samples: 4607

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 9.14% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8836` |
| 2 | 8.23% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5145` |
| 3 | 6.47% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 4 | 4.86% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8783` |
| 5 | 4.67% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3770` |
| 6 | 4.58% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 7 | 4.30% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 2.65% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 9 | 2.34% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3643` |
| 10 | 1.98% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 11 | 1.80% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 12 | 1.76% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3723` |
| 13 | 1.54% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5874` |
| 14 | 1.37% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2697` |
| 15 | 1.37% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5151` |
| 16 | 1.11% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 17 | 1.06% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5872` |
| 18 | 1.02% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 19 | 0.98% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2735` |
| 20 | 0.93% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 8477` |
| 21 | 0.78% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 22 | 0.78% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5182` |
| 23 | 0.76% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8787` |
| 24 | 0.72% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8797` |
| 25 | 0.56% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5190` |
| 26 | 0.56% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 27 | 0.54% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 28 | 0.43% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5223` |
| 29 | 0.43% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2267` |
| 30 | 0.41% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 8479` |
| 31 | 0.30% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5081` |
| 32 | 0.28% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 33 | 0.28% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 34 | 0.28% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 35 | 0.24% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 36 | 0.24% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 37 | 0.24% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 38 | 0.24% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 39 | 0.22% | `dev.planeta9091.centities.CeHooks.probeDeltaCommon(Entity, double, double, double) line: 1212` |
| 40 | 0.22% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8794` |
| 41 | 0.22% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3547` |
| 42 | 0.20% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 43 | 0.20% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 44 | 0.20% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 45 | 0.20% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 46 | 0.20% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 47 | 0.20% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 48 | 0.20% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 49 | 0.20% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 50 | 0.20% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3655` |
| 51 | 0.17% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 52 | 0.17% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 277` |
| 53 | 0.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4963` |
| 54 | 0.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5094` |
| 55 | 0.15% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 56 | 0.15% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 57 | 0.15% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 58 | 0.15% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2687` |
| 59 | 0.15% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 60 | 0.15% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8766` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 13.91% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 9.20% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 6.84% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8832` |
| 4 | 6.77% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5182` |
| 5 | 4.86% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3676` |
| 6 | 4.58% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 7 | 3.30% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5874` |
| 8 | 3.06% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2079` |
| 9 | 2.67% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3545` |
| 10 | 2.67% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 11 | 2.26% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3674` |
| 12 | 2.02% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8824` |
| 13 | 1.98% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 14 | 1.71% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8790` |
| 15 | 1.63% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3643` |
| 16 | 1.37% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 17 | 1.13% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 18 | 1.09% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 19 | 0.85% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2710` |
| 20 | 0.56% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3644` |
| 21 | 0.54% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 22 | 0.52% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 762` |
| 23 | 0.50% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8782` |
| 24 | 0.35% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 25 | 0.33% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 26 | 0.33% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 27 | 0.33% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 28 | 0.30% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 29 | 0.28% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |
| 30 | 0.26% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |

GC pauses: 0, total 0 ms

