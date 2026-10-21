# JVM hotspots (JFR ExecutionSample)

Samples: 81487

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 12.47% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.82% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 2.34% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 4 | 2.01% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 1.97% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 6 | 1.69% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 7 | 1.41% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 8 | 1.36% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 9 | 1.20% | `java.util.HashMap.getNode(Object) line: 585` |
| 10 | 1.19% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 11 | 1.18% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.15% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 13 | 0.99% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 14 | 0.96% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 15 | 0.95% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 16 | 0.94% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 17 | 0.92% | `java.lang.Enum.ordinal() line: 128` |
| 18 | 0.92% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 19 | 0.91% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 20 | 0.90% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 21 | 0.90% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 22 | 0.90% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 208` |
| 23 | 0.85% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1215` |
| 24 | 0.84% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1213` |
| 25 | 0.82% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 26 | 0.76% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 27 | 0.72% | `net.minecraft.server.level.ServerChunkCache.getChunkNow(int, int) line: 337` |
| 28 | 0.70% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 29 | 0.66% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 61` |
| 30 | 0.62% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 31 | 0.61% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 32 | 0.57% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 33 | 0.56% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 34 | 0.55% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 35 | 0.53% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 36 | 0.50% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 196` |
| 37 | 0.47% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 323` |
| 38 | 0.47% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 39 | 0.44% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 40 | 0.44% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 41 | 0.43% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 42 | 0.43% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1010` |
| 43 | 0.43% | `java.util.EnumMap.get(Object) line: 247` |
| 44 | 0.43% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 45 | 0.42% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 46 | 0.42% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 47 | 0.42% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 48 | 0.42% | `it.unimi.dsi.fastutil.objects.ObjectOpenHashSet$SetIterator.<init>(ObjectOpenHashSet, ObjectOpenHashSet$1) line: 534` |
| 49 | 0.41% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 81` |
| 50 | 0.41% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 51 | 0.37% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 52 | 0.36% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 53 | 0.36% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |
| 54 | 0.36% | `java.util.ArrayDeque.size() line: 648` |
| 55 | 0.35% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 56 | 0.34% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 57 | 0.33% | `net.minecraft.world.level.chunk.PalettedContainer.readPalette(PalettedContainer$Data, int) line: 96` |
| 58 | 0.33% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 59 | 0.33% | `net.minecraft.server.level.ServerLevel.lambda$tick$4(TickRateManager, ProfilerFiller, Entity) line: 795` |
| 60 | 0.31% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 12.47% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 4.04% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 2.05% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 4 | 2.02% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 5 | 2.01% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 6 | 1.98% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 7 | 1.94% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 8 | 1.79% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 9 | 1.74% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1161` |
| 10 | 1.69% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 11 | 1.45% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 12 | 1.40% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 13 | 1.34% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 14 | 1.34% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 15 | 1.27% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 16 | 1.21% | `java.util.HashMap.containsKey(Object) line: 602` |
| 17 | 1.13% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 18 | 1.06% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4724` |
| 19 | 1.01% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 20 | 0.99% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 21 | 0.92% | `java.util.EnumMap.get(Object) line: 247` |
| 22 | 0.91% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 23 | 0.89% | `net.minecraft.world.entity.Entity.isSprinting() line: 3501` |
| 24 | 0.87% | `java.util.Map.computeIfAbsent(Object, Function) line: 1064` |
| 25 | 0.72% | `net.minecraft.server.level.ServerChunkCache.hasChunk(int, int) line: 419` |
| 26 | 0.67% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |
| 27 | 0.66% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 12` |
| 28 | 0.65% | `net.minecraft.server.level.ChunkMap.tick() line: 1025` |
| 29 | 0.59% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 30 | 0.56% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 212` |

GC pauses: 0, total 0 ms

