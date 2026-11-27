# JVM hotspots (JFR ExecutionSample)

Samples: 52982

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 40.40% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getNextAcquire() line: 1837` |
| 2 | 3.03% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 2.68% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 4 | 1.21% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 0.95% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 6 | 0.89% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 7 | 0.85% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 8 | 0.78% | `java.util.EnumMap.get(Object) line: 248` |
| 9 | 0.67% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 10 | 0.67% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 11 | 0.66% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 12 | 0.61% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 13 | 0.58% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 14 | 0.55% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 15 | 0.54% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 16 | 0.54% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 17 | 0.51% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 18 | 0.50% | `java.util.HashMap.getNode(Object) line: 586` |
| 19 | 0.47% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 20 | 0.45% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 21 | 0.44% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 22 | 0.43% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 23 | 0.42% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1485` |
| 24 | 0.41% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 25 | 0.40% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 26 | 0.38% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 27 | 0.36% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 28 | 0.35% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 29 | 0.34% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3938` |
| 30 | 0.33% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 31 | 0.33% | `java.lang.Enum.ordinal() line: 128` |
| 32 | 0.33% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 33 | 0.32% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 34 | 0.32% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 35 | 0.31% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 36 | 0.30% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 37 | 0.30% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 38 | 0.29% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 39 | 0.28% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 40 | 0.28% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 41 | 0.28% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 42 | 0.27% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 43 | 0.27% | `java.lang.String.hashCode() line: 2430` |
| 44 | 0.25% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 45 | 0.25% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 46 | 0.25% | `net.minecraft.world.entity.ai.attributes.AttributeSupplier.getAttributeInstance(Holder) line: 18` |
| 47 | 0.25% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 48 | 0.25% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 49 | 0.24% | `java.util.AbstractMap$SimpleImmutableEntry.getKey() line: 774` |
| 50 | 0.23% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 51 | 0.23% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 52 | 0.23% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 53 | 0.23% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 54 | 0.23% | `java.util.EnumMap$EntryIterator$Entry.getKey() line: 589` |
| 55 | 0.22% | `net.minecraft.core.Holder$Reference.is(TagKey) line: 195` |
| 56 | 0.22% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 57 | 0.22% | `java.util.stream.MatchOps$1MatchSink.accept(Object) line: 90` |
| 58 | 0.22% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 59 | 0.21% | `java.lang.ref.Reference.refersToImpl(Object) line: 391` |
| 60 | 0.21% | `net.minecraft.world.entity.LivingEntity$SwingState.tick() line: 5027` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 40.40% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 247` |
| 2 | 3.05% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 2.89% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 1.75% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 0.95% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3804` |
| 6 | 0.90% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 7 | 0.82% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 8 | 0.76% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 9 | 0.67% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 894` |
| 10 | 0.67% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 11 | 0.60% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 12 | 0.57% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 13 | 0.54% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 14 | 0.52% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 15 | 0.51% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 16 | 0.50% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 17 | 0.50% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3822` |
| 18 | 0.49% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 19 | 0.47% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2822` |
| 20 | 0.47% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2515` |
| 21 | 0.46% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 22 | 0.45% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 23 | 0.44% | `java.util.HashMap.get(Object) line: 565` |
| 24 | 0.44% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 25 | 0.42% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 26 | 0.42% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 27 | 0.40% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 28 | 0.40% | `java.util.ArrayList.toArray() line: 370` |
| 29 | 0.39% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 30 | 0.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |

GC pauses: 0, total 0 ms

