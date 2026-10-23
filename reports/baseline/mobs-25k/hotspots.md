# JVM hotspots (JFR ExecutionSample)

Samples: 37849

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 23.03% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 5.22% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 5.08% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 1.98% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 5 | 1.95% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4706` |
| 6 | 1.51% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 1.28% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 8 | 1.21% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 9 | 1.20% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 10 | 1.03% | `java.util.EnumMap.get(Object) line: 248` |
| 11 | 0.95% | `java.util.HashMap.getNode(Object) line: 586` |
| 12 | 0.92% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 13 | 0.88% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 14 | 0.79% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 15 | 0.75% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 16 | 0.74% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 17 | 0.72% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3907` |
| 18 | 0.67% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 19 | 0.66% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3926` |
| 20 | 0.63% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 21 | 0.62% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 22 | 0.61% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 23 | 0.60% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3868` |
| 24 | 0.58% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 25 | 0.57% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 26 | 0.55% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 4089` |
| 27 | 0.54% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 28 | 0.48% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 29 | 0.48% | `java.lang.String.hashCode() line: 2430` |
| 30 | 0.48% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 31 | 0.43% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 32 | 0.43% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 33 | 0.43% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 34 | 0.43% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 35 | 0.42% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 36 | 0.41% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 37 | 0.41% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 38 | 0.41% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 39 | 0.41% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 40 | 0.40% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 41 | 0.36% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 42 | 0.36% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 43 | 0.36% | `java.util.Optional.isPresent() line: 154` |
| 44 | 0.35% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 45 | 0.35% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 46 | 0.34% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 47 | 0.34% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 48 | 0.32% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 49 | 0.32% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 50 | 0.32% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 51 | 0.27% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 52 | 0.27% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 53 | 0.27% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 54 | 0.26% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 55 | 0.25% | `java.lang.Enum.ordinal() line: 128` |
| 56 | 0.25% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 57 | 0.25% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap$MapIterator.nextEntry() line: 655` |
| 58 | 0.24% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |
| 59 | 0.23% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(BlockPos) line: 15` |
| 60 | 0.23% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 23.03% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 8.22% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 7.16% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 2.38% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 1.21% | `net.minecraft.world.entity.ConversionTracker.tick() line: 79` |
| 6 | 1.12% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 7 | 0.90% | `java.util.HashMap.containsKey(Object) line: 603` |
| 8 | 0.89% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 9 | 0.80% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 10 | 0.72% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 11 | 0.72% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 12 | 0.71% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3810` |
| 13 | 0.68% | `net.minecraft.world.entity.Mob.serverAiStep() line: 836` |
| 14 | 0.67% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 15 | 0.67% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 16 | 0.65% | `net.minecraft.world.entity.Mob.serverAiStep() line: 843` |
| 17 | 0.64% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 18 | 0.63% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 19 | 0.62% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 20 | 0.61% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 21 | 0.61% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 22 | 0.61% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 23 | 0.60% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3798` |
| 24 | 0.59% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 25 | 0.59% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 26 | 0.58% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 27 | 0.55% | `net.minecraft.world.entity.EntitySelector.lambda$pushableBy$0(Entity, Team, Team$CollisionRule, Entity) line: 52` |
| 28 | 0.54% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 29 | 0.52% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 30 | 0.51% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |

GC pauses: 0, total 0 ms

