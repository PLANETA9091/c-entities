# JVM hotspots (JFR ExecutionSample)

Samples: 52917

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 35.05% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.05% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 3.82% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 2.24% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 5 | 2.21% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 6 | 1.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 7 | 0.93% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 8 | 0.92% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 9 | 0.87% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 10 | 0.87% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 11 | 0.85% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 12 | 0.80% | `java.util.HashMap.getNode(Object) line: 586` |
| 13 | 0.71% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 14 | 0.70% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 15 | 0.66% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 16 | 0.65% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 17 | 0.60% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 18 | 0.58% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 19 | 0.55% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 20 | 0.53% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 21 | 0.51% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 22 | 0.49% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 23 | 0.48% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 24 | 0.47% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 25 | 0.47% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 26 | 0.45% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 27 | 0.44% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 28 | 0.40% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 29 | 0.40% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 30 | 0.40% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 31 | 0.39% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 32 | 0.39% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 33 | 0.37% | `java.util.EnumMap.get(Object) line: 248` |
| 34 | 0.36% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 35 | 0.34% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 36 | 0.33% | `java.lang.String.hashCode() line: 2430` |
| 37 | 0.33% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 38 | 0.32% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3926` |
| 39 | 0.31% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 40 | 0.30% | `jdk.internal.util.ArraysSupport.hashCodeOfUnsigned(byte[], int, int, int) line: 275` |
| 41 | 0.30% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 42 | 0.30% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 43 | 0.28% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 44 | 0.28% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 45 | 0.27% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 46 | 0.26% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 47 | 0.26% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(BlockPos) line: 15` |
| 48 | 0.25% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 49 | 0.25% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 50 | 0.25% | `java.util.stream.MatchOps$1MatchSink.accept(Object) line: 90` |
| 51 | 0.23% | `java.lang.Enum.ordinal() line: 128` |
| 52 | 0.23% | `java.util.Optional.isPresent() line: 154` |
| 53 | 0.23% | `java.util.AbstractMap$SimpleImmutableEntry.getKey() line: 774` |
| 54 | 0.22% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 55 | 0.22% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 56 | 0.22% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 4089` |
| 57 | 0.22% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 58 | 0.21% | `net.minecraft.core.Holder$Reference.is(TagKey) line: 195` |
| 59 | 0.21% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 60 | 0.21% | `java.util.ArrayDeque.size() line: 647` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 35.05% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.28% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 6.26% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 1.41% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 1.09% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 6 | 0.92% | `net.minecraft.world.entity.ConversionTracker.tick() line: 79` |
| 7 | 0.91% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 8 | 0.89% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 9 | 0.87% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 10 | 0.75% | `java.util.HashMap.containsKey(Object) line: 603` |
| 11 | 0.71% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 12 | 0.70% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 13 | 0.59% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 14 | 0.56% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 15 | 0.55% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 16 | 0.55% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 17 | 0.54% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2510` |
| 18 | 0.51% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 19 | 0.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 20 | 0.49% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 21 | 0.47% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 22 | 0.47% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 23 | 0.46% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 24 | 0.46% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 25 | 0.45% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 26 | 0.43% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 27 | 0.42% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 28 | 0.42% | `net.minecraft.world.item.enchantment.Enchantment.tick(ServerLevel, int, EnchantedItemInUse, Entity) line: 400` |
| 29 | 0.42% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3810` |
| 30 | 0.41% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |

GC pauses: 0, total 0 ms

