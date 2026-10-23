# JVM hotspots (JFR ExecutionSample)

Samples: 24479

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 14.11% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.77% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 2.10% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 1.50% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 5 | 1.49% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 6 | 1.46% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4706` |
| 7 | 1.34% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 8 | 1.21% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 9 | 1.21% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 10 | 1.16% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 11 | 1.10% | `java.util.HashMap.getNode(Object) line: 586` |
| 12 | 1.01% | `java.util.EnumMap.get(Object) line: 248` |
| 13 | 0.99% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3907` |
| 14 | 0.97% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 15 | 0.95% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 16 | 0.93% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 17 | 0.91% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 18 | 0.91% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 19 | 0.78% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 20 | 0.78% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 21 | 0.76% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 22 | 0.72% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 23 | 0.71% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3868` |
| 24 | 0.71% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 25 | 0.62% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 26 | 0.62% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 27 | 0.55% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 28 | 0.54% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 29 | 0.52% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 30 | 0.51% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 31 | 0.50% | `java.lang.String.hashCode() line: 2430` |
| 32 | 0.50% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3926` |
| 33 | 0.49% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 34 | 0.49% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 35 | 0.49% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 36 | 0.49% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 37 | 0.46% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 38 | 0.46% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 39 | 0.44% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 40 | 0.44% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 41 | 0.44% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 42 | 0.43% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1325` |
| 43 | 0.42% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 44 | 0.41% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(BlockPos) line: 15` |
| 45 | 0.41% | `java.util.AbstractMap$SimpleImmutableEntry.getKey() line: 774` |
| 46 | 0.40% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 47 | 0.40% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 48 | 0.39% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 49 | 0.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 50 | 0.38% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 51 | 0.37% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 52 | 0.37% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 53 | 0.36% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 54 | 0.33% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 55 | 0.32% | `java.util.Optional.isPresent() line: 154` |
| 56 | 0.32% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 57 | 0.32% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 58 | 0.32% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 59 | 0.31% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 60 | 0.31% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 14.11% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 4.66% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 4.19% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 3.02% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 1.50% | `net.minecraft.world.entity.ConversionTracker.tick() line: 79` |
| 6 | 1.39% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 7 | 1.09% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 8 | 1.01% | `java.util.HashMap.containsKey(Object) line: 603` |
| 9 | 0.99% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 10 | 0.99% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2195` |
| 11 | 0.95% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 12 | 0.87% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 13 | 0.78% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 14 | 0.78% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 15 | 0.76% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 16 | 0.76% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 17 | 0.72% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 18 | 0.71% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3798` |
| 19 | 0.67% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 20 | 0.64% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 21 | 0.63% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 22 | 0.63% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 23 | 0.62% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 24 | 0.61% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3810` |
| 25 | 0.60% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 26 | 0.59% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 27 | 0.58% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 937` |
| 28 | 0.55% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 29 | 0.54% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 30 | 0.54% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |

GC pauses: 0, total 0 ms

