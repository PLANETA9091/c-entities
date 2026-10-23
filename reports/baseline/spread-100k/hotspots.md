# JVM hotspots (JFR ExecutionSample)

Samples: 65621

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 12.01% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.60% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 2.26% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 2.17% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 5 | 1.50% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 6 | 1.49% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 7 | 1.37% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 8 | 1.31% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 9 | 1.26% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 10 | 1.23% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 11 | 1.13% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 12 | 1.07% | `java.util.HashMap.getNode(Object) line: 585` |
| 13 | 1.00% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 14 | 0.99% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 15 | 0.94% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 16 | 0.93% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 17 | 0.92% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1213` |
| 18 | 0.91% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 19 | 0.90% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 20 | 0.85% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 21 | 0.82% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1215` |
| 22 | 0.81% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processPendingFullUpdate() line: 1501` |
| 23 | 0.78% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 24 | 0.74% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 25 | 0.74% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 26 | 0.73% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 27 | 0.72% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 28 | 0.71% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 204` |
| 29 | 0.68% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 61` |
| 30 | 0.66% | `java.lang.Enum.ordinal() line: 128` |
| 31 | 0.66% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 32 | 0.65% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 208` |
| 33 | 0.63% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 34 | 0.58% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 47` |
| 35 | 0.57% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 36 | 0.56% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 37 | 0.50% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 38 | 0.50% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 39 | 0.49% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 40 | 0.49% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(ResourceLocation) line: 68` |
| 41 | 0.48% | `java.util.EnumMap.get(Object) line: 247` |
| 42 | 0.46% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 196` |
| 43 | 0.45% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 44 | 0.45% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 45 | 0.43% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 46 | 0.43% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 47 | 0.43% | `it.unimi.dsi.fastutil.objects.ObjectOpenHashSet$SetIterator.<init>(ObjectOpenHashSet, ObjectOpenHashSet$1) line: 534` |
| 48 | 0.42% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 49 | 0.42% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 50 | 0.41% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 51 | 0.41% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1010` |
| 52 | 0.40% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 81` |
| 53 | 0.39% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 54 | 0.38% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 55 | 0.37% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 56 | 0.34% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 57 | 0.34% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |
| 58 | 0.34% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |
| 59 | 0.34% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 60 | 0.33% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges() line: 3348` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 12.01% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 4.08% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 2.26% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 4 | 2.19% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 5 | 2.13% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 6 | 1.78% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1161` |
| 7 | 1.73% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 8 | 1.67% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 9 | 1.58% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 10 | 1.55% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 11 | 1.49% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 12 | 1.45% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 13 | 1.33% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 14 | 1.24% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 15 | 1.16% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 16 | 1.13% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 17 | 1.07% | `java.util.HashMap.containsKey(Object) line: 602` |
| 18 | 0.92% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 19 | 0.86% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 904` |
| 20 | 0.84% | `net.minecraft.world.entity.Entity.isSprinting() line: 3501` |
| 21 | 0.81% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1462` |
| 22 | 0.78% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 23 | 0.71% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |
| 24 | 0.70% | `java.util.EnumMap.get(Object) line: 247` |
| 25 | 0.68% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 12` |
| 26 | 0.67% | `java.util.Map.computeIfAbsent(Object, Function) line: 1064` |
| 27 | 0.67% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 28 | 0.65% | `net.minecraft.world.entity.Entity.collide(Vec3) line: 1516` |
| 29 | 0.64% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoFluidPushing() line: 1963` |
| 30 | 0.64% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |

GC pauses: 0, total 0 ms

