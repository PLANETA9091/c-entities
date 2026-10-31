# JVM hotspots (JFR ExecutionSample)

Samples: 5887

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 12.57% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8824` |
| 2 | 10.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5133` |
| 3 | 5.25% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 5.08% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 3.96% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 6 | 3.28% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3770` |
| 7 | 2.92% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8771` |
| 8 | 1.99% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 9 | 1.55% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 10 | 1.36% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5860` |
| 11 | 1.05% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 12 | 1.05% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 13 | 1.05% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3643` |
| 14 | 0.85% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8785` |
| 15 | 0.80% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2697` |
| 16 | 0.78% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 17 | 0.78% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5170` |
| 18 | 0.73% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5139` |
| 19 | 0.71% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 20 | 0.68% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2735` |
| 21 | 0.56% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3723` |
| 22 | 0.54% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5862` |
| 23 | 0.54% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 24 | 0.48% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5081` |
| 25 | 0.48% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2267` |
| 26 | 0.36% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 8467` |
| 27 | 0.36% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 8465` |
| 28 | 0.36% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 277` |
| 29 | 0.31% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 30 | 0.31% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4952` |
| 31 | 0.29% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 32 | 0.29% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 33 | 0.29% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5178` |
| 34 | 0.27% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5082` |
| 35 | 0.25% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 36 | 0.25% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2688` |
| 37 | 0.25% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 38 | 0.24% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 39 | 0.24% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3547` |
| 40 | 0.24% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8775` |
| 41 | 0.22% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 42 | 0.22% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 43 | 0.22% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2687` |
| 44 | 0.22% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 45 | 0.20% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 46 | 0.20% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 47 | 0.20% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2709` |
| 48 | 0.20% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3718` |
| 49 | 0.20% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3767` |
| 50 | 0.20% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3725` |
| 51 | 0.19% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 52 | 0.17% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 53 | 0.17% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 54 | 0.17% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 55 | 0.17% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3545` |
| 56 | 0.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4951` |
| 57 | 0.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5138` |
| 58 | 0.15% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 59 | 0.15% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 60 | 0.15% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8754` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 15.24% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 12.71% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 5.28% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 4 | 4.52% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5170` |
| 5 | 4.43% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8820` |
| 6 | 4.40% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8812` |
| 7 | 3.67% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3676` |
| 8 | 2.89% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2079` |
| 9 | 2.56% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5862` |
| 10 | 2.11% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 11 | 2.11% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3545` |
| 12 | 1.78% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 13 | 1.56% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 14 | 1.07% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3674` |
| 15 | 0.78% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 16 | 0.76% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 17 | 0.70% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2710` |
| 18 | 0.65% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 762` |
| 19 | 0.58% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 20 | 0.56% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3643` |
| 21 | 0.48% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 22 | 0.44% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8778` |
| 23 | 0.42% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 24 | 0.36% | `dev.planeta9091.centities.CeHooks.mergeScanNative(ItemEntity) line: 711` |
| 25 | 0.32% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2236` |
| 26 | 0.29% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |
| 27 | 0.27% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 28 | 0.27% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 425` |
| 29 | 0.24% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 30 | 0.24% | `net.minecraft.world.level.levelgen.NoiseBasedChunkGenerator.buildSurface(ChunkAccess, NoiseChunk, RandomState, BiomeManager, Set, MaterialRule, ServerLevel) line: 227` |

GC pauses: 0, total 0 ms

