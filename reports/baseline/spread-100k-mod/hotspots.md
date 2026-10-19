# JVM hotspots (JFR ExecutionSample)

Samples: 80522

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 19.99% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.58% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 2.42% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 1.90% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 5 | 1.66% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 6 | 1.61% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 7 | 1.52% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 8 | 1.35% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1215` |
| 9 | 1.31% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 10 | 1.20% | `java.util.HashMap.getNode(Object) line: 585` |
| 11 | 1.13% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 12 | 1.08% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 13 | 1.05% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 14 | 1.01% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 15 | 0.99% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 208` |
| 16 | 0.91% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 17 | 0.91% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 18 | 0.89% | `net.minecraft.server.level.ServerChunkCache.getChunkNow(int, int) line: 337` |
| 19 | 0.89% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 20 | 0.86% | `java.lang.Enum.ordinal() line: 128` |
| 21 | 0.86% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 22 | 0.84% | `dev.planeta9091.centities.CeHooks.batchUpsert(int, int, AABB) line: 235` |
| 23 | 0.81% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 24 | 0.77% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 25 | 0.71% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 26 | 0.70% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 27 | 0.69% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 61` |
| 28 | 0.68% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 29 | 0.66% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 30 | 0.66% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 31 | 0.65% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 32 | 0.63% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 33 | 0.57% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 34 | 0.55% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 196` |
| 35 | 0.50% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 135` |
| 36 | 0.49% | `java.lang.ThreadLocal.get() line: 172` |
| 37 | 0.48% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 38 | 0.44% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 39 | 0.41% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 40 | 0.41% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 41 | 0.41% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 42 | 0.38% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 43 | 0.37% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 44 | 0.37% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 45 | 0.37% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 444` |
| 46 | 0.37% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 47 | 0.35% | `java.util.EnumMap.get(Object) line: 247` |
| 48 | 0.34% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges() line: 3348` |
| 49 | 0.33% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |
| 50 | 0.33% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 51 | 0.32% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 52 | 0.32% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 81` |
| 53 | 0.32% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 54 | 0.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 323` |
| 55 | 0.31% | `net.minecraft.world.item.ItemStack.getDamageValue() line: 597` |
| 56 | 0.30% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 57 | 0.29% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 58 | 0.28% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 59 | 0.28% | `it.unimi.dsi.fastutil.objects.ObjectOpenHashSet$SetIterator.<init>(ObjectOpenHashSet, ObjectOpenHashSet$1) line: 534` |
| 60 | 0.27% | `dev.planeta9091.centities.CeHooks.resumeIndexing() line: 420` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 19.99% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 3.91% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 2.42% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 4 | 2.29% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 5 | 2.00% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 6 | 1.66% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 7 | 1.61% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 8 | 1.59% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 9 | 1.59% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 10 | 1.42% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 11 | 1.41% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 12 | 1.37% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1161` |
| 13 | 1.37% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 14 | 1.21% | `java.util.HashMap.containsKey(Object) line: 602` |
| 15 | 1.14% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 16 | 1.12% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 17 | 1.01% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 18 | 0.99% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 19 | 0.95% | `dev.planeta9091.centities.CeHooks.resumeIndexing() line: 424` |
| 20 | 0.89% | `net.minecraft.server.level.ServerChunkCache.hasChunk(int, int) line: 419` |
| 21 | 0.86% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4724` |
| 22 | 0.80% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 23 | 0.77% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 24 | 0.72% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 12` |
| 25 | 0.70% | `java.util.Map.computeIfAbsent(Object, Function) line: 1064` |
| 26 | 0.67% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 27 | 0.61% | `net.minecraft.server.level.FullChunkStatus.isOrAfter(FullChunkStatus) line: 10` |
| 28 | 0.59% | `net.minecraft.world.entity.Entity.isSprinting() line: 3501` |
| 29 | 0.57% | `java.util.EnumMap.get(Object) line: 247` |
| 30 | 0.57% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |

GC pauses: 0, total 0 ms

