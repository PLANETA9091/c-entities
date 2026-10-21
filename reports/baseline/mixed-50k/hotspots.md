# JVM hotspots (JFR ExecutionSample)

Samples: 13188

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 24.77% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.04% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 1.86% | `java.util.HashMap.getNode(Object) line: 585` |
| 4 | 1.80% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 5 | 1.79% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 6 | 1.28% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 7 | 1.16% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 1.02% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 9 | 0.96% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 10 | 0.89% | `java.lang.ThreadLocal.get() line: 172` |
| 11 | 0.77% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 12 | 0.75% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 13 | 0.74% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 14 | 0.72% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 15 | 0.71% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 16 | 0.70% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3459` |
| 17 | 0.69% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 18 | 0.68% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 19 | 0.67% | `java.lang.Enum.ordinal() line: 128` |
| 20 | 0.64% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 69` |
| 21 | 0.61% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 22 | 0.61% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 23 | 0.61% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 24 | 0.61% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 25 | 0.58% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 26 | 0.55% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 27 | 0.55% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 28 | 0.55% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 29 | 0.54% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 30 | 0.54% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 61` |
| 31 | 0.49% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 32 | 0.49% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 33 | 0.49% | `java.util.EnumMap.get(Object) line: 247` |
| 34 | 0.48% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 444` |
| 35 | 0.45% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 36 | 0.45% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 37 | 0.43% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 38 | 0.42% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 39 | 0.42% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 40 | 0.41% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 41 | 0.40% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 42 | 0.40% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(ResourceLocation) line: 68` |
| 43 | 0.39% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 44 | 0.39% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4882` |
| 45 | 0.39% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.getDouble(Object) line: 431` |
| 46 | 0.38% | `java.util.EnumMap.isValidKey(Object) line: 319` |
| 47 | 0.38% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1414` |
| 48 | 0.37% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 49 | 0.37% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 50 | 0.36% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 135` |
| 51 | 0.36% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1169` |
| 52 | 0.36% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 53 | 0.35% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |
| 54 | 0.34% | `net.minecraft.util.Mth.wrapDegrees(float) line: 221` |
| 55 | 0.30% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 56 | 0.29% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 57 | 0.29% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 58 | 0.29% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 59 | 0.29% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 60 | 0.28% | `net.minecraft.world.entity.InterpolationHandler.hasActiveInterpolation() line: 69` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 24.77% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 5.47% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 1.92% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 4 | 1.85% | `java.util.HashMap.containsKey(Object) line: 602` |
| 5 | 1.79% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 6 | 1.72% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 7 | 1.46% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 8 | 1.30% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 1.30% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 10 | 1.10% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 11 | 0.94% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 12 | 0.89% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 13 | 0.82% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 14 | 0.75% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 15 | 0.75% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 16 | 0.71% | `net.minecraft.util.profiling.Profiler.get() line: 47` |
| 17 | 0.71% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 119` |
| 18 | 0.69% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 19 | 0.65% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 20 | 0.64% | `net.minecraft.world.entity.Entity.isSprinting() line: 3501` |
| 21 | 0.63% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 22 | 0.63% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 212` |
| 23 | 0.63% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 24 | 0.63% | `net.minecraft.world.entity.LivingEntity.tick() line: 3267` |
| 25 | 0.58% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 26 | 0.55% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4724` |
| 27 | 0.55% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoFluidPushing() line: 1963` |
| 28 | 0.54% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 38` |
| 29 | 0.54% | `java.util.Map.computeIfAbsent(Object, Function) line: 1064` |
| 30 | 0.54% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 12` |

GC pauses: 0, total 0 ms

