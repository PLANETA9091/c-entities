# JVM hotspots (JFR ExecutionSample)

Samples: 4966

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 9.79% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 2 | 9.28% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5473` |
| 3 | 6.85% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 4.79% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 3.34% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4054` |
| 6 | 2.15% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 7 | 1.71% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3927` |
| 8 | 1.47% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 9 | 1.29% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9407` |
| 10 | 1.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5478` |
| 11 | 1.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5549` |
| 12 | 1.19% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 13 | 1.15% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 14 | 1.01% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2760` |
| 15 | 0.95% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 16 | 0.93% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9404` |
| 17 | 0.91% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6251` |
| 18 | 0.85% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 19 | 0.79% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2722` |
| 20 | 0.64% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4007` |
| 21 | 0.58% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6241` |
| 22 | 0.52% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 23 | 0.40% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 277` |
| 24 | 0.36% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5290` |
| 25 | 0.34% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 26 | 0.34% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 27 | 0.34% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2734` |
| 28 | 0.28% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 29 | 0.28% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 30 | 0.28% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5534` |
| 31 | 0.26% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 32 | 0.26% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 33 | 0.26% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5289` |
| 34 | 0.24% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5530` |
| 35 | 0.24% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5378` |
| 36 | 0.24% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5422` |
| 37 | 0.24% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9444` |
| 38 | 0.24% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9412` |
| 39 | 0.22% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 40 | 0.22% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 41 | 0.22% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 42 | 0.22% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 43 | 0.22% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5420` |
| 44 | 0.22% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 45 | 0.22% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4051` |
| 46 | 0.20% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 47 | 0.20% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 48 | 0.18% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 49 | 0.18% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 50 | 0.18% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 51 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5419` |
| 52 | 0.18% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2713` |
| 53 | 0.18% | `dev.planeta9091.centities.CeHooks.vtabHash(int) line: 376` |
| 54 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5291` |
| 55 | 0.18% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3939` |
| 56 | 0.18% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 57 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5423` |
| 58 | 0.16% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 59 | 0.16% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2705` |
| 60 | 0.16% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4004` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 16.77% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 9.79% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9452` |
| 3 | 6.89% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 4 | 3.77% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3960` |
| 5 | 3.28% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2104` |
| 6 | 2.78% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3824` |
| 7 | 2.78% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9521` |
| 8 | 2.64% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6251` |
| 9 | 2.15% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 10 | 2.07% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 11 | 1.99% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9511` |
| 12 | 1.69% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 13 | 1.23% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3958` |
| 14 | 1.07% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3927` |
| 15 | 1.01% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5534` |
| 16 | 0.93% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2735` |
| 17 | 0.85% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 18 | 0.62% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 19 | 0.46% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 20 | 0.44% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 21 | 0.40% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 711` |
| 22 | 0.40% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9520` |
| 23 | 0.34% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 24 | 0.34% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.schedule(int, int, ChunkStatus, NewChunkHolder, List) line: 673` |
| 25 | 0.28% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 26 | 0.28% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 27 | 0.28% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3928` |
| 28 | 0.26% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 29 | 0.26% | `net.minecraft.util.StaticCache2D.map(StaticCache2D$MappingFunction) line: 36` |
| 30 | 0.24% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 425` |

GC pauses: 0, total 0 ms

