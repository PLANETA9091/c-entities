# JVM hotspots (JFR ExecutionSample)

Samples: 67815

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 14.53% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.55% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 1.88% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 4 | 1.85% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 5 | 1.63% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 6 | 1.63% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 7 | 1.59% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 8 | 1.56% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 9 | 1.42% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 10 | 1.41% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 11 | 1.37% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 12 | 1.35% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 13 | 1.28% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 14 | 1.07% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 204` |
| 15 | 1.07% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 16 | 1.03% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 17 | 1.00% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 18 | 0.98% | `java.util.HashMap.getNode(Object) line: 585` |
| 19 | 0.96% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 208` |
| 20 | 0.91% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 21 | 0.88% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 22 | 0.88% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 792` |
| 23 | 0.88% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 24 | 0.86% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 25 | 0.85% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 26 | 0.78% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 27 | 0.75% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 28 | 0.75% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 29 | 0.74% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 30 | 0.72% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 47` |
| 31 | 0.71% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4732` |
| 32 | 0.69% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 33 | 0.68% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(ResourceLocation) line: 68` |
| 34 | 0.61% | `java.util.EnumMap.isValidKey(Object) line: 319` |
| 35 | 0.60% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 36 | 0.59% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 37 | 0.58% | `it.unimi.dsi.fastutil.objects.ObjectOpenHashSet$SetIterator.<init>(ObjectOpenHashSet, ObjectOpenHashSet$1) line: 534` |
| 38 | 0.55% | `java.lang.Enum.ordinal() line: 128` |
| 39 | 0.53% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 40 | 0.52% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 41 | 0.52% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 42 | 0.49% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 323` |
| 43 | 0.47% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 44 | 0.47% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 45 | 0.46% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 196` |
| 46 | 0.46% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 47 | 0.45% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 48 | 0.45% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 49 | 0.44% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 50 | 0.42% | `java.util.EnumMap.get(Object) line: 247` |
| 51 | 0.42% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 52 | 0.42% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 53 | 0.39% | `java.lang.ThreadLocal.get() line: 172` |
| 54 | 0.38% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 55 | 0.38% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 56 | 0.38% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 57 | 0.38% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 58 | 0.37% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 59 | 0.37% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 60 | 0.37% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1215` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 14.53% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 5.06% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 2.81% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 4 | 2.55% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 5 | 2.44% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 6 | 2.21% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 7 | 2.11% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 8 | 1.91% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 9 | 1.82% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 10 | 1.70% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 11 | 1.65% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 12 | 1.49% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 13 | 1.42% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 14 | 1.29% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 15 | 1.28% | `java.util.stream.AbstractPipeline.wrapAndCopyInto(Sink, Spliterator) line: 499` |
| 16 | 1.09% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 17 | 1.00% | `java.util.HashMap.containsKey(Object) line: 602` |
| 18 | 0.89% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 19 | 0.88% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1725` |
| 20 | 0.84% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 904` |
| 21 | 0.78% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 22 | 0.76% | `java.util.Map.computeIfAbsent(Object, Function) line: 1064` |
| 23 | 0.75% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 24 | 0.73% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |
| 25 | 0.72% | `net.minecraft.world.entity.Entity.isSprinting() line: 3501` |
| 26 | 0.72% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2717` |
| 27 | 0.68% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 533` |
| 28 | 0.64% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 212` |
| 29 | 0.60% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 30 | 0.60% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |

GC pauses: 0, total 0 ms

