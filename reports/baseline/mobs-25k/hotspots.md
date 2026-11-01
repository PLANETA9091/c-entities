# JVM hotspots (JFR ExecutionSample)

Samples: 54705

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 37.51% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.70% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 3.45% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 2.44% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 5 | 1.91% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 6 | 1.78% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 7 | 1.01% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 0.98% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 9 | 0.79% | `java.util.HashMap.getNode(Object) line: 586` |
| 10 | 0.79% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 11 | 0.78% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 12 | 0.75% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 13 | 0.71% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 14 | 0.63% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 15 | 0.60% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 16 | 0.57% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 17 | 0.57% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 18 | 0.52% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 19 | 0.52% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 20 | 0.49% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 21 | 0.48% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 22 | 0.47% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 23 | 0.44% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 24 | 0.41% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 237` |
| 25 | 0.38% | `java.util.EnumMap.get(Object) line: 248` |
| 26 | 0.37% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 27 | 0.37% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 28 | 0.36% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 29 | 0.35% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 30 | 0.33% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 31 | 0.33% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3926` |
| 32 | 0.32% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 4089` |
| 33 | 0.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 34 | 0.31% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 35 | 0.30% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 36 | 0.30% | `java.lang.String.hashCode() line: 2430` |
| 37 | 0.30% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1484` |
| 38 | 0.30% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 39 | 0.29% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 40 | 0.29% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 41 | 0.28% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 42 | 0.28% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 43 | 0.28% | `java.lang.Enum.ordinal() line: 128` |
| 44 | 0.27% | `net.minecraft.server.level.ServerChunkCache.pollTask() line: 449` |
| 45 | 0.27% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 46 | 0.25% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 47 | 0.24% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 48 | 0.23% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 49 | 0.23% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 50 | 0.23% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 51 | 0.23% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 52 | 0.22% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 53 | 0.22% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 54 | 0.22% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 55 | 0.21% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 56 | 0.21% | `java.util.ArrayDeque.size() line: 647` |
| 57 | 0.20% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(BlockPos) line: 15` |
| 58 | 0.20% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 59 | 0.19% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 60 | 0.19% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 37.51% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.13% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 6.03% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 1.49% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 0.98% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 6 | 0.95% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 7 | 0.78% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 8 | 0.78% | `net.minecraft.world.entity.ConversionTracker.tick() line: 79` |
| 9 | 0.77% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 10 | 0.76% | `java.util.HashMap.containsKey(Object) line: 603` |
| 11 | 0.73% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 12 | 0.70% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 13 | 0.63% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 14 | 0.60% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 15 | 0.51% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 16 | 0.51% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 17 | 0.46% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 18 | 0.45% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 19 | 0.44% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 20 | 0.42% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 21 | 0.40% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2515` |
| 22 | 0.40% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 23 | 0.39% | `it.unimi.dsi.fastutil.objects.ReferenceCollections$UnmodifiableCollection.contains(Object) line: 344` |
| 24 | 0.39% | `net.minecraft.world.item.enchantment.Enchantment.tick(ServerLevel, int, EnchantedItemInUse, Entity) line: 400` |
| 25 | 0.39% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 26 | 0.37% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3810` |
| 27 | 0.37% | `java.util.ArrayList.toArray() line: 370` |
| 28 | 0.37% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 29 | 0.36% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 30 | 0.35% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |

GC pauses: 0, total 0 ms

