# JVM hotspots (JFR ExecutionSample)

Samples: 5048

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 11.83% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8933` |
| 2 | 6.89% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 3 | 5.86% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8986` |
| 4 | 4.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5273` |
| 5 | 3.59% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 3.17% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6019` |
| 7 | 2.34% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3739` |
| 8 | 2.04% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3866` |
| 9 | 1.76% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 10 | 1.55% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3819` |
| 11 | 1.53% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2697` |
| 12 | 1.49% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 13 | 1.31% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5279` |
| 14 | 1.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 15 | 1.27% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 16 | 1.15% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 17 | 1.13% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 18 | 1.09% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 19 | 1.01% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 20 | 0.95% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 21 | 0.91% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2735` |
| 22 | 0.85% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5310` |
| 23 | 0.81% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 24 | 0.59% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 8627` |
| 25 | 0.44% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 8629` |
| 26 | 0.40% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6009` |
| 27 | 0.38% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8937` |
| 28 | 0.38% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 277` |
| 29 | 0.36% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5351` |
| 30 | 0.36% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3732` |
| 31 | 0.32% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2694` |
| 32 | 0.32% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5092` |
| 33 | 0.30% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 34 | 0.30% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 35 | 0.28% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 36 | 0.28% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 37 | 0.28% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3863` |
| 38 | 0.28% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3814` |
| 39 | 0.26% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 40 | 0.26% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 41 | 0.26% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 42 | 0.26% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 43 | 0.24% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 44 | 0.24% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2709` |
| 45 | 0.24% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3772` |
| 46 | 0.22% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 47 | 0.22% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 48 | 0.22% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3751` |
| 49 | 0.22% | `net.minecraft.world.phys.Vec3.horizontalDistanceSqr() line: 199` |
| 50 | 0.20% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2776` |
| 51 | 0.20% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 52 | 0.20% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2688` |
| 53 | 0.20% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 54 | 0.20% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3636` |
| 55 | 0.18% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 56 | 0.18% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2700` |
| 57 | 0.18% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3815` |
| 58 | 0.16% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 59 | 0.16% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupBlock() line: 824` |
| 60 | 0.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5179` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 12.48% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8982` |
| 2 | 9.33% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 3 | 7.03% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5310` |
| 4 | 6.06% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 4.34% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2079` |
| 6 | 4.20% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6019` |
| 7 | 3.68% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 8 | 2.54% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3772` |
| 9 | 2.24% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3770` |
| 10 | 1.78% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3641` |
| 11 | 1.70% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 12 | 1.60% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8940` |
| 13 | 1.45% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8974` |
| 14 | 1.37% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3739` |
| 15 | 1.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 16 | 1.29% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 17 | 1.15% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 18 | 1.09% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 19 | 1.07% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 20 | 1.01% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3740` |
| 21 | 0.77% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8933` |
| 22 | 0.71% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2710` |
| 23 | 0.46% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8932` |
| 24 | 0.44% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 25 | 0.38% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 26 | 0.36% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 711` |
| 27 | 0.30% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 28 | 0.30% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 29 | 0.28% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 30 | 0.28% | `it.unimi.dsi.fastutil.objects.ReferenceCollections$UnmodifiableCollection.contains(Object) line: 344` |

GC pauses: 0, total 0 ms

