# JVM hotspots (JFR ExecutionSample)

Samples: 50311

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 31.77% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.86% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 3.48% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 3.22% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 5 | 2.53% | `java.util.HashMap.getNode(Object) line: 586` |
| 6 | 2.33% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 7 | 1.84% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 8 | 0.92% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 9 | 0.84% | `java.util.EnumMap.get(Object) line: 248` |
| 10 | 0.84% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 11 | 0.79% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 12 | 0.77% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 13 | 0.75% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 14 | 0.70% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 15 | 0.69% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 16 | 0.65% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 17 | 0.59% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 18 | 0.57% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 19 | 0.57% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 20 | 0.55% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 21 | 0.47% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 22 | 0.46% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 23 | 0.46% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3937` |
| 24 | 0.41% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 25 | 0.41% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 26 | 0.41% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 27 | 0.39% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 28 | 0.38% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 29 | 0.38% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 30 | 0.37% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 31 | 0.37% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 32 | 0.36% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 33 | 0.36% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 34 | 0.35% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 35 | 0.35% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 36 | 0.35% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 37 | 0.34% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 38 | 0.33% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 39 | 0.32% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 40 | 0.32% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 41 | 0.31% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 42 | 0.30% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 43 | 0.29% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 44 | 0.29% | `java.lang.String.hashCode() line: 2430` |
| 45 | 0.28% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 46 | 0.27% | `java.util.ArrayDeque.size() line: 647` |
| 47 | 0.26% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 48 | 0.26% | `java.lang.Enum.ordinal() line: 128` |
| 49 | 0.25% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 50 | 0.25% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 51 | 0.24% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 52 | 0.21% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 53 | 0.21% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 54 | 0.21% | `net.minecraft.world.entity.LivingEntity$SwingState.tick() line: 5026` |
| 55 | 0.21% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 56 | 0.20% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 57 | 0.20% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 58 | 0.19% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 59 | 0.19% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 60 | 0.19% | `java.util.EnumMap.isValidKey(Object) line: 320` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 31.77% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.97% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 6.70% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 2.33% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3803` |
| 5 | 2.23% | `java.util.HashMap.containsKey(Object) line: 603` |
| 6 | 1.47% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 7 | 0.94% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 8 | 0.83% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 9 | 0.77% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 10 | 0.75% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 11 | 0.69% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 12 | 0.68% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2515` |
| 13 | 0.60% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 14 | 0.59% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3821` |
| 15 | 0.57% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 16 | 0.53% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 17 | 0.52% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 18 | 0.48% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 19 | 0.48% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 20 | 0.47% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 848` |
| 21 | 0.44% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 22 | 0.43% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 23 | 0.43% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 24 | 0.40% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 25 | 0.40% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 26 | 0.40% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 27 | 0.39% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 28 | 0.38% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 29 | 0.38% | `java.util.HashMap.get(Object) line: 565` |
| 30 | 0.38% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |

GC pauses: 0, total 0 ms

