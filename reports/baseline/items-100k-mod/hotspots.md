# JVM hotspots (JFR ExecutionSample)

Samples: 5940

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 14.01% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4803` |
| 2 | 11.41% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8558` |
| 3 | 4.97% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 4.19% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 4.09% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8505` |
| 6 | 3.05% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 374` |
| 7 | 2.74% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3504` |
| 8 | 2.66% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 9 | 1.43% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4867` |
| 10 | 1.41% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 11 | 1.18% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2516` |
| 12 | 1.14% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5594` |
| 13 | 1.13% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3396` |
| 14 | 1.04% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 15 | 0.89% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 16 | 0.84% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 17 | 0.76% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 18 | 0.69% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 19 | 0.69% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 20 | 0.67% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4904` |
| 21 | 0.56% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8519` |
| 22 | 0.51% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5596` |
| 23 | 0.44% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 24 | 0.40% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3457` |
| 25 | 0.39% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4686` |
| 26 | 0.39% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 27 | 0.32% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8509` |
| 28 | 0.30% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 277` |
| 29 | 0.30% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 30 | 0.27% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 31 | 0.27% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3389` |
| 32 | 0.25% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 33 | 0.25% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 34 | 0.24% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 35 | 0.24% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 36 | 0.22% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 37 | 0.22% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 38 | 0.22% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 39 | 0.22% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4773` |
| 40 | 0.20% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 41 | 0.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4685` |
| 42 | 0.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4814` |
| 43 | 0.19% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 44 | 0.19% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(int, int) line: 929` |
| 45 | 0.19% | `java.lang.Integer.valueOf(int) line: 1006` |
| 46 | 0.19% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3459` |
| 47 | 0.17% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 48 | 0.17% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 49 | 0.17% | `java.util.HashMap.getNode(Object) line: 577` |
| 50 | 0.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4945` |
| 51 | 0.15% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 52 | 0.15% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 53 | 0.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4815` |
| 54 | 0.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4873` |
| 55 | 0.15% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 8199` |
| 56 | 0.15% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3402` |
| 57 | 0.13% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 58 | 0.13% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3300` |
| 59 | 0.13% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4912` |
| 60 | 0.13% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4817` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 19.18% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 11.53% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 5.24% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8554` |
| 4 | 5.00% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 5 | 3.65% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8546` |
| 6 | 3.00% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3417` |
| 7 | 2.79% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3299` |
| 8 | 2.53% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5596` |
| 9 | 2.14% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2512` |
| 10 | 1.90% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 1970` |
| 11 | 1.90% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 12 | 1.82% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 13 | 1.40% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 14 | 1.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4904` |
| 15 | 0.93% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2151` |
| 16 | 0.84% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2529` |
| 17 | 0.77% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3416` |
| 18 | 0.76% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 19 | 0.74% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 20 | 0.62% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3396` |
| 21 | 0.44% | `java.util.HashMap.get(Object) line: 565` |
| 22 | 0.44% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8512` |
| 23 | 0.37% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 24 | 0.35% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 25 | 0.30% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 622` |
| 26 | 0.27% | `net.minecraft.util.StaticCache2D.map(StaticCache2D$MappingFunction) line: 36` |
| 27 | 0.25% | `com.sun.crypto.provider.CounterMode.crypt(byte[], int, int, byte[], int) line: 195` |
| 28 | 0.24% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 29 | 0.24% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3397` |
| 30 | 0.24% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |

GC pauses: 0, total 0 ms

