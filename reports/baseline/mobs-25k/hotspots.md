# JVM hotspots (JFR ExecutionSample)

Samples: 51636

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 36.33% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.83% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 3.78% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 2.29% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 5 | 1.98% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 6 | 1.36% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 7 | 0.88% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 8 | 0.86% | `java.util.HashMap.getNode(Object) line: 586` |
| 9 | 0.86% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 10 | 0.84% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 11 | 0.75% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 12 | 0.71% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 13 | 0.65% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 14 | 0.62% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 15 | 0.62% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 16 | 0.61% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 17 | 0.60% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 18 | 0.60% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 19 | 0.58% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 20 | 0.55% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 21 | 0.50% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 22 | 0.49% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 23 | 0.49% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 24 | 0.45% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 25 | 0.45% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 26 | 0.43% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 27 | 0.43% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 4089` |
| 28 | 0.39% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 29 | 0.39% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 30 | 0.39% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 31 | 0.36% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 32 | 0.35% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 33 | 0.34% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 34 | 0.32% | `net.minecraft.core.Holder$Reference.is(TagKey) line: 195` |
| 35 | 0.30% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3926` |
| 36 | 0.30% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(BlockPos) line: 15` |
| 37 | 0.30% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 38 | 0.29% | `java.util.EnumMap.get(Object) line: 248` |
| 39 | 0.29% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 40 | 0.29% | `java.lang.ThreadLocal.get() line: 171` |
| 41 | 0.29% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 42 | 0.29% | `java.lang.String.hashCode() line: 2430` |
| 43 | 0.28% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 44 | 0.27% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 45 | 0.26% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 46 | 0.26% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 47 | 0.26% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 48 | 0.25% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 49 | 0.25% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 50 | 0.24% | `java.lang.Enum.ordinal() line: 128` |
| 51 | 0.24% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 52 | 0.24% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 53 | 0.23% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 54 | 0.23% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 55 | 0.22% | `java.util.ArrayDeque.size() line: 647` |
| 56 | 0.22% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 57 | 0.22% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 58 | 0.21% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 59 | 0.21% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 60 | 0.21% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 36.33% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.18% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 6.12% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 1.10% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 1.02% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 6 | 0.86% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 7 | 0.79% | `java.util.HashMap.containsKey(Object) line: 603` |
| 8 | 0.75% | `net.minecraft.world.entity.ConversionTracker.tick() line: 79` |
| 9 | 0.69% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 10 | 0.65% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 11 | 0.65% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 12 | 0.64% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 13 | 0.60% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 14 | 0.58% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 15 | 0.55% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 16 | 0.54% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 17 | 0.54% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 18 | 0.52% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 19 | 0.51% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 20 | 0.47% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2515` |
| 21 | 0.45% | `net.minecraft.world.entity.EntitySelector.lambda$pushableBy$0(Entity, Team, Team$CollisionRule, Entity) line: 52` |
| 22 | 0.45% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 23 | 0.45% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 24 | 0.44% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 25 | 0.41% | `net.minecraft.world.item.enchantment.Enchantment.tick(ServerLevel, int, EnchantedItemInUse, Entity) line: 400` |
| 26 | 0.41% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 27 | 0.40% | `it.unimi.dsi.fastutil.objects.ReferenceCollections$UnmodifiableCollection.contains(Object) line: 344` |
| 28 | 0.40% | `java.util.ArrayList.toArray() line: 370` |
| 29 | 0.39% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 30 | 0.39% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |

GC pauses: 0, total 0 ms

