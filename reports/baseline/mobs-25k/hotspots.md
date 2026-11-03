# JVM hotspots (JFR ExecutionSample)

Samples: 54310

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 35.81% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.80% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 3.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 2.47% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 5 | 2.08% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 6 | 1.40% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 7 | 1.04% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 0.91% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 9 | 0.90% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 10 | 0.78% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 11 | 0.78% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 12 | 0.74% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 13 | 0.74% | `java.util.HashMap.getNode(Object) line: 586` |
| 14 | 0.70% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 15 | 0.66% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 16 | 0.64% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 17 | 0.61% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 18 | 0.59% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 19 | 0.56% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 20 | 0.54% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 21 | 0.52% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 22 | 0.50% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 23 | 0.49% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 24 | 0.45% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 25 | 0.45% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 26 | 0.43% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 27 | 0.40% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 28 | 0.40% | `java.util.EnumMap.get(Object) line: 248` |
| 29 | 0.37% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 30 | 0.36% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 31 | 0.36% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 32 | 0.35% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 33 | 0.34% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 34 | 0.34% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 35 | 0.34% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 36 | 0.34% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 37 | 0.33% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 38 | 0.32% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 39 | 0.31% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(BlockPos) line: 15` |
| 40 | 0.30% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 41 | 0.30% | `java.lang.String.hashCode() line: 2430` |
| 42 | 0.30% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 43 | 0.29% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3926` |
| 44 | 0.27% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 45 | 0.27% | `net.minecraft.core.Holder$Reference.is(TagKey) line: 195` |
| 46 | 0.26% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1484` |
| 47 | 0.26% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 48 | 0.25% | `java.util.ArrayDeque.size() line: 647` |
| 49 | 0.25% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 50 | 0.25% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 51 | 0.25% | `java.lang.Enum.ordinal() line: 128` |
| 52 | 0.24% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 53 | 0.24% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 848` |
| 54 | 0.24% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 4089` |
| 55 | 0.24% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 56 | 0.23% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 57 | 0.23% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 58 | 0.22% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 59 | 0.22% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 60 | 0.22% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 35.81% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.27% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 6.00% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 1.17% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 1.08% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 6 | 1.04% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 7 | 0.91% | `net.minecraft.world.entity.ConversionTracker.tick() line: 79` |
| 8 | 0.87% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 0.78% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 10 | 0.75% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 11 | 0.67% | `java.util.HashMap.containsKey(Object) line: 603` |
| 12 | 0.64% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 13 | 0.58% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 14 | 0.56% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 15 | 0.53% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 16 | 0.53% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 17 | 0.52% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 18 | 0.45% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 19 | 0.45% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 20 | 0.45% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 21 | 0.45% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 22 | 0.41% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 23 | 0.41% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2515` |
| 24 | 0.41% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 25 | 0.41% | `net.minecraft.server.level.ServerLevel.isPositionEntityTicking(BlockPos) line: 2956` |
| 26 | 0.40% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 27 | 0.40% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 28 | 0.40% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 29 | 0.39% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 30 | 0.39% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |

GC pauses: 0, total 0 ms

