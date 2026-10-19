# JVM hotspots (JFR ExecutionSample)

Samples: 68544

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 22.99% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.20% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 2.14% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 1.71% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 5 | 1.37% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 6 | 1.31% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 7 | 1.30% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 8 | 1.27% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 9 | 1.19% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 10 | 1.13% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 11 | 1.09% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 204` |
| 12 | 1.04% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 13 | 0.97% | `java.util.HashMap.getNode(Object) line: 585` |
| 14 | 0.95% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 15 | 0.89% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 16 | 0.86% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 17 | 0.85% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 18 | 0.79% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processPendingFullUpdate() line: 1501` |
| 19 | 0.79% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 20 | 0.78% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 21 | 0.77% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 22 | 0.76% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 208` |
| 23 | 0.75% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4732` |
| 24 | 0.75% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 25 | 0.75% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 26 | 0.72% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 27 | 0.67% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 28 | 0.66% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 29 | 0.66% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 30 | 0.66% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 61` |
| 31 | 0.64% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 32 | 0.64% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 33 | 0.61% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 34 | 0.60% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 35 | 0.60% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 36 | 0.60% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 37 | 0.58% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1215` |
| 38 | 0.55% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(ResourceLocation) line: 68` |
| 39 | 0.53% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 47` |
| 40 | 0.53% | `java.util.EnumMap.isValidKey(Object) line: 319` |
| 41 | 0.50% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 42 | 0.48% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 43 | 0.47% | `it.unimi.dsi.fastutil.objects.ObjectOpenHashSet$SetIterator.<init>(ObjectOpenHashSet, ObjectOpenHashSet$1) line: 534` |
| 44 | 0.46% | `java.lang.Enum.ordinal() line: 128` |
| 45 | 0.45% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 196` |
| 46 | 0.44% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 47 | 0.42% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 48 | 0.41% | `java.util.EnumMap.get(Object) line: 247` |
| 49 | 0.39% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 50 | 0.39% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 51 | 0.37% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 52 | 0.37% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 53 | 0.37% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 54 | 0.37% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 55 | 0.36% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 56 | 0.34% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 57 | 0.34% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 81` |
| 58 | 0.33% | `net.minecraft.world.level.chunk.PalettedContainer.readPalette(PalettedContainer$Data, int) line: 96` |
| 59 | 0.33% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 60 | 0.33% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 323` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 22.99% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 5.98% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 2.64% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 4 | 2.40% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 5 | 2.14% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 6 | 1.89% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 7 | 1.76% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 8 | 1.74% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 9 | 1.49% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 10 | 1.39% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 11 | 1.35% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 12 | 1.21% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 13 | 1.19% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 14 | 0.98% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 15 | 0.98% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 16 | 0.98% | `java.util.HashMap.containsKey(Object) line: 602` |
| 17 | 0.79% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1462` |
| 18 | 0.77% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 904` |
| 19 | 0.69% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 20 | 0.65% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 12` |
| 21 | 0.64% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 22 | 0.63% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |
| 23 | 0.62% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 24 | 0.61% | `net.minecraft.world.entity.Entity.isSprinting() line: 3501` |
| 25 | 0.61% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 26 | 0.60% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 27 | 0.59% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1154` |
| 28 | 0.56% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 212` |
| 29 | 0.55% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 533` |
| 30 | 0.54% | `java.util.Map.computeIfAbsent(Object, Function) line: 1064` |

GC pauses: 0, total 0 ms

