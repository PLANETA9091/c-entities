# JVM hotspots (JFR ExecutionSample)

Samples: 4453

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 9.27% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7773` |
| 2 | 7.57% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4115` |
| 3 | 7.52% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 4 | 6.85% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7720` |
| 5 | 5.43% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2811` |
| 6 | 4.13% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 3.48% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2713` |
| 8 | 3.01% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2764` |
| 9 | 2.65% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 10 | 2.49% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 11 | 1.95% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 12 | 1.57% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4835` |
| 13 | 1.44% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4121` |
| 14 | 1.19% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 15 | 1.17% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 16 | 0.88% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 17 | 0.83% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 18 | 0.81% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4833` |
| 19 | 0.74% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4152` |
| 20 | 0.72% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 21 | 0.61% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2005` |
| 22 | 0.61% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7414` |
| 23 | 0.58% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 24 | 0.56% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7416` |
| 25 | 0.56% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4160` |
| 26 | 0.54% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7724` |
| 27 | 0.52% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 28 | 0.43% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7734` |
| 29 | 0.34% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity) line: 761` |
| 30 | 0.31% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1986` |
| 31 | 0.31% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 32 | 0.31% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 272` |
| 33 | 0.31% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1678` |
| 34 | 0.29% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 35 | 0.29% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 36 | 0.29% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2760` |
| 37 | 0.29% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 38 | 0.22% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 39 | 0.20% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 40 | 0.20% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 41 | 0.18% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 42 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4114` |
| 43 | 0.18% | `java.lang.Double.isFinite(double) line: 1023` |
| 44 | 0.18% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2618` |
| 45 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4184` |
| 46 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3976` |
| 47 | 0.18% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2766` |
| 48 | 0.16% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 49 | 0.16% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 50 | 0.16% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 51 | 0.16% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 52 | 0.16% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 53 | 0.16% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 54 | 0.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3975` |
| 55 | 0.16% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7731` |
| 56 | 0.16% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2761` |
| 57 | 0.13% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 58 | 0.13% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 874` |
| 59 | 0.13% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 60 | 0.13% | `jdk.internal.reflect.MethodHandleBooleanFieldAccessorImpl.setBoolean(Object, boolean) line: 132` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 12.40% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 9.45% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 8.15% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7769` |
| 4 | 7.81% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4152` |
| 5 | 5.64% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2734` |
| 6 | 4.22% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4835` |
| 7 | 3.68% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2733` |
| 8 | 2.76% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2616` |
| 9 | 2.52% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 10 | 2.49% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 11 | 2.07% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7761` |
| 12 | 1.77% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2713` |
| 13 | 1.53% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 14 | 1.28% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7727` |
| 15 | 1.24% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 16 | 1.19% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1528` |
| 17 | 1.19% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 18 | 1.17% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 19 | 0.83% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2714` |
| 20 | 0.74% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7719` |
| 21 | 0.72% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 22 | 0.47% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7720` |
| 23 | 0.40% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 430` |
| 24 | 0.36% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 25 | 0.36% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1990` |
| 26 | 0.34% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 27 | 0.31% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 28 | 0.31% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 1025` |
| 29 | 0.31% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 417` |
| 30 | 0.27% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |

GC pauses: 0, total 0 ms

