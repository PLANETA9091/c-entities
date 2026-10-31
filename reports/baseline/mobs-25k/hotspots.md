# JVM hotspots (JFR ExecutionSample)

Samples: 39321

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 31.72% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 5.03% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 3.96% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 3.38% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 5 | 2.56% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 6 | 1.83% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 7 | 1.10% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 8 | 1.07% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 9 | 0.96% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 10 | 0.95% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 11 | 0.84% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 12 | 0.84% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 13 | 0.83% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 14 | 0.83% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 15 | 0.73% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 16 | 0.72% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 17 | 0.71% | `java.util.HashMap.getNode(Object) line: 586` |
| 18 | 0.68% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 19 | 0.66% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 20 | 0.61% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 21 | 0.55% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 22 | 0.55% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 23 | 0.54% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 24 | 0.50% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 25 | 0.50% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3926` |
| 26 | 0.49% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 27 | 0.48% | `java.util.EnumMap.get(Object) line: 248` |
| 28 | 0.47% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 29 | 0.47% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 30 | 0.46% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 31 | 0.45% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3696` |
| 32 | 0.43% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 33 | 0.43% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 34 | 0.43% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 35 | 0.40% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(BlockPos) line: 15` |
| 36 | 0.39% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 37 | 0.39% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 38 | 0.35% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 39 | 0.35% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 40 | 0.35% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 41 | 0.33% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 42 | 0.33% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 43 | 0.33% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 44 | 0.32% | `java.lang.String.hashCode() line: 2430` |
| 45 | 0.31% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 46 | 0.30% | `java.util.ArrayDeque.size() line: 647` |
| 47 | 0.29% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 48 | 0.28% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 49 | 0.27% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 50 | 0.26% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 51 | 0.25% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 52 | 0.25% | `java.lang.Enum.ordinal() line: 128` |
| 53 | 0.25% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 54 | 0.25% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 55 | 0.24% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 56 | 0.24% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 57 | 0.23% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 58 | 0.23% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 59 | 0.23% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 60 | 0.23% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 31.72% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 8.41% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 7.00% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 1.41% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 1.22% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 6 | 1.18% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 7 | 1.04% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 8 | 0.96% | `net.minecraft.world.entity.ConversionTracker.tick() line: 79` |
| 9 | 0.84% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 10 | 0.83% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 11 | 0.74% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 12 | 0.73% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 13 | 0.73% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 14 | 0.69% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 15 | 0.68% | `java.util.HashMap.containsKey(Object) line: 603` |
| 16 | 0.66% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2510` |
| 17 | 0.61% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 18 | 0.59% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 19 | 0.57% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3810` |
| 20 | 0.57% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 21 | 0.54% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 22 | 0.52% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 23 | 0.51% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 24 | 0.50% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3799` |
| 25 | 0.49% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 26 | 0.49% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 27 | 0.48% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 28 | 0.48% | `net.minecraft.server.level.ServerLevel.isPositionEntityTicking(BlockPos) line: 2956` |
| 29 | 0.47% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 30 | 0.47% | `net.minecraft.world.entity.Entity.getBlockPosBelowThatAffectsMyMovement() line: 1567` |

GC pauses: 0, total 0 ms

