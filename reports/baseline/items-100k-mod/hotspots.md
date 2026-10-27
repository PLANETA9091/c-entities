# JVM hotspots (JFR ExecutionSample)

Samples: 4725

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 13.04% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7462` |
| 2 | 10.26% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4522` |
| 3 | 5.23% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 4.38% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7409` |
| 5 | 3.07% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2381` |
| 6 | 2.67% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 7 | 2.16% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 8 | 2.05% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 9 | 1.82% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3813` |
| 10 | 1.44% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2335` |
| 11 | 1.08% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 12 | 1.06% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 13 | 1.04% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 14 | 1.02% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7413` |
| 15 | 0.83% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 16 | 0.76% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2519` |
| 17 | 0.72% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 18 | 0.70% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7423` |
| 19 | 0.68% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 20 | 0.59% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 21 | 0.57% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 22 | 0.55% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1715` |
| 23 | 0.49% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 24 | 0.42% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 25 | 0.38% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 26 | 0.38% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 27 | 0.38% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2410` |
| 28 | 0.36% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 29 | 0.36% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2421` |
| 30 | 0.34% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 31 | 0.34% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 32 | 0.32% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 33 | 0.30% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 34 | 0.25% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 35 | 0.25% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 36 | 0.23% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 37 | 0.23% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 38 | 0.23% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 39 | 0.21% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 40 | 0.21% | `com.google.common.collect.RegularImmutableList.get(int) line: 80` |
| 41 | 0.21% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1475` |
| 42 | 0.21% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5187` |
| 43 | 0.21% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4524` |
| 44 | 0.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3819` |
| 45 | 0.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3850` |
| 46 | 0.21% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 47 | 0.19% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 48 | 0.19% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 49 | 0.19% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(int, int) line: 929` |
| 50 | 0.19% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1696` |
| 51 | 0.19% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2319` |
| 52 | 0.17% | `net.minecraft.world.level.chunk.ProtoChunk.moonrise$getBlock(int, int, int) line: 67` |
| 53 | 0.17% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 54 | 0.17% | `net.minecraft.world.level.Level.isClientSide() line: 924` |
| 55 | 0.17% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2326` |
| 56 | 0.15% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder.getCurrentGenStatus() line: 1322` |
| 57 | 0.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3858` |
| 58 | 0.15% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7103` |
| 59 | 0.15% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1699` |
| 60 | 0.15% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2367` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 13.21% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 2 | 10.69% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 6.94% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4524` |
| 4 | 6.56% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7458` |
| 5 | 3.51% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7450` |
| 6 | 3.22% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 7 | 2.75% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3850` |
| 8 | 2.41% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 9 | 2.33% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2324` |
| 10 | 2.03% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 11 | 1.23% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1307` |
| 12 | 1.08% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 13 | 1.06% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 14 | 0.87% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7408` |
| 15 | 0.83% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2442` |
| 16 | 0.80% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7416` |
| 17 | 0.68% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2404` |
| 18 | 0.66% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1700` |
| 19 | 0.59% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 20 | 0.57% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 21 | 0.57% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |
| 22 | 0.44% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2335` |
| 23 | 0.42% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 24 | 0.42% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 25 | 0.42% | `net.minecraft.util.StaticCache2D.map(StaticCache2D$MappingFunction) line: 36` |
| 26 | 0.36% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 27 | 0.36% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 1025` |
| 28 | 0.34% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 29 | 0.34% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.schedule(int, int, ChunkStatus, NewChunkHolder, List) line: 673` |
| 30 | 0.34% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |

GC pauses: 0, total 0 ms

