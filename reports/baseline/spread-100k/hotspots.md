# JVM hotspots (JFR ExecutionSample)

Samples: 63223

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 13.41% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.54% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 3.26% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 3.14% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 5 | 2.13% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 2.00% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 1.88% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 1.76% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 9 | 1.73% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 10 | 1.69% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 11 | 1.62% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 12 | 1.56% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 13 | 1.33% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 14 | 1.33% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 15 | 1.25% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 16 | 1.24% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 17 | 1.17% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 18 | 1.16% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 19 | 1.05% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 20 | 1.03% | `java.util.EnumMap.get(Object) line: 248` |
| 21 | 0.99% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 22 | 0.98% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 23 | 0.94% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 24 | 0.87% | `java.util.ArrayDeque.pollFirst() line: 378` |
| 25 | 0.85% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 26 | 0.84% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 27 | 0.83% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 28 | 0.82% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 29 | 0.73% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 30 | 0.69% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 31 | 0.67% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 32 | 0.66% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 33 | 0.61% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 34 | 0.58% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 35 | 0.58% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 36 | 0.56% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 37 | 0.51% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 38 | 0.47% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 39 | 0.46% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 40 | 0.45% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 41 | 0.44% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 42 | 0.44% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 43 | 0.44% | `java.util.HashMap.getNode(Object) line: 586` |
| 44 | 0.44% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 45 | 0.43% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 46 | 0.42% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 47 | 0.42% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 48 | 0.39% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 49 | 0.37% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 323` |
| 50 | 0.37% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 51 | 0.37% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 52 | 0.36% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 53 | 0.36% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 54 | 0.36% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 55 | 0.35% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 56 | 0.35% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 57 | 0.35% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 58 | 0.35% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 92` |
| 59 | 0.33% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 60 | 0.32% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 13.41% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 5.38% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 3.54% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 4 | 3.37% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 5 | 3.14% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 6 | 2.95% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 7 | 2.32% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 8 | 2.29% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 1.88% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 10 | 1.84% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 11 | 1.84% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 12 | 1.36% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 13 | 1.34% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 14 | 1.28% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 15 | 1.24% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 16 | 1.13% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 17 | 1.10% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 18 | 1.05% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 19 | 1.02% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 20 | 0.87% | `java.util.ArrayDeque.removeFirst() line: 358` |
| 21 | 0.87% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 22 | 0.83% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |
| 23 | 0.73% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 24 | 0.71% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 25 | 0.69% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 26 | 0.67% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 27 | 0.66% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 28 | 0.64% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 29 | 0.64% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 30 | 0.64% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |

GC pauses: 0, total 0 ms

