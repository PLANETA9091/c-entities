# JVM hotspots (JFR ExecutionSample)

Samples: 114097

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 30.40% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 7.50% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 3.06% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 2.25% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 5 | 1.95% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 6 | 1.85% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 7 | 1.22% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 8 | 1.21% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 9 | 1.21% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 10 | 1.05% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 11 | 1.04% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 12 | 1.04% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 0.87% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 14 | 0.80% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 15 | 0.78% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 16 | 0.75% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 17 | 0.75% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 18 | 0.73% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 19 | 0.71% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 20 | 0.67% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 21 | 0.67% | `java.util.HashMap.getNode(Object) line: 585` |
| 22 | 0.66% | `java.lang.ThreadLocal.get() line: 172` |
| 23 | 0.61% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 47` |
| 24 | 0.60% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(ResourceLocation) line: 68` |
| 25 | 0.58% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 26 | 0.57% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 27 | 0.57% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 28 | 0.55% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 29 | 0.50% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 30 | 0.48% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 31 | 0.47% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 32 | 0.46% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 33 | 0.45% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 34 | 0.44% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 35 | 0.41% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 36 | 0.40% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 37 | 0.37% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 38 | 0.36% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 39 | 0.35% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 40 | 0.35% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 41 | 0.33% | `java.lang.Enum.ordinal() line: 128` |
| 42 | 0.33% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 43 | 0.32% | `net.minecraft.world.entity.Entity.getX() line: 4899` |
| 44 | 0.31% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 45 | 0.30% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 46 | 0.29% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 47 | 0.29% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1310` |
| 48 | 0.29% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 49 | 0.28% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges() line: 3348` |
| 50 | 0.27% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 51 | 0.27% | `java.util.EnumMap.get(Object) line: 247` |
| 52 | 0.26% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 53 | 0.26% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |
| 54 | 0.25% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 55 | 0.24% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 56 | 0.24% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1318` |
| 57 | 0.24% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 58 | 0.23% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 59 | 0.23% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 135` |
| 60 | 0.23% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 30.40% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 10.61% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 4.09% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 4 | 2.03% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 5 | 1.95% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 6 | 1.85% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 7 | 1.76% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 8 | 1.60% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 9 | 1.24% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 10 | 1.17% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 11 | 0.99% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 12 | 0.83% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 13 | 0.76% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 14 | 0.76% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 15 | 0.74% | `java.util.Map.computeIfAbsent(Object, Function) line: 1064` |
| 16 | 0.73% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 17 | 0.73% | `java.util.stream.AbstractPipeline.wrapAndCopyInto(Sink, Spliterator) line: 499` |
| 18 | 0.72% | `net.minecraft.world.entity.Entity.isSprinting() line: 3501` |
| 19 | 0.68% | `net.minecraft.util.profiling.Profiler.get() line: 47` |
| 20 | 0.67% | `java.util.HashMap.containsKey(Object) line: 602` |
| 21 | 0.66% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 22 | 0.65% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |
| 23 | 0.62% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 24 | 0.61% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2717` |
| 25 | 0.60% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 533` |
| 26 | 0.57% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 27 | 0.49% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 28 | 0.46% | `java.util.EnumMap.get(Object) line: 247` |
| 29 | 0.46% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 30 | 0.44% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |

GC pauses: 0, total 0 ms

