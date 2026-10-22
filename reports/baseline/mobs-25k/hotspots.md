# JVM hotspots (JFR ExecutionSample)

Samples: 89732

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 48.76% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 9.15% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 1.79% | `java.util.HashMap.getNode(Object) line: 585` |
| 4 | 1.37% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 5 | 1.16% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 1.02% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 7 | 0.62% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 8 | 0.61% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 9 | 0.58% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 10 | 0.57% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 11 | 0.57% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 12 | 0.50% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 13 | 0.49% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 14 | 0.39% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 15 | 0.36% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 16 | 0.35% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 17 | 0.33% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 18 | 0.32% | `java.util.EnumMap.get(Object) line: 247` |
| 19 | 0.32% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 20 | 0.32% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 21 | 0.31% | `java.lang.Enum.ordinal() line: 128` |
| 22 | 0.30% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 23 | 0.29% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 24 | 0.29% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 25 | 0.28% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 26 | 0.27% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.size() line: 422` |
| 27 | 0.26% | `java.lang.ThreadLocal.get() line: 172` |
| 28 | 0.25% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 29 | 0.25% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3690` |
| 30 | 0.25% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 31 | 0.24% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1414` |
| 32 | 0.24% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 33 | 0.24% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 34 | 0.23% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 35 | 0.23% | `net.minecraft.core.Vec3i.getZ() line: 71` |
| 36 | 0.23% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4882` |
| 37 | 0.22% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 38 | 0.21% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 444` |
| 39 | 0.21% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 40 | 0.21% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 41 | 0.21% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 42 | 0.21% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 43 | 0.20% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 44 | 0.20% | `io.netty.channel.ChannelOutboundBuffer.addFlush() line: 159` |
| 45 | 0.20% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 70` |
| 46 | 0.20% | `sun.misc.Unsafe.compareAndSwapLong(Object, long, long, long) line: 938` |
| 47 | 0.19% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 48 | 0.19% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1313` |
| 49 | 0.18% | `java.lang.invoke.VarHandleLongs$FieldInstanceReadWrite.weakCompareAndSetRelease(VarHandle, Object, long, long) line: 241` |
| 50 | 0.18% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 51 | 0.17% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 134` |
| 52 | 0.17% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3459` |
| 53 | 0.17% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 54 | 0.17% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 55 | 0.17% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 56 | 0.16% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 437` |
| 57 | 0.16% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 58 | 0.16% | `net.minecraft.world.level.pathfinder.Path.getNextNodePos() line: 89` |
| 59 | 0.16% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightInterface.getSkyLightValue(BlockPos, ChunkAccess) line: 256` |
| 60 | 0.16% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet$SetIterator.next() line: 522` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 48.76% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 10.93% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 1.80% | `java.util.HashMap.containsKey(Object) line: 602` |
| 4 | 1.73% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 5 | 1.16% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 6 | 1.03% | `java.util.stream.AbstractPipeline.wrapAndCopyInto(Sink, Spliterator) line: 499` |
| 7 | 0.62% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 8 | 0.57% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 9 | 0.40% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 10 | 0.39% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 11 | 0.38% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 12 | 0.37% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 13 | 0.35% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2274` |
| 14 | 0.35% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 15 | 0.33% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightInterface.getRawBrightness(BlockPos, int) line: 322` |
| 16 | 0.33% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 17 | 0.32% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 18 | 0.32% | `ca.spottedleaf.moonrise.common.util.TickThread.ensureTickThread(String) line: 27` |
| 19 | 0.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 549` |
| 20 | 0.31% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 21 | 0.31% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 22 | 0.30% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 78` |
| 23 | 0.30% | `net.minecraft.world.entity.Mob.serverAiStep() line: 759` |
| 24 | 0.29% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 25 | 0.29% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 26 | 0.29% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 27 | 0.28% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 212` |
| 28 | 0.28% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3571` |
| 29 | 0.27% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 30 | 0.27% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |

GC pauses: 0, total 0 ms

