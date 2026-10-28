# JVM hotspots (JFR ExecutionSample)

Samples: 4991

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 13.70% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7773` |
| 2 | 8.44% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4115` |
| 3 | 5.75% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 4 | 4.49% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7720` |
| 5 | 4.15% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 3.81% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 7 | 3.73% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2811` |
| 8 | 3.07% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 9 | 2.70% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2713` |
| 10 | 1.88% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4833` |
| 11 | 1.40% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 12 | 1.12% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 13 | 1.08% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4121` |
| 14 | 0.96% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 15 | 0.88% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2764` |
| 16 | 0.78% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4152` |
| 17 | 0.74% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 18 | 0.66% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 19 | 0.66% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7724` |
| 20 | 0.62% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 21 | 0.60% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 272` |
| 22 | 0.52% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4835` |
| 23 | 0.50% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 24 | 0.50% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 25 | 0.42% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7414` |
| 26 | 0.40% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2005` |
| 27 | 0.40% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4160` |
| 28 | 0.38% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7416` |
| 29 | 0.34% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 30 | 0.30% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 31 | 0.28% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 32 | 0.28% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 33 | 0.28% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 34 | 0.28% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 35 | 0.28% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 36 | 0.26% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2760` |
| 37 | 0.22% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 38 | 0.22% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5187` |
| 39 | 0.22% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7734` |
| 40 | 0.22% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4065` |
| 41 | 0.20% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 42 | 0.20% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 43 | 0.20% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2725` |
| 44 | 0.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3976` |
| 45 | 0.18% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 46 | 0.18% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 47 | 0.18% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 48 | 0.18% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5195` |
| 49 | 0.18% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1989` |
| 50 | 0.18% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7703` |
| 51 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4062` |
| 52 | 0.18% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2766` |
| 53 | 0.18% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2706` |
| 54 | 0.18% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2618` |
| 55 | 0.16% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 56 | 0.16% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 57 | 0.16% | `net.minecraft.server.level.WorldGenRegion.lambda$new$2(ChunkAccess, ChunkStep, GenerationChunkHolder, int, int) line: 124` |
| 58 | 0.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4063` |
| 59 | 0.16% | `net.minecraft.world.level.Level.isClientSide() line: 924` |
| 60 | 0.14% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 13.76% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 2 | 13.18% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 3 | 6.13% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4152` |
| 4 | 5.81% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7769` |
| 5 | 3.93% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2734` |
| 6 | 3.87% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4835` |
| 7 | 3.81% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 8 | 3.21% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2616` |
| 9 | 2.54% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 10 | 2.50% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7761` |
| 11 | 1.42% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2733` |
| 12 | 1.40% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 13 | 1.30% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2713` |
| 14 | 1.12% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 15 | 1.00% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 16 | 0.90% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 17 | 0.86% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1528` |
| 18 | 0.86% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7727` |
| 19 | 0.74% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 20 | 0.74% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7719` |
| 21 | 0.60% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 417` |
| 22 | 0.50% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2714` |
| 23 | 0.48% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 24 | 0.46% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1990` |
| 25 | 0.42% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 26 | 0.40% | `dev.planeta9091.centities.CeHooks.isMergableMirror(ItemEntity) line: 400` |
| 27 | 0.38% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 28 | 0.30% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 29 | 0.30% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 30 | 0.28% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 1025` |

GC pauses: 0, total 0 ms

