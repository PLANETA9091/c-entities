# JVM hotspots (JFR ExecutionSample)

Samples: 70213

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 20.25% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.91% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 3 | 2.66% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 4 | 2.46% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 2.43% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 2.01% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 7 | 1.67% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 8 | 1.44% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.36% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 10 | 1.34% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 11 | 1.16% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 12 | 1.13% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 13 | 1.04% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 14 | 0.98% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 15 | 0.94% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 16 | 0.88% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 17 | 0.86% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 18 | 0.86% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 19 | 0.80% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 20 | 0.79% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 21 | 0.78% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 22 | 0.71% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 23 | 0.67% | `java.util.EnumMap.get(Object) line: 248` |
| 24 | 0.64% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 25 | 0.63% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 26 | 0.62% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 27 | 0.62% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 28 | 0.61% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 29 | 0.60% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 30 | 0.57% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 31 | 0.57% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 32 | 0.56% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 33 | 0.53% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 34 | 0.51% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 35 | 0.49% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 36 | 0.46% | `java.util.Optional.ifPresent(Consumer) line: 178` |
| 37 | 0.45% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 38 | 0.45% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 39 | 0.45% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 40 | 0.41% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 41 | 0.40% | `it.unimi.dsi.fastutil.objects.ObjectOpenHashSet.add(Object) line: 368` |
| 42 | 0.39% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 43 | 0.38% | `net.minecraft.util.SimpleBitStorage.get(int) line: 346` |
| 44 | 0.37% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 45 | 0.34% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 46 | 0.34% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 47 | 0.34% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 48 | 0.34% | `net.minecraft.world.entity.Entity.baseTick() line: 914` |
| 49 | 0.34% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 50 | 0.33% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 51 | 0.33% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 52 | 0.32% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 53 | 0.30% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 320` |
| 54 | 0.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 323` |
| 55 | 0.30% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 56 | 0.30% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 57 | 0.29% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 58 | 0.29% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 59 | 0.28% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 60 | 0.28% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 20.25% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 3.95% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 3.85% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 3.55% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 5 | 2.66% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 6 | 2.46% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 7 | 1.80% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 8 | 1.79% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 9 | 1.60% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 10 | 1.37% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 11 | 1.19% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 12 | 1.09% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 13 | 1.04% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 14 | 1.02% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 15 | 0.93% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 16 | 0.86% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 17 | 0.86% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 18 | 0.72% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 19 | 0.71% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |
| 20 | 0.62% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 21 | 0.62% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 22 | 0.61% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 23 | 0.59% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 24 | 0.56% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 25 | 0.56% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 26 | 0.54% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 27 | 0.54% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 28 | 0.53% | `net.minecraft.world.level.chunk.PalettedContainer.readPalette(PalettedContainer$Data, int) line: 79` |
| 29 | 0.51% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |
| 30 | 0.51% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |

GC pauses: 0, total 0 ms

