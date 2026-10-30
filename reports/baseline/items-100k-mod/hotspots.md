# JVM hotspots (JFR ExecutionSample)

Samples: 5039

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 12.05% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4761` |
| 2 | 11.69% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8516` |
| 3 | 6.03% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 4.37% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 3.43% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 374` |
| 6 | 3.39% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8463` |
| 7 | 3.10% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3462` |
| 8 | 2.34% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 9 | 2.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 10 | 2.12% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3354` |
| 11 | 1.49% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 12 | 1.23% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 13 | 1.13% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 14 | 1.01% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4825` |
| 15 | 0.99% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2474` |
| 16 | 0.93% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3415` |
| 17 | 0.89% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 18 | 0.81% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 19 | 0.79% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 20 | 0.75% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5552` |
| 21 | 0.71% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5554` |
| 22 | 0.64% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8467` |
| 23 | 0.56% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 24 | 0.52% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4862` |
| 25 | 0.42% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 26 | 0.42% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8477` |
| 27 | 0.38% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 8157` |
| 28 | 0.34% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4870` |
| 29 | 0.30% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 30 | 0.30% | `java.util.HashMap.getNode(Object) line: 577` |
| 31 | 0.28% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 32 | 0.28% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2486` |
| 33 | 0.26% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 34 | 0.26% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4731` |
| 35 | 0.24% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 36 | 0.24% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 37 | 0.24% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 38 | 0.24% | `dev.planeta9091.centities.CeHooks.probeDeltaCommon(Entity, double, double, double) line: 1097` |
| 39 | 0.24% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4831` |
| 40 | 0.22% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 41 | 0.22% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 42 | 0.22% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 43 | 0.22% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 44 | 0.22% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5187` |
| 45 | 0.20% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 46 | 0.20% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3347` |
| 47 | 0.18% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 48 | 0.18% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 49 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4644` |
| 50 | 0.18% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3417` |
| 51 | 0.16% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 52 | 0.16% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 53 | 0.16% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 54 | 0.16% | `java.lang.Integer.valueOf(int) line: 1006` |
| 55 | 0.16% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8471` |
| 56 | 0.16% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3259` |
| 57 | 0.16% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 277` |
| 58 | 0.14% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 59 | 0.14% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2465` |
| 60 | 0.14% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8474` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 16.51% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 11.89% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 6.05% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 4 | 4.84% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8512` |
| 5 | 3.39% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5554` |
| 6 | 3.31% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3375` |
| 7 | 2.78% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8504` |
| 8 | 2.46% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2470` |
| 9 | 2.38% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3257` |
| 10 | 2.32% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 11 | 1.81% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1935` |
| 12 | 1.61% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 13 | 1.45% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 14 | 1.45% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4862` |
| 15 | 1.39% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3374` |
| 16 | 1.37% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3354` |
| 17 | 1.05% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2113` |
| 18 | 0.99% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2487` |
| 19 | 0.99% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8470` |
| 20 | 0.85% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 21 | 0.81% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 22 | 0.56% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3355` |
| 23 | 0.54% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8462` |
| 24 | 0.50% | `java.util.HashMap.get(Object) line: 565` |
| 25 | 0.44% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 26 | 0.44% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 27 | 0.42% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 28 | 0.34% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 29 | 0.30% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 30 | 0.28% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |

GC pauses: 0, total 0 ms

