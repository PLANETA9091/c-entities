# JVM hotspots (JFR ExecutionSample)

Samples: 6164

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 13.97% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9269` |
| 2 | 10.74% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5427` |
| 3 | 5.91% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 4.19% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 3.99% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4020` |
| 6 | 3.93% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 7 | 3.84% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9146` |
| 8 | 1.78% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 9 | 1.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 10 | 1.01% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3893` |
| 11 | 0.91% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9228` |
| 12 | 0.89% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 13 | 0.78% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 14 | 0.78% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2742` |
| 15 | 0.68% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 16 | 0.68% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 17 | 0.67% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5465` |
| 18 | 0.63% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2704` |
| 19 | 0.62% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6166` |
| 20 | 0.47% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3973` |
| 21 | 0.45% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6176` |
| 22 | 0.42% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5433` |
| 23 | 0.41% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 24 | 0.37% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 8786` |
| 25 | 0.37% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 277` |
| 26 | 0.36% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 27 | 0.34% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9153` |
| 28 | 0.34% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5081` |
| 29 | 0.31% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 30 | 0.29% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 31 | 0.29% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5475` |
| 32 | 0.29% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5246` |
| 33 | 0.28% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5332` |
| 34 | 0.28% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2695` |
| 35 | 0.28% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3905` |
| 36 | 0.26% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 37 | 0.26% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5245` |
| 38 | 0.24% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 39 | 0.24% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 8784` |
| 40 | 0.23% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 41 | 0.23% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 42 | 0.23% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 43 | 0.23% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3797` |
| 44 | 0.23% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3795` |
| 45 | 0.21% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 46 | 0.21% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 47 | 0.19% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 48 | 0.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5373` |
| 49 | 0.19% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9158` |
| 50 | 0.18% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(int, int) line: 929` |
| 51 | 0.18% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 392` |
| 52 | 0.18% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 53 | 0.18% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3790` |
| 54 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5375` |
| 55 | 0.18% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6167` |
| 56 | 0.16% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 57 | 0.16% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 58 | 0.16% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 59 | 0.16% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2701` |
| 60 | 0.16% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5192` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 15.44% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 14.39% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 5.95% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 4 | 5.71% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9265` |
| 5 | 4.36% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5465` |
| 6 | 4.32% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3926` |
| 7 | 3.24% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9257` |
| 8 | 2.87% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6176` |
| 9 | 2.69% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2086` |
| 10 | 1.91% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3795` |
| 11 | 1.67% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 12 | 1.41% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 13 | 1.38% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 14 | 0.88% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3924` |
| 15 | 0.78% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 16 | 0.63% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9157` |
| 17 | 0.62% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 18 | 0.60% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3893` |
| 19 | 0.58% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2717` |
| 20 | 0.41% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 21 | 0.39% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 711` |
| 22 | 0.32% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 23 | 0.32% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9146` |
| 24 | 0.32% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 25 | 0.31% | `net.minecraft.util.StaticCache2D.map(StaticCache2D$MappingFunction) line: 36` |
| 26 | 0.29% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 425` |
| 27 | 0.29% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3894` |
| 28 | 0.28% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9145` |
| 29 | 0.26% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 30 | 0.26% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |

GC pauses: 0, total 0 ms

