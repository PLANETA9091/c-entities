# JVM hotspots (JFR ExecutionSample)

Samples: 54293

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 8.95% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.17% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 2.87% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 4 | 2.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 5 | 2.33% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 6 | 2.14% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 7 | 2.10% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 8 | 2.04% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 9 | 1.83% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 10 | 1.62% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 11 | 1.37% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 1.36% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 13 | 1.29% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 14 | 1.28% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 15 | 1.28% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 16 | 1.12% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 17 | 1.11% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 18 | 1.08% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 19 | 1.02% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 20 | 0.96% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 21 | 0.96% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 22 | 0.91% | `java.util.HashMap.getNode(Object) line: 585` |
| 23 | 0.86% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 24 | 0.85% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 25 | 0.81% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 26 | 0.77% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 47` |
| 27 | 0.75% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(ResourceLocation) line: 68` |
| 28 | 0.74% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 29 | 0.74% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 30 | 0.74% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1215` |
| 31 | 0.73% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 32 | 0.72% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 33 | 0.63% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 34 | 0.62% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 35 | 0.60% | `java.lang.Enum.ordinal() line: 128` |
| 36 | 0.58% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 208` |
| 37 | 0.58% | `it.unimi.dsi.fastutil.objects.ObjectOpenHashSet$SetIterator.<init>(ObjectOpenHashSet, ObjectOpenHashSet$1) line: 534` |
| 38 | 0.57% | `net.minecraft.server.level.ServerChunkCache.getChunkNow(int, int) line: 337` |
| 39 | 0.55% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 40 | 0.53% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1661` |
| 41 | 0.52% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 323` |
| 42 | 0.52% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 43 | 0.50% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 81` |
| 44 | 0.50% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 45 | 0.48% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1213` |
| 46 | 0.48% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 47 | 0.47% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 48 | 0.45% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 49 | 0.45% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 50 | 0.45% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 51 | 0.44% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |
| 52 | 0.43% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 53 | 0.42% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 54 | 0.42% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1414` |
| 55 | 0.41% | `java.util.ArrayDeque.size() line: 648` |
| 56 | 0.41% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 57 | 0.41% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges() line: 3348` |
| 58 | 0.40% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3690` |
| 59 | 0.37% | `net.minecraft.server.level.ServerLevel.lambda$tick$4(TickRateManager, ProfilerFiller, Entity) line: 795` |
| 60 | 0.37% | `net.minecraft.world.entity.Entity.isEyeInFluid(TagKey) line: 2077` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 8.95% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 3.72% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 3.17% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 4 | 3.03% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 5 | 2.57% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 2.33% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 7 | 2.24% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 8 | 2.14% | `java.util.stream.AbstractPipeline.wrapAndCopyInto(Sink, Spliterator) line: 499` |
| 9 | 1.90% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 10 | 1.83% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 11 | 1.75% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 12 | 1.62% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 13 | 1.60% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 14 | 1.39% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 15 | 1.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 16 | 1.26% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1161` |
| 17 | 1.11% | `java.util.Map.computeIfAbsent(Object, Function) line: 1064` |
| 18 | 1.09% | `net.minecraft.world.entity.Entity.isSprinting() line: 3501` |
| 19 | 1.08% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 20 | 1.00% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 21 | 0.97% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 22 | 0.95% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 23 | 0.94% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |
| 24 | 0.91% | `java.util.HashMap.containsKey(Object) line: 602` |
| 25 | 0.88% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4724` |
| 26 | 0.87% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 27 | 0.86% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 28 | 0.76% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2717` |
| 29 | 0.75% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 533` |
| 30 | 0.74% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 235` |

GC pauses: 0, total 0 ms

