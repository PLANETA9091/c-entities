# JVM hotspots (JFR ExecutionSample)

Samples: 76652

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 26.03% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.45% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 3 | 3.45% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 3.22% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 3.11% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 6 | 3.01% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 1.87% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 1.77% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 9 | 1.60% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 10 | 1.37% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 11 | 1.33% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 12 | 1.28% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 13 | 1.23% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 14 | 1.17% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 15 | 1.17% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 16 | 1.10% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 17 | 1.09% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 18 | 1.02% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 19 | 0.90% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 20 | 0.84% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 21 | 0.76% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 22 | 0.75% | `java.util.EnumMap.get(Object) line: 248` |
| 23 | 0.71% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 24 | 0.61% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 25 | 0.59% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 26 | 0.58% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 27 | 0.57% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 28 | 0.55% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 29 | 0.54% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 30 | 0.52% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 31 | 0.48% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 32 | 0.48% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 33 | 0.47% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 34 | 0.46% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 35 | 0.45% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 36 | 0.39% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 37 | 0.38% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 92` |
| 38 | 0.37% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 39 | 0.37% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 40 | 0.36% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 41 | 0.36% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 42 | 0.35% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 43 | 0.32% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 44 | 0.31% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 45 | 0.31% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 46 | 0.30% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 47 | 0.30% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 48 | 0.29% | `net.minecraft.server.level.ServerChunkCache.pollTask() line: 449` |
| 49 | 0.29% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 50 | 0.27% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 51 | 0.27% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 52 | 0.27% | `net.minecraft.world.entity.Entity.baseTick() line: 914` |
| 53 | 0.26% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 54 | 0.25% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 55 | 0.24% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 56 | 0.24% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 57 | 0.23% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 58 | 0.23% | `java.util.ArrayDeque.size() line: 647` |
| 59 | 0.23% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 60 | 0.22% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 26.03% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.39% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 4.88% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 4.83% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 5 | 3.45% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 6 | 3.22% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 7 | 2.27% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 8 | 2.03% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 1.85% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 10 | 1.34% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 11 | 1.23% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 12 | 1.09% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 13 | 1.00% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 14 | 0.92% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 15 | 0.83% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 16 | 0.82% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 17 | 0.76% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 18 | 0.71% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 19 | 0.64% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 20 | 0.64% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 21 | 0.61% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 22 | 0.61% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 23 | 0.54% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 24 | 0.50% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3584` |
| 25 | 0.48% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 26 | 0.47% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |
| 27 | 0.44% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 28 | 0.43% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 29 | 0.40% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 30 | 0.39% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |

GC pauses: 0, total 0 ms

