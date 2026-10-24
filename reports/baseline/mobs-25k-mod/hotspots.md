# JVM hotspots (JFR ExecutionSample)

Samples: 19049

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.71% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.05% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1443` |
| 3 | 1.83% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3907` |
| 4 | 1.80% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 5 | 1.77% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 6 | 1.47% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 7 | 1.35% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 8 | 1.33% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 9 | 1.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 10 | 1.29% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 11 | 1.26% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 12 | 1.15% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 13 | 1.12% | `java.util.EnumMap.get(Object) line: 248` |
| 14 | 0.96% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1369` |
| 15 | 0.92% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3026` |
| 16 | 0.79% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 17 | 0.77% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 18 | 0.77% | `dev.planeta9091.centities.CeHooks.cellOfBox(AABB) line: 817` |
| 19 | 0.75% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4706` |
| 20 | 0.72% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 21 | 0.72% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 22 | 0.71% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 23 | 0.70% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 24 | 0.70% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 25 | 0.69% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3868` |
| 26 | 0.67% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 27 | 0.67% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 28 | 0.64% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 29 | 0.63% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 30 | 0.62% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 2898` |
| 31 | 0.58% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 32 | 0.58% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 33 | 0.57% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 34 | 0.52% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 35 | 0.52% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 36 | 0.50% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 37 | 0.50% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 38 | 0.50% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 39 | 0.48% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 40 | 0.46% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 41 | 0.45% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 42 | 0.44% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 43 | 0.43% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 44 | 0.42% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 45 | 0.41% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 46 | 0.40% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 47 | 0.40% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 48 | 0.40% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1372` |
| 49 | 0.39% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 50 | 0.38% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 51 | 0.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 52 | 0.38% | `net.minecraft.world.entity.Entity.getX() line: 5094` |
| 53 | 0.36% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap$MapIterator.nextEntry() line: 655` |
| 54 | 0.35% | `java.util.ArrayDeque.size() line: 647` |
| 55 | 0.34% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 56 | 0.33% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 57 | 0.33% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 58 | 0.33% | `ca.spottedleaf.moonrise.patches.starlight.light.SWMRNibbleArray.getVisible(int) line: 412` |
| 59 | 0.32% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 60 | 0.32% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.71% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 2.33% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1036` |
| 3 | 1.86% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 1.83% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 5 | 1.81% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 6 | 1.77% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1373` |
| 7 | 1.57% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 8 | 1.47% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 9 | 1.42% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 10 | 1.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 11 | 1.26% | `net.minecraft.world.entity.ConversionTracker.tick() line: 79` |
| 12 | 1.25% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 13 | 1.14% | `dev.planeta9091.centities.CeHooks.gateCell(Level, AABB) line: 926` |
| 14 | 1.11% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 15 | 1.01% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 16 | 0.99% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 17 | 0.91% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 18 | 0.88% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 19 | 0.88% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 20 | 0.87% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 21 | 0.87% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 22 | 0.87% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1382` |
| 23 | 0.78% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 24 | 0.76% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 25 | 0.76% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 26 | 0.74% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 27 | 0.73% | `net.minecraft.world.entity.LivingEntity.handleRelativeFrictionAndCalculateMovement(Vec3, float) line: 3303` |
| 28 | 0.72% | `java.util.ArrayList.toArray() line: 370` |
| 29 | 0.72% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 30 | 0.69% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3798` |

GC pauses: 0, total 0 ms

