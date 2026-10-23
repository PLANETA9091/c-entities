# JVM hotspots (JFR ExecutionSample)

Samples: 106744

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 28.71% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 7.24% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 3.94% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 4 | 2.80% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 2.03% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 6 | 1.95% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 7 | 1.47% | `java.util.HashMap.getNode(Object) line: 585` |
| 8 | 1.28% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 9 | 1.25% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 10 | 1.19% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.18% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 12 | 1.06% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 13 | 0.94% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 14 | 0.94% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 15 | 0.92% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 16 | 0.92% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 17 | 0.85% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 18 | 0.68% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 19 | 0.67% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 20 | 0.67% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 21 | 0.65% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 47` |
| 22 | 0.65% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 23 | 0.63% | `it.unimi.dsi.fastutil.objects.ObjectOpenHashSet$SetIterator.<init>(ObjectOpenHashSet, ObjectOpenHashSet$1) line: 534` |
| 24 | 0.58% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 61` |
| 25 | 0.57% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 26 | 0.55% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 27 | 0.53% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 28 | 0.53% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 29 | 0.52% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 30 | 0.50% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 31 | 0.48% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 32 | 0.47% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 33 | 0.47% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 34 | 0.46% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 35 | 0.41% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 36 | 0.41% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 37 | 0.41% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 38 | 0.39% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 39 | 0.37% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 40 | 0.37% | `java.lang.Enum.ordinal() line: 128` |
| 41 | 0.36% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 42 | 0.35% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 43 | 0.33% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 44 | 0.33% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges() line: 3348` |
| 45 | 0.32% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 46 | 0.31% | `java.util.EnumMap.get(Object) line: 247` |
| 47 | 0.31% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 48 | 0.31% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 49 | 0.31% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 50 | 0.29% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 51 | 0.27% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 81` |
| 52 | 0.27% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 53 | 0.24% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 54 | 0.24% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |
| 55 | 0.24% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1302` |
| 56 | 0.24% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 57 | 0.23% | `java.util.ArrayDeque.size() line: 648` |
| 58 | 0.23% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 135` |
| 59 | 0.22% | `net.minecraft.core.Vec3i.getZ() line: 71` |
| 60 | 0.22% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.clear() line: 442` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 28.71% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 11.62% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 3.97% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 4 | 2.11% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 2.03% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 6 | 1.95% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 7 | 1.57% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 8 | 1.48% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 9 | 1.47% | `java.util.HashMap.containsKey(Object) line: 602` |
| 10 | 1.26% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 11 | 1.17% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 12 | 1.05% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 13 | 0.98% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 14 | 0.92% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 15 | 0.80% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 16 | 0.79% | `java.util.Map.computeIfAbsent(Object, Function) line: 1064` |
| 17 | 0.71% | `java.util.EnumMap.get(Object) line: 247` |
| 18 | 0.70% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 19 | 0.69% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |
| 20 | 0.68% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 21 | 0.65% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2717` |
| 22 | 0.63% | `it.unimi.dsi.fastutil.objects.ObjectOpenHashSet.iterator() line: 654` |
| 23 | 0.62% | `net.minecraft.world.entity.Entity.isSprinting() line: 3501` |
| 24 | 0.62% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 25 | 0.61% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 26 | 0.57% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 12` |
| 27 | 0.48% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 28 | 0.44% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 904` |
| 29 | 0.44% | `net.minecraft.world.entity.LivingEntity.detectEquipmentUpdates() line: 3320` |
| 30 | 0.43% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |

GC pauses: 0, total 0 ms

