# JVM hotspots (JFR ExecutionSample)

Samples: 38438

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 25.40% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 5.03% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 4.98% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 2.52% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 5 | 2.38% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4706` |
| 6 | 1.53% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 1.23% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 8 | 0.99% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 9 | 0.95% | `java.util.EnumMap.get(Object) line: 248` |
| 10 | 0.85% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 11 | 0.85% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 12 | 0.82% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 13 | 0.79% | `java.util.HashMap.getNode(Object) line: 586` |
| 14 | 0.72% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3907` |
| 15 | 0.70% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 16 | 0.70% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 17 | 0.64% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 18 | 0.64% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 19 | 0.58% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 20 | 0.57% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 21 | 0.57% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3868` |
| 22 | 0.56% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 23 | 0.54% | `java.lang.String.hashCode() line: 2430` |
| 24 | 0.53% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 25 | 0.50% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 26 | 0.50% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 27 | 0.47% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3926` |
| 28 | 0.47% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 29 | 0.46% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 30 | 0.45% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 31 | 0.44% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 32 | 0.41% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 33 | 0.40% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 34 | 0.40% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 35 | 0.40% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 36 | 0.40% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 37 | 0.39% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 38 | 0.38% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 39 | 0.37% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 40 | 0.37% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 41 | 0.36% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 42 | 0.36% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 43 | 0.34% | `java.util.ArrayDeque.size() line: 647` |
| 44 | 0.33% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 45 | 0.33% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 4089` |
| 46 | 0.33% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 47 | 0.32% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 48 | 0.32% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 49 | 0.31% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 50 | 0.30% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 51 | 0.29% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 52 | 0.28% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 53 | 0.27% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 54 | 0.27% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 55 | 0.25% | `java.util.Optional.isPresent() line: 154` |
| 56 | 0.24% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 57 | 0.24% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 58 | 0.23% | `java.lang.Double.isFinite(double) line: 1023` |
| 59 | 0.22% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 60 | 0.22% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 25.40% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 8.16% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 7.36% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 2.01% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 1.16% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 6 | 1.03% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 7 | 0.88% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 8 | 0.82% | `net.minecraft.world.entity.ConversionTracker.tick() line: 79` |
| 9 | 0.81% | `java.util.HashMap.containsKey(Object) line: 603` |
| 10 | 0.73% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 11 | 0.72% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 12 | 0.65% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 13 | 0.64% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 14 | 0.60% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 15 | 0.57% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 16 | 0.57% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3798` |
| 17 | 0.56% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 18 | 0.55% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 19 | 0.54% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 20 | 0.54% | `net.minecraft.world.entity.Mob.serverAiStep() line: 836` |
| 21 | 0.54% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3810` |
| 22 | 0.52% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 23 | 0.52% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 24 | 0.52% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 25 | 0.51% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2509` |
| 26 | 0.49% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 27 | 0.49% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 28 | 0.48% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 29 | 0.48% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 30 | 0.47% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |

GC pauses: 0, total 0 ms

