# JVM hotspots (JFR ExecutionSample)

Samples: 115206

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 32.95% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 5.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 4.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 4 | 2.85% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 1.61% | `java.util.HashMap.getNode(Object) line: 585` |
| 6 | 1.60% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 7 | 1.24% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 8 | 1.13% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 9 | 1.02% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 10 | 0.95% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 11 | 0.87% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 12 | 0.83% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 0.77% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 14 | 0.69% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 15 | 0.67% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 16 | 0.64% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 17 | 0.62% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 18 | 0.61% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 19 | 0.61% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 20 | 0.52% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 21 | 0.51% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 22 | 0.48% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 23 | 0.47% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 24 | 0.47% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 25 | 0.47% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 61` |
| 26 | 0.46% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 27 | 0.46% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 28 | 0.45% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 29 | 0.45% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 30 | 0.44% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 31 | 0.42% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(ResourceLocation) line: 68` |
| 32 | 0.41% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 33 | 0.40% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 34 | 0.39% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 35 | 0.39% | `it.unimi.dsi.fastutil.objects.ObjectOpenHashSet$SetIterator.<init>(ObjectOpenHashSet, ObjectOpenHashSet$1) line: 534` |
| 36 | 0.38% | `java.lang.Enum.ordinal() line: 128` |
| 37 | 0.37% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 38 | 0.34% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 47` |
| 39 | 0.34% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 40 | 0.33% | `java.lang.ThreadLocal.get() line: 172` |
| 41 | 0.33% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 42 | 0.32% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 43 | 0.30% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 44 | 0.30% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 45 | 0.29% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 46 | 0.29% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 47 | 0.29% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 48 | 0.29% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 49 | 0.27% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 50 | 0.26% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 51 | 0.25% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 52 | 0.25% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 53 | 0.23% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 54 | 0.23% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges() line: 3348` |
| 55 | 0.23% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |
| 56 | 0.22% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 57 | 0.22% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 135` |
| 58 | 0.21% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 59 | 0.21% | `java.util.EnumMap.get(Object) line: 247` |
| 60 | 0.21% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 32.95% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 10.12% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 4.07% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 4 | 1.82% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 5 | 1.62% | `java.util.HashMap.containsKey(Object) line: 602` |
| 6 | 1.60% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 7 | 1.30% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 8 | 1.24% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 9 | 1.15% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 10 | 1.02% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 11 | 0.87% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 12 | 0.83% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 13 | 0.79% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 14 | 0.69% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 15 | 0.65% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 16 | 0.64% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 17 | 0.61% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 18 | 0.59% | `net.minecraft.world.entity.Entity.isSprinting() line: 3501` |
| 19 | 0.52% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 20 | 0.52% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |
| 21 | 0.47% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 12` |
| 22 | 0.46% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 23 | 0.45% | `java.util.EnumMap.get(Object) line: 247` |
| 24 | 0.45% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 25 | 0.42% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 533` |
| 26 | 0.42% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 27 | 0.41% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 28 | 0.40% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 29 | 0.40% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 30 | 0.39% | `net.minecraft.world.entity.Entity.getGravity() line: 1842` |

GC pauses: 0, total 0 ms

