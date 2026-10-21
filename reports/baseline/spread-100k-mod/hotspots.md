# JVM hotspots (JFR ExecutionSample)

Samples: 73702

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 23.89% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.46% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 2.86% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 1.72% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 5 | 1.37% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 6 | 1.36% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 7 | 1.32% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 8 | 1.29% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 9 | 1.25% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 10 | 1.12% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 11 | 0.97% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1215` |
| 12 | 0.97% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 13 | 0.96% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 14 | 0.95% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 15 | 0.89% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 16 | 0.86% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 17 | 0.85% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 208` |
| 18 | 0.80% | `net.minecraft.server.level.ServerChunkCache.getChunkNow(int, int) line: 337` |
| 19 | 0.79% | `java.lang.Enum.ordinal() line: 128` |
| 20 | 0.78% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 21 | 0.76% | `java.util.HashMap.getNode(Object) line: 585` |
| 22 | 0.74% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 23 | 0.72% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 24 | 0.71% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 25 | 0.69% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 26 | 0.69% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 27 | 0.69% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 28 | 0.66% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 29 | 0.63% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 30 | 0.57% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 31 | 0.56% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 32 | 0.56% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 33 | 0.54% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 34 | 0.53% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 35 | 0.51% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 36 | 0.50% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 37 | 0.49% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(ResourceLocation) line: 68` |
| 38 | 0.46% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 204` |
| 39 | 0.43% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4882` |
| 40 | 0.42% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 41 | 0.42% | `java.lang.ThreadLocal.get() line: 172` |
| 42 | 0.38% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 115` |
| 43 | 0.38% | `it.unimi.dsi.fastutil.objects.ObjectOpenHashSet$SetIterator.<init>(ObjectOpenHashSet, ObjectOpenHashSet$1) line: 534` |
| 44 | 0.38% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 47` |
| 45 | 0.37% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges() line: 3348` |
| 46 | 0.37% | `java.util.EnumMap.get(Object) line: 247` |
| 47 | 0.37% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 48 | 0.37% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 49 | 0.37% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.size() line: 422` |
| 50 | 0.36% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 196` |
| 51 | 0.35% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 52 | 0.35% | `net.minecraft.util.Mth.wrapDegrees(float) line: 221` |
| 53 | 0.34% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1213` |
| 54 | 0.31% | `net.minecraft.world.level.pathfinder.Path.isDone() line: 38` |
| 55 | 0.31% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 56 | 0.31% | `net.minecraft.world.entity.Mob.checkDespawn() line: 715` |
| 57 | 0.31% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 58 | 0.30% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 59 | 0.29% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 60 | 0.29% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 23.89% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 5.49% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 2.86% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 4 | 2.06% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 5 | 1.94% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 6 | 1.81% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 7 | 1.39% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 8 | 1.39% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 9 | 1.36% | `java.util.stream.AbstractPipeline.wrapAndCopyInto(Sink, Spliterator) line: 499` |
| 10 | 1.35% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1161` |
| 11 | 1.29% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 12 | 1.27% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 13 | 1.25% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 14 | 1.22% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 15 | 1.15% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 16 | 1.08% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 17 | 0.90% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 18 | 0.80% | `net.minecraft.server.level.ServerChunkCache.hasChunk(int, int) line: 419` |
| 19 | 0.77% | `java.util.HashMap.containsKey(Object) line: 602` |
| 20 | 0.77% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 21 | 0.71% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 235` |
| 22 | 0.71% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 23 | 0.68% | `net.minecraft.world.entity.Entity.setDeltaMovement(double, double, double) line: 4891` |
| 24 | 0.65% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 25 | 0.65% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4724` |
| 26 | 0.64% | `net.minecraft.world.entity.Entity.isSprinting() line: 3501` |
| 27 | 0.64% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 28 | 0.61% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 29 | 0.55% | `net.minecraft.server.level.FullChunkStatus.isOrAfter(FullChunkStatus) line: 10` |
| 30 | 0.54% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |

GC pauses: 0, total 0 ms

