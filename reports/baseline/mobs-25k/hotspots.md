# JVM hotspots (JFR ExecutionSample)

Samples: 36079

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 23.93% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 5.48% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 5.23% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 2.24% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4706` |
| 5 | 2.06% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 6 | 1.49% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 1.19% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 8 | 1.19% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 9 | 1.03% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 10 | 0.88% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3907` |
| 11 | 0.84% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 12 | 0.83% | `java.util.HashMap.getNode(Object) line: 586` |
| 13 | 0.76% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 14 | 0.73% | `java.util.EnumMap.get(Object) line: 248` |
| 15 | 0.71% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 16 | 0.69% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 17 | 0.69% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 18 | 0.68% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 19 | 0.67% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 20 | 0.62% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 21 | 0.61% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 22 | 0.59% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 23 | 0.54% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 24 | 0.51% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3926` |
| 25 | 0.48% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 26 | 0.47% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3868` |
| 27 | 0.46% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 28 | 0.45% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 29 | 0.45% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 30 | 0.45% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 31 | 0.42% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 32 | 0.42% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 33 | 0.40% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 34 | 0.39% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 35 | 0.38% | `java.lang.String.hashCode() line: 2430` |
| 36 | 0.38% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 37 | 0.38% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 38 | 0.37% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 39 | 0.36% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 40 | 0.35% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap$MapIterator.nextEntry() line: 655` |
| 41 | 0.33% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 42 | 0.33% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 43 | 0.33% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 44 | 0.32% | `net.minecraft.world.item.ItemStack.getDamageValue() line: 573` |
| 45 | 0.32% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 46 | 0.32% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 47 | 0.31% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 48 | 0.30% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 49 | 0.30% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 50 | 0.30% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 51 | 0.29% | `java.util.Optional.isPresent() line: 154` |
| 52 | 0.27% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 53 | 0.27% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 54 | 0.27% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 55 | 0.26% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 56 | 0.25% | `java.lang.Double.isFinite(double) line: 1023` |
| 57 | 0.25% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 58 | 0.25% | `java.lang.Enum.ordinal() line: 128` |
| 59 | 0.25% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 60 | 0.25% | `java.util.AbstractMap$SimpleImmutableEntry.getKey() line: 774` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 23.93% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 8.53% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 7.72% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 2.16% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 1.19% | `net.minecraft.world.entity.ConversionTracker.tick() line: 79` |
| 6 | 0.98% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 7 | 0.91% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 8 | 0.89% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 9 | 0.78% | `java.util.HashMap.containsKey(Object) line: 603` |
| 10 | 0.70% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 11 | 0.69% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 12 | 0.69% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 13 | 0.63% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 14 | 0.62% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 15 | 0.61% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 16 | 0.59% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 17 | 0.59% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3810` |
| 18 | 0.57% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 19 | 0.56% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 20 | 0.55% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 21 | 0.55% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 22 | 0.53% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 23 | 0.52% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 24 | 0.51% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 25 | 0.49% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 26 | 0.48% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 27 | 0.48% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 28 | 0.47% | `net.minecraft.world.entity.Mob.serverAiStep() line: 843` |
| 29 | 0.47% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3798` |
| 30 | 0.46% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |

GC pauses: 0, total 0 ms

