# JVM hotspots (JFR ExecutionSample)

Samples: 79545

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 27.04% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.60% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.50% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 3.25% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3907` |
| 5 | 3.20% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4706` |
| 6 | 2.67% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 7 | 2.33% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 1.89% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.75% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 10 | 1.58% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2403` |
| 11 | 1.26% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 12 | 0.99% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 13 | 0.99% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 14 | 0.96% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 15 | 0.91% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 16 | 0.89% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 17 | 0.89% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 18 | 0.71% | `java.util.EnumMap.get(Object) line: 248` |
| 19 | 0.71% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 20 | 0.70% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 21 | 0.68% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 22 | 0.67% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 23 | 0.65% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 24 | 0.64% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3868` |
| 25 | 0.63% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 26 | 0.61% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 27 | 0.60% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 28 | 0.59% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 29 | 0.54% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 30 | 0.48% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 31 | 0.47% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 32 | 0.47% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 33 | 0.46% | `java.lang.String.equals(Object) line: 1926` |
| 34 | 0.39% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 35 | 0.38% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 36 | 0.37% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 37 | 0.37% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1574` |
| 38 | 0.36% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 39 | 0.35% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 40 | 0.34% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 41 | 0.33% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 42 | 0.33% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 43 | 0.31% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 44 | 0.31% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 45 | 0.30% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 46 | 0.29% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 47 | 0.29% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 150` |
| 48 | 0.29% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 49 | 0.29% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 50 | 0.29% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 51 | 0.29% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 52 | 0.28% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 53 | 0.26% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 54 | 0.26% | `java.util.Optional.ifPresent(Consumer) line: 178` |
| 55 | 0.26% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 56 | 0.25% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 92` |
| 57 | 0.25% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 58 | 0.25% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 59 | 0.24% | `net.minecraft.world.entity.Entity.baseTick() line: 913` |
| 60 | 0.24% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 27.04% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 5.49% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 5.35% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 3.95% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 5 | 3.50% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 6 | 3.25% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 7 | 2.26% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 8 | 2.18% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 9 | 1.62% | `net.minecraft.world.entity.Entity.setOldPos() line: 2395` |
| 10 | 1.17% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 11 | 0.99% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 12 | 0.96% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 13 | 0.91% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 14 | 0.89% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 15 | 0.71% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 16 | 0.67% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 17 | 0.66% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2212` |
| 18 | 0.65% | `net.minecraft.world.entity.Entity.applyGravity() line: 2090` |
| 19 | 0.63% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 969` |
| 20 | 0.60% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 21 | 0.54% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 22 | 0.51% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 23 | 0.50% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 24 | 0.48% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 25 | 0.47% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1475` |
| 26 | 0.46% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 27 | 0.46% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 28 | 0.42% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1411` |
| 29 | 0.42% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 30 | 0.42% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |

GC pauses: 0, total 0 ms

