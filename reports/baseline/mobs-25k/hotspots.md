# JVM hotspots (JFR ExecutionSample)

Samples: 39862

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 32.61% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.13% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 3.69% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 3.46% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 5 | 2.52% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 6 | 1.47% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 1.25% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 8 | 1.24% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 9 | 1.13% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 10 | 1.06% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 11 | 0.94% | `java.util.HashMap.getNode(Object) line: 586` |
| 12 | 0.86% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 13 | 0.84% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 14 | 0.82% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 15 | 0.80% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 16 | 0.79% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 17 | 0.72% | `java.util.EnumMap.get(Object) line: 248` |
| 18 | 0.68% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 19 | 0.62% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 20 | 0.61% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 21 | 0.58% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 22 | 0.56% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 23 | 0.54% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3926` |
| 24 | 0.54% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 25 | 0.49% | `jdk.internal.util.ArraysSupport.hashCodeOfUnsigned(byte[], int, int, int) line: 275` |
| 26 | 0.49% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 27 | 0.49% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 28 | 0.49% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 29 | 0.46% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 30 | 0.44% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 31 | 0.41% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 32 | 0.41% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 33 | 0.41% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 34 | 0.40% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 35 | 0.37% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 36 | 0.37% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 37 | 0.37% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(BlockPos) line: 15` |
| 38 | 0.36% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 39 | 0.36% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 40 | 0.34% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 41 | 0.33% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 42 | 0.33% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 43 | 0.32% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 44 | 0.31% | `java.lang.String.hashCode() line: 2430` |
| 45 | 0.31% | `java.util.Optional.isPresent() line: 154` |
| 46 | 0.31% | `net.minecraft.world.item.ItemStack.getDamageValue() line: 573` |
| 47 | 0.30% | `java.util.ArrayDeque.size() line: 647` |
| 48 | 0.30% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 49 | 0.30% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 50 | 0.28% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 51 | 0.27% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 52 | 0.26% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 53 | 0.25% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 54 | 0.25% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 55 | 0.25% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 56 | 0.25% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 57 | 0.23% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 58 | 0.23% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 59 | 0.22% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 60 | 0.22% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 32.61% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 7.49% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 7.15% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 2.30% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 1.25% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 6 | 1.13% | `net.minecraft.world.entity.ConversionTracker.tick() line: 79` |
| 7 | 1.07% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 8 | 0.88% | `java.util.HashMap.containsKey(Object) line: 603` |
| 9 | 0.86% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 10 | 0.83% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 11 | 0.71% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 12 | 0.68% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 13 | 0.67% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 14 | 0.66% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2510` |
| 15 | 0.63% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 16 | 0.62% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 17 | 0.60% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3810` |
| 18 | 0.55% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 19 | 0.54% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 20 | 0.54% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 21 | 0.51% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 22 | 0.50% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 23 | 0.49% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 24 | 0.49% | `java.lang.StringLatin1.hashCode(byte[]) line: 195` |
| 25 | 0.49% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 26 | 0.47% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 27 | 0.46% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 28 | 0.46% | `net.minecraft.server.level.ServerLevel.isPositionEntityTicking(BlockPos) line: 2956` |
| 29 | 0.44% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3799` |
| 30 | 0.44% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |

GC pauses: 0, total 0 ms

