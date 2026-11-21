# JVM hotspots (JFR ExecutionSample)

Samples: 42586

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 30.92% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.76% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 3 | 3.27% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 3.11% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 5 | 2.84% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 6 | 2.68% | `java.util.HashMap.getNode(Object) line: 586` |
| 7 | 1.42% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 8 | 1.40% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 9 | 1.15% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 10 | 0.96% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 11 | 0.90% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 12 | 0.74% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 0.73% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 14 | 0.67% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 15 | 0.63% | `net.minecraft.world.item.ItemStack.getDamageValue() line: 574` |
| 16 | 0.62% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 17 | 0.60% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 18 | 0.56% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 19 | 0.56% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 20 | 0.56% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 21 | 0.55% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3937` |
| 22 | 0.55% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 23 | 0.54% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 24 | 0.52% | `java.util.EnumMap.get(Object) line: 248` |
| 25 | 0.52% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 26 | 0.49% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 27 | 0.46% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 28 | 0.45% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 29 | 0.45% | `java.lang.String.hashCode() line: 2430` |
| 30 | 0.43% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 31 | 0.41% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 32 | 0.40% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 33 | 0.39% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 34 | 0.36% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 35 | 0.36% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 36 | 0.35% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 37 | 0.35% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 38 | 0.34% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 39 | 0.34% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 40 | 0.33% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 41 | 0.32% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 237` |
| 42 | 0.32% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 43 | 0.31% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 4100` |
| 44 | 0.30% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 45 | 0.29% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 46 | 0.29% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 47 | 0.29% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 48 | 0.28% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 49 | 0.28% | `java.util.ArrayDeque.size() line: 647` |
| 50 | 0.28% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 51 | 0.27% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 52 | 0.27% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 53 | 0.26% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 54 | 0.23% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 55 | 0.23% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 56 | 0.22% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 57 | 0.22% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 58 | 0.22% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 59 | 0.21% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 60 | 0.21% | `net.minecraft.world.entity.LivingEntity$SwingState.tick() line: 5026` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 30.92% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 7.03% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 5.46% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 2.84% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3803` |
| 5 | 2.66% | `java.util.HashMap.containsKey(Object) line: 603` |
| 6 | 2.15% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 7 | 1.42% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 8 | 0.95% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 0.90% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 894` |
| 10 | 0.87% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 11 | 0.72% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2515` |
| 12 | 0.67% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 13 | 0.64% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3821` |
| 14 | 0.64% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 15 | 0.64% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 16 | 0.61% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 17 | 0.58% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 18 | 0.57% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 19 | 0.56% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 20 | 0.50% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 21 | 0.50% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 22 | 0.49% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 23 | 0.48% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 24 | 0.47% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 25 | 0.47% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 26 | 0.46% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 27 | 0.45% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3804` |
| 28 | 0.43% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 29 | 0.43% | `it.unimi.dsi.fastutil.objects.ReferenceCollections$UnmodifiableCollection.contains(Object) line: 344` |
| 30 | 0.41% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |

GC pauses: 0, total 0 ms

