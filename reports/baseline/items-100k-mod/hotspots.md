# JVM hotspots (JFR ExecutionSample)

Samples: 4944

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 14.12% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4944` |
| 2 | 9.22% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8699` |
| 3 | 5.83% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8646` |
| 4 | 5.20% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 5 | 5.04% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3645` |
| 6 | 4.37% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.59% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 8 | 2.43% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 389` |
| 9 | 2.27% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3537` |
| 10 | 2.04% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 11 | 1.72% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2601` |
| 12 | 1.60% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3598` |
| 13 | 1.40% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5735` |
| 14 | 1.31% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5737` |
| 15 | 1.29% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5008` |
| 16 | 1.15% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 17 | 1.13% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 18 | 1.07% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 19 | 0.95% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 20 | 0.83% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5045` |
| 21 | 0.75% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 22 | 0.67% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 23 | 0.59% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 8340` |
| 24 | 0.57% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8660` |
| 25 | 0.51% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8650` |
| 26 | 0.40% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 27 | 0.38% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 28 | 0.38% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5086` |
| 29 | 0.34% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2613` |
| 30 | 0.28% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 31 | 0.26% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 32 | 0.26% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8657` |
| 33 | 0.24% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5014` |
| 34 | 0.24% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 35 | 0.24% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 36 | 0.22% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 37 | 0.22% | `dev.planeta9091.centities.CeHooks.probeDeltaCommon(Entity, double, double, double) line: 1121` |
| 38 | 0.20% | `net.minecraft.world.level.chunk.LevelChunkSection.updateBlockCallback(int, int, int, BlockState, BlockState) line: 123` |
| 39 | 0.20% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 40 | 0.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5053` |
| 41 | 0.18% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 42 | 0.18% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 277` |
| 43 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5019` |
| 44 | 0.18% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 45 | 0.16% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 46 | 0.16% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 47 | 0.16% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 48 | 0.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5007` |
| 49 | 0.14% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 50 | 0.14% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 51 | 0.14% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 8342` |
| 52 | 0.14% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4984` |
| 53 | 0.14% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4957` |
| 54 | 0.14% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4826` |
| 55 | 0.14% | `net.minecraft.world.level.Level.isClientSide() line: 924` |
| 56 | 0.14% | `net.minecraft.world.phys.Vec3.horizontalDistanceSqr() line: 199` |
| 57 | 0.14% | `jdk.internal.reflect.MethodHandleBooleanFieldAccessorImpl.setBoolean(Object, boolean) line: 132` |
| 58 | 0.12% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 59 | 0.12% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2592` |
| 60 | 0.12% | `java.lang.Integer.valueOf(int) line: 1006` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 18.99% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 9.34% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 7.36% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8695` |
| 4 | 5.20% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 5 | 5.18% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3558` |
| 6 | 3.03% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5737` |
| 7 | 2.83% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 8 | 2.73% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3440` |
| 9 | 2.43% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 1988` |
| 10 | 2.08% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8687` |
| 11 | 2.06% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 12 | 1.88% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3557` |
| 13 | 1.72% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2597` |
| 14 | 1.58% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8653` |
| 15 | 1.48% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5045` |
| 16 | 1.15% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 17 | 1.01% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2614` |
| 18 | 0.99% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3537` |
| 19 | 0.97% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 20 | 0.75% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 21 | 0.73% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 22 | 0.71% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2173` |
| 23 | 0.65% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8645` |
| 24 | 0.40% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 1025` |
| 25 | 0.38% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8646` |
| 26 | 0.34% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 27 | 0.24% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 28 | 0.24% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3538` |
| 29 | 0.22% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 30 | 0.22% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |

GC pauses: 0, total 0 ms

