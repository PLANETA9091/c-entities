# JVM hotspots (JFR ExecutionSample)

Samples: 52505

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 32.36% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.43% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 3.28% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 2.14% | `java.util.HashMap.getNode(Object) line: 586` |
| 5 | 1.95% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 6 | 1.40% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 7 | 1.37% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 8 | 0.94% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 9 | 0.81% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 10 | 0.81% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 11 | 0.77% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 12 | 0.70% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 13 | 0.67% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 14 | 0.66% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 15 | 0.59% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 16 | 0.59% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 17 | 0.58% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 18 | 0.57% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.<init>(Map, float) line: 124` |
| 19 | 0.53% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 20 | 0.53% | `java.util.EnumMap.get(Object) line: 248` |
| 21 | 0.53% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 22 | 0.52% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 23 | 0.47% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 24 | 0.44% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 25 | 0.44% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 26 | 0.43% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 27 | 0.43% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 28 | 0.39% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 29 | 0.38% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 30 | 0.38% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 31 | 0.37% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 32 | 0.37% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 33 | 0.34% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1484` |
| 34 | 0.33% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 35 | 0.32% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 36 | 0.32% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 37 | 0.31% | `java.util.AbstractMap$SimpleImmutableEntry.getKey() line: 774` |
| 38 | 0.30% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 39 | 0.30% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 40 | 0.29% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 41 | 0.29% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 42 | 0.29% | `java.lang.String.hashCode() line: 2430` |
| 43 | 0.29% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 44 | 0.29% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 45 | 0.27% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 46 | 0.27% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 47 | 0.26% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 48 | 0.26% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 49 | 0.26% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 50 | 0.25% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 51 | 0.25% | `java.lang.Enum.ordinal() line: 128` |
| 52 | 0.25% | `net.minecraft.core.Holder$Reference.is(TagKey) line: 195` |
| 53 | 0.24% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 54 | 0.24% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 4100` |
| 55 | 0.23% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3937` |
| 56 | 0.23% | `net.minecraft.world.entity.LivingEntity.isCollidable(boolean) line: 4094` |
| 57 | 0.22% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 58 | 0.22% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 59 | 0.22% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 60 | 0.22% | `java.util.stream.MatchOps$1MatchSink.accept(Object) line: 90` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 32.36% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 5.67% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 5.37% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 1.95% | `java.util.HashMap.containsKey(Object) line: 603` |
| 5 | 1.37% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3803` |
| 6 | 1.19% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 7 | 0.84% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 8 | 0.79% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 0.77% | `net.minecraft.world.entity.ConversionTracker.tick() line: 79` |
| 10 | 0.73% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 11 | 0.62% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 12 | 0.61% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 13 | 0.59% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 14 | 0.58% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 15 | 0.58% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 16 | 0.57% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 17 | 0.57% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.<init>(Map) line: 134` |
| 18 | 0.53% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 19 | 0.51% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 20 | 0.49% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2515` |
| 21 | 0.49% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 22 | 0.48% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 23 | 0.47% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 24 | 0.46% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 25 | 0.46% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 26 | 0.46% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 27 | 0.43% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 28 | 0.42% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 29 | 0.41% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 30 | 0.39% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |

GC pauses: 0, total 0 ms

