# JVM hotspots (JFR ExecutionSample)

Samples: 82659

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 28.29% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.77% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 3 | 3.75% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 3.53% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 2.60% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 2.07% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 7 | 1.82% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 8 | 1.60% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.34% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 10 | 1.32% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 11 | 1.23% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 12 | 1.22% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 13 | 1.20% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 14 | 1.19% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 15 | 1.10% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 16 | 1.09% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 17 | 1.02% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 18 | 0.94% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 19 | 0.91% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 20 | 0.86% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 21 | 0.82% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 22 | 0.81% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 23 | 0.71% | `java.util.EnumMap.get(Object) line: 248` |
| 24 | 0.63% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 25 | 0.61% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 26 | 0.55% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 27 | 0.54% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 28 | 0.50% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 29 | 0.48% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 30 | 0.47% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 31 | 0.47% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 32 | 0.47% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 33 | 0.46% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 34 | 0.45% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 35 | 0.44% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 36 | 0.38% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 92` |
| 37 | 0.38% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 38 | 0.36% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 39 | 0.35% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 40 | 0.34% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 41 | 0.33% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 42 | 0.32% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 43 | 0.32% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 44 | 0.32% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 45 | 0.31% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 46 | 0.29% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 47 | 0.28% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 48 | 0.28% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 49 | 0.26% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 50 | 0.26% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 51 | 0.25% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 52 | 0.25% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 53 | 0.25% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 54 | 0.24% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 55 | 0.24% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 56 | 0.24% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 57 | 0.24% | `net.minecraft.world.entity.Entity.baseTick() line: 914` |
| 58 | 0.24% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 59 | 0.23% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 60 | 0.23% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 28.29% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 8.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 4.45% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 4.20% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 3.53% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 6 | 2.07% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 7 | 2.07% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 8 | 1.85% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 1.57% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 10 | 1.32% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 11 | 1.30% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 12 | 1.19% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 13 | 1.10% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 14 | 0.90% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 15 | 0.87% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 16 | 0.85% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 17 | 0.81% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 18 | 0.70% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 19 | 0.63% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 20 | 0.62% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 21 | 0.61% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 22 | 0.55% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 23 | 0.50% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3584` |
| 24 | 0.48% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 25 | 0.47% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 26 | 0.47% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 27 | 0.44% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 28 | 0.40% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 29 | 0.38% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 30 | 0.37% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |

GC pauses: 0, total 0 ms

