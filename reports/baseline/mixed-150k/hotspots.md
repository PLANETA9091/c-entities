# JVM hotspots (JFR ExecutionSample)

Samples: 70695

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 26.10% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.26% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 3.16% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 3.06% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 5 | 2.92% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 2.38% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 7 | 2.29% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 8 | 2.00% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.50% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 10 | 1.41% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 11 | 1.37% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 12 | 1.31% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 13 | 1.22% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 14 | 1.16% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 15 | 1.14% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 16 | 1.11% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 17 | 1.09% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 18 | 1.06% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 19 | 0.91% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 20 | 0.80% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 21 | 0.80% | `java.util.EnumMap.get(Object) line: 248` |
| 22 | 0.75% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 23 | 0.75% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 24 | 0.67% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 25 | 0.66% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 26 | 0.66% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 27 | 0.56% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 28 | 0.55% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 29 | 0.53% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 30 | 0.50% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 31 | 0.49% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 32 | 0.44% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 33 | 0.43% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 34 | 0.42% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 35 | 0.41% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 36 | 0.41% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 37 | 0.38% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 92` |
| 38 | 0.37% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 39 | 0.37% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 40 | 0.36% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 41 | 0.36% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 42 | 0.35% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 43 | 0.35% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 44 | 0.34% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 45 | 0.34% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 46 | 0.32% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 47 | 0.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 48 | 0.31% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 49 | 0.29% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 50 | 0.28% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 51 | 0.28% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 52 | 0.27% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 53 | 0.26% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 54 | 0.26% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 55 | 0.24% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 56 | 0.24% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 57 | 0.24% | `net.minecraft.world.entity.Entity.baseTick() line: 914` |
| 58 | 0.23% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1424` |
| 59 | 0.23% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 60 | 0.22% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 26.10% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.17% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 5.33% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 4.92% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 3.16% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 6 | 2.40% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 7 | 2.38% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 8 | 2.00% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 1.68% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 10 | 1.22% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 11 | 1.16% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 12 | 1.11% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 13 | 1.06% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 14 | 0.90% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 15 | 0.86% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 16 | 0.79% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 17 | 0.79% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 18 | 0.66% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 19 | 0.66% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 20 | 0.63% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 21 | 0.58% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 22 | 0.56% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 23 | 0.53% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 24 | 0.50% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 25 | 0.49% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 26 | 0.48% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 27 | 0.45% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 28 | 0.43% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 29 | 0.43% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 30 | 0.43% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3584` |

GC pauses: 0, total 0 ms

