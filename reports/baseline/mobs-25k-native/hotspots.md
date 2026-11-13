# JVM hotspots (JFR ExecutionSample)

Samples: 54063

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 32.77% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 3.26% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 2.21% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 5 | 2.13% | `java.util.HashMap.getNode(Object) line: 586` |
| 6 | 1.27% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 7 | 1.03% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 8 | 1.01% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 9 | 0.97% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 10 | 0.87% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 11 | 0.74% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 12 | 0.72% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 13 | 0.63% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 14 | 0.62% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 15 | 0.59% | `java.util.EnumMap.get(Object) line: 248` |
| 16 | 0.54% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 17 | 0.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 18 | 0.50% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 19 | 0.48% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 20 | 0.46% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 21 | 0.44% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 22 | 0.42% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 23 | 0.41% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 24 | 0.41% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 25 | 0.38% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 26 | 0.38% | `net.minecraft.world.item.ItemStack.getDamageValue() line: 573` |
| 27 | 0.38% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 237` |
| 28 | 0.38% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 29 | 0.35% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 30 | 0.35% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 31 | 0.34% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 32 | 0.33% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1484` |
| 33 | 0.33% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 34 | 0.33% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 35 | 0.33% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 36 | 0.32% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 37 | 0.32% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 38 | 0.32% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 39 | 0.32% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 40 | 0.32% | `java.lang.String.hashCode() line: 2430` |
| 41 | 0.31% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 42 | 0.31% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 43 | 0.30% | `java.lang.Enum.ordinal() line: 128` |
| 44 | 0.30% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 45 | 0.29% | `java.util.stream.MatchOps$1MatchSink.accept(Object) line: 90` |
| 46 | 0.29% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 47 | 0.29% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 48 | 0.28% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3937` |
| 49 | 0.28% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 50 | 0.28% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 51 | 0.28% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 52 | 0.27% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 53 | 0.25% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 4100` |
| 54 | 0.25% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 55 | 0.24% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 56 | 0.24% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 57 | 0.23% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 58 | 0.23% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 59 | 0.23% | `java.util.AbstractMap$SimpleImmutableEntry.getKey() line: 774` |
| 60 | 0.23% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 32.77% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 5.53% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 5.47% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 1.89% | `java.util.HashMap.containsKey(Object) line: 603` |
| 5 | 1.48% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 1.03% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3803` |
| 7 | 1.01% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 8 | 0.87% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 0.84% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 10 | 0.80% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 11 | 0.73% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 12 | 0.66% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 13 | 0.56% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 14 | 0.54% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 15 | 0.53% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 16 | 0.51% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 17 | 0.50% | `it.unimi.dsi.fastutil.objects.ReferenceCollections$UnmodifiableCollection.contains(Object) line: 344` |
| 18 | 0.50% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 19 | 0.49% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 20 | 0.46% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 21 | 0.43% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 22 | 0.43% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2515` |
| 23 | 0.43% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3821` |
| 24 | 0.43% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 25 | 0.43% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 26 | 0.41% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 27 | 0.40% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 28 | 0.38% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 29 | 0.38% | `net.minecraft.world.item.ItemStack.hurtAndBreak(int, ServerLevel, LivingEntity, Consumer, boolean) line: 621` |
| 30 | 0.36% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |

GC pauses: 0, total 0 ms

