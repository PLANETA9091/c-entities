# JVM hotspots (JFR ExecutionSample)

Samples: 156902

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 45.75% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 6.15% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 1.99% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 1.72% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 5 | 1.60% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 6 | 1.48% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 7 | 1.29% | `java.util.HashMap.getNode(Object) line: 585` |
| 8 | 1.22% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 9 | 0.92% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 10 | 0.85% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 11 | 0.79% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 12 | 0.76% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 13 | 0.66% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1010` |
| 14 | 0.61% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 15 | 0.61% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 16 | 0.57% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 17 | 0.56% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 18 | 0.56% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 19 | 0.51% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 20 | 0.51% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 21 | 0.47% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 22 | 0.46% | `java.lang.Enum.ordinal() line: 128` |
| 23 | 0.44% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 24 | 0.43% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 61` |
| 25 | 0.40% | `java.lang.ThreadLocal.get() line: 172` |
| 26 | 0.40% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 27 | 0.39% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 28 | 0.38% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 29 | 0.36% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 30 | 0.34% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 31 | 0.34% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 32 | 0.31% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 33 | 0.30% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 994` |
| 34 | 0.30% | `it.unimi.dsi.fastutil.objects.ObjectOpenHashSet$SetIterator.<init>(ObjectOpenHashSet, ObjectOpenHashSet$1) line: 534` |
| 35 | 0.30% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 36 | 0.29% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 37 | 0.29% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 444` |
| 38 | 0.28% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 39 | 0.27% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 40 | 0.25% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 41 | 0.25% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 42 | 0.24% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 43 | 0.23% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 44 | 0.23% | `java.util.EnumMap.get(Object) line: 247` |
| 45 | 0.22% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 46 | 0.22% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 47 | 0.22% | `net.minecraft.world.item.ItemStack.getMaxStackSize() line: 581` |
| 48 | 0.21% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 119` |
| 49 | 0.21% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 47` |
| 50 | 0.20% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges() line: 3348` |
| 51 | 0.20% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 52 | 0.19% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 53 | 0.19% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 54 | 0.17% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 437` |
| 55 | 0.16% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 56 | 0.16% | `net.minecraft.world.entity.LivingEntity.getArrowCount() line: 2528` |
| 57 | 0.16% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 58 | 0.16% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 135` |
| 59 | 0.15% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 60 | 0.15% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 45.75% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 9.45% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 2.90% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 4 | 1.60% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 5 | 1.48% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 6 | 1.29% | `java.util.HashMap.containsKey(Object) line: 602` |
| 7 | 1.14% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 8 | 0.92% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 9 | 0.87% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 10 | 0.86% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 11 | 0.79% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 12 | 0.76% | `net.minecraft.server.level.ChunkMap.tick() line: 1025` |
| 13 | 0.66% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 14 | 0.66% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 15 | 0.62% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 16 | 0.61% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 17 | 0.59% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 18 | 0.58% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 19 | 0.43% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 12` |
| 20 | 0.42% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |
| 21 | 0.39% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 22 | 0.39% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 23 | 0.38% | `java.util.EnumMap.get(Object) line: 247` |
| 24 | 0.38% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 25 | 0.32% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 904` |
| 26 | 0.32% | `net.minecraft.world.entity.Entity.isSprinting() line: 3501` |
| 27 | 0.31% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 28 | 0.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 29 | 0.30% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 30 | 0.30% | `it.unimi.dsi.fastutil.objects.ObjectOpenHashSet.iterator() line: 654` |

GC pauses: 0, total 0 ms

