# JVM hotspots (JFR ExecutionSample)

Samples: 101900

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 39.76% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.40% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 3 | 3.14% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 2.27% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 1.61% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 1.39% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 1.37% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 8 | 1.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 9 | 1.33% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 10 | 1.01% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 11 | 0.97% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 12 | 0.82% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 13 | 0.80% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 14 | 0.79% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 15 | 0.75% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 16 | 0.73% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 17 | 0.72% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 18 | 0.66% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 19 | 0.62% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 20 | 0.58% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 21 | 0.56% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 22 | 0.53% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 23 | 0.53% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 24 | 0.51% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 25 | 0.50% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 26 | 0.50% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 27 | 0.49% | `java.util.EnumMap.get(Object) line: 248` |
| 28 | 0.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 29 | 0.48% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 30 | 0.47% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 31 | 0.46% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 32 | 0.45% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 33 | 0.45% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 34 | 0.43% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 35 | 0.42% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 36 | 0.41% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 37 | 0.36% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 38 | 0.36% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 39 | 0.35% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 40 | 0.34% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 41 | 0.33% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 42 | 0.32% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 43 | 0.31% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 44 | 0.31% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 45 | 0.29% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 46 | 0.29% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 47 | 0.28% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 48 | 0.25% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 49 | 0.25% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 50 | 0.25% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 51 | 0.24% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 52 | 0.24% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 53 | 0.24% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 54 | 0.24% | `java.util.Optional.ifPresent(Consumer) line: 178` |
| 55 | 0.24% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 92` |
| 56 | 0.24% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 57 | 0.23% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 58 | 0.22% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 59 | 0.22% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 60 | 0.22% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 39.76% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 4.89% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 3.65% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 3.58% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 5 | 3.14% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 6 | 1.78% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 7 | 1.47% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 8 | 1.33% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 9 | 1.03% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 10 | 0.95% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 11 | 0.80% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 12 | 0.75% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 13 | 0.74% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 14 | 0.70% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 15 | 0.66% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 16 | 0.62% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 17 | 0.56% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 18 | 0.51% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 19 | 0.50% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 20 | 0.49% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 21 | 0.49% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 22 | 0.48% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 23 | 0.47% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 24 | 0.44% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 25 | 0.43% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 26 | 0.43% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 27 | 0.42% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 28 | 0.41% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 29 | 0.39% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 30 | 0.39% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |

GC pauses: 0, total 0 ms

