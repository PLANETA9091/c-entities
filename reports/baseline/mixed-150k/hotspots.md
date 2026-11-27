# JVM hotspots (JFR ExecutionSample)

Samples: 100202

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 37.09% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.26% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 3 | 3.17% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 2.72% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 5 | 2.63% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 1.57% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 1.46% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 1.39% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 9 | 1.36% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 10 | 1.29% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 11 | 1.14% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 12 | 0.86% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 13 | 0.70% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 14 | 0.68% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 15 | 0.67% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 16 | 0.67% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 17 | 0.61% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 18 | 0.58% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 19 | 0.57% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 20 | 0.57% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 21 | 0.55% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 22 | 0.55% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 23 | 0.55% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 24 | 0.54% | `java.util.EnumMap.get(Object) line: 248` |
| 25 | 0.53% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 26 | 0.52% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 27 | 0.46% | `net.minecraft.world.entity.LivingEntity.updateGlowingStatus() line: 1025` |
| 28 | 0.43% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 29 | 0.43% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 30 | 0.40% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 31 | 0.40% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 32 | 0.39% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 33 | 0.38% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 34 | 0.38% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 35 | 0.35% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 36 | 0.35% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 37 | 0.34% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 38 | 0.33% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 39 | 0.32% | `net.minecraft.world.item.ItemStack.getMaxDamage() line: 582` |
| 40 | 0.32% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 41 | 0.30% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 42 | 0.30% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 43 | 0.29% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 44 | 0.29% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 45 | 0.26% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 46 | 0.26% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 47 | 0.26% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 48 | 0.25% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 49 | 0.25% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 50 | 0.25% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 51 | 0.25% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1518` |
| 52 | 0.25% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 53 | 0.24% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 54 | 0.24% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3786` |
| 55 | 0.24% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 56 | 0.24% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 150` |
| 57 | 0.23% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 58 | 0.22% | `net.minecraft.world.entity.Entity.baseTick() line: 914` |
| 59 | 0.22% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 60 | 0.22% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 37.09% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 4.57% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 4.19% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 3.20% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 5 | 3.17% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 894` |
| 6 | 2.72% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3804` |
| 7 | 1.74% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 8 | 1.34% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 9 | 1.25% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 10 | 1.20% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 11 | 0.96% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 12 | 0.92% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 13 | 0.67% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 14 | 0.61% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 15 | 0.59% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 16 | 0.58% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 17 | 0.57% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 18 | 0.53% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 19 | 0.46% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 20 | 0.46% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 997` |
| 21 | 0.43% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 22 | 0.42% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 23 | 0.42% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 24 | 0.40% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 25 | 0.40% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 26 | 0.40% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 27 | 0.40% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 28 | 0.39% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2818` |
| 29 | 0.38% | `net.minecraft.world.entity.LivingEntity.tick() line: 3492` |
| 30 | 0.36% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |

GC pauses: 0, total 0 ms

