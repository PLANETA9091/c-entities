# JVM hotspots (JFR ExecutionSample)

Samples: 5960

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 12.03% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7773` |
| 2 | 10.97% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4115` |
| 3 | 5.60% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 5.23% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 5 | 4.43% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 6 | 3.84% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2811` |
| 7 | 2.82% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7720` |
| 8 | 2.11% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 9 | 1.51% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4833` |
| 10 | 1.21% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 11 | 1.07% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7734` |
| 12 | 1.02% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2713` |
| 13 | 0.89% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 14 | 0.84% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4121` |
| 15 | 0.81% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 16 | 0.77% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 17 | 0.74% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 18 | 0.64% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4835` |
| 19 | 0.62% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4152` |
| 20 | 0.60% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 21 | 0.59% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2005` |
| 22 | 0.55% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 23 | 0.50% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2764` |
| 24 | 0.44% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 25 | 0.39% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4160` |
| 26 | 0.37% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7416` |
| 27 | 0.34% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 28 | 0.34% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7724` |
| 29 | 0.30% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 30 | 0.30% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7414` |
| 31 | 0.30% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 272` |
| 32 | 0.29% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3976` |
| 33 | 0.27% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 34 | 0.27% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 35 | 0.25% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 36 | 0.25% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 37 | 0.25% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 38 | 0.23% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 39 | 0.23% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.schedule(int, int, ChunkStatus, NewChunkHolder, List, Priority) line: 740` |
| 40 | 0.20% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 41 | 0.20% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2618` |
| 42 | 0.18% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 43 | 0.18% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 44 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4114` |
| 45 | 0.18% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2725` |
| 46 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4064` |
| 47 | 0.18% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2766` |
| 48 | 0.17% | `net.minecraft.world.level.levelgen.material.rule.ConditionRule.lambda$compile$0(ConditionEvaluator, RuleEvaluator, int, int, int) line: 27` |
| 49 | 0.17% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 50 | 0.17% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 51 | 0.17% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 52 | 0.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4126` |
| 53 | 0.15% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 54 | 0.15% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 55 | 0.15% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 56 | 0.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3975` |
| 57 | 0.15% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2760` |
| 58 | 0.15% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1989` |
| 59 | 0.15% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2719` |
| 60 | 0.15% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1986` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 15.79% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 12.15% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 5.23% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 4 | 4.95% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4152` |
| 5 | 4.71% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7761` |
| 6 | 4.63% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7769` |
| 7 | 4.09% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2734` |
| 8 | 2.43% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4835` |
| 9 | 2.27% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 10 | 2.23% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2616` |
| 11 | 1.80% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 12 | 1.21% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 13 | 1.14% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1528` |
| 14 | 0.92% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2733` |
| 15 | 0.81% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 16 | 0.74% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 17 | 0.67% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2713` |
| 18 | 0.60% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 19 | 0.59% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1990` |
| 20 | 0.59% | `dev.planeta9091.centities.CeHooks.isMergableMirror(ItemEntity) line: 400` |
| 21 | 0.57% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7727` |
| 22 | 0.45% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.schedule(int, int, ChunkStatus, NewChunkHolder, List) line: 673` |
| 23 | 0.39% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 24 | 0.37% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 25 | 0.30% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7719` |
| 26 | 0.30% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 417` |
| 27 | 0.29% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 28 | 0.25% | `com.sun.crypto.provider.CounterMode.crypt(byte[], int, int, byte[], int) line: 195` |
| 29 | 0.25% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 30 | 0.22% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |

GC pauses: 0, total 0 ms

