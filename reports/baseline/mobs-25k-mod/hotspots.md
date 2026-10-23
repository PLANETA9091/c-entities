# JVM hotspots (JFR ExecutionSample)

Samples: 24338

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 13.02% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.70% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 1.97% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 1.56% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 5 | 1.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 6 | 1.47% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 7 | 1.41% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 8 | 1.28% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3907` |
| 9 | 1.20% | `java.util.HashMap.getNode(Object) line: 586` |
| 10 | 1.18% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4706` |
| 11 | 1.17% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 12 | 1.12% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1325` |
| 13 | 1.06% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 14 | 1.03% | `java.util.EnumMap.get(Object) line: 248` |
| 15 | 0.97% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 16 | 0.91% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 17 | 0.90% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 18 | 0.89% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 19 | 0.85% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 20 | 0.83% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 21 | 0.83% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 22 | 0.81% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 23 | 0.79% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 24 | 0.74% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3868` |
| 25 | 0.64% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 26 | 0.63% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 27 | 0.62% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 28 | 0.61% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 29 | 0.56% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 30 | 0.53% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 31 | 0.53% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 32 | 0.52% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(BlockPos) line: 15` |
| 33 | 0.52% | `jdk.internal.util.ArraysSupport.hashCodeOfUnsigned(byte[], int, int, int) line: 275` |
| 34 | 0.50% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 35 | 0.50% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 36 | 0.49% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 37 | 0.49% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 38 | 0.48% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 39 | 0.48% | `java.lang.String.hashCode() line: 2430` |
| 40 | 0.46% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 41 | 0.45% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 42 | 0.44% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 43 | 0.44% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 44 | 0.44% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3926` |
| 45 | 0.42% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 46 | 0.41% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 47 | 0.40% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 48 | 0.40% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 49 | 0.39% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 50 | 0.38% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 51 | 0.38% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 52 | 0.37% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 53 | 0.37% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 54 | 0.35% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 55 | 0.35% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 56 | 0.33% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1251` |
| 57 | 0.32% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 58 | 0.32% | `java.util.ArrayDeque.size() line: 647` |
| 59 | 0.30% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 60 | 0.30% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 13.02% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 4.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 3.86% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 2.87% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 1.47% | `net.minecraft.world.entity.ConversionTracker.tick() line: 79` |
| 6 | 1.44% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 7 | 1.29% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 8 | 1.27% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 1.13% | `java.util.HashMap.containsKey(Object) line: 603` |
| 10 | 0.94% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 11 | 0.94% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 937` |
| 12 | 0.89% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 13 | 0.85% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2195` |
| 14 | 0.83% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 15 | 0.82% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 16 | 0.81% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 17 | 0.81% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 18 | 0.79% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 19 | 0.76% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 20 | 0.74% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 21 | 0.73% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3798` |
| 22 | 0.70% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 923` |
| 23 | 0.69% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 24 | 0.68% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 25 | 0.67% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 26 | 0.67% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 27 | 0.62% | `net.minecraft.server.level.ServerLevel.isPositionEntityTicking(BlockPos) line: 2956` |
| 28 | 0.62% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 29 | 0.62% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 30 | 0.58% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1255` |

GC pauses: 0, total 0 ms

