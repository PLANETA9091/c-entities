# JVM hotspots (JFR ExecutionSample)

Samples: 57867

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 13.04% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.21% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 2.98% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 4 | 2.59% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 2.16% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 6 | 1.86% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 7 | 1.84% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 1.72% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.71% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 10 | 1.35% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 11 | 1.17% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 12 | 1.09% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 13 | 1.08% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 14 | 1.02% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 15 | 1.02% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 16 | 1.01% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 17 | 0.97% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 18 | 0.96% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 19 | 0.86% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 20 | 0.86% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 21 | 0.84% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 22 | 0.82% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 23 | 0.82% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 24 | 0.73% | `java.util.EnumMap.get(Object) line: 248` |
| 25 | 0.70% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 26 | 0.69% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 27 | 0.66% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 28 | 0.60% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 29 | 0.59% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 30 | 0.56% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 31 | 0.54% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 32 | 0.52% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 33 | 0.51% | `java.util.Optional.ifPresent(Consumer) line: 178` |
| 34 | 0.51% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 35 | 0.51% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 36 | 0.48% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 37 | 0.47% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 38 | 0.47% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 39 | 0.46% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 40 | 0.44% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 41 | 0.42% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 42 | 0.42% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 43 | 0.42% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 44 | 0.42% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 45 | 0.41% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 46 | 0.40% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 47 | 0.38% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 48 | 0.38% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 49 | 0.37% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 50 | 0.36% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 51 | 0.35% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 52 | 0.35% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 53 | 0.35% | `net.minecraft.world.entity.Entity.baseTick() line: 914` |
| 54 | 0.34% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 55 | 0.33% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 56 | 0.33% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 57 | 0.33% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 58 | 0.33% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 59 | 0.33% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 60 | 0.33% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 13.04% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 4.92% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 4.68% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 4.63% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 5 | 2.59% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 6 | 2.22% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 7 | 2.18% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 8 | 2.17% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 9 | 1.26% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 10 | 1.21% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 11 | 1.08% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 12 | 1.03% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 13 | 1.03% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 14 | 0.88% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 15 | 0.86% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 16 | 0.82% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 17 | 0.77% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 18 | 0.71% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 19 | 0.65% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 20 | 0.63% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 21 | 0.62% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 22 | 0.61% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 23 | 0.61% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 24 | 0.59% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 25 | 0.58% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 26 | 0.58% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 27 | 0.57% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 28 | 0.56% | `net.minecraft.world.entity.Entity.setDeltaMovement(double, double, double) line: 5087` |
| 29 | 0.56% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 30 | 0.52% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |

GC pauses: 0, total 0 ms

