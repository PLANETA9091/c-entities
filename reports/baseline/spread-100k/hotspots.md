# JVM hotspots (JFR ExecutionSample)

Samples: 73515

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 25.51% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.93% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 2.47% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 4 | 2.28% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 1.93% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 6 | 1.86% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 7 | 1.51% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 8 | 1.50% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 9 | 1.26% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 10 | 1.24% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 11 | 1.21% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.16% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 13 | 1.15% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 14 | 1.09% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 15 | 0.89% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 16 | 0.86% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 17 | 0.81% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 18 | 0.81% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 19 | 0.81% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 20 | 0.73% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 21 | 0.70% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 22 | 0.68% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 23 | 0.67% | `java.util.ArrayDeque.pollFirst() line: 378` |
| 24 | 0.67% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 25 | 0.61% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 26 | 0.60% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3739` |
| 27 | 0.59% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 28 | 0.58% | `java.util.EnumMap.get(Object) line: 248` |
| 29 | 0.54% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 30 | 0.50% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 31 | 0.49% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 32 | 0.48% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 33 | 0.47% | `java.util.Optional.ifPresent(Consumer) line: 178` |
| 34 | 0.45% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 35 | 0.44% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 36 | 0.43% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 37 | 0.42% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 38 | 0.40% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 39 | 0.38% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 40 | 0.36% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 41 | 0.36% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 42 | 0.36% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 43 | 0.35% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 44 | 0.35% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 45 | 0.34% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 46 | 0.33% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 47 | 0.32% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 48 | 0.32% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 49 | 0.32% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 50 | 0.32% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 51 | 0.32% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 52 | 0.30% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 53 | 0.29% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 54 | 0.29% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 55 | 0.28% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 56 | 0.28% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 57 | 0.28% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 58 | 0.28% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 59 | 0.28% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 60 | 0.27% | `java.util.HashMap.getNode(Object) line: 586` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 25.51% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 3.60% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 3.48% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 3.12% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 5 | 2.93% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 6 | 1.93% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3803` |
| 7 | 1.91% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 8 | 1.83% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 9 | 1.51% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 10 | 1.24% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 11 | 1.17% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 12 | 0.97% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 13 | 0.86% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 14 | 0.86% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 15 | 0.81% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 16 | 0.80% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 17 | 0.72% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 18 | 0.72% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 19 | 0.67% | `java.util.ArrayDeque.removeFirst() line: 358` |
| 20 | 0.66% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 21 | 0.59% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 22 | 0.55% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 23 | 0.52% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 24 | 0.51% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 25 | 0.51% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 26 | 0.50% | `java.util.Collections$UnmodifiableCollection.iterator() line: 1076` |
| 27 | 0.49% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 28 | 0.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 29 | 0.47% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 30 | 0.47% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |

GC pauses: 0, total 0 ms

