# JVM hotspots (JFR ExecutionSample)

Samples: 17902

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 29.45% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 6.00% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 1.50% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 4 | 1.43% | `java.util.HashMap.getNode(Object) line: 585` |
| 5 | 1.40% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 6 | 1.06% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 7 | 1.03% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 8 | 1.02% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 9 | 0.84% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 10 | 0.75% | `net.minecraft.world.entity.Entity.lambda$checkInsideBlocks$2(LongSet, Vec3, Vec3, InsideBlockEffectApplier$StepBasedCollector, BlockPos, int) line: 1693` |
| 11 | 0.72% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 12 | 0.66% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 13 | 0.65% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 14 | 0.64% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 15 | 0.63% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 16 | 0.61% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 17 | 0.59% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 18 | 0.58% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 19 | 0.57% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 20 | 0.53% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 21 | 0.53% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 22 | 0.48% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 23 | 0.47% | `net.minecraft.core.Vec3i.getZ() line: 71` |
| 24 | 0.47% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 25 | 0.45% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 26 | 0.42% | `java.util.EnumMap.get(Object) line: 247` |
| 27 | 0.41% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4882` |
| 28 | 0.41% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1414` |
| 29 | 0.40% | `java.lang.Enum.ordinal() line: 128` |
| 30 | 0.39% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 31 | 0.38% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1313` |
| 32 | 0.37% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 33 | 0.36% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 34 | 0.34% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 35 | 0.34% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 36 | 0.33% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 37 | 0.31% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 38 | 0.31% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 39 | 0.30% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 444` |
| 40 | 0.30% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 41 | 0.28% | `io.netty.channel.ChannelOutboundBuffer.addFlush() line: 159` |
| 42 | 0.28% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 43 | 0.27% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 134` |
| 44 | 0.27% | `net.minecraft.world.level.pathfinder.Path.isDone() line: 38` |
| 45 | 0.25% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3459` |
| 46 | 0.25% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.size() line: 422` |
| 47 | 0.25% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet$SetIterator.next() line: 522` |
| 48 | 0.24% | `ca.spottedleaf.moonrise.patches.starlight.light.SWMRNibbleArray.set(int, int) line: 422` |
| 49 | 0.24% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 50 | 0.23% | `io.netty.buffer.PooledUnsafeDirectByteBuf.newInstance(int) line: 39` |
| 51 | 0.23% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 52 | 0.23% | `sun.misc.Unsafe.compareAndSwapLong(Object, long, long, long) line: 938` |
| 53 | 0.22% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 54 | 0.22% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 55 | 0.22% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 56 | 0.22% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 57 | 0.22% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 437` |
| 58 | 0.21% | `java.lang.invoke.VarHandleLongs$FieldInstanceReadWrite.weakCompareAndSetRelease(VarHandle, Object, long, long) line: 241` |
| 59 | 0.21% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 60 | 0.21% | `net.minecraft.core.Vec3i.equals(Object) line: 45` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 29.45% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 7.47% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 1.79% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 4 | 1.50% | `java.util.stream.AbstractPipeline.wrapAndCopyInto(Sink, Spliterator) line: 499` |
| 5 | 1.44% | `java.util.HashMap.containsKey(Object) line: 602` |
| 6 | 1.41% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 7 | 1.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 8 | 0.77% | `net.minecraft.world.level.BlockGetter.forEachBlockIntersectedBetween(Vec3, Vec3, AABB, BlockGetter$BlockStepVisitor) line: 219` |
| 9 | 0.73% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 10 | 0.68% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 11 | 0.66% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 12 | 0.63% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 13 | 0.63% | `ca.spottedleaf.moonrise.common.util.TickThread.ensureTickThread(String) line: 27` |
| 14 | 0.62% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 15 | 0.62% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 16 | 0.58% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 17 | 0.57% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 78` |
| 18 | 0.54% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 19 | 0.54% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 20 | 0.53% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 21 | 0.53% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 22 | 0.45% | `net.minecraft.world.entity.Entity.getBlockZ() line: 4931` |
| 23 | 0.44% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 24 | 0.44% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 25 | 0.44% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 26 | 0.41% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 235` |
| 27 | 0.41% | `net.minecraft.world.entity.Mob.serverAiStep() line: 759` |
| 28 | 0.40% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 29 | 0.39% | `net.minecraft.world.entity.Entity.setDeltaMovement(double, double, double) line: 4891` |
| 30 | 0.39% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 212` |

GC pauses: 0, total 0 ms

