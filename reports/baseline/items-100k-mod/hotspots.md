# JVM hotspots (JFR ExecutionSample)

Samples: 5411

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.34% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 2 | 3.59% | `java.lang.invoke.MethodHandles$Lookup.<init>(Class, Class, int) line: 1644` |
| 3 | 2.98% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 4 | 2.94% | `dev.planeta9091.centities.CeHooks.arenaRef(long) line: 2008` |
| 5 | 2.64% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 6 | 2.53% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 7 | 2.27% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 8 | 2.16% | `java.util.HashMap.getNode(Object) line: 585` |
| 9 | 1.89% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 10 | 1.37% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1415` |
| 11 | 1.33% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 12 | 1.22% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1302` |
| 13 | 1.16% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 14 | 1.09% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 15 | 1.07% | `net.minecraft.server.level.ServerLevel.lambda$tick$4(TickRateManager, ProfilerFiller, Entity) line: 795` |
| 16 | 1.05% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity) line: 570` |
| 17 | 1.00% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 18 | 1.00% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 19 | 0.98% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 1917` |
| 20 | 0.87% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.getDouble(Object) line: 431` |
| 21 | 0.87% | `net.minecraft.world.level.gameevent.GameEventDispatcher.post(Holder, Vec3, GameEvent$Context) line: 40` |
| 22 | 0.85% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 23 | 0.78% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 24 | 0.76% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 25 | 0.72% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 26 | 0.72% | `net.minecraft.world.level.chunk.LevelChunk.getListenerRegistry(int) line: 260` |
| 27 | 0.68% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 28 | 0.68% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 994` |
| 29 | 0.55% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1411` |
| 30 | 0.52% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 31 | 0.52% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1215` |
| 32 | 0.52% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 33 | 0.52% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1410` |
| 34 | 0.50% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |
| 35 | 0.48% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 36 | 0.48% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 37 | 0.48% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |
| 38 | 0.46% | `net.minecraft.world.level.chunk.ImposterProtoChunk.getNoiseBiome(int, int, int) line: 175` |
| 39 | 0.46% | `ca.spottedleaf.moonrise.patches.starlight.light.SWMRNibbleArray.set(int, int) line: 422` |
| 40 | 0.46% | `java.util.concurrent.atomic.AtomicInteger.get() line: 90` |
| 41 | 0.46% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1169` |
| 42 | 0.44% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 43 | 0.43% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 44 | 0.41% | `net.minecraft.server.level.ServerChunkCache.getChunkAtIfLoadedImmediately(int, int) line: 285` |
| 45 | 0.39% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 46 | 0.39% | `net.minecraft.world.item.ItemStack.isEmpty() line: 313` |
| 47 | 0.39% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 48 | 0.37% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 49 | 0.37% | `net.minecraft.world.level.gameevent.EuclideanGameEventListenerRegistry.visitInRangeListeners(Holder, Vec3, GameEvent$Context, GameEventListenerRegistry$ListenerVisitor) line: 83` |
| 50 | 0.35% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1404` |
| 51 | 0.35% | `net.minecraft.world.entity.Entity.isInLava() line: 2081` |
| 52 | 0.33% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4881` |
| 53 | 0.33% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1414` |
| 54 | 0.33% | `net.minecraft.world.level.gameevent.GameEventDispatcher.post(Holder, Vec3, GameEvent$Context) line: 54` |
| 55 | 0.31% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 56 | 0.31% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1423` |
| 57 | 0.30% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 82` |
| 58 | 0.30% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 59 | 0.30% | `net.minecraft.world.level.chunk.LevelChunk.getListenerRegistry(int) line: 262` |
| 60 | 0.30% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.25% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.29% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1417` |
| 3 | 3.88% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 4 | 3.59% | `java.lang.invoke.MethodHandles$Lookup.<init>(Class) line: 1637` |
| 5 | 2.85% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity)` |
| 6 | 2.81% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 1909` |
| 7 | 2.55% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 8 | 2.53% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 9 | 2.49% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 10 | 2.46% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 11 | 2.13% | `java.util.HashMap.containsKey(Object) line: 602` |
| 12 | 1.90% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 13 | 1.79% | `net.minecraft.world.level.gameevent.GameEventDispatcher.post(Holder, Vec3, GameEvent$Context) line: 54` |
| 14 | 1.35% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 15 | 1.33% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 16 | 1.26% | `net.minecraft.server.level.ServerLevel.gameEvent(Holder, Vec3, GameEvent$Context) line: 1731` |
| 17 | 1.24% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 18 | 1.16% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 19 | 1.15% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 20 | 1.00% | `net.minecraft.world.entity.Entity.isInLava() line: 2081` |
| 21 | 0.92% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 22 | 0.81% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 23 | 0.76% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 1943` |
| 24 | 0.76% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1161` |
| 25 | 0.74% | `dev.planeta9091.centities.CeHooks.getEntities(Level, Entity, AABB, Predicate) line: 454` |
| 26 | 0.72% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 27 | 0.70% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 28 | 0.68% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 29 | 0.67% | `net.minecraft.world.entity.Entity.getOnPosLegacy() line: 1402` |
| 30 | 0.65% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |

GC pauses: 0, total 0 ms

