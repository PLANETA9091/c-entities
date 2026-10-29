# JVM hotspots (JFR ExecutionSample)

Samples: 5501

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 12.80% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7826` |
| 2 | 9.23% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4168` |
| 3 | 5.40% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 4.94% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 4.45% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 6 | 3.09% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2863` |
| 7 | 2.98% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 8 | 2.94% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7773` |
| 9 | 1.73% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2765` |
| 10 | 1.35% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 11 | 1.20% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 12 | 1.13% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 13 | 0.85% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4886` |
| 14 | 0.84% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4174` |
| 15 | 0.82% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 16 | 0.78% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4205` |
| 17 | 0.73% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 18 | 0.67% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 19 | 0.67% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 20 | 0.60% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4888` |
| 21 | 0.58% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7787` |
| 22 | 0.53% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7469` |
| 23 | 0.53% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4213` |
| 24 | 0.53% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2816` |
| 25 | 0.51% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 272` |
| 26 | 0.49% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2018` |
| 27 | 0.45% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5076` |
| 28 | 0.38% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 29 | 0.31% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 30 | 0.27% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7777` |
| 31 | 0.25% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 32 | 0.25% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 33 | 0.24% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 34 | 0.24% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 35 | 0.24% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 36 | 0.24% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4028` |
| 37 | 0.24% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7467` |
| 38 | 0.22% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 39 | 0.22% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2812` |
| 40 | 0.22% | `java.lang.Double.isFinite(double) line: 1023` |
| 41 | 0.20% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 42 | 0.20% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 43 | 0.20% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 44 | 0.20% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 45 | 0.20% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2002` |
| 46 | 0.20% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 47 | 0.20% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2663` |
| 48 | 0.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4237` |
| 49 | 0.20% | `net.minecraft.world.level.Level.isClientSide() line: 924` |
| 50 | 0.20% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2777` |
| 51 | 0.18% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 52 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4115` |
| 53 | 0.18% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1999` |
| 54 | 0.16% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1475` |
| 55 | 0.16% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7781` |
| 56 | 0.16% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2758` |
| 57 | 0.16% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 58 | 0.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4029` |
| 59 | 0.15% | `net.minecraft.world.level.levelgen.Heightmap.primeHeightmaps(ChunkAccess, Set) line: 76` |
| 60 | 0.15% | `ca.spottedleaf.moonrise.patches.starlight.light.SWMRNibbleArray.getUpdating(int) line: 393` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 14.29% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 12.89% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 5.53% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 4 | 4.85% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4205` |
| 5 | 4.16% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7822` |
| 6 | 4.16% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7814` |
| 7 | 3.40% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2786` |
| 8 | 3.18% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2668` |
| 9 | 2.94% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4888` |
| 10 | 1.95% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 11 | 1.64% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 12 | 1.36% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 13 | 1.11% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2765` |
| 14 | 1.02% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1541` |
| 15 | 0.98% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2785` |
| 16 | 0.85% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 17 | 0.82% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 18 | 0.78% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 19 | 0.56% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2003` |
| 20 | 0.51% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 417` |
| 21 | 0.51% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7780` |
| 22 | 0.49% | `dev.planeta9091.centities.CeHooks.isMergableMirror(ItemEntity) line: 400` |
| 23 | 0.42% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 24 | 0.38% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 25 | 0.27% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 26 | 0.27% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 27 | 0.25% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 28 | 0.24% | `net.minecraft.world.level.chunk.status.ChunkStatusTasks.lambda$buildTerrain$0(ChunkAccess) line: 130` |
| 29 | 0.24% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7772` |
| 30 | 0.22% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |

GC pauses: 0, total 0 ms

