# JVM hotspots (JFR ExecutionSample)

Samples: 48865

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 42.80% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.23% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 3 | 2.83% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 4 | 2.03% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 1.56% | `java.util.HashMap.getNode(Object) line: 586` |
| 6 | 1.30% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 7 | 1.23% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 8 | 1.06% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 9 | 1.05% | `net.minecraft.world.entity.LivingEntity.isCollidable(boolean) line: 4094` |
| 10 | 0.83% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 11 | 0.82% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 12 | 0.81% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 13 | 0.72% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 14 | 0.63% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3739` |
| 15 | 0.55% | `java.util.EnumMap.get(Object) line: 248` |
| 16 | 0.55% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 17 | 0.50% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 18 | 0.49% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 66` |
| 19 | 0.48% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 20 | 0.45% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 21 | 0.44% | `java.util.Optional.ifPresent(Consumer) line: 178` |
| 22 | 0.43% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 23 | 0.41% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 24 | 0.41% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 25 | 0.41% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 26 | 0.40% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 27 | 0.39% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 28 | 0.37% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 29 | 0.36% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 30 | 0.35% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 31 | 0.35% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 32 | 0.33% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 33 | 0.32% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 34 | 0.32% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 35 | 0.30% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 36 | 0.30% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 37 | 0.30% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 38 | 0.30% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 39 | 0.29% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 40 | 0.27% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 41 | 0.26% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 42 | 0.26% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 43 | 0.25% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 44 | 0.25% | `net.minecraft.world.entity.LivingEntity$SwingState.tick() line: 5026` |
| 45 | 0.25% | `net.minecraft.world.entity.ai.control.MoveControl.tick() line: 96` |
| 46 | 0.25% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 47 | 0.24% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 48 | 0.23% | `net.minecraft.util.Mth.wrapDegrees(float) line: 226` |
| 49 | 0.22% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 50 | 0.21% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 51 | 0.21% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 52 | 0.21% | `java.util.ArrayDeque.size() line: 647` |
| 53 | 0.20% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 54 | 0.20% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 55 | 0.19% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 56 | 0.19% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 57 | 0.18% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 58 | 0.17% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 59 | 0.17% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 60 | 0.17% | `net.minecraft.core.Vec3i.getX() line: 63` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 42.80% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.26% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 4.71% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 1.35% | `java.util.HashMap.containsKey(Object) line: 603` |
| 5 | 1.06% | `net.minecraft.world.entity.LivingEntity.isPushable() line: 4084` |
| 6 | 1.05% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 7 | 1.04% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 8 | 0.84% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3803` |
| 9 | 0.77% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 10 | 0.77% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 11 | 0.76% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 12 | 0.73% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 13 | 0.71% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 14 | 0.62% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 15 | 0.59% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 16 | 0.59% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 17 | 0.55% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 18 | 0.49% | `net.minecraft.world.item.enchantment.ItemEnchantments.entrySet() line: 101` |
| 19 | 0.45% | `net.minecraft.world.entity.Mob.serverAiStep() line: 843` |
| 20 | 0.43% | `net.minecraft.world.entity.Entity.getBlockPosBelowThatAffectsMyMovement() line: 1567` |
| 21 | 0.43% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 22 | 0.42% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 23 | 0.41% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 24 | 0.39% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 25 | 0.38% | `net.minecraft.world.entity.Mob.serverAiStep() line: 836` |
| 26 | 0.37% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 27 | 0.36% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 28 | 0.36% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 848` |
| 29 | 0.35% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 30 | 0.34% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |

GC pauses: 0, total 0 ms

