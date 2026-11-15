# JVM hotspots (JFR ExecutionSample)

Samples: 51238

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 38.23% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.95% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 3 | 2.33% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 1.86% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 1.46% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 1.35% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 7 | 1.28% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 8 | 1.15% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 9 | 1.09% | `java.util.HashMap.getNode(Object) line: 586` |
| 10 | 1.04% | `net.minecraft.world.entity.LivingEntity.tick() line: 3491` |
| 11 | 0.90% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 12 | 0.90% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 13 | 0.87% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 14 | 0.83% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 15 | 0.74% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 16 | 0.67% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 17 | 0.65% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 18 | 0.62% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 19 | 0.54% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 20 | 0.53% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3937` |
| 21 | 0.51% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 22 | 0.49% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 23 | 0.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 24 | 0.47% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 66` |
| 25 | 0.47% | `net.minecraft.util.Mth.wrapDegrees(float) line: 226` |
| 26 | 0.44% | `java.util.Optional.ifPresent(Consumer) line: 178` |
| 27 | 0.44% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 28 | 0.43% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 29 | 0.42% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 30 | 0.40% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 31 | 0.40% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 32 | 0.39% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(BlockPos) line: 15` |
| 33 | 0.39% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 34 | 0.37% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 35 | 0.37% | `java.util.EnumMap.get(Object) line: 248` |
| 36 | 0.36% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 37 | 0.34% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 38 | 0.34% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3707` |
| 39 | 0.33% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 40 | 0.33% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 41 | 0.32% | `java.util.EnumMap$EntryIterator$Entry.getKey() line: 589` |
| 42 | 0.32% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 43 | 0.31% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 44 | 0.31% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 45 | 0.30% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 4100` |
| 46 | 0.30% | `java.util.ArrayDeque.size() line: 647` |
| 47 | 0.29% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 48 | 0.29% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 49 | 0.29% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 50 | 0.28% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 51 | 0.27% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 52 | 0.27% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 53 | 0.27% | `java.lang.String.hashCode() line: 2430` |
| 54 | 0.26% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 55 | 0.26% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 56 | 0.25% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 57 | 0.25% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 58 | 0.24% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 59 | 0.23% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 60 | 0.23% | `net.minecraft.world.item.ItemStack.getDamageValue() line: 574` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 38.23% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 5.81% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 4.12% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 2.00% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 1.12% | `net.minecraft.world.entity.Mob.tick() line: 410` |
| 6 | 1.08% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 7 | 0.90% | `net.minecraft.world.entity.ConversionTracker.tick() line: 79` |
| 8 | 0.89% | `java.util.HashMap.containsKey(Object) line: 603` |
| 9 | 0.88% | `net.minecraft.world.level.chunk.LevelChunk.getFluidState(int, int, int) line: 355` |
| 10 | 0.83% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 11 | 0.77% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 12 | 0.75% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 13 | 0.74% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 14 | 0.70% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 15 | 0.68% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 16 | 0.66% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 17 | 0.62% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 18 | 0.55% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3821` |
| 19 | 0.51% | `net.minecraft.world.item.enchantment.ItemEnchantments.entrySet() line: 101` |
| 20 | 0.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 21 | 0.49% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 22 | 0.49% | `net.minecraft.world.entity.EntitySelector.lambda$pushableBy$0(Entity, Team, Team$CollisionRule, Entity) line: 52` |
| 23 | 0.49% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 24 | 0.48% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2515` |
| 25 | 0.48% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 26 | 0.45% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 27 | 0.45% | `net.minecraft.world.entity.Mob.serverAiStep() line: 836` |
| 28 | 0.44% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 29 | 0.44% | `net.minecraft.server.level.ServerLevel.isPositionEntityTicking(BlockPos) line: 2956` |
| 30 | 0.43% | `net.minecraft.world.entity.Mob.serverAiStep() line: 843` |

GC pauses: 0, total 0 ms

