# JVM hotspots (JFR ExecutionSample)

Samples: 13569

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 9.88% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 5.04% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getAtIndexVolatile(ConcurrentLong2ReferenceChainedHashTable$TableEntry[], int) line: 166` |
| 3 | 4.08% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 2.98% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 5 | 1.91% | `java.util.HashMap.getNode(Object) line: 585` |
| 6 | 1.42% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 1.19% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 8 | 1.18% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 9 | 1.11% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 10 | 1.09% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 11 | 1.08% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 12 | 1.07% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 13 | 1.05% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 14 | 0.96% | `net.minecraft.world.entity.Entity.lambda$checkInsideBlocks$2(LongSet, Vec3, Vec3, InsideBlockEffectApplier$StepBasedCollector, BlockPos, int) line: 1693` |
| 15 | 0.84% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 16 | 0.83% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 17 | 0.83% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1414` |
| 18 | 0.83% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 19 | 0.74% | `java.util.EnumMap.get(Object) line: 247` |
| 20 | 0.74% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 21 | 0.66% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 22 | 0.66% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 23 | 0.65% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 24 | 0.63% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 25 | 0.63% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 26 | 0.60% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1313` |
| 27 | 0.57% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 28 | 0.55% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(ResourceLocation) line: 68` |
| 29 | 0.54% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 30 | 0.54% | `net.minecraft.core.Vec3i.getZ() line: 71` |
| 31 | 0.54% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 32 | 0.50% | `java.lang.invoke.VarHandleLongs$FieldInstanceReadWrite.weakCompareAndSetRelease(VarHandle, Object, long, long) line: 241` |
| 33 | 0.49% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 34 | 0.44% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 35 | 0.44% | `sun.misc.Unsafe.compareAndSwapLong(Object, long, long, long) line: 938` |
| 36 | 0.43% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 37 | 0.43% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 38 | 0.41% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3459` |
| 39 | 0.41% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 40 | 0.41% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 41 | 0.38% | `java.lang.Enum.ordinal() line: 128` |
| 42 | 0.38% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 43 | 0.38% | `io.netty.channel.ChannelOutboundBuffer.addFlush() line: 159` |
| 44 | 0.36% | `java.lang.ThreadLocal.get() line: 172` |
| 45 | 0.35% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 134` |
| 46 | 0.34% | `io.netty.buffer.PooledUnsafeDirectByteBuf.newInstance(int) line: 39` |
| 47 | 0.33% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 48 | 0.33% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 49 | 0.33% | `java.util.ArrayDeque.addLast(Object) line: 304` |
| 50 | 0.33% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 51 | 0.32% | `java.util.ArrayDeque.size() line: 648` |
| 52 | 0.32% | `java.util.concurrent.atomic.AtomicIntegerFieldUpdater$AtomicIntegerFieldUpdaterImpl.compareAndSet(Object, int, int) line: 491` |
| 53 | 0.32% | `java.util.EnumMap.entrySet() line: 469` |
| 54 | 0.32% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 55 | 0.30% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 56 | 0.30% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 328` |
| 57 | 0.29% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 58 | 0.29% | `io.netty.buffer.PooledByteBuf.deallocate() line: 175` |
| 59 | 0.29% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 60 | 0.29% | `io.netty.channel.unix.IovArray.processMessage(Object) line: 227` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 9.88% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.12% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 5.04% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 194` |
| 4 | 2.98% | `java.util.stream.AbstractPipeline.wrapAndCopyInto(Sink, Spliterator) line: 499` |
| 5 | 1.91% | `java.util.HashMap.containsKey(Object) line: 602` |
| 6 | 1.67% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 7 | 1.50% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 8 | 1.28% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 9 | 1.13% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 10 | 1.05% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 11 | 1.04% | `ca.spottedleaf.moonrise.common.util.TickThread.ensureTickThread(String) line: 27` |
| 12 | 1.00% | `net.minecraft.world.level.BlockGetter.forEachBlockIntersectedBetween(Vec3, Vec3, AABB, BlockGetter$BlockStepVisitor) line: 219` |
| 13 | 0.97% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 14 | 0.86% | `net.minecraft.world.entity.Entity.getBlockPosBelowThatAffectsMyMovement() line: 1406` |
| 15 | 0.85% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 235` |
| 16 | 0.80% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 17 | 0.76% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2721` |
| 18 | 0.74% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 19 | 0.69% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 20 | 0.66% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 21 | 0.65% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 78` |
| 22 | 0.60% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 23 | 0.60% | `net.minecraft.world.entity.LivingEntity.tick() line: 3315` |
| 24 | 0.57% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 25 | 0.57% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 26 | 0.57% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 27 | 0.57% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 28 | 0.56% | `net.minecraft.world.entity.Mob.serverAiStep() line: 771` |
| 29 | 0.56% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 30 | 0.55% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 533` |

GC pauses: 0, total 0 ms

