# JVM hotspots (JFR ExecutionSample)

Samples: 55152

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 36.86% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.15% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 2.89% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 4 | 2.47% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 5 | 2.00% | `java.util.HashMap.getNode(Object) line: 586` |
| 6 | 1.21% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 7 | 1.02% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 8 | 0.99% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 9 | 0.94% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 10 | 0.81% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 11 | 0.78% | `net.minecraft.world.entity.LivingEntity.isCollidable(boolean) line: 4095` |
| 12 | 0.74% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 13 | 0.67% | `java.util.EnumMap.get(Object) line: 248` |
| 14 | 0.61% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 15 | 0.60% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 16 | 0.51% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 17 | 0.50% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 18 | 0.49% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 19 | 0.49% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 20 | 0.48% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 21 | 0.47% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 22 | 0.44% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 23 | 0.40% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 24 | 0.40% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 25 | 0.37% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 26 | 0.37% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 27 | 0.37% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 28 | 0.34% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1485` |
| 29 | 0.34% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 30 | 0.33% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 31 | 0.32% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 32 | 0.32% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 33 | 0.32% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 34 | 0.32% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 35 | 0.31% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 36 | 0.30% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 37 | 0.30% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 38 | 0.28% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 39 | 0.27% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 40 | 0.27% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 41 | 0.26% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 42 | 0.26% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 43 | 0.26% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 4101` |
| 44 | 0.25% | `java.lang.Enum.ordinal() line: 128` |
| 45 | 0.25% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 46 | 0.24% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 47 | 0.24% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3938` |
| 48 | 0.24% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 49 | 0.24% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 50 | 0.24% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 51 | 0.23% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 52 | 0.23% | `java.util.stream.MatchOps$1MatchSink.accept(Object) line: 90` |
| 53 | 0.22% | `net.minecraft.world.entity.LivingEntity$SwingState.tick() line: 5027` |
| 54 | 0.22% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 55 | 0.22% | `java.lang.String.hashCode() line: 2430` |
| 56 | 0.22% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 57 | 0.21% | `java.lang.ref.Reference.refersToImpl(Object) line: 391` |
| 58 | 0.21% | `java.util.AbstractMap$SimpleImmutableEntry.getKey() line: 774` |
| 59 | 0.21% | `net.minecraft.core.Holder$Reference.is(TagKey) line: 195` |
| 60 | 0.19% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 36.86% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 5.61% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 5.10% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 1.78% | `java.util.HashMap.containsKey(Object) line: 603` |
| 5 | 1.44% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 1.21% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3804` |
| 7 | 1.04% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 8 | 0.81% | `net.minecraft.world.entity.LivingEntity.isPushable() line: 4085` |
| 9 | 0.71% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 10 | 0.69% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 11 | 0.61% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 894` |
| 12 | 0.51% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 13 | 0.49% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 14 | 0.49% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 15 | 0.47% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 16 | 0.47% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 17 | 0.46% | `it.unimi.dsi.fastutil.objects.ReferenceCollections$UnmodifiableCollection.contains(Object) line: 344` |
| 18 | 0.43% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 19 | 0.43% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 20 | 0.42% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2822` |
| 21 | 0.42% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 22 | 0.41% | `java.util.ArrayList.toArray() line: 370` |
| 23 | 0.40% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 24 | 0.40% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 25 | 0.40% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 26 | 0.40% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 27 | 0.40% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 28 | 0.39% | `net.minecraft.world.item.enchantment.Enchantment.tick(ServerLevel, int, EnchantedItemInUse, Entity) line: 400` |
| 29 | 0.39% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 30 | 0.37% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |

GC pauses: 0, total 0 ms

