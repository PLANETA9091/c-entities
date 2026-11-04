# JVM hotspots (JFR ExecutionSample)

Samples: 48131

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 39.20% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.38% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 3 | 2.79% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 2.42% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 5 | 2.21% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 1.38% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 7 | 1.24% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 8 | 1.08% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 9 | 0.99% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 10 | 0.96% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 11 | 0.92% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 0.88% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 13 | 0.80% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 14 | 0.77% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3739` |
| 15 | 0.68% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 16 | 0.66% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 17 | 0.65% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |
| 18 | 0.62% | `java.util.Optional.ifPresent(Consumer) line: 178` |
| 19 | 0.60% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 20 | 0.56% | `java.util.HashMap.getNode(Object) line: 586` |
| 21 | 0.55% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 22 | 0.54% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 23 | 0.53% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 24 | 0.52% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 25 | 0.52% | `java.lang.String.hashCode() line: 2430` |
| 26 | 0.51% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 27 | 0.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 28 | 0.48% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 29 | 0.47% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(BlockPos) line: 15` |
| 30 | 0.47% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 31 | 0.47% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3937` |
| 32 | 0.46% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 33 | 0.46% | `jdk.internal.util.ArraysSupport.hashCodeOfUnsigned(byte[], int, int, int) line: 275` |
| 34 | 0.44% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 35 | 0.43% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 36 | 0.41% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 37 | 0.38% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3706` |
| 38 | 0.36% | `java.util.EnumMap.get(Object) line: 248` |
| 39 | 0.36% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 40 | 0.35% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 41 | 0.33% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 42 | 0.33% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 43 | 0.33% | `java.util.ArrayDeque.size() line: 647` |
| 44 | 0.33% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 45 | 0.33% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 46 | 0.32% | `java.util.Optional.isPresent() line: 154` |
| 47 | 0.32% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 48 | 0.29% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 49 | 0.29% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 50 | 0.27% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 51 | 0.27% | `net.minecraft.util.Mth.wrapDegrees(float) line: 226` |
| 52 | 0.26% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 53 | 0.26% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 54 | 0.23% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 55 | 0.22% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 56 | 0.22% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 57 | 0.21% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 58 | 0.21% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 59 | 0.21% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 60 | 0.21% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 39.20% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.60% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 4.71% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 1.42% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 5 | 1.37% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 1.25% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 7 | 1.21% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 8 | 1.08% | `net.minecraft.world.entity.ConversionTracker.tick() line: 79` |
| 9 | 0.96% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 10 | 0.91% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 11 | 0.91% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 12 | 0.83% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 13 | 0.80% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 14 | 0.78% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 15 | 0.69% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 16 | 0.66% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3803` |
| 17 | 0.65% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 196` |
| 18 | 0.62% | `java.util.HashMap.containsKey(Object) line: 603` |
| 19 | 0.60% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 20 | 0.55% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 21 | 0.54% | `net.minecraft.server.level.ServerLevel.isPositionEntityTicking(BlockPos) line: 2956` |
| 22 | 0.53% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2515` |
| 23 | 0.53% | `net.minecraft.world.item.enchantment.ItemEnchantments.entrySet() line: 101` |
| 24 | 0.53% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 25 | 0.52% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 26 | 0.50% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3821` |
| 27 | 0.47% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 28 | 0.47% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 29 | 0.46% | `java.lang.StringLatin1.hashCode(byte[]) line: 195` |
| 30 | 0.45% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |

GC pauses: 0, total 0 ms

