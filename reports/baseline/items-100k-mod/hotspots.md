# JVM hotspots (JFR ExecutionSample)

Samples: 5351

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 10.63% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10280` |
| 2 | 7.19% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 3 | 3.49% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6794` |
| 4 | 3.29% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.73% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4469` |
| 6 | 2.64% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10271` |
| 7 | 2.60% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6021` |
| 8 | 2.13% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4337` |
| 9 | 2.11% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 10 | 2.00% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 11 | 1.85% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 12 | 1.68% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3100` |
| 13 | 1.40% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5965` |
| 14 | 1.27% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 15 | 1.27% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |
| 16 | 1.23% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4417` |
| 17 | 1.16% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3138` |
| 18 | 1.14% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6125` |
| 19 | 1.10% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 20 | 1.05% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6092` |
| 21 | 0.88% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 22 | 0.82% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 23 | 0.78% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10275` |
| 24 | 0.60% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5956` |
| 25 | 0.54% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6078` |
| 26 | 0.50% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6784` |
| 27 | 0.49% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5767` |
| 28 | 0.47% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 29 | 0.39% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3112` |
| 30 | 0.39% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5988` |
| 31 | 0.37% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 277` |
| 32 | 0.36% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 33 | 0.36% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4330` |
| 34 | 0.34% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 35 | 0.32% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 36 | 0.32% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 37 | 0.30% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 38 | 0.30% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 39 | 0.30% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 40 | 0.30% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 41 | 0.30% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5766` |
| 42 | 0.30% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4412` |
| 43 | 0.30% | `net.minecraft.world.phys.Vec3.horizontalDistanceSqr() line: 199` |
| 44 | 0.26% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 45 | 0.26% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 46 | 0.26% | `java.util.Arrays.fill(long[], long) line: 3144` |
| 47 | 0.26% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3091` |
| 48 | 0.26% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5903` |
| 49 | 0.26% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 4419` |
| 50 | 0.24% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 51 | 0.24% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 52 | 0.24% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6019` |
| 53 | 0.24% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10310` |
| 54 | 0.22% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupBlock() line: 824` |
| 55 | 0.22% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4349` |
| 56 | 0.21% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 57 | 0.21% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3090` |
| 58 | 0.21% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3179` |
| 59 | 0.21% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4223` |
| 60 | 0.19% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 14.56% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10463` |
| 2 | 12.65% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 3 | 5.76% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5956` |
| 4 | 4.65% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2482` |
| 5 | 4.09% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 6 | 3.40% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6794` |
| 7 | 3.23% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4234` |
| 8 | 3.05% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4370` |
| 9 | 2.11% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4689` |
| 10 | 2.07% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4368` |
| 11 | 2.00% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 12 | 1.89% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 13 | 1.83% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 14 | 1.31% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4337` |
| 15 | 1.29% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6078` |
| 16 | 1.27% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 17 | 0.92% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 18 | 0.82% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4338` |
| 19 | 0.47% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6096` |
| 20 | 0.43% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 21 | 0.43% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 22 | 0.37% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 23 | 0.37% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 711` |
| 24 | 0.36% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 25 | 0.34% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 26 | 0.30% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 27 | 0.30% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |
| 28 | 0.30% | `dev.planeta9091.centities.CeHooks.itemImmunities(ItemEntity) line: 4445` |
| 29 | 0.28% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.idFor(Object, PaletteResize) line: 126` |
| 30 | 0.28% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5738` |

GC pauses: 0, total 0 ms

