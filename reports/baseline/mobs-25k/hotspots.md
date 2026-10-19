# JVM hotspots (JFR ExecutionSample)

Samples: 40073

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 22.99% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 6.07% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 5.23% | `java.util.HashMap.getNode(Object) line: 585` |
| 4 | 1.98% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 1.82% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 61` |
| 6 | 1.81% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 7 | 1.42% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 8 | 1.18% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 9 | 1.18% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 10 | 1.15% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 11 | 1.11% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 12 | 1.07% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 1.07% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 14 | 1.07% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 15 | 1.01% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 320` |
| 16 | 1.00% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 17 | 0.96% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 18 | 0.88% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 19 | 0.86% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 20 | 0.85% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 21 | 0.83% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 22 | 0.79% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 23 | 0.78% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 24 | 0.75% | `net.minecraft.world.entity.LivingEntity.doPush(Entity) line: 3738` |
| 25 | 0.68% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1313` |
| 26 | 0.66% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1422` |
| 27 | 0.63% | `java.util.EnumMap.get(Object) line: 247` |
| 28 | 0.63% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 29 | 0.60% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 30 | 0.57% | `net.minecraft.util.Mth.wrapDegrees(float) line: 221` |
| 31 | 0.56% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 32 | 0.56% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 33 | 0.56% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(ResourceLocation) line: 68` |
| 34 | 0.54% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1215` |
| 35 | 0.53% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1075` |
| 36 | 0.52% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4882` |
| 37 | 0.48% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 38 | 0.45% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 39 | 0.45% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 70` |
| 40 | 0.43% | `net.minecraft.core.Vec3i.getZ() line: 71` |
| 41 | 0.43% | `net.minecraft.world.level.pathfinder.Path.isDone() line: 38` |
| 42 | 0.42% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 43 | 0.40% | `net.minecraft.world.entity.Entity.level() line: 5199` |
| 44 | 0.39% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 45 | 0.39% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 444` |
| 46 | 0.38% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 47 | 0.37% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3459` |
| 48 | 0.37% | `java.util.ArrayDeque.size() line: 648` |
| 49 | 0.36% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 50 | 0.35% | `java.util.EnumMap.isValidKey(Object) line: 319` |
| 51 | 0.35% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 52 | 0.35% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 53 | 0.33% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 54 | 0.33% | `net.minecraft.world.level.pathfinder.Path.getNextNodePos() line: 89` |
| 55 | 0.33% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 47` |
| 56 | 0.33% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 57 | 0.33% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 134` |
| 58 | 0.32% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 59 | 0.31% | `it.unimi.dsi.fastutil.objects.Object2IntAVLTreeMap.isEmpty() line: 942` |
| 60 | 0.29% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 147` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 22.99% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 8.27% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 5.23% | `java.util.HashMap.containsKey(Object) line: 602` |
| 4 | 2.33% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 5 | 1.85% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 12` |
| 6 | 1.63% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 7 | 1.53% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 8 | 1.42% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 9 | 1.29% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 10 | 1.19% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2274` |
| 11 | 1.08% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 12 | 1.07% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 13 | 0.97% | `net.minecraft.world.scores.Scoreboard.getPlayersTeam(String) line: 317` |
| 14 | 0.96% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 78` |
| 15 | 0.86% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 16 | 0.85% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 17 | 0.84% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 18 | 0.81% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 19 | 0.81% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 235` |
| 20 | 0.79% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 21 | 0.77% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 22 | 0.77% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2721` |
| 23 | 0.75% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3693` |
| 24 | 0.74% | `net.minecraft.world.entity.Mob.serverAiStep() line: 759` |
| 25 | 0.68% | `net.minecraft.world.entity.LivingEntity.tick() line: 3315` |
| 26 | 0.66% | `net.minecraft.world.entity.Entity.getBlockPosBelowThatAffectsMyMovement() line: 1406` |
| 27 | 0.65% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 28 | 0.64% | `net.minecraft.world.entity.Mob.serverAiStep() line: 764` |
| 29 | 0.61% | `it.unimi.dsi.fastutil.objects.ReferenceCollections$UnmodifiableCollection.contains(Object) line: 344` |
| 30 | 0.60% | `net.minecraft.world.entity.Mob.serverAiStep() line: 774` |

GC pauses: 0, total 0 ms

