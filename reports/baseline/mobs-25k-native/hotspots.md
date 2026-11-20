# JVM hotspots (JFR ExecutionSample)

Samples: 43424

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 31.35% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.78% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 3 | 3.20% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 3.09% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 5 | 2.81% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 6 | 2.06% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 1.35% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 8 | 1.34% | `java.util.HashMap.getNode(Object) line: 586` |
| 9 | 1.16% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 10 | 1.02% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 11 | 0.84% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 0.74% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 13 | 0.74% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 14 | 0.73% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 15 | 0.72% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 16 | 0.61% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 17 | 0.60% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 18 | 0.59% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3937` |
| 19 | 0.54% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 20 | 0.53% | `java.util.EnumMap.get(Object) line: 248` |
| 21 | 0.53% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 22 | 0.51% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 23 | 0.51% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 24 | 0.51% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 25 | 0.50% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 26 | 0.49% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(BlockPos) line: 15` |
| 27 | 0.47% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 28 | 0.47% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 29 | 0.46% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 30 | 0.46% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 31 | 0.43% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 32 | 0.41% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 33 | 0.39% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 34 | 0.38% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 35 | 0.38% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 36 | 0.37% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 37 | 0.37% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 38 | 0.35% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 39 | 0.34% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 40 | 0.32% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 41 | 0.32% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 42 | 0.32% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 43 | 0.30% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 44 | 0.30% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 45 | 0.30% | `java.lang.String.hashCode() line: 2430` |
| 46 | 0.29% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 47 | 0.29% | `net.minecraft.world.item.ItemStack.getDamageValue() line: 574` |
| 48 | 0.28% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 4100` |
| 49 | 0.28% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 50 | 0.27% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 51 | 0.25% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 52 | 0.24% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 53 | 0.24% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 54 | 0.24% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 55 | 0.24% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 56 | 0.23% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 57 | 0.23% | `java.util.ArrayDeque.size() line: 647` |
| 58 | 0.22% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 59 | 0.20% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 60 | 0.20% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 31.35% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.98% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 5.56% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 2.89% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 2.81% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3803` |
| 6 | 1.35% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 7 | 1.08% | `java.util.HashMap.containsKey(Object) line: 603` |
| 8 | 1.07% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 0.89% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 10 | 0.80% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2515` |
| 11 | 0.73% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 894` |
| 12 | 0.67% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3821` |
| 13 | 0.65% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 14 | 0.64% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 15 | 0.62% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 16 | 0.62% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 17 | 0.59% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 18 | 0.58% | `net.minecraft.server.level.ServerLevel.isPositionEntityTicking(BlockPos) line: 2957` |
| 19 | 0.53% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 20 | 0.53% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 21 | 0.51% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 22 | 0.51% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 23 | 0.51% | `net.minecraft.world.entity.LivingEntity.tick() line: 3491` |
| 24 | 0.49% | `net.minecraft.world.entity.Entity.getBlockPosBelowThatAffectsMyMovement() line: 1567` |
| 25 | 0.49% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 26 | 0.47% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 27 | 0.47% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 28 | 0.47% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 29 | 0.46% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3804` |
| 30 | 0.44% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |

GC pauses: 0, total 0 ms

