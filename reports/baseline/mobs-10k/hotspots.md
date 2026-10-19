# JVM hotspots (JFR ExecutionSample)

Samples: 9542

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 13.88% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.76% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 3 | 2.20% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 2.18% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 61` |
| 5 | 2.02% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 6 | 1.73% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 1.60% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 8 | 1.50% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 9 | 1.49% | `java.util.HashMap.getNode(Object) line: 585` |
| 10 | 1.45% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 11 | 1.35% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 12 | 1.34% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 13 | 1.29% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 14 | 1.28% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 15 | 1.26% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 16 | 1.22% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 17 | 1.22% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 18 | 1.15% | `java.util.EnumMap.get(Object) line: 247` |
| 19 | 1.10% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 20 | 1.02% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 21 | 1.01% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1313` |
| 22 | 0.89% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 23 | 0.81% | `java.util.Collections.unmodifiableSet(Set) line: 1238` |
| 24 | 0.79% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 25 | 0.75% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 26 | 0.73% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 27 | 0.67% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 28 | 0.65% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 3849` |
| 29 | 0.65% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 30 | 0.60% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3456` |
| 31 | 0.58% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 32 | 0.58% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 33 | 0.56% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1422` |
| 34 | 0.54% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 35 | 0.54% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 36 | 0.52% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 47` |
| 37 | 0.52% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 38 | 0.51% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 39 | 0.51% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 40 | 0.50% | `java.lang.ThreadLocal.get() line: 172` |
| 41 | 0.50% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 134` |
| 42 | 0.49% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 43 | 0.47% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 44 | 0.46% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 45 | 0.45% | `net.minecraft.core.Vec3i.getZ() line: 71` |
| 46 | 0.44% | `net.minecraft.world.level.pathfinder.Path.isDone() line: 38` |
| 47 | 0.42% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 48 | 0.42% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 49 | 0.39% | `java.util.EnumMap.isValidKey(Object) line: 319` |
| 50 | 0.38% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 51 | 0.38% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 52 | 0.37% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 53 | 0.37% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 54 | 0.37% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 55 | 0.37% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 56 | 0.36% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 57 | 0.36% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 320` |
| 58 | 0.35% | `java.util.ArrayDeque.size() line: 648` |
| 59 | 0.35% | `net.minecraft.world.entity.LivingEntity.doPush(Entity) line: 3738` |
| 60 | 0.34% | `net.minecraft.world.level.chunk.PalettedContainer.readPalette(PalettedContainer$Data, int) line: 96` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 13.88% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.40% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 2.16% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 12` |
| 4 | 2.06% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 5 | 1.85% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 6 | 1.65% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 7 | 1.60% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 8 | 1.49% | `java.util.HashMap.containsKey(Object) line: 602` |
| 9 | 1.34% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 10 | 1.33% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 11 | 1.29% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 78` |
| 12 | 1.28% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 13 | 1.22% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 14 | 1.07% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 15 | 1.04% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 16 | 1.04% | `net.minecraft.world.entity.Mob.serverAiStep() line: 759` |
| 17 | 1.03% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 18 | 1.01% | `net.minecraft.world.entity.LivingEntity.tick() line: 3315` |
| 19 | 0.95% | `net.minecraft.world.item.enchantment.ItemEnchantments.entrySet() line: 99` |
| 20 | 0.94% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 21 | 0.94% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2721` |
| 22 | 0.90% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 23 | 0.86% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 24 | 0.85% | `net.minecraft.world.entity.Mob.serverAiStep() line: 764` |
| 25 | 0.84% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 26 | 0.83% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |
| 27 | 0.82% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges() line: 3348` |
| 28 | 0.81% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 29 | 0.73% | `java.util.stream.AbstractPipeline.wrapAndCopyInto(Sink, Spliterator) line: 499` |
| 30 | 0.68% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |

GC pauses: 0, total 0 ms

