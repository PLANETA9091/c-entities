# JVM hotspots (JFR ExecutionSample)

Samples: 31965

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 27.88% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.18% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 4.13% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 2.85% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 5 | 2.47% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 6 | 1.70% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 7 | 1.56% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 8 | 1.46% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 9 | 1.45% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 10 | 1.40% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 11 | 1.34% | `java.util.HashMap.getNode(Object) line: 586` |
| 12 | 1.15% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 13 | 1.10% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 14 | 1.04% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 15 | 1.03% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 16 | 0.90% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 17 | 0.80% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 18 | 0.80% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3926` |
| 19 | 0.78% | `java.util.EnumMap.get(Object) line: 248` |
| 20 | 0.76% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 21 | 0.70% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 22 | 0.67% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 23 | 0.67% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 24 | 0.67% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 25 | 0.64% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 26 | 0.60% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 27 | 0.57% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 28 | 0.56% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 29 | 0.54% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 30 | 0.54% | `jdk.internal.util.ArraysSupport.hashCodeOfUnsigned(byte[], int, int, int) line: 275` |
| 31 | 0.51% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 32 | 0.49% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 33 | 0.46% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 34 | 0.46% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getNextAcquire() line: 1837` |
| 35 | 0.44% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 36 | 0.44% | `java.util.ArrayDeque.size() line: 647` |
| 37 | 0.43% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 38 | 0.43% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 39 | 0.40% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 40 | 0.40% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(BlockPos) line: 15` |
| 41 | 0.39% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 42 | 0.38% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 43 | 0.37% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 44 | 0.36% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 45 | 0.34% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 46 | 0.34% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 47 | 0.34% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 48 | 0.33% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 49 | 0.33% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 50 | 0.32% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 4089` |
| 51 | 0.31% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 52 | 0.29% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 53 | 0.28% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 54 | 0.27% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 55 | 0.24% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 56 | 0.24% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 993` |
| 57 | 0.23% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 58 | 0.23% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 59 | 0.22% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 60 | 0.21% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 27.88% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 7.64% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 6.60% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 1.99% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 1.56% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 6 | 1.48% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 7 | 1.46% | `net.minecraft.world.entity.ConversionTracker.tick() line: 79` |
| 8 | 1.34% | `java.util.HashMap.containsKey(Object) line: 603` |
| 9 | 1.11% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 10 | 1.04% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 11 | 1.03% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 12 | 0.91% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 13 | 0.90% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 14 | 0.87% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 15 | 0.87% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 16 | 0.84% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3810` |
| 17 | 0.78% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 18 | 0.78% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2515` |
| 19 | 0.71% | `net.minecraft.world.entity.Entity.getBlockPosBelowThatAffectsMyMovement() line: 1567` |
| 20 | 0.67% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 21 | 0.66% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 22 | 0.65% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 23 | 0.64% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 24 | 0.64% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 25 | 0.60% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 26 | 0.60% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3804` |
| 27 | 0.58% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 28 | 0.57% | `net.minecraft.world.entity.Mob.serverAiStep() line: 843` |
| 29 | 0.56% | `net.minecraft.world.entity.Mob.serverAiStep() line: 836` |
| 30 | 0.54% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |

GC pauses: 0, total 0 ms

