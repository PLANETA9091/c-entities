# JVM hotspots (JFR ExecutionSample)

Samples: 36414

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 24.25% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 5.38% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 5.18% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 1.99% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 5 | 1.98% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 6 | 1.59% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 1.13% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 8 | 1.01% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 9 | 0.99% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 10 | 0.90% | `java.util.HashMap.getNode(Object) line: 586` |
| 11 | 0.88% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 12 | 0.82% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 13 | 0.82% | `java.util.EnumMap.get(Object) line: 248` |
| 14 | 0.78% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 15 | 0.78% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 16 | 0.74% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 17 | 0.72% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 18 | 0.71% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 19 | 0.68% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 20 | 0.63% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 21 | 0.62% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 22 | 0.53% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 23 | 0.53% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 24 | 0.53% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 25 | 0.50% | `java.lang.String.hashCode() line: 2430` |
| 26 | 0.49% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 27 | 0.48% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3926` |
| 28 | 0.45% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 29 | 0.45% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 30 | 0.43% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 31 | 0.43% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 32 | 0.42% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 33 | 0.41% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 34 | 0.41% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 35 | 0.41% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 36 | 0.38% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 4089` |
| 37 | 0.37% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 38 | 0.37% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 39 | 0.37% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 40 | 0.37% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 41 | 0.35% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap$MapIterator.nextEntry() line: 655` |
| 42 | 0.34% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 43 | 0.34% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 44 | 0.30% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 45 | 0.30% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 46 | 0.29% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 47 | 0.29% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 48 | 0.29% | `java.util.Optional.isPresent() line: 154` |
| 49 | 0.28% | `java.lang.Double.isFinite(double) line: 1023` |
| 50 | 0.28% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 51 | 0.28% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 52 | 0.27% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 53 | 0.27% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 54 | 0.27% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 55 | 0.25% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 56 | 0.25% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 57 | 0.25% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 58 | 0.24% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 59 | 0.23% | `java.lang.Enum.ordinal() line: 128` |
| 60 | 0.23% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 24.25% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 8.47% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 7.37% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 2.28% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 1.05% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 6 | 1.04% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 7 | 1.01% | `net.minecraft.world.entity.ConversionTracker.tick() line: 79` |
| 8 | 0.88% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 9 | 0.82% | `java.util.HashMap.containsKey(Object) line: 603` |
| 10 | 0.81% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 11 | 0.72% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 12 | 0.72% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 13 | 0.70% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 14 | 0.64% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 15 | 0.63% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 16 | 0.62% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 17 | 0.55% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 18 | 0.54% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 19 | 0.53% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3810` |
| 20 | 0.53% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 21 | 0.52% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 22 | 0.52% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 23 | 0.51% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 24 | 0.50% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 25 | 0.49% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 26 | 0.47% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 27 | 0.46% | `net.minecraft.world.entity.Mob.serverAiStep() line: 836` |
| 28 | 0.45% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 29 | 0.45% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 30 | 0.45% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |

GC pauses: 0, total 0 ms

