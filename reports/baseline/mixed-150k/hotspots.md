# JVM hotspots (JFR ExecutionSample)

Samples: 122388

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 36.09% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 5.66% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 3.88% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 4 | 2.83% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 1.43% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 6 | 1.04% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 7 | 1.04% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 8 | 0.95% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 9 | 0.87% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 10 | 0.84% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 11 | 0.82% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 0.81% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 13 | 0.80% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 14 | 0.64% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 15 | 0.64% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 16 | 0.64% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 17 | 0.63% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 18 | 0.62% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 19 | 0.57% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 20 | 0.49% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 21 | 0.46% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 22 | 0.46% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 23 | 0.45% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 24 | 0.45% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 25 | 0.43% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 26 | 0.43% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 27 | 0.43% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 28 | 0.43% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 29 | 0.42% | `java.util.HashMap.getNode(Object) line: 585` |
| 30 | 0.42% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 31 | 0.41% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(ResourceLocation) line: 68` |
| 32 | 0.40% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 33 | 0.39% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 34 | 0.39% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 61` |
| 35 | 0.38% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 36 | 0.38% | `it.unimi.dsi.fastutil.objects.ObjectOpenHashSet$SetIterator.<init>(ObjectOpenHashSet, ObjectOpenHashSet$1) line: 534` |
| 37 | 0.37% | `net.minecraft.world.entity.Entity.lambda$checkInsideBlocks$2(LongSet, Vec3, Vec3, InsideBlockEffectApplier$StepBasedCollector, BlockPos, int) line: 1693` |
| 38 | 0.35% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 39 | 0.33% | `java.lang.Enum.ordinal() line: 128` |
| 40 | 0.32% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 41 | 0.31% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 42 | 0.31% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 43 | 0.31% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 44 | 0.29% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 45 | 0.29% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4752` |
| 46 | 0.28% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 47` |
| 47 | 0.28% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 48 | 0.27% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 49 | 0.26% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 50 | 0.26% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 51 | 0.24% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 52 | 0.24% | `java.util.EnumMap.get(Object) line: 247` |
| 53 | 0.22% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 54 | 0.21% | `java.util.EnumMap.isValidKey(Object) line: 319` |
| 55 | 0.20% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 56 | 0.20% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges() line: 3348` |
| 57 | 0.19% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 58 | 0.19% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1661` |
| 59 | 0.19% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 135` |
| 60 | 0.19% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 444` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 36.09% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 10.01% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 3.85% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 4 | 1.59% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 5 | 1.43% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 6 | 1.25% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 7 | 1.17% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 8 | 1.15% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 9 | 1.04% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 10 | 0.96% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 11 | 0.95% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 12 | 0.90% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 13 | 0.81% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 14 | 0.65% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 15 | 0.65% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 16 | 0.59% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 17 | 0.57% | `net.minecraft.world.entity.Entity.isSprinting() line: 3501` |
| 18 | 0.53% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoFluidPushing() line: 1963` |
| 19 | 0.53% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 20 | 0.49% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 21 | 0.48% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 22 | 0.46% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |
| 23 | 0.44% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 24 | 0.43% | `java.util.HashMap.containsKey(Object) line: 602` |
| 25 | 0.42% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 26 | 0.41% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 533` |
| 27 | 0.40% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 904` |
| 28 | 0.39% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 12` |
| 29 | 0.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 30 | 0.38% | `net.minecraft.world.level.BlockGetter.forEachBlockIntersectedBetween(Vec3, Vec3, AABB, BlockGetter$BlockStepVisitor) line: 219` |

GC pauses: 0, total 0 ms

