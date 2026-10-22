# JVM hotspots (JFR ExecutionSample)

Samples: 47703

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 46.76% | `java.util.HashMap.getNode(Object) line: 585` |
| 2 | 1.87% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 1.61% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 1.43% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 5 | 1.22% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 6 | 0.93% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 7 | 0.85% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 0.79% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 9 | 0.74% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 10 | 0.74% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 11 | 0.60% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 0.60% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 13 | 0.60% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1187` |
| 14 | 0.57% | `net.minecraft.world.entity.Entity.lambda$checkInsideBlocks$2(LongSet, Vec3, Vec3, InsideBlockEffectApplier$StepBasedCollector, BlockPos, int) line: 1693` |
| 15 | 0.56% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 16 | 0.54% | `dev.planeta9091.centities.CeHooks.cellOfBox(AABB) line: 701` |
| 17 | 0.53% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 18 | 0.51% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 19 | 0.48% | `java.util.EnumMap.get(Object) line: 247` |
| 20 | 0.48% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 21 | 0.46% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 22 | 0.45% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 23 | 0.45% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 24 | 0.44% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 25 | 0.42% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 26 | 0.40% | `java.lang.Enum.ordinal() line: 128` |
| 27 | 0.39% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 28 | 0.38% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 29 | 0.36% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 30 | 0.34% | `net.minecraft.core.Vec3i.getZ() line: 71` |
| 31 | 0.34% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1414` |
| 32 | 0.34% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1313` |
| 33 | 0.33% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 34 | 0.33% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3459` |
| 35 | 0.33% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 981` |
| 36 | 0.32% | `io.netty.channel.ChannelOutboundBuffer.addFlush() line: 159` |
| 37 | 0.30% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 38 | 0.28% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 39 | 0.28% | `net.minecraft.world.level.chunk.LevelChunk.getListenerRegistry(int) line: 262` |
| 40 | 0.27% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 134` |
| 41 | 0.27% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 42 | 0.26% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 43 | 0.26% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 44 | 0.26% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 45 | 0.25% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.size() line: 422` |
| 46 | 0.25% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 444` |
| 47 | 0.25% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 437` |
| 48 | 0.24% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1198` |
| 49 | 0.24% | `net.minecraft.world.level.pathfinder.Path.isDone() line: 38` |
| 50 | 0.23% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 51 | 0.23% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 52 | 0.22% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 70` |
| 53 | 0.22% | `net.minecraft.world.entity.Entity.getFirstPassenger() line: 4495` |
| 54 | 0.22% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 55 | 0.21% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 56 | 0.21% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1010` |
| 57 | 0.20% | `net.minecraft.world.entity.Entity.getId() line: 618` |
| 58 | 0.19% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightInterface.getSkyLightValue(BlockPos, ChunkAccess) line: 228` |
| 59 | 0.19% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1429` |
| 60 | 0.18% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 46.77% | `java.util.HashMap.containsKey(Object) line: 602` |
| 2 | 1.87% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 1.85% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 1.51% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 5 | 1.43% | `java.util.stream.AbstractPipeline.wrapAndCopyInto(Sink, Spliterator) line: 499` |
| 6 | 1.22% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 7 | 1.20% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 918` |
| 8 | 0.93% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 9 | 0.88% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 10 | 0.59% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 11 | 0.59% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 12 | 0.59% | `net.minecraft.world.level.BlockGetter.forEachBlockIntersectedBetween(Vec3, Vec3, AABB, BlockGetter$BlockStepVisitor) line: 219` |
| 13 | 0.57% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 235` |
| 14 | 0.56% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 15 | 0.55% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1196` |
| 16 | 0.52% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 17 | 0.51% | `ca.spottedleaf.moonrise.common.util.TickThread.ensureTickThread(String) line: 27` |
| 18 | 0.50% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 19 | 0.46% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 20 | 0.45% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 21 | 0.45% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 22 | 0.45% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 23 | 0.45% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 78` |
| 24 | 0.44% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 25 | 0.42% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2274` |
| 26 | 0.39% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 27 | 0.39% | `net.minecraft.world.entity.Entity.collide(Vec3) line: 1516` |
| 28 | 0.38% | `net.minecraft.world.level.gameevent.GameEventDispatcher.post(Holder, Vec3, GameEvent$Context) line: 54` |
| 29 | 0.38% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 212` |
| 30 | 0.37% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightInterface.getRawBrightness(BlockPos, int) line: 322` |

GC pauses: 0, total 0 ms

