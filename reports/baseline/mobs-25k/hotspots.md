# JVM hotspots (JFR ExecutionSample)

Samples: 38240

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 24.17% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 5.54% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 4.95% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 2.30% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 5 | 1.99% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 6 | 1.30% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 1.20% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 8 | 1.04% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 9 | 0.99% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 10 | 0.96% | `java.util.HashMap.getNode(Object) line: 586` |
| 11 | 0.89% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 12 | 0.80% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 13 | 0.79% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 14 | 0.78% | `java.util.EnumMap.get(Object) line: 248` |
| 15 | 0.76% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 16 | 0.76% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 17 | 0.75% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 18 | 0.70% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 19 | 0.66% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 20 | 0.66% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 21 | 0.66% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 22 | 0.65% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 23 | 0.54% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 24 | 0.51% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3926` |
| 25 | 0.49% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 26 | 0.48% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 27 | 0.47% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 28 | 0.46% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 29 | 0.44% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 30 | 0.43% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 31 | 0.41% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 32 | 0.41% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 33 | 0.40% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 34 | 0.40% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 35 | 0.37% | `java.lang.String.hashCode() line: 2430` |
| 36 | 0.37% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 37 | 0.37% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 38 | 0.36% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 39 | 0.36% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 40 | 0.36% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 41 | 0.35% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 4089` |
| 42 | 0.35% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 43 | 0.33% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 44 | 0.33% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 45 | 0.33% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 46 | 0.32% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 47 | 0.32% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 48 | 0.32% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 49 | 0.31% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 50 | 0.30% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 51 | 0.29% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 52 | 0.29% | `java.lang.Enum.ordinal() line: 128` |
| 53 | 0.29% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 54 | 0.29% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 55 | 0.29% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 56 | 0.28% | `java.util.ArrayDeque.size() line: 647` |
| 57 | 0.24% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 58 | 0.23% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 59 | 0.23% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 60 | 0.22% | `java.util.EnumMap.isValidKey(Object) line: 320` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 24.17% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 8.10% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 7.84% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 1.96% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 1.05% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 6 | 1.00% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 7 | 0.99% | `net.minecraft.world.entity.ConversionTracker.tick() line: 79` |
| 8 | 0.95% | `java.util.HashMap.containsKey(Object) line: 603` |
| 9 | 0.90% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 10 | 0.80% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 11 | 0.70% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 12 | 0.70% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 13 | 0.70% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 14 | 0.69% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 15 | 0.68% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 16 | 0.65% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 17 | 0.64% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 18 | 0.55% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3810` |
| 19 | 0.55% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 20 | 0.54% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 21 | 0.54% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 22 | 0.54% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 23 | 0.51% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2510` |
| 24 | 0.50% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 25 | 0.49% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3799` |
| 26 | 0.48% | `net.minecraft.world.entity.Mob.serverAiStep() line: 843` |
| 27 | 0.47% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 28 | 0.46% | `net.minecraft.world.entity.Mob.serverAiStep() line: 836` |
| 29 | 0.45% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 30 | 0.45% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |

GC pauses: 0, total 0 ms

