# JVM hotspots (JFR ExecutionSample)

Samples: 140525

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 41.65% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 6.47% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 2.22% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 1.56% | `java.util.HashMap.getNode(Object) line: 585` |
| 5 | 1.37% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 1.12% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 7 | 1.03% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 8 | 1.01% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 9 | 0.92% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 10 | 0.92% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 11 | 0.85% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 0.84% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 13 | 0.80% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 14 | 0.74% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 15 | 0.67% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 16 | 0.66% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 17 | 0.66% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 61` |
| 18 | 0.65% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 19 | 0.63% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 20 | 0.63% | `java.lang.ThreadLocal.get() line: 172` |
| 21 | 0.58% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 22 | 0.57% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 23 | 0.56% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 24 | 0.54% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(ResourceLocation) line: 68` |
| 25 | 0.54% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 26 | 0.52% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 27 | 0.52% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 28 | 0.49% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 29 | 0.47% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 30 | 0.45% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 31 | 0.44% | `java.lang.Enum.ordinal() line: 128` |
| 32 | 0.43% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 33 | 0.43% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 34 | 0.41% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 35 | 0.37% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 47` |
| 36 | 0.36% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 37 | 0.35% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 38 | 0.35% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 39 | 0.35% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 40 | 0.34% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 41 | 0.34% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 42 | 0.32% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 43 | 0.31% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 44 | 0.30% | `java.util.ArrayDeque.size() line: 648` |
| 45 | 0.30% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 46 | 0.30% | `net.minecraft.world.item.ItemStack.getDamageValue() line: 597` |
| 47 | 0.30% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges() line: 3348` |
| 48 | 0.30% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 49 | 0.28% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |
| 50 | 0.27% | `java.util.EnumMap.get(Object) line: 247` |
| 51 | 0.27% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 52 | 0.27% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 53 | 0.25% | `net.minecraft.world.entity.Entity.lambda$checkInsideBlocks$2(LongSet, Vec3, Vec3, InsideBlockEffectApplier$StepBasedCollector, BlockPos, int) line: 1693` |
| 54 | 0.25% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 55 | 0.25% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.getDouble(Object) line: 431` |
| 56 | 0.24% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 57 | 0.24% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 58 | 0.24% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1169` |
| 59 | 0.23% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 60 | 0.22% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 135` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 41.65% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 7.73% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 2.22% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 4 | 1.64% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 5 | 1.56% | `java.util.HashMap.containsKey(Object) line: 602` |
| 6 | 1.52% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 7 | 1.40% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 8 | 1.33% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 9 | 1.11% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 10 | 1.04% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 11 | 1.03% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 12 | 0.94% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 13 | 0.92% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 14 | 0.75% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 15 | 0.70% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 16 | 0.69% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 17 | 0.69% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 18 | 0.67% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 12` |
| 19 | 0.63% | `net.minecraft.world.entity.Entity.isSprinting() line: 3501` |
| 20 | 0.58% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 21 | 0.56% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 22 | 0.56% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 23 | 0.54% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 533` |
| 24 | 0.53% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 25 | 0.53% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 26 | 0.50% | `net.minecraft.util.profiling.Profiler.get() line: 47` |
| 27 | 0.46% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |
| 28 | 0.46% | `net.minecraft.world.entity.Entity.setDeltaMovement(double, double, double) line: 4891` |
| 29 | 0.42% | `java.util.Map.computeIfAbsent(Object, Function) line: 1064` |
| 30 | 0.39% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 212` |

GC pauses: 0, total 0 ms

