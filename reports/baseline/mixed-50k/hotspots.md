# JVM hotspots (JFR ExecutionSample)

Samples: 12096

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 10.16% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.83% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 3.33% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 4 | 3.10% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 5 | 2.14% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 6 | 2.02% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 7 | 1.74% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 8 | 1.74% | `java.util.HashMap.getNode(Object) line: 585` |
| 9 | 1.73% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.69% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 11 | 1.58% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 12 | 1.44% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 13 | 1.41% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 14 | 1.26% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 15 | 1.22% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 16 | 1.20% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 17 | 1.19% | `java.lang.ThreadLocal.get() line: 172` |
| 18 | 1.04% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 47` |
| 19 | 1.01% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 20 | 0.98% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 21 | 0.98% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 22 | 0.93% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 23 | 0.88% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 61` |
| 24 | 0.82% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 25 | 0.79% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 26 | 0.75% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 27 | 0.74% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 28 | 0.71% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 29 | 0.70% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 30 | 0.69% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 31 | 0.69% | `java.lang.Enum.ordinal() line: 128` |
| 32 | 0.65% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 33 | 0.62% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(ResourceLocation) line: 68` |
| 34 | 0.59% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1661` |
| 35 | 0.56% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 36 | 0.54% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 37 | 0.52% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 38 | 0.51% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 39 | 0.50% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 40 | 0.49% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1414` |
| 41 | 0.48% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 42 | 0.46% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 43 | 0.46% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 44 | 0.45% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges() line: 3348` |
| 45 | 0.44% | `net.minecraft.world.entity.Entity.lambda$checkInsideBlocks$2(LongSet, Vec3, Vec3, InsideBlockEffectApplier$StepBasedCollector, BlockPos, int) line: 1693` |
| 46 | 0.44% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 47 | 0.44% | `net.minecraft.world.entity.Entity.isEyeInFluid(TagKey) line: 2077` |
| 48 | 0.42% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1302` |
| 49 | 0.42% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1169` |
| 50 | 0.38% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 51 | 0.37% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 52 | 0.37% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 81` |
| 53 | 0.37% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 135` |
| 54 | 0.36% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 55 | 0.36% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 56 | 0.36% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 57 | 0.34% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 58 | 0.33% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 59 | 0.33% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 60 | 0.33% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.getDouble(Object) line: 431` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 10.16% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 4.24% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 3.83% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 4 | 3.38% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 5 | 3.33% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 6 | 3.17% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 7 | 2.51% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 8 | 2.08% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 1.76% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 10 | 1.74% | `java.util.HashMap.containsKey(Object) line: 602` |
| 11 | 1.69% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 12 | 1.49% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 13 | 1.43% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 14 | 1.26% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |
| 15 | 1.22% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 16 | 1.17% | `net.minecraft.util.profiling.Profiler.get() line: 47` |
| 17 | 1.16% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 18 | 1.15% | `net.minecraft.world.entity.Entity.isSprinting() line: 3501` |
| 19 | 1.15% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 20 | 1.09% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 21 | 1.04% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2717` |
| 22 | 0.98% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 23 | 0.88% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 12` |
| 24 | 0.82% | `java.util.Map.computeIfAbsent(Object, Function) line: 1064` |
| 25 | 0.82% | `net.minecraft.world.entity.Entity.getGravity() line: 1842` |
| 26 | 0.79% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 157` |
| 27 | 0.77% | `java.util.EnumMap.get(Object) line: 247` |
| 28 | 0.75% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 29 | 0.66% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 30 | 0.63% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1317` |

GC pauses: 0, total 0 ms

