# JVM hotspots (JFR ExecutionSample)

Samples: 10197

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 14.92% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.36% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 2.56% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 4 | 1.78% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 5 | 1.64% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 6 | 1.58% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 7 | 1.57% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 1.32% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 9 | 1.29% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 61` |
| 10 | 1.27% | `java.util.HashMap.getNode(Object) line: 585` |
| 11 | 1.18% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 12 | 1.12% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 13 | 1.02% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 14 | 0.96% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 15 | 0.94% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 16 | 0.89% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 17 | 0.84% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 18 | 0.82% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 19 | 0.80% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 20 | 0.79% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 21 | 0.76% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 22 | 0.76% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 23 | 0.75% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 24 | 0.72% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 25 | 0.70% | `java.util.EnumMap.get(Object) line: 247` |
| 26 | 0.68% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 27 | 0.65% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 28 | 0.64% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 29 | 0.63% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 30 | 0.62% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 31 | 0.60% | `java.lang.ThreadLocal.get() line: 172` |
| 32 | 0.57% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4882` |
| 33 | 0.55% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1313` |
| 34 | 0.54% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1422` |
| 35 | 0.53% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3456` |
| 36 | 0.49% | `java.util.Collections.unmodifiableSet(Set) line: 1238` |
| 37 | 0.48% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 38 | 0.45% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 437` |
| 39 | 0.45% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 47` |
| 40 | 0.44% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 41 | 0.44% | `net.minecraft.core.Vec3i.getZ() line: 71` |
| 42 | 0.44% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 43 | 0.43% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 44 | 0.40% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 45 | 0.40% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 46 | 0.40% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 47 | 0.37% | `java.util.EnumMap.isValidKey(Object) line: 319` |
| 48 | 0.37% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 320` |
| 49 | 0.37% | `net.minecraft.world.level.pathfinder.Path.isDone() line: 38` |
| 50 | 0.36% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 51 | 0.34% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 134` |
| 52 | 0.33% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 53 | 0.32% | `net.minecraft.world.level.chunk.PalettedContainer.readPalette(PalettedContainer$Data, int) line: 96` |
| 54 | 0.32% | `ca.spottedleaf.concurrentutil.util.HashUtil.mix(long) line: 92` |
| 55 | 0.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 56 | 0.29% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 57 | 0.29% | `net.minecraft.world.level.pathfinder.PathTypeCache.get(int, long) line: 23` |
| 58 | 0.28% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 59 | 0.28% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 60 | 0.27% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 14.92% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 7.80% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 2.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 4 | 1.99% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 5 | 1.32% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 6 | 1.30% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 12` |
| 7 | 1.25% | `java.util.HashMap.containsKey(Object) line: 602` |
| 8 | 1.16% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 9 | 1.11% | `ca.spottedleaf.moonrise.common.util.TickThread.ensureTickThread(String) line: 27` |
| 10 | 1.10% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 11 | 1.07% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 12 | 1.03% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 13 | 1.02% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 78` |
| 14 | 0.94% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 15 | 0.87% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 16 | 0.85% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 17 | 0.82% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 18 | 0.80% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2721` |
| 19 | 0.78% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 20 | 0.78% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 21 | 0.76% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 22 | 0.72% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 23 | 0.71% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 24 | 0.70% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 25 | 0.65% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 26 | 0.65% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 27 | 0.64% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartC() line: 948` |
| 28 | 0.64% | `net.minecraft.world.entity.Mob.serverAiStep() line: 759` |
| 29 | 0.62% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 235` |
| 30 | 0.62% | `net.minecraft.world.entity.Entity.getBlockPosBelowThatAffectsMyMovement() line: 1406` |

GC pauses: 0, total 0 ms

