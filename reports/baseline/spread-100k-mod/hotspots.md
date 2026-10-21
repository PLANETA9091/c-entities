# JVM hotspots (JFR ExecutionSample)

Samples: 82197

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 13.12% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.79% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 2.32% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 4 | 1.99% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 1.68% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 6 | 1.53% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 7 | 1.41% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 8 | 1.32% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 9 | 1.26% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 10 | 1.16% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 11 | 1.10% | `java.util.HashMap.getNode(Object) line: 585` |
| 12 | 1.10% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 13 | 1.07% | `java.lang.Enum.ordinal() line: 128` |
| 14 | 1.04% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 15 | 1.02% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 16 | 1.01% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 17 | 0.99% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 18 | 0.97% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 19 | 0.96% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 20 | 0.95% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 21 | 0.94% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 208` |
| 22 | 0.89% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 23 | 0.89% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 24 | 0.87% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 25 | 0.85% | `net.minecraft.server.level.ServerChunkCache.getChunkNow(int, int) line: 337` |
| 26 | 0.84% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1215` |
| 27 | 0.82% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 28 | 0.82% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 29 | 0.79% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 30 | 0.70% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 31 | 0.70% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1213` |
| 32 | 0.65% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 61` |
| 33 | 0.64% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 34 | 0.61% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 35 | 0.55% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 196` |
| 36 | 0.55% | `java.lang.ThreadLocal$ThreadLocalMap.getEntry(ThreadLocal) line: 504` |
| 37 | 0.48% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 38 | 0.48% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 39 | 0.48% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 47` |
| 40 | 0.46% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 323` |
| 41 | 0.45% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 135` |
| 42 | 0.44% | `java.util.EnumMap.get(Object) line: 247` |
| 43 | 0.43% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 44 | 0.43% | `it.unimi.dsi.fastutil.objects.ObjectOpenHashSet$SetIterator.<init>(ObjectOpenHashSet, ObjectOpenHashSet$1) line: 534` |
| 45 | 0.42% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(ResourceLocation) line: 68` |
| 46 | 0.42% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 47 | 0.41% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 48 | 0.41% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 49 | 0.40% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 50 | 0.38% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 51 | 0.37% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 52 | 0.37% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 53 | 0.37% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1010` |
| 54 | 0.37% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 55 | 0.35% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 81` |
| 56 | 0.35% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 3849` |
| 57 | 0.34% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |
| 58 | 0.32% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 59 | 0.32% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 60 | 0.32% | `net.minecraft.world.level.chunk.PalettedContainer.readPalette(PalettedContainer$Data, int) line: 96` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 13.12% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 3.99% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 2.23% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 4 | 2.14% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 5 | 2.00% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 6 | 1.99% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 7 | 1.68% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 8 | 1.62% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 9 | 1.59% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1161` |
| 10 | 1.54% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 11 | 1.49% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 12 | 1.43% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 13 | 1.43% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 14 | 1.41% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 15 | 1.32% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 16 | 1.11% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4724` |
| 17 | 1.11% | `java.util.HashMap.containsKey(Object) line: 602` |
| 18 | 1.08% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 19 | 1.06% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 20 | 1.04% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 21 | 1.02% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 22 | 0.87% | `java.util.EnumMap.get(Object) line: 247` |
| 23 | 0.86% | `net.minecraft.world.entity.Entity.isSprinting() line: 3501` |
| 24 | 0.85% | `net.minecraft.server.level.ServerChunkCache.hasChunk(int, int) line: 419` |
| 25 | 0.81% | `java.lang.ThreadLocal.get(Thread) line: 187` |
| 26 | 0.80% | `java.util.Map.computeIfAbsent(Object, Function) line: 1064` |
| 27 | 0.69% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |
| 28 | 0.67% | `net.minecraft.server.level.FullChunkStatus.isOrAfter(FullChunkStatus) line: 10` |
| 29 | 0.65% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 30 | 0.65% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 12` |

GC pauses: 0, total 0 ms

