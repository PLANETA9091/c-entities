# JVM hotspots (JFR ExecutionSample)

Samples: 51890

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 35.76% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.86% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 3.71% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 2.33% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 5 | 1.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 6 | 1.12% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 1.10% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 8 | 0.94% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 9 | 0.89% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 10 | 0.89% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 11 | 0.88% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 0.86% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 13 | 0.82% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 14 | 0.81% | `java.util.HashMap.getNode(Object) line: 586` |
| 15 | 0.78% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 16 | 0.74% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 17 | 0.58% | `java.util.EnumMap.get(Object) line: 248` |
| 18 | 0.58% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 19 | 0.56% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 20 | 0.51% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 21 | 0.51% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 22 | 0.50% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 23 | 0.50% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 24 | 0.50% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 25 | 0.45% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 26 | 0.44% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 27 | 0.40% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 28 | 0.40% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 29 | 0.39% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 30 | 0.36% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 31 | 0.34% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 32 | 0.34% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 33 | 0.33% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 34 | 0.33% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 35 | 0.33% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 36 | 0.33% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3926` |
| 37 | 0.32% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 38 | 0.31% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 39 | 0.31% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 40 | 0.30% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 41 | 0.30% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 42 | 0.29% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 43 | 0.29% | `java.lang.String.hashCode() line: 2430` |
| 44 | 0.28% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 45 | 0.28% | `java.lang.Enum.ordinal() line: 128` |
| 46 | 0.28% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 4089` |
| 47 | 0.28% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 48 | 0.27% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 49 | 0.27% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 50 | 0.26% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 51 | 0.25% | `java.util.ArrayDeque.size() line: 647` |
| 52 | 0.25% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(BlockPos) line: 15` |
| 53 | 0.24% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 54 | 0.24% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 55 | 0.24% | `java.util.AbstractMap$SimpleImmutableEntry.getKey() line: 774` |
| 56 | 0.24% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 57 | 0.23% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 58 | 0.22% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 59 | 0.22% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 60 | 0.22% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 35.76% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.23% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 6.18% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 1.62% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 1.10% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 6 | 0.89% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 7 | 0.89% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 8 | 0.86% | `net.minecraft.world.entity.ConversionTracker.tick() line: 79` |
| 9 | 0.83% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 10 | 0.80% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 11 | 0.74% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 12 | 0.72% | `java.util.HashMap.containsKey(Object) line: 603` |
| 13 | 0.58% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 14 | 0.56% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 15 | 0.54% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 16 | 0.54% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 17 | 0.53% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 18 | 0.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 19 | 0.50% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 20 | 0.45% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 21 | 0.45% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2510` |
| 22 | 0.43% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 23 | 0.41% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 24 | 0.41% | `net.minecraft.world.item.enchantment.Enchantment.tick(ServerLevel, int, EnchantedItemInUse, Entity) line: 400` |
| 25 | 0.41% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 26 | 0.40% | `net.minecraft.world.entity.Entity.getBlockPosBelowThatAffectsMyMovement() line: 1567` |
| 27 | 0.40% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 28 | 0.40% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 29 | 0.40% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3810` |
| 30 | 0.37% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |

GC pauses: 0, total 0 ms

