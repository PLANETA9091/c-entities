# JVM hotspots (JFR ExecutionSample)

Samples: 41415

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 30.32% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.50% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 3 | 3.39% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 3.15% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 1.59% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 6 | 1.51% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 1.49% | `java.util.HashMap.getNode(Object) line: 586` |
| 8 | 1.08% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 9 | 0.97% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 10 | 0.89% | `java.util.EnumMap.get(Object) line: 248` |
| 11 | 0.83% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 0.82% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 13 | 0.80% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 14 | 0.75% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 15 | 0.75% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 16 | 0.73% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 17 | 0.72% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 18 | 0.71% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 19 | 0.67% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 20 | 0.62% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 21 | 0.58% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 22 | 0.58% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3938` |
| 23 | 0.57% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 24 | 0.51% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 25 | 0.49% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 26 | 0.48% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 27 | 0.44% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 28 | 0.42% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 29 | 0.42% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 30 | 0.41% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 31 | 0.41% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(BlockPos) line: 15` |
| 32 | 0.41% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 33 | 0.41% | `java.lang.String.hashCode() line: 2430` |
| 34 | 0.40% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 35 | 0.39% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3708` |
| 36 | 0.39% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 37 | 0.39% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 38 | 0.36% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 39 | 0.36% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 40 | 0.35% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 41 | 0.35% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 42 | 0.34% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 43 | 0.33% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 44 | 0.33% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 45 | 0.31% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 46 | 0.30% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 47 | 0.29% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 48 | 0.29% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1518` |
| 49 | 0.28% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 50 | 0.28% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 51 | 0.28% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 4101` |
| 52 | 0.26% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1485` |
| 53 | 0.26% | `java.util.ArrayDeque.size() line: 647` |
| 54 | 0.25% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 55 | 0.25% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 56 | 0.24% | `java.util.HashMap.getNode(Object) line: 588` |
| 57 | 0.23% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 58 | 0.23% | `net.minecraft.world.entity.LivingEntity.isPushable() line: 4085` |
| 59 | 0.23% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 60 | 0.22% | `net.minecraft.world.entity.LivingEntity.getItemBySlot(EquipmentSlot) line: 2902` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 30.32% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.65% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 6.13% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 2.33% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 1.33% | `java.util.HashMap.containsKey(Object) line: 603` |
| 6 | 0.94% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 7 | 0.82% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 894` |
| 8 | 0.80% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 9 | 0.75% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 10 | 0.72% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2822` |
| 11 | 0.72% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3822` |
| 12 | 0.71% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2515` |
| 13 | 0.67% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 14 | 0.67% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 15 | 0.66% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 16 | 0.62% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 17 | 0.60% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 18 | 0.59% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 19 | 0.57% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 20 | 0.56% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 21 | 0.53% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 22 | 0.50% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 23 | 0.50% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 24 | 0.49% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 25 | 0.49% | `net.minecraft.world.entity.EntitySelector.lambda$pushableBy$0(Entity, Team, Team$CollisionRule, Entity) line: 52` |
| 26 | 0.49% | `net.minecraft.server.level.ServerLevel.isPositionEntityTicking(BlockPos) line: 2957` |
| 27 | 0.48% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 28 | 0.47% | `net.minecraft.world.entity.Entity.getBlockPosBelowThatAffectsMyMovement() line: 1567` |
| 29 | 0.45% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 30 | 0.45% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |

GC pauses: 0, total 0 ms

