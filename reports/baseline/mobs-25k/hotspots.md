# JVM hotspots (JFR ExecutionSample)

Samples: 33532

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 21.54% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.83% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 4.66% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 3.82% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 5 | 2.87% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 6 | 2.40% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 1.61% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 8 | 1.56% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 9 | 1.35% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 10 | 1.28% | `java.util.EnumMap.get(Object) line: 248` |
| 11 | 1.24% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 12 | 1.13% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 1.06% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 14 | 1.00% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 15 | 0.90% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 16 | 0.87% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 17 | 0.79% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 18 | 0.78% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |
| 19 | 0.76% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3926` |
| 20 | 0.72% | `java.util.HashMap.getNode(Object) line: 586` |
| 21 | 0.69% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 22 | 0.67% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 23 | 0.65% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 24 | 0.64% | `net.minecraft.world.entity.Entity.push(Entity) line: 2472` |
| 25 | 0.60% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 26 | 0.56% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 27 | 0.55% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 28 | 0.55% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 29 | 0.55% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 30 | 0.53% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 31 | 0.52% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 32 | 0.50% | `java.lang.String.hashCode() line: 2430` |
| 33 | 0.48% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 34 | 0.48% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 35 | 0.46% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 36 | 0.45% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 37 | 0.45% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 38 | 0.44% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 39 | 0.44% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 40 | 0.42% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 41 | 0.41% | `java.util.Optional.isPresent() line: 154` |
| 42 | 0.40% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 43 | 0.40% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 44 | 0.40% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 45 | 0.38% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 46 | 0.37% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 47 | 0.36% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 48 | 0.36% | `java.util.ArrayDeque.size() line: 647` |
| 49 | 0.35% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 50 | 0.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 51 | 0.31% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 52 | 0.30% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 53 | 0.30% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 54 | 0.30% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 55 | 0.28% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 56 | 0.27% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 57 | 0.25% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 58 | 0.25% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 59 | 0.24% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 60 | 0.23% | `net.minecraft.world.entity.LivingEntity$SwingState.tick() line: 5015` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 21.54% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 8.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 8.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 3.53% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 1.56% | `net.minecraft.world.entity.ConversionTracker.tick() line: 79` |
| 6 | 1.47% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 7 | 1.24% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 8 | 1.11% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 9 | 1.01% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 10 | 0.90% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 11 | 0.90% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 12 | 0.88% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 13 | 0.81% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 14 | 0.80% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3810` |
| 15 | 0.78% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 196` |
| 16 | 0.77% | `net.minecraft.world.entity.LivingEntity.push(Entity) line: 2948` |
| 17 | 0.76% | `java.util.HashMap.containsKey(Object) line: 603` |
| 18 | 0.71% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 19 | 0.68% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2510` |
| 20 | 0.67% | `net.minecraft.world.item.enchantment.ItemEnchantments.entrySet() line: 101` |
| 21 | 0.67% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 22 | 0.66% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 23 | 0.65% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 24 | 0.64% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 25 | 0.61% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 26 | 0.59% | `net.minecraft.world.entity.Mob.serverAiStep() line: 836` |
| 27 | 0.57% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 28 | 0.56% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 29 | 0.55% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 30 | 0.54% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |

GC pauses: 0, total 0 ms

