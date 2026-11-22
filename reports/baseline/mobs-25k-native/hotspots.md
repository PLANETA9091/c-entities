# JVM hotspots (JFR ExecutionSample)

Samples: 42198

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 31.11% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.06% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 3 | 3.40% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 3.15% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 5 | 2.82% | `java.util.HashMap.getNode(Object) line: 586` |
| 6 | 1.19% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 7 | 1.16% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 8 | 1.13% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 9 | 1.10% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 10 | 0.97% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 11 | 0.93% | `net.minecraft.world.entity.LivingEntity.isCollidable(boolean) line: 4094` |
| 12 | 0.88% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 13 | 0.81% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 14 | 0.77% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 15 | 0.75% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 16 | 0.71% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 17 | 0.70% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 18 | 0.68% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 19 | 0.66% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 20 | 0.65% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 21 | 0.58% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 22 | 0.56% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 23 | 0.53% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3937` |
| 24 | 0.52% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 25 | 0.50% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 26 | 0.49% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 27 | 0.45% | `java.util.EnumMap.get(Object) line: 248` |
| 28 | 0.45% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 29 | 0.45% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 30 | 0.45% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 31 | 0.44% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 32 | 0.43% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 33 | 0.43% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 34 | 0.40% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 35 | 0.40% | `java.lang.String.hashCode() line: 2430` |
| 36 | 0.38% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 37 | 0.36% | `it.unimi.dsi.fastutil.objects.Object2IntAVLTreeMap$1.<init>(Object2IntAVLTreeMap) line: 1109` |
| 38 | 0.35% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 39 | 0.34% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 40 | 0.34% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 41 | 0.34% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 42 | 0.33% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 43 | 0.33% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 44 | 0.31% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 45 | 0.31% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 46 | 0.30% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 47 | 0.28% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 48 | 0.27% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 49 | 0.27% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 50 | 0.26% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 853` |
| 51 | 0.26% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1484` |
| 52 | 0.26% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 53 | 0.25% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 54 | 0.25% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 55 | 0.24% | `net.minecraft.world.entity.LivingEntity.getItemBySlot(EquipmentSlot) line: 2901` |
| 56 | 0.23% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 57 | 0.23% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 58 | 0.22% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 59 | 0.22% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 60 | 0.21% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 31.11% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 7.46% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 5.82% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 2.46% | `java.util.HashMap.containsKey(Object) line: 603` |
| 5 | 1.83% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 1.19% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3803` |
| 7 | 1.01% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 8 | 0.96% | `net.minecraft.world.entity.LivingEntity.isPushable() line: 4084` |
| 9 | 0.82% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 10 | 0.81% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 894` |
| 11 | 0.77% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 12 | 0.77% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 13 | 0.73% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2515` |
| 14 | 0.66% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 15 | 0.65% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 16 | 0.64% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 17 | 0.64% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3821` |
| 18 | 0.57% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 19 | 0.56% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 20 | 0.54% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 21 | 0.51% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 22 | 0.49% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 23 | 0.48% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 24 | 0.48% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 25 | 0.48% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 26 | 0.45% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 27 | 0.45% | `java.util.HashMap.get(Object) line: 565` |
| 28 | 0.45% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3804` |
| 29 | 0.44% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 30 | 0.43% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |

GC pauses: 0, total 0 ms

