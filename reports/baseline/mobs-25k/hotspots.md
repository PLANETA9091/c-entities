# JVM hotspots (JFR ExecutionSample)

Samples: 33276

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 26.74% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.34% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 4.13% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 2.07% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 5 | 1.82% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 1.80% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 7 | 1.75% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 8 | 1.51% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 9 | 1.45% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 10 | 1.25% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 11 | 1.18% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 12 | 1.16% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 13 | 1.13% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 14 | 1.12% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 15 | 0.96% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 16 | 0.95% | `java.util.EnumMap.get(Object) line: 248` |
| 17 | 0.91% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 18 | 0.89% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 19 | 0.89% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 20 | 0.87% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 21 | 0.84% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3926` |
| 22 | 0.80% | `java.lang.String.hashCode() line: 2430` |
| 23 | 0.72% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 24 | 0.70% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 25 | 0.70% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 26 | 0.69% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 27 | 0.63% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 28 | 0.61% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 29 | 0.60% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 30 | 0.60% | `java.util.HashMap.getNode(Object) line: 586` |
| 31 | 0.60% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 32 | 0.55% | `jdk.internal.util.ArraysSupport.hashCodeOfUnsigned(byte[], int, int, int) line: 275` |
| 33 | 0.51% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 34 | 0.51% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 35 | 0.50% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 36 | 0.49% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 37 | 0.49% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 38 | 0.48% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(BlockPos) line: 15` |
| 39 | 0.44% | `java.util.ArrayDeque.size() line: 647` |
| 40 | 0.43% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 41 | 0.42% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 42 | 0.42% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 43 | 0.41% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 44 | 0.38% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 45 | 0.38% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 46 | 0.38% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 47 | 0.36% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 48 | 0.35% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 49 | 0.35% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 50 | 0.35% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 51 | 0.30% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 52 | 0.30% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 53 | 0.29% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 993` |
| 54 | 0.28% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 55 | 0.28% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 56 | 0.28% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 57 | 0.27% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 58 | 0.25% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 59 | 0.25% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 60 | 0.25% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 26.74% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 7.61% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 6.40% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 2.78% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 1.51% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 6 | 1.50% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 7 | 1.19% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 8 | 1.13% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 9 | 1.12% | `net.minecraft.world.entity.ConversionTracker.tick() line: 79` |
| 10 | 1.10% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 11 | 1.10% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 12 | 0.93% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 13 | 0.92% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 14 | 0.90% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2510` |
| 15 | 0.89% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 16 | 0.87% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3810` |
| 17 | 0.80% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 18 | 0.80% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 19 | 0.79% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 20 | 0.72% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 21 | 0.70% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 22 | 0.69% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 23 | 0.65% | `net.minecraft.world.entity.Mob.serverAiStep() line: 836` |
| 24 | 0.64% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 25 | 0.64% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 26 | 0.63% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 27 | 0.61% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 28 | 0.60% | `net.minecraft.world.entity.Entity.getBlockPosBelowThatAffectsMyMovement() line: 1567` |
| 29 | 0.60% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 30 | 0.59% | `java.util.HashMap.containsKey(Object) line: 603` |

GC pauses: 0, total 0 ms

