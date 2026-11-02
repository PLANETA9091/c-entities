# JVM hotspots (JFR ExecutionSample)

Samples: 50819

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 33.87% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.01% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 3.78% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 3.04% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 5 | 1.82% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 6 | 1.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 7 | 1.13% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 1.04% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 9 | 0.93% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 10 | 0.92% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 11 | 0.84% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 12 | 0.80% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 13 | 0.78% | `java.util.HashMap.getNode(Object) line: 586` |
| 14 | 0.78% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 15 | 0.73% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 16 | 0.68% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 17 | 0.62% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 18 | 0.60% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 19 | 0.59% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 20 | 0.59% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 21 | 0.54% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 22 | 0.50% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 23 | 0.49% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 24 | 0.47% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 25 | 0.46% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 26 | 0.44% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 27 | 0.41% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 28 | 0.38% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 29 | 0.38% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 30 | 0.37% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 31 | 0.35% | `java.lang.String.hashCode() line: 2430` |
| 32 | 0.35% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 33 | 0.35% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 34 | 0.34% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 35 | 0.34% | `java.util.EnumMap.get(Object) line: 248` |
| 36 | 0.33% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 37 | 0.33% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 38 | 0.32% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 39 | 0.32% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3926` |
| 40 | 0.30% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 41 | 0.29% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(BlockPos) line: 15` |
| 42 | 0.27% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 43 | 0.26% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 4089` |
| 44 | 0.25% | `jdk.internal.util.ArraysSupport.hashCodeOfUnsigned(byte[], int, int, int) line: 275` |
| 45 | 0.24% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 46 | 0.24% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 47 | 0.24% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 48 | 0.23% | `java.util.Optional.isPresent() line: 154` |
| 49 | 0.23% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 50 | 0.22% | `java.lang.Enum.ordinal() line: 128` |
| 51 | 0.22% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 52 | 0.22% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 53 | 0.21% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 54 | 0.21% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 55 | 0.21% | `net.minecraft.core.Holder$Reference.is(TagKey) line: 195` |
| 56 | 0.21% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 57 | 0.21% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 58 | 0.20% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 59 | 0.20% | `java.util.AbstractMap$SimpleImmutableEntry.getKey() line: 774` |
| 60 | 0.20% | `java.util.stream.MatchOps$1MatchSink.accept(Object) line: 90` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 33.87% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.22% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 5.82% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 1.55% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 1.50% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 6 | 1.14% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 7 | 0.93% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 8 | 0.85% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 9 | 0.82% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 10 | 0.81% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 11 | 0.80% | `net.minecraft.world.entity.ConversionTracker.tick() line: 79` |
| 12 | 0.74% | `java.util.HashMap.containsKey(Object) line: 603` |
| 13 | 0.62% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 14 | 0.59% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 15 | 0.57% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 16 | 0.55% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 17 | 0.51% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 18 | 0.51% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 19 | 0.50% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 20 | 0.48% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 21 | 0.47% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2515` |
| 22 | 0.46% | `net.minecraft.world.entity.Mob.serverAiStep() line: 843` |
| 23 | 0.46% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 24 | 0.45% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 25 | 0.43% | `java.util.ArrayList.toArray() line: 370` |
| 26 | 0.42% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 27 | 0.42% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 28 | 0.42% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 29 | 0.41% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3810` |
| 30 | 0.41% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |

GC pauses: 0, total 0 ms

