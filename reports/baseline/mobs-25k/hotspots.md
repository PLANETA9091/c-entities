# JVM hotspots (JFR ExecutionSample)

Samples: 33006

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 27.09% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.42% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 4.15% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 2.17% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 5 | 1.96% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 1.75% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 7 | 1.58% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 8 | 1.55% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 9 | 1.34% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 10 | 1.30% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 11 | 1.28% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 12 | 1.19% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 13 | 1.12% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 14 | 1.11% | `java.util.EnumMap.get(Object) line: 248` |
| 15 | 0.99% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 16 | 0.99% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 17 | 0.94% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 18 | 0.84% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 19 | 0.81% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3926` |
| 20 | 0.78% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 21 | 0.73% | `java.lang.String.hashCode() line: 2430` |
| 22 | 0.72% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 23 | 0.70% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 24 | 0.66% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 25 | 0.64% | `java.util.HashMap.getNode(Object) line: 586` |
| 26 | 0.60% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 27 | 0.57% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 28 | 0.56% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 29 | 0.56% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 30 | 0.54% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 31 | 0.52% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 32 | 0.49% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 33 | 0.48% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 34 | 0.46% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 35 | 0.46% | `java.util.Optional.isPresent() line: 154` |
| 36 | 0.46% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 37 | 0.45% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(BlockPos) line: 15` |
| 38 | 0.43% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 39 | 0.42% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 40 | 0.41% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 41 | 0.41% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 42 | 0.40% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 43 | 0.39% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 44 | 0.38% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 45 | 0.37% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 46 | 0.37% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 47 | 0.36% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 48 | 0.35% | `java.util.ArrayDeque.size() line: 647` |
| 49 | 0.35% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 50 | 0.34% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 51 | 0.33% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 52 | 0.31% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 53 | 0.31% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 993` |
| 54 | 0.30% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 55 | 0.29% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 56 | 0.29% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 57 | 0.28% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 58 | 0.27% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 59 | 0.26% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 60 | 0.26% | `net.minecraft.world.entity.LivingEntity$SwingState.tick() line: 5015` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 27.09% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 7.50% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 6.59% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 2.93% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 1.55% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 6 | 1.41% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 7 | 1.34% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 8 | 1.28% | `net.minecraft.world.entity.ConversionTracker.tick() line: 79` |
| 9 | 1.03% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 10 | 0.98% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 11 | 0.98% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 12 | 0.94% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 13 | 0.85% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 14 | 0.83% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3810` |
| 15 | 0.81% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2510` |
| 16 | 0.80% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 17 | 0.78% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 18 | 0.74% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 19 | 0.70% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 20 | 0.67% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 21 | 0.65% | `java.util.HashMap.containsKey(Object) line: 603` |
| 22 | 0.60% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 23 | 0.60% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 24 | 0.59% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 25 | 0.58% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 26 | 0.58% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 27 | 0.56% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 28 | 0.56% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 29 | 0.54% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 30 | 0.53% | `net.minecraft.world.entity.Mob.serverAiStep() line: 843` |

GC pauses: 0, total 0 ms

