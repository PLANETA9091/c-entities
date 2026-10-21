# JVM hotspots (JFR ExecutionSample)

Samples: 78041

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 20.50% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.91% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 2.38% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 2.24% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 5 | 1.82% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 6 | 1.76% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 7 | 1.35% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 8 | 1.30% | `dev.planeta9091.centities.CeHooks.cellOfBox(AABB) line: 663` |
| 9 | 1.25% | `java.util.HashMap.getNode(Object) line: 585` |
| 10 | 1.24% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 11 | 1.21% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1215` |
| 12 | 1.19% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 13 | 1.18% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 14 | 1.10% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 15 | 1.06% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 16 | 1.05% | `net.minecraft.server.level.ServerChunkCache.getChunkNow(int, int) line: 337` |
| 17 | 1.04% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 18 | 1.03% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 19 | 0.98% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 20 | 0.90% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 21 | 0.90% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 22 | 0.85% | `java.lang.Enum.ordinal() line: 128` |
| 23 | 0.81% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 208` |
| 24 | 0.69% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 25 | 0.69% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 26 | 0.66% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1126` |
| 27 | 0.65% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 28 | 0.64% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 29 | 0.63% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 30 | 0.59% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 31 | 0.58% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 32 | 0.56% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 33 | 0.52% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 34 | 0.50% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 35 | 0.49% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 36 | 0.49% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 37 | 0.46% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 196` |
| 38 | 0.43% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(ResourceLocation) line: 68` |
| 39 | 0.43% | `java.lang.ThreadLocal.get() line: 172` |
| 40 | 0.43% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 41 | 0.40% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 444` |
| 42 | 0.40% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 43 | 0.39% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 44 | 0.38% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 45 | 0.38% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 46 | 0.36% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 47` |
| 47 | 0.36% | `it.unimi.dsi.fastutil.objects.ObjectOpenHashSet$SetIterator.<init>(ObjectOpenHashSet, ObjectOpenHashSet$1) line: 534` |
| 48 | 0.35% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 49 | 0.33% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 81` |
| 50 | 0.32% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 51 | 0.32% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges() line: 3348` |
| 52 | 0.31% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 53 | 0.30% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 54 | 0.30% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 437` |
| 55 | 0.29% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.size() line: 422` |
| 56 | 0.29% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 57 | 0.28% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 58 | 0.28% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 70` |
| 59 | 0.27% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 204` |
| 60 | 0.27% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 173` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 20.50% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 4.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 2.38% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 4 | 2.38% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 5 | 2.24% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 6 | 1.91% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 7 | 1.80% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 8 | 1.80% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 9 | 1.49% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 10 | 1.40% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 11 | 1.31% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 12 | 1.30% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1135` |
| 13 | 1.24% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 14 | 1.24% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1161` |
| 15 | 1.18% | `java.util.stream.AbstractPipeline.wrapAndCopyInto(Sink, Spliterator) line: 499` |
| 16 | 1.16% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 17 | 1.15% | `java.util.HashMap.containsKey(Object) line: 602` |
| 18 | 1.14% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 19 | 1.05% | `net.minecraft.server.level.ServerChunkCache.hasChunk(int, int) line: 419` |
| 20 | 0.97% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 21 | 0.97% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 22 | 0.90% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 931` |
| 23 | 0.86% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4724` |
| 24 | 0.80% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 25 | 0.79% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 26 | 0.71% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 27 | 0.68% | `java.util.Map.computeIfAbsent(Object, Function) line: 1064` |
| 28 | 0.64% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |
| 29 | 0.62% | `net.minecraft.server.level.FullChunkStatus.isOrAfter(FullChunkStatus) line: 10` |
| 30 | 0.54% | `java.util.EnumMap.get(Object) line: 247` |

GC pauses: 0, total 0 ms

