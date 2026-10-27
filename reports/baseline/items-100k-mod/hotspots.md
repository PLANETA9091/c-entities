# JVM hotspots (JFR ExecutionSample)

Samples: 3389

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 13.72% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7462` |
| 2 | 6.34% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7409` |
| 3 | 4.99% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4522` |
| 4 | 4.84% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 5 | 4.37% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.80% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 7 | 2.21% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2381` |
| 8 | 2.12% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2335` |
| 9 | 2.04% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 10 | 1.95% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 11 | 1.59% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3813` |
| 12 | 1.24% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 13 | 1.18% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 14 | 1.03% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 15 | 1.00% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1715` |
| 16 | 0.83% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 17 | 0.68% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 18 | 0.68% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2519` |
| 19 | 0.68% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 20 | 0.59% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 21 | 0.56% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2421` |
| 22 | 0.50% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7413` |
| 23 | 0.47% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 24 | 0.44% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 25 | 0.41% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7423` |
| 26 | 0.38% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 27 | 0.38% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 28 | 0.38% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 29 | 0.35% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 30 | 0.35% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 31 | 0.35% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2410` |
| 32 | 0.32% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 33 | 0.32% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3819` |
| 34 | 0.30% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 35 | 0.27% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 36 | 0.27% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 37 | 0.27% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1699` |
| 38 | 0.27% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 39 | 0.27% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3850` |
| 40 | 0.24% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 41 | 0.24% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 42 | 0.24% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(int, int) line: 929` |
| 43 | 0.24% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 44 | 0.24% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2367` |
| 45 | 0.21% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 46 | 0.21% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2326` |
| 47 | 0.18% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 48 | 0.18% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 49 | 0.18% | `net.minecraft.world.level.chunk.ProtoChunk.moonrise$getBlock(int, int, int) line: 67` |
| 50 | 0.18% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 51 | 0.18% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 52 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3858` |
| 53 | 0.15% | `java.util.Comparator.lambda$comparingLong$6043328a$1(ToLongFunction, Object, Object) line: 515` |
| 54 | 0.15% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.schedule(int, int, ChunkStatus, NewChunkHolder, List, Priority) line: 743` |
| 55 | 0.15% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder.lambda$setGenerationTask$0(ChunkProgressionTask, ChunkStatus, List, ChunkAccess, Throwable) line: 1616` |
| 56 | 0.15% | `ca.spottedleaf.moonrise.patches.starlight.light.SWMRNibbleArray.getUpdating(int) line: 393` |
| 57 | 0.15% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.schedule(int, int, ChunkStatus, NewChunkHolder, List, Priority) line: 740` |
| 58 | 0.15% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7105` |
| 59 | 0.15% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2324` |
| 60 | 0.15% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2406` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 13.84% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 2 | 7.47% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7458` |
| 3 | 7.35% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4524` |
| 4 | 5.40% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 4.93% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2324` |
| 6 | 2.98% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 7 | 2.95% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3850` |
| 8 | 2.66% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7450` |
| 9 | 2.12% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 10 | 1.95% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 11 | 1.80% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2335` |
| 12 | 1.71% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1307` |
| 13 | 1.18% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 14 | 1.06% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7416` |
| 15 | 1.03% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 16 | 0.91% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 17 | 0.86% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2442` |
| 18 | 0.68% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 19 | 0.62% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 20 | 0.62% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7408` |
| 21 | 0.62% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1700` |
| 22 | 0.59% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |
| 23 | 0.53% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2336` |
| 24 | 0.47% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2404` |
| 25 | 0.44% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 26 | 0.41% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 27 | 0.38% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 28 | 0.38% | `net.minecraft.server.MinecraftServer.lambda$spin$0(AtomicReference) line: 299` |
| 29 | 0.35% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 1025` |
| 30 | 0.32% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.schedule(int, int, ChunkStatus, NewChunkHolder, List) line: 673` |

GC pauses: 0, total 0 ms

