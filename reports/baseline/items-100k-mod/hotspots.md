# JVM hotspots (JFR ExecutionSample)

Samples: 5875

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 12.83% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8242` |
| 2 | 10.30% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4560` |
| 3 | 6.28% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 5.28% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 4.44% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 6 | 3.03% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3255` |
| 7 | 2.79% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8189` |
| 8 | 1.97% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 9 | 1.75% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5278` |
| 10 | 1.33% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 11 | 1.00% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 12 | 0.97% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3147` |
| 13 | 0.94% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8203` |
| 14 | 0.87% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2336` |
| 15 | 0.87% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4566` |
| 16 | 0.83% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 17 | 0.71% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 18 | 0.66% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 19 | 0.60% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2317` |
| 20 | 0.53% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7885` |
| 21 | 0.53% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3208` |
| 22 | 0.49% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4597` |
| 23 | 0.48% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5280` |
| 24 | 0.48% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1968` |
| 25 | 0.46% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 26 | 0.43% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 276` |
| 27 | 0.41% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 28 | 0.39% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 29 | 0.34% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8193` |
| 30 | 0.29% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 31 | 0.29% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 32 | 0.29% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4605` |
| 33 | 0.27% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 34 | 0.26% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 35 | 0.26% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 36 | 0.26% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3140` |
| 37 | 0.24% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 38 | 0.22% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 39 | 0.22% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3210` |
| 40 | 0.20% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 41 | 0.20% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 42 | 0.20% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 43 | 0.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4421` |
| 44 | 0.19% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 45 | 0.19% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 46 | 0.19% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7883` |
| 47 | 0.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4509` |
| 48 | 0.17% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 49 | 0.17% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 50 | 0.17% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 51 | 0.17% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 52 | 0.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4508` |
| 53 | 0.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4629` |
| 54 | 0.17% | `dev.planeta9091.centities.CeHooks.probeDeltaCommon(Entity, double, double, double) line: 975` |
| 55 | 0.15% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 56 | 0.15% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(int, int) line: 929` |
| 57 | 0.15% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2312` |
| 58 | 0.15% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8197` |
| 59 | 0.15% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3205` |
| 60 | 0.15% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8172` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 14.54% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 12.99% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 6.31% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 4 | 4.89% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4597` |
| 5 | 4.51% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8238` |
| 6 | 4.43% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8230` |
| 7 | 3.30% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3168` |
| 8 | 2.49% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1809` |
| 9 | 2.45% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 10 | 2.42% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5280` |
| 11 | 2.09% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3050` |
| 12 | 1.77% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 13 | 1.33% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 14 | 1.06% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3167` |
| 15 | 0.73% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 16 | 0.71% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 17 | 0.58% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3147` |
| 18 | 0.58% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 563` |
| 19 | 0.48% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2321` |
| 20 | 0.43% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 21 | 0.43% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 22 | 0.43% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8196` |
| 23 | 0.43% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 539` |
| 24 | 0.41% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 25 | 0.34% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8188` |
| 26 | 0.32% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8189` |
| 27 | 0.29% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 28 | 0.29% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 29 | 0.26% | `java.math.BigInteger.modPow(BigInteger, BigInteger) line: 2924` |
| 30 | 0.26% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |

GC pauses: 0, total 0 ms

