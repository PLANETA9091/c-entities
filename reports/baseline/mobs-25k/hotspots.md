# JVM hotspots (JFR ExecutionSample)

Samples: 52501

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 37.30% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.89% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 2.67% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 4 | 2.24% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 5 | 2.11% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 6 | 1.93% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 7 | 1.09% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 8 | 0.95% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 9 | 0.90% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 10 | 0.85% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 11 | 0.80% | `java.util.HashMap.getNode(Object) line: 586` |
| 12 | 0.79% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 13 | 0.77% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 14 | 0.71% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 15 | 0.66% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 16 | 0.65% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 17 | 0.63% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 18 | 0.56% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 19 | 0.50% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 20 | 0.48% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 21 | 0.47% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 22 | 0.44% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 23 | 0.43% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 24 | 0.42% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 25 | 0.41% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 26 | 0.41% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 27 | 0.40% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 28 | 0.38% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 29 | 0.37% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 30 | 0.36% | `java.util.EnumMap.get(Object) line: 248` |
| 31 | 0.35% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 32 | 0.34% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 33 | 0.34% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 34 | 0.32% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 35 | 0.31% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 36 | 0.31% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 37 | 0.30% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 38 | 0.29% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(BlockPos) line: 15` |
| 39 | 0.29% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 40 | 0.29% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 41 | 0.29% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3926` |
| 42 | 0.28% | `java.lang.String.hashCode() line: 2430` |
| 43 | 0.27% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 44 | 0.27% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 45 | 0.27% | `net.minecraft.world.level.levelgen.BitRandomSource.nextInt(int) line: 31` |
| 46 | 0.26% | `net.minecraft.core.Holder$Reference.is(TagKey) line: 195` |
| 47 | 0.26% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 48 | 0.26% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 49 | 0.25% | `java.lang.Enum.ordinal() line: 128` |
| 50 | 0.25% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 4089` |
| 51 | 0.25% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 52 | 0.25% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 53 | 0.23% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 54 | 0.23% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 55 | 0.22% | `java.util.AbstractMap$SimpleImmutableEntry.getKey() line: 774` |
| 56 | 0.22% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 57 | 0.22% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 58 | 0.22% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 59 | 0.20% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 60 | 0.20% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 37.30% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.55% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 6.28% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 1.21% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 1.02% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 6 | 0.85% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 7 | 0.84% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 8 | 0.77% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 9 | 0.71% | `net.minecraft.world.entity.ConversionTracker.tick() line: 79` |
| 10 | 0.67% | `java.util.HashMap.containsKey(Object) line: 603` |
| 11 | 0.67% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 12 | 0.66% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 13 | 0.66% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 14 | 0.56% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 15 | 0.55% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 16 | 0.53% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 17 | 0.51% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 18 | 0.50% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 19 | 0.49% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 20 | 0.48% | `java.util.ArrayList.toArray() line: 370` |
| 21 | 0.47% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 22 | 0.45% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2515` |
| 23 | 0.44% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 24 | 0.44% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 25 | 0.42% | `net.minecraft.server.level.ServerLevel.isPositionEntityTicking(BlockPos) line: 2956` |
| 26 | 0.39% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 27 | 0.39% | `net.minecraft.world.entity.Mob.serverAiStep() line: 836` |
| 28 | 0.38% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 29 | 0.38% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3810` |
| 30 | 0.38% | `net.minecraft.world.item.enchantment.Enchantment.tick(ServerLevel, int, EnchantedItemInUse, Entity) line: 400` |

GC pauses: 0, total 0 ms

