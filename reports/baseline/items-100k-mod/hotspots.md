# JVM hotspots (JFR ExecutionSample)

Samples: 12444

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.68% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 5.66% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 4.74% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 4 | 4.66% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 5 | 3.61% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 6 | 3.17% | `dev.planeta9091.centities.CeHooks.arenaRef(long) line: 2014` |
| 7 | 2.43% | `java.util.HashMap.getNode(Object) line: 585` |
| 8 | 1.88% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1010` |
| 9 | 1.74% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 10 | 1.58% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 11 | 1.43% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 12 | 1.34% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 13 | 1.29% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 1821` |
| 14 | 1.28% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 15 | 1.19% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 131` |
| 16 | 1.18% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1302` |
| 17 | 1.11% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 18 | 1.07% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.getDouble(Object) line: 431` |
| 19 | 1.06% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 20 | 1.05% | `net.minecraft.world.entity.Entity.isInLava() line: 2081` |
| 21 | 0.88% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 22 | 0.81% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 23 | 0.80% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1169` |
| 24 | 0.73% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 25 | 0.72% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 26 | 0.72% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 27 | 0.66% | `java.lang.invoke.MethodHandles$Lookup.<init>(Class, Class, int) line: 1644` |
| 28 | 0.64% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 29 | 0.64% | `net.minecraft.server.level.ServerLevel.lambda$tick$4(TickRateManager, ProfilerFiller, Entity) line: 795` |
| 30 | 0.61% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 1834` |
| 31 | 0.60% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 32 | 0.60% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |
| 33 | 0.48% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 34 | 0.47% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity) line: 576` |
| 35 | 0.46% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 134` |
| 36 | 0.44% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 37 | 0.43% | `net.minecraft.world.entity.Entity.getType() line: 605` |
| 38 | 0.43% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 39 | 0.41% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 40 | 0.38% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1661` |
| 41 | 0.37% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 42 | 0.36% | `net.minecraft.world.phys.Vec3.lengthSqr() line: 183` |
| 43 | 0.36% | `ca.spottedleaf.moonrise.common.misc.NearbyPlayers$TrackedChunk.getPlayers(NearbyPlayers$NearbyMapType) line: 181` |
| 44 | 0.35% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1213` |
| 45 | 0.35% | `java.lang.ThreadLocal.get() line: 172` |
| 46 | 0.35% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 47 | 0.34% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 48 | 0.33% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1421` |
| 49 | 0.33% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 1829` |
| 50 | 0.32% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 51 | 0.31% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |
| 52 | 0.31% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 53 | 0.31% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1125` |
| 54 | 0.31% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.getDouble(Object) line: 430` |
| 55 | 0.30% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 56 | 0.29% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |
| 57 | 0.28% | `net.minecraft.world.item.ItemStack.isEmpty() line: 313` |
| 58 | 0.28% | `dev.planeta9091.centities.CeHooks.appendItem(ItemEntity, int) line: 1771` |
| 59 | 0.28% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 60 | 0.27% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1008` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.68% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 2 | 5.92% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 3 | 5.66% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 5.34% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 4.64% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 6 | 3.46% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 7 | 3.46% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 8 | 2.43% | `java.util.HashMap.containsKey(Object) line: 602` |
| 9 | 2.22% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 1915` |
| 10 | 2.18% | `net.minecraft.server.level.ChunkMap.tick() line: 1025` |
| 11 | 2.03% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 12 | 1.65% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 13 | 1.58% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 14 | 1.37% | `net.minecraft.world.entity.Entity.isInLava() line: 2081` |
| 15 | 1.35% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 16 | 1.13% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 17 | 1.07% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1423` |
| 18 | 1.00% | `dev.planeta9091.centities.CeHooks.itemSkip(ItemEntity) line: 1706` |
| 19 | 1.00% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 1817` |
| 20 | 0.92% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 21 | 0.91% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 188` |
| 22 | 0.76% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity)` |
| 23 | 0.76% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 24 | 0.75% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 25 | 0.72% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 26 | 0.66% | `java.lang.invoke.MethodHandles$Lookup.<init>(Class) line: 1637` |
| 27 | 0.63% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 28 | 0.63% | `net.minecraft.world.entity.Entity.collide(Vec3) line: 1516` |
| 29 | 0.60% | `net.minecraft.world.level.Level.isOutsideBuildHeight(BlockPos) line: 776` |
| 30 | 0.59% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |

GC pauses: 0, total 0 ms

