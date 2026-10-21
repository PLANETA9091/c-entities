# JVM hotspots (JFR ExecutionSample)

Samples: 13163

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 13.88% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.72% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 3.56% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 2.18% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 5 | 2.14% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 6 | 1.76% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 7 | 1.60% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 8 | 1.53% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 9 | 1.53% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.48% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 11 | 1.32% | `java.util.HashMap.getNode(Object) line: 585` |
| 12 | 1.27% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 13 | 1.22% | `java.lang.ThreadLocal.get() line: 172` |
| 14 | 1.18% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 15 | 1.09% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 61` |
| 16 | 1.08% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 17 | 1.06% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 18 | 1.05% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 19 | 1.00% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 20 | 0.93% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 47` |
| 21 | 0.91% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 22 | 0.87% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 23 | 0.84% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 24 | 0.84% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 25 | 0.84% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 26 | 0.82% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(ResourceLocation) line: 68` |
| 27 | 0.77% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 28 | 0.74% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 29 | 0.71% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 30 | 0.69% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 31 | 0.69% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 32 | 0.68% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 33 | 0.66% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 34 | 0.63% | `java.lang.Enum.ordinal() line: 128` |
| 35 | 0.60% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 36 | 0.52% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 37 | 0.51% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 38 | 0.49% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 39 | 0.48% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1414` |
| 40 | 0.48% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1661` |
| 41 | 0.48% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 42 | 0.44% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 43 | 0.44% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 44 | 0.43% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 45 | 0.43% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 46 | 0.43% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 47 | 0.43% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 48 | 0.41% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1302` |
| 49 | 0.40% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 69` |
| 50 | 0.40% | `net.minecraft.world.entity.Entity.lambda$checkInsideBlocks$2(LongSet, Vec3, Vec3, InsideBlockEffectApplier$StepBasedCollector, BlockPos, int) line: 1693` |
| 51 | 0.36% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 52 | 0.36% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 53 | 0.36% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges() line: 3348` |
| 54 | 0.36% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 55 | 0.35% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 56 | 0.35% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1169` |
| 57 | 0.34% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 58 | 0.33% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 59 | 0.33% | `java.util.EnumMap.get(Object) line: 247` |
| 60 | 0.33% | `net.minecraft.util.Mth.floor(double) line: 66` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 13.88% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 4.66% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 3.72% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 4 | 3.41% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 5 | 2.61% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 6 | 2.52% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 7 | 1.79% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 8 | 1.64% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 1.52% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 10 | 1.48% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 11 | 1.47% | `java.util.Map.computeIfAbsent(Object, Function) line: 1064` |
| 12 | 1.39% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 13 | 1.33% | `java.util.HashMap.containsKey(Object) line: 602` |
| 14 | 1.33% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 15 | 1.16% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 16 | 1.16% | `net.minecraft.world.entity.Entity.isSprinting() line: 3501` |
| 17 | 1.12% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 18 | 1.11% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 19 | 1.09% | `net.minecraft.util.profiling.Profiler.get() line: 47` |
| 20 | 1.09% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 12` |
| 21 | 1.06% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 22 | 1.06% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 23 | 1.06% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |
| 24 | 0.94% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2717` |
| 25 | 0.84% | `net.minecraft.world.entity.Entity.getGravity() line: 1842` |
| 26 | 0.82% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 533` |
| 27 | 0.77% | `java.util.EnumMap.get(Object) line: 247` |
| 28 | 0.73% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 157` |
| 29 | 0.69% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 30 | 0.66% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 904` |

GC pauses: 0, total 0 ms

