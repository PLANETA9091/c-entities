# JVM hotspots (JFR ExecutionSample)

Samples: 65393

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 13.21% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.12% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 2.23% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 1.98% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 5 | 1.95% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 6 | 1.58% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 7 | 1.52% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 8 | 1.41% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 9 | 1.37% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 10 | 1.30% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 11 | 1.28% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 12 | 1.19% | `java.util.HashMap.getNode(Object) line: 585` |
| 13 | 1.11% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 14 | 1.02% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1215` |
| 15 | 1.00% | `dev.planeta9091.centities.CeHooks.cellOfBox(AABB) line: 730` |
| 16 | 0.94% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 17 | 0.89% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 208` |
| 18 | 0.87% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 19 | 0.87% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 204` |
| 20 | 0.86% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 21 | 0.86% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1213` |
| 22 | 0.83% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 23 | 0.83% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 24 | 0.82% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 25 | 0.81% | `java.lang.Enum.ordinal() line: 128` |
| 26 | 0.79% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 27 | 0.73% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 28 | 0.73% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 29 | 0.71% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 30 | 0.69% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 31 | 0.69% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 32 | 0.65% | `java.lang.ThreadLocal$ThreadLocalMap.getEntry(ThreadLocal) line: 504` |
| 33 | 0.61% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 34 | 0.60% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 35 | 0.58% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 47` |
| 36 | 0.58% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 196` |
| 37 | 0.56% | `it.unimi.dsi.fastutil.objects.ObjectOpenHashSet$SetIterator.<init>(ObjectOpenHashSet, ObjectOpenHashSet$1) line: 534` |
| 38 | 0.55% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 39 | 0.53% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 40 | 0.51% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 41 | 0.46% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 42 | 0.45% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 43 | 0.45% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 44 | 0.44% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 45 | 0.42% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 46 | 0.41% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 47 | 0.41% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 48 | 0.40% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 49 | 0.40% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 81` |
| 50 | 0.38% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1010` |
| 51 | 0.37% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1200` |
| 52 | 0.37% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 53 | 0.35% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 54 | 0.33% | `java.util.EnumMap.get(Object) line: 247` |
| 55 | 0.32% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges() line: 3348` |
| 56 | 0.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 57 | 0.31% | `io.netty.channel.ChannelOutboundBuffer.addFlush() line: 159` |
| 58 | 0.31% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 59 | 0.31% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 60 | 0.31% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 13.21% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 4.73% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 2.55% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 4 | 2.23% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 5 | 2.11% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 6 | 1.98% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 7 | 1.91% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 8 | 1.90% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1161` |
| 9 | 1.73% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 10 | 1.58% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 11 | 1.53% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 12 | 1.47% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 13 | 1.42% | `java.util.stream.AbstractPipeline.wrapAndCopyInto(Sink, Spliterator) line: 499` |
| 14 | 1.41% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 15 | 1.20% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 16 | 1.20% | `java.util.HashMap.containsKey(Object) line: 602` |
| 17 | 1.16% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 18 | 1.01% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 19 | 1.01% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1209` |
| 20 | 0.93% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4724` |
| 21 | 0.91% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 22 | 0.86% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 23 | 0.86% | `java.lang.ThreadLocal.get(Thread) line: 187` |
| 24 | 0.83% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 25 | 0.70% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |
| 26 | 0.69% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 904` |
| 27 | 0.67% | `java.util.EnumMap.get(Object) line: 247` |
| 28 | 0.63% | `net.minecraft.world.entity.Entity.isSprinting() line: 3501` |
| 29 | 0.62% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 30 | 0.58% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2717` |

GC pauses: 0, total 0 ms

