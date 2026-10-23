# JVM hotspots (JFR ExecutionSample)

Samples: 66696

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 11.81% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.50% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 2.27% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 1.98% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 5 | 1.52% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 6 | 1.39% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 7 | 1.36% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 8 | 1.30% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 9 | 1.26% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 10 | 1.23% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 11 | 1.22% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.21% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 13 | 1.18% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 14 | 1.13% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 15 | 1.09% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1215` |
| 16 | 0.96% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 17 | 0.90% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 18 | 0.86% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 19 | 0.86% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 20 | 0.76% | `java.util.HashMap.getNode(Object) line: 585` |
| 21 | 0.75% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 22 | 0.74% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 23 | 0.71% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 24 | 0.69% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 61` |
| 25 | 0.69% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 47` |
| 26 | 0.67% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 208` |
| 27 | 0.67% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 204` |
| 28 | 0.67% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 29 | 0.66% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 30 | 0.64% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 31 | 0.62% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 32 | 0.61% | `java.lang.Enum.ordinal() line: 128` |
| 33 | 0.60% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 34 | 0.57% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 35 | 0.55% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 36 | 0.55% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(ResourceLocation) line: 68` |
| 37 | 0.55% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 38 | 0.52% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 196` |
| 39 | 0.52% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 40 | 0.49% | `it.unimi.dsi.fastutil.objects.ObjectOpenHashSet$SetIterator.<init>(ObjectOpenHashSet, ObjectOpenHashSet$1) line: 534` |
| 41 | 0.48% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 42 | 0.48% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 43 | 0.47% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 44 | 0.45% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 45 | 0.45% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 46 | 0.43% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 47 | 0.43% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 48 | 0.43% | `java.util.EnumMap.get(Object) line: 247` |
| 49 | 0.43% | `java.lang.ThreadLocal.get() line: 172` |
| 50 | 0.41% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 51 | 0.39% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges() line: 3348` |
| 52 | 0.38% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 53 | 0.35% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 81` |
| 54 | 0.35% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 55 | 0.35% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 3849` |
| 56 | 0.34% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |
| 57 | 0.33% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |
| 58 | 0.32% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 59 | 0.32% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 60 | 0.31% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 11.81% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 3.93% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 2.27% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 4 | 2.20% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 5 | 2.14% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 6 | 1.97% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 7 | 1.67% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 8 | 1.52% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 9 | 1.47% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 10 | 1.46% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 11 | 1.45% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 12 | 1.32% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 13 | 1.25% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 14 | 1.23% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 15 | 1.12% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1161` |
| 16 | 1.09% | `net.minecraft.world.entity.Entity.isSprinting() line: 3501` |
| 17 | 1.08% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 18 | 0.94% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 19 | 0.90% | `java.util.EnumMap.get(Object) line: 247` |
| 20 | 0.87% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 21 | 0.81% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 22 | 0.79% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 23 | 0.77% | `java.util.HashMap.containsKey(Object) line: 602` |
| 24 | 0.70% | `java.util.Map.computeIfAbsent(Object, Function) line: 1064` |
| 25 | 0.70% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 12` |
| 26 | 0.68% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2717` |
| 27 | 0.65% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 28 | 0.64% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |
| 29 | 0.63% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 30 | 0.62% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 78` |

GC pauses: 0, total 0 ms

