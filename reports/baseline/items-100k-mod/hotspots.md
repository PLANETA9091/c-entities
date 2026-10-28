# JVM hotspots (JFR ExecutionSample)

Samples: 4257

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 8.97% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7629` |
| 2 | 6.95% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3980` |
| 3 | 6.74% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 4 | 6.15% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2676` |
| 5 | 5.47% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7576` |
| 6 | 3.66% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.98% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2629` |
| 8 | 2.94% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 9 | 2.77% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 10 | 2.26% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2578` |
| 11 | 1.95% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4691` |
| 12 | 1.83% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 13 | 1.67% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3986` |
| 14 | 1.25% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 15 | 1.08% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 16 | 1.03% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7270` |
| 17 | 1.01% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 18 | 0.89% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4017` |
| 19 | 0.87% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7580` |
| 20 | 0.80% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7272` |
| 21 | 0.73% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4689` |
| 22 | 0.56% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 270` |
| 23 | 0.54% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 24 | 0.54% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 25 | 0.52% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7590` |
| 26 | 0.49% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 27 | 0.47% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4025` |
| 28 | 0.47% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 29 | 0.40% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 30 | 0.40% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1870` |
| 31 | 0.33% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4042` |
| 32 | 0.31% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 33 | 0.28% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 34 | 0.28% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 35 | 0.28% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity) line: 653` |
| 36 | 0.28% | `jdk.internal.reflect.MethodHandleBooleanFieldAccessorImpl.setBoolean(Object, boolean) line: 132` |
| 37 | 0.28% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2571` |
| 38 | 0.26% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1851` |
| 39 | 0.26% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 40 | 0.26% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1543` |
| 41 | 0.23% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 42 | 0.21% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 43 | 0.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3929` |
| 44 | 0.19% | `net.minecraft.nbt.ListTag.getIdToWrite(List, int) line: 37` |
| 45 | 0.19% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1854` |
| 46 | 0.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3840` |
| 47 | 0.19% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7584` |
| 48 | 0.19% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 49 | 0.19% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7587` |
| 50 | 0.16% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 51 | 0.16% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2625` |
| 52 | 0.16% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2584` |
| 53 | 0.16% | `java.lang.Iterable.forEach(Consumer) line: 75` |
| 54 | 0.14% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 55 | 0.14% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 56 | 0.14% | `net.minecraft.world.level.levelgen.Heightmap.update(int, int, int, BlockState) line: 101` |
| 57 | 0.14% | `net.minecraft.util.SimpleBitStorage.set(int, int) line: 321` |
| 58 | 0.14% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5195` |
| 59 | 0.14% | `java.util.AbstractCollection.isEmpty() line: 91` |
| 60 | 0.14% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2590` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 12.05% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 9.11% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 7.35% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7625` |
| 4 | 6.95% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4017` |
| 5 | 6.37% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2599` |
| 6 | 3.59% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4691` |
| 7 | 3.36% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2598` |
| 8 | 2.94% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 9 | 2.84% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2481` |
| 10 | 2.82% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 11 | 2.18% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 12 | 1.86% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7617` |
| 13 | 1.76% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2578` |
| 14 | 1.34% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 15 | 1.27% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 16 | 1.20% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7583` |
| 17 | 1.13% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 18 | 0.99% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1393` |
| 19 | 0.61% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7575` |
| 20 | 0.54% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 21 | 0.52% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 309` |
| 22 | 0.49% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |
| 23 | 0.47% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2579` |
| 24 | 0.40% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 25 | 0.38% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 26 | 0.38% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7576` |
| 27 | 0.38% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 322` |
| 28 | 0.33% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 29 | 0.31% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 1025` |
| 30 | 0.31% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1855` |

GC pauses: 0, total 0 ms

