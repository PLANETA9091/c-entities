# JVM hotspots (JFR ExecutionSample)

Samples: 73587

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 51.79% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 6.56% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 3 | 2.73% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 4 | 1.44% | `java.util.HashMap.getNode(Object) line: 586` |
| 5 | 1.28% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 6 | 0.97% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 0.92% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 8 | 0.67% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 9 | 0.64% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 10 | 0.63% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 11 | 0.58% | `net.minecraft.world.entity.LivingEntity.isCollidable(boolean) line: 4095` |
| 12 | 0.53% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 13 | 0.48% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 14 | 0.47% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 15 | 0.46% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 16 | 0.37% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 17 | 0.36% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 18 | 0.36% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 19 | 0.33% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 20 | 0.32% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 21 | 0.31% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 22 | 0.31% | `net.minecraft.world.item.ItemStack.getDamageValue() line: 574` |
| 23 | 0.28% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 24 | 0.28% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 25 | 0.26% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 26 | 0.25% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 27 | 0.25% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 28 | 0.25% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 29 | 0.24% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 66` |
| 30 | 0.23% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 31 | 0.23% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 32 | 0.23% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 33 | 0.22% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 34 | 0.22% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 35 | 0.22% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 36 | 0.22% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 37 | 0.22% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 38 | 0.21% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3938` |
| 39 | 0.18% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 40 | 0.18% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 4101` |
| 41 | 0.17% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 42 | 0.17% | `java.util.EnumMap.get(Object) line: 248` |
| 43 | 0.17% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 44 | 0.17% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 45 | 0.17% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 46 | 0.16% | `java.util.stream.MatchOps$1MatchSink.accept(Object) line: 90` |
| 47 | 0.16% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 48 | 0.15% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 49 | 0.15% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 50 | 0.15% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 51 | 0.14% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 52 | 0.14% | `net.minecraft.world.level.levelgen.BitRandomSource.nextInt(int) line: 31` |
| 53 | 0.13% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 54 | 0.13% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 541` |
| 55 | 0.13% | `com.google.common.collect.RegularImmutableMap.get(Object, ImmutableMapEntry[], int) line: 282` |
| 56 | 0.13% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1485` |
| 57 | 0.12% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 58 | 0.12% | `java.lang.String.hashCode() line: 2430` |
| 59 | 0.12% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap$MapIterator.nextEntry() line: 655` |
| 60 | 0.12% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 51.79% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 7.53% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 4.72% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 1.28% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3804` |
| 5 | 1.25% | `java.util.HashMap.containsKey(Object) line: 603` |
| 6 | 0.95% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 7 | 0.65% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 8 | 0.60% | `net.minecraft.world.entity.LivingEntity.isPushable() line: 4085` |
| 9 | 0.48% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 894` |
| 10 | 0.46% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 11 | 0.42% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 12 | 0.42% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 13 | 0.42% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 14 | 0.40% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2515` |
| 15 | 0.38% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 16 | 0.36% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 17 | 0.34% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 18 | 0.34% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 19 | 0.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 20 | 0.32% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 21 | 0.30% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 22 | 0.28% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3822` |
| 23 | 0.28% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 24 | 0.27% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 25 | 0.25% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 26 | 0.25% | `java.util.Collections$UnmodifiableCollection.iterator() line: 1076` |
| 27 | 0.25% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 28 | 0.25% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2822` |
| 29 | 0.24% | `java.util.HashMap.get(Object) line: 565` |
| 30 | 0.24% | `com.google.common.collect.RegularImmutableMap.get(Object) line: 262` |

GC pauses: 0, total 0 ms

