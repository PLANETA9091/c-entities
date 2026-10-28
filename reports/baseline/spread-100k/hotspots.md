# JVM hotspots (JFR ExecutionSample)

Samples: 52523

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 14.50% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.14% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 2.60% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 4 | 2.46% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 5 | 2.29% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 6 | 1.73% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 1.51% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 1.45% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 9 | 1.35% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 10 | 1.35% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 11 | 1.34% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 12 | 1.15% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 13 | 1.15% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 14 | 1.12% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 15 | 1.12% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 16 | 1.02% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 17 | 0.99% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 18 | 0.98% | `java.util.EnumMap.get(Object) line: 248` |
| 19 | 0.97% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 20 | 0.86% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 21 | 0.84% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 22 | 0.84% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 23 | 0.82% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 24 | 0.80% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 25 | 0.69% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 26 | 0.64% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 27 | 0.63% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 28 | 0.63% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 29 | 0.58% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 30 | 0.56% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 31 | 0.55% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 32 | 0.54% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 33 | 0.53% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 34 | 0.52% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 35 | 0.51% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 36 | 0.50% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 37 | 0.50% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 38 | 0.49% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 39 | 0.48% | `java.util.Optional.ifPresent(Consumer) line: 178` |
| 40 | 0.48% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 41 | 0.46% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 42 | 0.46% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 43 | 0.43% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 44 | 0.43% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 45 | 0.43% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 46 | 0.43% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 47 | 0.43% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 48 | 0.42% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 49 | 0.42% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 50 | 0.42% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 51 | 0.41% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 52 | 0.39% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 53 | 0.39% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 54 | 0.38% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 55 | 0.38% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 56 | 0.37% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 57 | 0.37% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 58 | 0.35% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 59 | 0.34% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 60 | 0.33% | `java.util.ArrayDeque.size() line: 647` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 14.50% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 5.85% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 3.94% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 3.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 5 | 3.06% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 6 | 2.33% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 7 | 1.35% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 8 | 1.35% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 9 | 1.24% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1408` |
| 10 | 1.12% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 11 | 1.12% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 12 | 1.12% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 13 | 1.05% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 14 | 0.87% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 15 | 0.86% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 16 | 0.85% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 17 | 0.84% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 18 | 0.81% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 19 | 0.78% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 20 | 0.77% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 21 | 0.76% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 22 | 0.70% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 23 | 0.70% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 24 | 0.69% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 25 | 0.68% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 26 | 0.63% | `java.util.Collections$UnmodifiableCollection.iterator() line: 1076` |
| 27 | 0.60% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 28 | 0.59% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 29 | 0.57% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 30 | 0.57% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |

GC pauses: 0, total 0 ms

