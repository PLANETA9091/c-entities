# JVM hotspots (JFR ExecutionSample)

Samples: 5522

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 11.79% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10280` |
| 2 | 7.53% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 3 | 5.60% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6021` |
| 4 | 5.54% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 5 | 4.17% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4469` |
| 6 | 2.57% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.05% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 8 | 1.99% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 9 | 1.59% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4337` |
| 10 | 1.58% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10271` |
| 11 | 1.48% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6092` |
| 12 | 1.41% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 13 | 1.07% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5956` |
| 14 | 1.00% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10275` |
| 15 | 0.85% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 16 | 0.83% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 17 | 0.76% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3100` |
| 18 | 0.72% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6794` |
| 19 | 0.72% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5965` |
| 20 | 0.54% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6078` |
| 21 | 0.51% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3138` |
| 22 | 0.45% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 23 | 0.45% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4417` |
| 24 | 0.38% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 392` |
| 25 | 0.38% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 277` |
| 26 | 0.36% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 27 | 0.33% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3090` |
| 28 | 0.33% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6784` |
| 29 | 0.31% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3091` |
| 30 | 0.31% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 31 | 0.27% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 32 | 0.27% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 33 | 0.27% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5767` |
| 34 | 0.25% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 35 | 0.25% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 36 | 0.25% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 37 | 0.25% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 38 | 0.25% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 39 | 0.24% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3112` |
| 40 | 0.22% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 41 | 0.22% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 42 | 0.22% | `dev.planeta9091.centities.CeHooks.vtabHash(int) line: 376` |
| 43 | 0.22% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4241` |
| 44 | 0.20% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 45 | 0.20% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(int, int) line: 929` |
| 46 | 0.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6074` |
| 47 | 0.20% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4223` |
| 48 | 0.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5906` |
| 49 | 0.18% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 50 | 0.18% | `java.util.Arrays.fill(long[], long) line: 3144` |
| 51 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5860` |
| 52 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5766` |
| 53 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6125` |
| 54 | 0.18% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4330` |
| 55 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5988` |
| 56 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5907` |
| 57 | 0.16% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 58 | 0.16% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 59 | 0.16% | `java.lang.Class.cast(Object) line: 3491` |
| 60 | 0.16% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3097` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 14.63% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10463` |
| 2 | 14.61% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 3 | 7.55% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 4 | 4.69% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5956` |
| 5 | 4.51% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4370` |
| 6 | 2.90% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2482` |
| 7 | 2.54% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4234` |
| 8 | 2.34% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6794` |
| 9 | 2.06% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 10 | 1.99% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4689` |
| 11 | 1.87% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 12 | 1.45% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6078` |
| 13 | 1.32% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 14 | 0.87% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4368` |
| 15 | 0.85% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 16 | 0.76% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4337` |
| 17 | 0.54% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 425` |
| 18 | 0.51% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 19 | 0.43% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 20 | 0.42% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 21 | 0.38% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 711` |
| 22 | 0.31% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4338` |
| 23 | 0.27% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 24 | 0.27% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 30` |
| 25 | 0.27% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |
| 26 | 0.25% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6096` |
| 27 | 0.24% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 28 | 0.24% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.schedule(int, int, ChunkStatus, NewChunkHolder, List) line: 673` |
| 29 | 0.24% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10459` |
| 30 | 0.22% | `com.sun.crypto.provider.CounterMode.crypt(byte[], int, int, byte[], int) line: 195` |

GC pauses: 0, total 0 ms

