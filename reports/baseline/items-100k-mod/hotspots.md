# JVM hotspots (JFR ExecutionSample)

Samples: 3888

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 8.26% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7656` |
| 2 | 6.87% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 3 | 6.48% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7603` |
| 4 | 6.38% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4007` |
| 5 | 4.84% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2703` |
| 6 | 3.29% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2656` |
| 7 | 3.27% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 3.01% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2605` |
| 9 | 3.01% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 10 | 2.11% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4718` |
| 11 | 1.98% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 12 | 1.95% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 13 | 1.80% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4013` |
| 14 | 1.29% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 15 | 1.18% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 16 | 1.00% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 17 | 0.87% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 18 | 0.82% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7297` |
| 19 | 0.80% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4044` |
| 20 | 0.69% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 21 | 0.64% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7607` |
| 22 | 0.64% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7299` |
| 23 | 0.62% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 24 | 0.54% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity) line: 653` |
| 25 | 0.54% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1897` |
| 26 | 0.51% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4716` |
| 27 | 0.49% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 28 | 0.46% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 29 | 0.44% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2652` |
| 30 | 0.41% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4052` |
| 31 | 0.36% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 32 | 0.33% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 33 | 0.31% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 34 | 0.28% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4069` |
| 35 | 0.28% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 36 | 0.26% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 37 | 0.26% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1570` |
| 38 | 0.26% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 270` |
| 39 | 0.23% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1878` |
| 40 | 0.23% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7617` |
| 41 | 0.23% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 42 | 0.21% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 43 | 0.21% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 44 | 0.21% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2598` |
| 45 | 0.21% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2658` |
| 46 | 0.18% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 47 | 0.18% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5187` |
| 48 | 0.18% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 49 | 0.18% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7611` |
| 50 | 0.18% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 51 | 0.18% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2617` |
| 52 | 0.18% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7586` |
| 53 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3954` |
| 54 | 0.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3953` |
| 55 | 0.15% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 56 | 0.15% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7614` |
| 57 | 0.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4006` |
| 58 | 0.15% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2510` |
| 59 | 0.13% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 60 | 0.13% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 447` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 11.65% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 8.38% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 7.95% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7652` |
| 4 | 7.38% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4044` |
| 5 | 5.14% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2626` |
| 6 | 4.35% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4718` |
| 7 | 3.99% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2625` |
| 8 | 3.11% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2508` |
| 9 | 2.83% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 10 | 2.03% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 11 | 1.98% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 12 | 1.77% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2605` |
| 13 | 1.44% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7644` |
| 14 | 1.34% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 15 | 1.26% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 16 | 1.18% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 17 | 1.00% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2606` |
| 18 | 0.98% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1420` |
| 19 | 0.95% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7610` |
| 20 | 0.77% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7602` |
| 21 | 0.69% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 22 | 0.59% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |
| 23 | 0.54% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 24 | 0.44% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7603` |
| 25 | 0.39% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 26 | 0.36% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 27 | 0.33% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 28 | 0.31% | `com.sun.crypto.provider.CounterMode.crypt(byte[], int, int, byte[], int) line: 195` |
| 29 | 0.31% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 322` |
| 30 | 0.26% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.idFor(Object, PaletteResize) line: 126` |

GC pauses: 0, total 0 ms

