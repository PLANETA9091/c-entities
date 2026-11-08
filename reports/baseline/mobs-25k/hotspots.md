# JVM hotspots (JFR ExecutionSample)

Samples: 56895

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 36.64% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.13% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 2.95% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 4 | 2.36% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 5 | 2.20% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 6 | 2.08% | `java.util.HashMap.getNode(Object) line: 586` |
| 7 | 1.85% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 8 | 0.99% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 9 | 0.87% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 10 | 0.78% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 11 | 0.76% | `net.minecraft.world.entity.LivingEntity.isCollidable(boolean) line: 4094` |
| 12 | 0.72% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 13 | 0.63% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 14 | 0.59% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 15 | 0.56% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 16 | 0.53% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 17 | 0.51% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 18 | 0.49% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 19 | 0.48% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 20 | 0.47% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 21 | 0.47% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 22 | 0.44% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 23 | 0.44% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 24 | 0.43% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 25 | 0.41% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 26 | 0.38% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 27 | 0.35% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 28 | 0.32% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 29 | 0.30% | `java.lang.String.hashCode() line: 2430` |
| 30 | 0.30% | `java.util.EnumMap.get(Object) line: 248` |
| 31 | 0.30% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 32 | 0.30% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 33 | 0.29% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 34 | 0.29% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 35 | 0.28% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 36 | 0.28% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 37 | 0.28% | `java.lang.Enum.ordinal() line: 128` |
| 38 | 0.28% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 39 | 0.27% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1484` |
| 40 | 0.27% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3937` |
| 41 | 0.26% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 4100` |
| 42 | 0.26% | `net.minecraft.core.Holder$Reference.is(TagKey) line: 195` |
| 43 | 0.25% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 44 | 0.25% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 45 | 0.25% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 46 | 0.25% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 47 | 0.24% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 48 | 0.24% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 49 | 0.24% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 50 | 0.23% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 51 | 0.23% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 52 | 0.23% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 53 | 0.23% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 853` |
| 54 | 0.23% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 55 | 0.22% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 56 | 0.22% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 57 | 0.21% | `java.util.AbstractMap$SimpleImmutableEntry.getKey() line: 774` |
| 58 | 0.20% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 59 | 0.18% | `java.util.stream.MatchOps$1MatchSink.accept(Object) line: 90` |
| 60 | 0.18% | `java.util.Spliterator.getExactSizeIfKnown() line: 414` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 36.64% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 5.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 5.04% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 2.20% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3803` |
| 5 | 1.83% | `java.util.HashMap.containsKey(Object) line: 603` |
| 6 | 1.33% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 7 | 0.84% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 8 | 0.79% | `net.minecraft.world.entity.LivingEntity.isPushable() line: 4084` |
| 9 | 0.77% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 10 | 0.72% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 11 | 0.68% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 12 | 0.62% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 13 | 0.55% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 14 | 0.54% | `it.unimi.dsi.fastutil.objects.ReferenceCollections$UnmodifiableCollection.contains(Object) line: 344` |
| 15 | 0.53% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 16 | 0.53% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 17 | 0.51% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 18 | 0.51% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 19 | 0.44% | `net.minecraft.world.item.enchantment.Enchantment.tick(ServerLevel, int, EnchantedItemInUse, Entity) line: 400` |
| 20 | 0.43% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 21 | 0.42% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3821` |
| 22 | 0.40% | `net.minecraft.server.level.ServerChunkCache.pollTask() line: 449` |
| 23 | 0.40% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 24 | 0.40% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 25 | 0.40% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 26 | 0.40% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 27 | 0.38% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 28 | 0.38% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 29 | 0.37% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 30 | 0.37% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |

GC pauses: 0, total 0 ms

