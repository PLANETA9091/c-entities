# JVM hotspots (JFR ExecutionSample)

Samples: 53167

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 37.36% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.73% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 3.65% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 2.48% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 5 | 1.82% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 6 | 1.17% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 7 | 0.96% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 0.94% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 9 | 0.79% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 10 | 0.77% | `java.util.HashMap.getNode(Object) line: 586` |
| 11 | 0.74% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 12 | 0.70% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 13 | 0.67% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 14 | 0.67% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 15 | 0.61% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 16 | 0.60% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 17 | 0.55% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 18 | 0.48% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 19 | 0.48% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 20 | 0.48% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 21 | 0.46% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 22 | 0.44% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 23 | 0.44% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 24 | 0.41% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 25 | 0.40% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 4089` |
| 26 | 0.39% | `java.util.EnumMap.get(Object) line: 248` |
| 27 | 0.39% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 28 | 0.38% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 29 | 0.37% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 30 | 0.36% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 31 | 0.36% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 32 | 0.35% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 33 | 0.34% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 34 | 0.34% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 35 | 0.33% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 36 | 0.32% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 37 | 0.32% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3926` |
| 38 | 0.31% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 39 | 0.30% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 40 | 0.30% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 41 | 0.28% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 42 | 0.28% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 43 | 0.28% | `java.lang.String.hashCode() line: 2430` |
| 44 | 0.27% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 45 | 0.27% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 46 | 0.25% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 47 | 0.25% | `java.util.AbstractMap$SimpleImmutableEntry.getKey() line: 774` |
| 48 | 0.23% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 49 | 0.23% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 50 | 0.23% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(BlockPos) line: 15` |
| 51 | 0.22% | `java.lang.Enum.ordinal() line: 128` |
| 52 | 0.21% | `net.minecraft.core.Holder$Reference.is(TagKey) line: 195` |
| 53 | 0.21% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 54 | 0.21% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 55 | 0.21% | `net.minecraft.world.entity.ai.attributes.AttributeSupplier.getAttributeInstance(Holder) line: 18` |
| 56 | 0.21% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 57 | 0.20% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 58 | 0.20% | `java.util.Spliterator.getExactSizeIfKnown() line: 414` |
| 59 | 0.20% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5081` |
| 60 | 0.19% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 37.36% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 6.13% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 1.15% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 0.96% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 6 | 0.80% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 7 | 0.79% | `net.minecraft.world.entity.ConversionTracker.tick() line: 79` |
| 8 | 0.73% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 9 | 0.70% | `java.util.HashMap.containsKey(Object) line: 603` |
| 10 | 0.70% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 11 | 0.67% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 12 | 0.66% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 13 | 0.61% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 14 | 0.53% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 15 | 0.53% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 16 | 0.52% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 17 | 0.52% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 18 | 0.49% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 19 | 0.49% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 20 | 0.48% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 21 | 0.45% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3810` |
| 22 | 0.43% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 23 | 0.43% | `net.minecraft.world.entity.EntitySelector.lambda$pushableBy$0(Entity, Team, Team$CollisionRule, Entity) line: 52` |
| 24 | 0.43% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2515` |
| 25 | 0.41% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 26 | 0.41% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 27 | 0.39% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 28 | 0.39% | `net.minecraft.world.entity.Mob.serverAiStep() line: 843` |
| 29 | 0.38% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 30 | 0.38% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |

GC pauses: 0, total 0 ms

