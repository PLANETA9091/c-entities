# JVM hotspots (JFR ExecutionSample)

Samples: 51806

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 31.94% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.48% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 3.26% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 2.40% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 5 | 2.07% | `java.util.HashMap.getNode(Object) line: 586` |
| 6 | 1.79% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 7 | 1.24% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 8 | 1.10% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 9 | 0.89% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 10 | 0.86% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 11 | 0.71% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 12 | 0.69% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 13 | 0.53% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 14 | 0.52% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 15 | 0.51% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 16 | 0.47% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 17 | 0.47% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 18 | 0.46% | `java.util.EnumMap.get(Object) line: 248` |
| 19 | 0.46% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 20 | 0.45% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 21 | 0.45% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 22 | 0.41% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 23 | 0.41% | `net.minecraft.world.item.ItemStack.getDamageValue() line: 574` |
| 24 | 0.40% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 25 | 0.38% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 26 | 0.38% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 27 | 0.38% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 28 | 0.36% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 29 | 0.35% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 30 | 0.34% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 31 | 0.34% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 32 | 0.34% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 33 | 0.33% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 34 | 0.33% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 35 | 0.33% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 36 | 0.31% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 37 | 0.30% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 38 | 0.30% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 39 | 0.29% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 40 | 0.29% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1484` |
| 41 | 0.28% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 42 | 0.27% | `java.util.ArrayDeque.size() line: 647` |
| 43 | 0.26% | `java.util.stream.MatchOps$1MatchSink.accept(Object) line: 90` |
| 44 | 0.26% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 45 | 0.26% | `java.lang.Enum.ordinal() line: 128` |
| 46 | 0.26% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 47 | 0.25% | `net.minecraft.world.entity.LivingEntity.isCollidable(boolean) line: 4094` |
| 48 | 0.25% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 49 | 0.25% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3937` |
| 50 | 0.23% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 51 | 0.22% | `net.minecraft.world.entity.LivingEntity$SwingState.tick() line: 5026` |
| 52 | 0.22% | `java.util.AbstractMap$SimpleImmutableEntry.getKey() line: 774` |
| 53 | 0.22% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 54 | 0.22% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 55 | 0.21% | `java.lang.String.hashCode() line: 2430` |
| 56 | 0.21% | `java.util.ImmutableCollections$SetN.probe(Object) line: 1258` |
| 57 | 0.21% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 58 | 0.21% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 59 | 0.20% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 60 | 0.20% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 31.94% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 5.42% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 5.27% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 2.40% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3803` |
| 5 | 1.87% | `java.util.HashMap.containsKey(Object) line: 603` |
| 6 | 1.60% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 7 | 0.92% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 8 | 0.87% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 9 | 0.71% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 894` |
| 10 | 0.68% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 11 | 0.62% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 12 | 0.58% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 13 | 0.56% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 14 | 0.53% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 15 | 0.50% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 16 | 0.49% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 17 | 0.47% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 18 | 0.47% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 19 | 0.45% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 20 | 0.43% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 21 | 0.43% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 22 | 0.41% | `java.util.ArrayList.toArray() line: 370` |
| 23 | 0.40% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 24 | 0.40% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 25 | 0.40% | `net.minecraft.world.item.enchantment.Enchantment.tick(ServerLevel, int, EnchantedItemInUse, Entity) line: 400` |
| 26 | 0.40% | `net.minecraft.world.item.ItemStack.hurtAndBreak(int, ServerLevel, LivingEntity, Consumer, boolean) line: 622` |
| 27 | 0.39% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 28 | 0.37% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 29 | 0.36% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3821` |
| 30 | 0.36% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |

GC pauses: 0, total 0 ms

