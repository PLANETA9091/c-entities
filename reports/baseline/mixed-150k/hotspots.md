# JVM hotspots (JFR ExecutionSample)

Samples: 57188

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 36.44% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 8.27% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 2.59% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 2.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 5 | 1.67% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 6 | 1.50% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 7 | 1.42% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 8 | 0.90% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 9 | 0.83% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 0.81% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 11 | 0.77% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 12 | 0.77% | `java.util.HashMap.getNode(Object) line: 585` |
| 13 | 0.75% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 14 | 0.75% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 15 | 0.74% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1215` |
| 16 | 0.74% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 17 | 0.66% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 18 | 0.60% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 19 | 0.58% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 20 | 0.54% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 21 | 0.53% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 22 | 0.51% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 23 | 0.51% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 24 | 0.50% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 25 | 0.49% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 61` |
| 26 | 0.47% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 27 | 0.46% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 28 | 0.43% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 29 | 0.40% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 30 | 0.37% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 31 | 0.36% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(ResourceLocation) line: 68` |
| 32 | 0.36% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 33 | 0.35% | `java.util.EnumMap.isValidKey(Object) line: 319` |
| 34 | 0.35% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 35 | 0.32% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 36 | 0.31% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 37 | 0.31% | `java.util.EnumMap.get(Object) line: 247` |
| 38 | 0.30% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 39 | 0.29% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.getDouble(Object) line: 431` |
| 40 | 0.28% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 41 | 0.28% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 42 | 0.27% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges() line: 3348` |
| 43 | 0.27% | `net.minecraft.world.level.chunk.PalettedContainer.readPalette(PalettedContainer$Data, int) line: 96` |
| 44 | 0.27% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 45 | 0.26% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 46 | 0.26% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 47 | 0.24% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 115` |
| 48 | 0.24% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 49 | 0.24% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 50 | 0.23% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 51 | 0.23% | `java.lang.Enum.ordinal() line: 128` |
| 52 | 0.23% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4732` |
| 53 | 0.22% | `it.unimi.dsi.fastutil.objects.ObjectOpenHashSet$SetIterator.<init>(ObjectOpenHashSet, ObjectOpenHashSet$1) line: 534` |
| 54 | 0.22% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 55 | 0.22% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4882` |
| 56 | 0.21% | `net.minecraft.world.entity.Entity.level() line: 5199` |
| 57 | 0.20% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 47` |
| 58 | 0.19% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |
| 59 | 0.19% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4881` |
| 60 | 0.19% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 135` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 36.44% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 12.18% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 4.24% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 4 | 1.50% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 5 | 1.36% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 6 | 1.21% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 7 | 1.15% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 8 | 1.13% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 9 | 0.86% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 10 | 0.81% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 11 | 0.79% | `java.util.HashMap.containsKey(Object) line: 602` |
| 12 | 0.75% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 13 | 0.75% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1154` |
| 14 | 0.74% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 15 | 0.72% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 16 | 0.63% | `net.minecraft.world.entity.Entity.setDeltaMovement(double, double, double) line: 4891` |
| 17 | 0.57% | `net.minecraft.world.entity.Entity.isSprinting() line: 3501` |
| 18 | 0.56% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 19 | 0.55% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 20 | 0.49% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 12` |
| 21 | 0.48% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 22 | 0.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 23 | 0.44% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 24 | 0.43% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 235` |
| 25 | 0.42% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 26 | 0.41% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 27 | 0.40% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 28 | 0.38% | `java.util.Map.computeIfAbsent(Object, Function) line: 1064` |
| 29 | 0.36% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 533` |
| 30 | 0.36% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoFluidPushing() line: 1963` |

GC pauses: 0, total 0 ms

