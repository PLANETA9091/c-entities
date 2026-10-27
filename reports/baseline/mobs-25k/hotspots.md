# JVM hotspots (JFR ExecutionSample)

Samples: 38141

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 21.71% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 5.60% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 5.53% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 3.51% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 5 | 2.71% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 6 | 2.03% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 1.35% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 8 | 1.28% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 9 | 1.23% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 10 | 1.22% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 11 | 1.11% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 12 | 0.90% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 13 | 0.85% | `java.util.HashMap.getNode(Object) line: 586` |
| 14 | 0.82% | `java.util.EnumMap.get(Object) line: 248` |
| 15 | 0.81% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 16 | 0.78% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 17 | 0.76% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 18 | 0.74% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 19 | 0.71% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 20 | 0.70% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 21 | 0.70% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 22 | 0.63% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3926` |
| 23 | 0.59% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 24 | 0.54% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 25 | 0.52% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 26 | 0.51% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 27 | 0.50% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 28 | 0.49% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 29 | 0.49% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 30 | 0.49% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 31 | 0.47% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 32 | 0.47% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 33 | 0.46% | `java.lang.String.hashCode() line: 2430` |
| 34 | 0.45% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 35 | 0.45% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 36 | 0.41% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 37 | 0.40% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 38 | 0.39% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 39 | 0.38% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 40 | 0.38% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 41 | 0.38% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 42 | 0.36% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 43 | 0.34% | `java.util.Optional.isPresent() line: 154` |
| 44 | 0.33% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 45 | 0.31% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 46 | 0.31% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 47 | 0.30% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 48 | 0.29% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 49 | 0.29% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 50 | 0.29% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 51 | 0.28% | `java.util.ArrayDeque.size() line: 647` |
| 52 | 0.28% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 53 | 0.27% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 54 | 0.25% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 55 | 0.24% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 56 | 0.24% | `java.lang.Double.isFinite(double) line: 1023` |
| 57 | 0.23% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 58 | 0.23% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 59 | 0.23% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 60 | 0.22% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 21.71% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 9.73% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 9.11% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 2.81% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 1.60% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 6 | 1.29% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 7 | 1.23% | `net.minecraft.world.entity.ConversionTracker.tick() line: 79` |
| 8 | 0.90% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 9 | 0.89% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 10 | 0.88% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 11 | 0.83% | `java.util.HashMap.containsKey(Object) line: 603` |
| 12 | 0.81% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 13 | 0.74% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 14 | 0.72% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 15 | 0.72% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 16 | 0.66% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3810` |
| 17 | 0.63% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 18 | 0.61% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 19 | 0.60% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 20 | 0.59% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 21 | 0.56% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 22 | 0.54% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 23 | 0.54% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 24 | 0.52% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2510` |
| 25 | 0.52% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 26 | 0.51% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 27 | 0.50% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 28 | 0.50% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 29 | 0.49% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 30 | 0.49% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3799` |

GC pauses: 0, total 0 ms

