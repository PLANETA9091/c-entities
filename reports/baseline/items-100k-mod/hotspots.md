# JVM hotspots (JFR ExecutionSample)

Samples: 5930

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 11.33% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7778` |
| 2 | 10.66% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4120` |
| 3 | 6.90% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 5.56% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 4.92% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 6 | 3.93% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2816` |
| 7 | 3.59% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7725` |
| 8 | 1.75% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 9 | 1.60% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 10 | 1.13% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2718` |
| 11 | 0.93% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7739` |
| 12 | 0.89% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 13 | 0.86% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4157` |
| 14 | 0.79% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4838` |
| 15 | 0.73% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 16 | 0.71% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 17 | 0.67% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4126` |
| 18 | 0.62% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 19 | 0.57% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2769` |
| 20 | 0.49% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 21 | 0.44% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 22 | 0.44% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 23 | 0.40% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2005` |
| 24 | 0.39% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7421` |
| 25 | 0.39% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7729` |
| 26 | 0.37% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7419` |
| 27 | 0.35% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1678` |
| 28 | 0.34% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4165` |
| 29 | 0.32% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4840` |
| 30 | 0.32% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2765` |
| 31 | 0.30% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 32 | 0.29% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 33 | 0.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 34 | 0.27% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 35 | 0.27% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 36 | 0.27% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1986` |
| 37 | 0.25% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 38 | 0.24% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 39 | 0.24% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 40 | 0.24% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3981` |
| 41 | 0.22% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 42 | 0.22% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1989` |
| 43 | 0.20% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 44 | 0.20% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 45 | 0.20% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2623` |
| 46 | 0.19% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 47 | 0.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3980` |
| 48 | 0.19% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 272` |
| 49 | 0.19% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2711` |
| 50 | 0.17% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 51 | 0.17% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 52 | 0.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4069` |
| 53 | 0.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4070` |
| 54 | 0.17% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 55 | 0.17% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2616` |
| 56 | 0.15% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(int, int) line: 929` |
| 57 | 0.15% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5187` |
| 58 | 0.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4189` |
| 59 | 0.13% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 60 | 0.13% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1475` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 15.28% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 11.50% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 6.90% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 4 | 5.45% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4157` |
| 5 | 5.24% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7774` |
| 6 | 4.74% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7766` |
| 7 | 4.18% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2739` |
| 8 | 2.28% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4840` |
| 9 | 1.84% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2621` |
| 10 | 1.75% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 11 | 1.60% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 12 | 1.37% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 13 | 1.11% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2738` |
| 14 | 1.06% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1528` |
| 15 | 0.89% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 16 | 0.73% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 17 | 0.54% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 18 | 0.52% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2718` |
| 19 | 0.52% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7732` |
| 20 | 0.49% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1990` |
| 21 | 0.47% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 430` |
| 22 | 0.46% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 23 | 0.39% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 24 | 0.34% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.schedule(int, int, ChunkStatus, NewChunkHolder, List) line: 673` |
| 25 | 0.34% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7725` |
| 26 | 0.32% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 27 | 0.30% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7724` |
| 28 | 0.29% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 29 | 0.29% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 30 | 0.24% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |

GC pauses: 0, total 0 ms

