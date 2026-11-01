# JVM hotspots (JFR ExecutionSample)

Samples: 5100

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 12.10% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9008` |
| 2 | 7.55% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5295` |
| 3 | 5.45% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 5.06% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 4.75% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 6 | 4.12% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8955` |
| 7 | 3.22% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3888` |
| 8 | 2.69% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 9 | 2.10% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 10 | 1.98% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3761` |
| 11 | 1.35% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 12 | 1.20% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6031` |
| 13 | 1.12% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3841` |
| 14 | 1.00% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 15 | 0.98% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5301` |
| 16 | 0.86% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 17 | 0.78% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6041` |
| 18 | 0.73% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2735` |
| 19 | 0.71% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 20 | 0.61% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8959` |
| 21 | 0.55% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8969` |
| 22 | 0.49% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 23 | 0.47% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 24 | 0.45% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5340` |
| 25 | 0.45% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5332` |
| 26 | 0.41% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2697` |
| 27 | 0.41% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 8651` |
| 28 | 0.39% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2709` |
| 29 | 0.35% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 8649` |
| 30 | 0.35% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3665` |
| 31 | 0.29% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 32 | 0.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 33 | 0.29% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5192` |
| 34 | 0.29% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 277` |
| 35 | 0.27% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5081` |
| 36 | 0.27% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5200` |
| 37 | 0.27% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 38 | 0.25% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 39 | 0.24% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 40 | 0.24% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 41 | 0.24% | `net.minecraft.world.phys.Vec3.horizontalDistanceSqr() line: 199` |
| 42 | 0.22% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 43 | 0.22% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 44 | 0.22% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 393` |
| 45 | 0.20% | `com.sun.crypto.provider.CounterMode.increment(byte[]) line: 169` |
| 46 | 0.20% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 47 | 0.20% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 48 | 0.20% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 49 | 0.20% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2694` |
| 50 | 0.20% | `java.lang.Double.isFinite(double) line: 1023` |
| 51 | 0.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5373` |
| 52 | 0.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5114` |
| 53 | 0.18% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 54 | 0.18% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2680` |
| 55 | 0.18% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3843` |
| 56 | 0.16% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 57 | 0.16% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2687` |
| 58 | 0.16% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2688` |
| 59 | 0.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5245` |
| 60 | 0.16% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 12.37% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 12.22% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 5.67% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9004` |
| 4 | 5.51% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 5 | 5.00% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5332` |
| 6 | 3.41% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3794` |
| 7 | 3.37% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8996` |
| 8 | 3.18% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6041` |
| 9 | 2.84% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3663` |
| 10 | 2.75% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2079` |
| 11 | 2.18% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 12 | 2.10% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 13 | 1.59% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3792` |
| 14 | 1.35% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 15 | 1.18% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3761` |
| 16 | 1.08% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8962` |
| 17 | 0.94% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 18 | 0.84% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2710` |
| 19 | 0.76% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 20 | 0.71% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 21 | 0.53% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8954` |
| 22 | 0.49% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 23 | 0.47% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3762` |
| 24 | 0.39% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 425` |
| 25 | 0.33% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 26 | 0.29% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 27 | 0.29% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 711` |
| 28 | 0.27% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 29 | 0.27% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 30 | 0.25% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |

GC pauses: 0, total 0 ms

