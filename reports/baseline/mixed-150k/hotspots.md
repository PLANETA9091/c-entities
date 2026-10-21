# JVM hotspots (JFR ExecutionSample)

Samples: 198300

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 50.06% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 7.02% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 2.39% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 4 | 1.96% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 1.46% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 6 | 1.29% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 7 | 1.28% | `java.util.HashMap.getNode(Object) line: 585` |
| 8 | 0.77% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 9 | 0.76% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 10 | 0.73% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 11 | 0.64% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 12 | 0.61% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 13 | 0.58% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 14 | 0.56% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 15 | 0.56% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 16 | 0.55% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 17 | 0.54% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 18 | 0.49% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 19 | 0.47% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 20 | 0.44% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 21 | 0.43% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 22 | 0.42% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 23 | 0.40% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 24 | 0.39% | `java.lang.Enum.ordinal() line: 128` |
| 25 | 0.34% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 26 | 0.34% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 27 | 0.33% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 28 | 0.33% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 29 | 0.33% | `java.lang.ThreadLocal.get() line: 172` |
| 30 | 0.32% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 31 | 0.29% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 32 | 0.28% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 33 | 0.28% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 34 | 0.27% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 35 | 0.27% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 36 | 0.26% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 37 | 0.26% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 38 | 0.25% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 39 | 0.25% | `it.unimi.dsi.fastutil.objects.ObjectOpenHashSet$SetIterator.<init>(ObjectOpenHashSet, ObjectOpenHashSet$1) line: 534` |
| 40 | 0.25% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 444` |
| 41 | 0.24% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 42 | 0.24% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 43 | 0.23% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 44 | 0.22% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 45 | 0.22% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges() line: 3348` |
| 46 | 0.22% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 47 | 0.22% | `net.minecraft.world.item.ItemStack.getDamageValue() line: 597` |
| 48 | 0.20% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 49 | 0.19% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 47` |
| 50 | 0.18% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |
| 51 | 0.18% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 52 | 0.16% | `java.util.EnumMap.get(Object) line: 247` |
| 53 | 0.16% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 54 | 0.15% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 437` |
| 55 | 0.15% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.size() line: 422` |
| 56 | 0.15% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 135` |
| 57 | 0.14% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 58 | 0.13% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 59 | 0.13% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 60 | 0.13% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 50.06% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 9.79% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 2.58% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 4 | 1.46% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 5 | 1.36% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 6 | 1.29% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 7 | 1.12% | `java.util.HashMap.containsKey(Object) line: 602` |
| 8 | 1.12% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 9 | 0.76% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 10 | 0.73% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 11 | 0.72% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 12 | 0.60% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 13 | 0.58% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 14 | 0.58% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 15 | 0.56% | `java.util.stream.AbstractPipeline.wrapAndCopyInto(Sink, Spliterator) line: 499` |
| 16 | 0.55% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 17 | 0.47% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 18 | 0.46% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 19 | 0.46% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 20 | 0.38% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 21 | 0.37% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 22 | 0.35% | `java.util.EnumMap.get(Object) line: 247` |
| 23 | 0.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 24 | 0.31% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |
| 25 | 0.31% | `net.minecraft.world.entity.Entity.isSprinting() line: 3501` |
| 26 | 0.30% | `net.minecraft.world.entity.LivingEntity.detectEquipmentUpdates() line: 3320` |
| 27 | 0.29% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 904` |
| 28 | 0.28% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 29 | 0.28% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 30 | 0.28% | `it.unimi.dsi.fastutil.objects.ReferenceCollections$UnmodifiableCollection.contains(Object) line: 344` |

GC pauses: 0, total 0 ms

