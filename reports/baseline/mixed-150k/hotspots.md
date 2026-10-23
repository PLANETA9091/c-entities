# JVM hotspots (JFR ExecutionSample)

Samples: 75204

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 22.81% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 6.62% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 2.99% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 4 | 2.83% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 2.40% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 6 | 1.99% | `java.util.HashMap.getNode(Object) line: 585` |
| 7 | 1.70% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 8 | 1.54% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 9 | 1.47% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.41% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 11 | 1.30% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 12 | 1.26% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 1.17% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 14 | 1.08% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 15 | 1.05% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 16 | 1.05% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 17 | 1.04% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 18 | 1.03% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 19 | 0.94% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 20 | 0.92% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 21 | 0.89% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 22 | 0.86% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 61` |
| 23 | 0.84% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 24 | 0.83% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 25 | 0.80% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 26 | 0.76% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 27 | 0.66% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 47` |
| 28 | 0.65% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(ResourceLocation) line: 68` |
| 29 | 0.64% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 30 | 0.57% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 31 | 0.53% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 32 | 0.53% | `it.unimi.dsi.fastutil.objects.ObjectOpenHashSet$SetIterator.<init>(ObjectOpenHashSet, ObjectOpenHashSet$1) line: 534` |
| 33 | 0.52% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 34 | 0.51% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 35 | 0.48% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 36 | 0.46% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 37 | 0.45% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 38 | 0.45% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1661` |
| 39 | 0.44% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 40 | 0.41% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1302` |
| 41 | 0.41% | `java.lang.Enum.ordinal() line: 128` |
| 42 | 0.40% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 43 | 0.40% | `net.minecraft.world.entity.Entity.lambda$checkInsideBlocks$2(LongSet, Vec3, Vec3, InsideBlockEffectApplier$StepBasedCollector, BlockPos, int) line: 1693` |
| 44 | 0.40% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 45 | 0.38% | `java.util.EnumMap.get(Object) line: 247` |
| 46 | 0.37% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 47 | 0.37% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 48 | 0.36% | `java.lang.ThreadLocal.get() line: 172` |
| 49 | 0.35% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1169` |
| 50 | 0.32% | `java.util.ArrayDeque.size() line: 648` |
| 51 | 0.32% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |
| 52 | 0.32% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges() line: 3348` |
| 53 | 0.30% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 54 | 0.30% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 81` |
| 55 | 0.30% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 135` |
| 56 | 0.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 57 | 0.29% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 58 | 0.27% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 59 | 0.26% | `net.minecraft.world.entity.LivingEntity.getArrowCount() line: 2528` |
| 60 | 0.26% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.clear() line: 442` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 22.81% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 9.23% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 3.74% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 4 | 2.99% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 5 | 2.77% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 2.40% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 7 | 2.01% | `java.util.HashMap.containsKey(Object) line: 602` |
| 8 | 1.88% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 9 | 1.70% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 10 | 1.59% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 11 | 1.34% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 12 | 1.22% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 13 | 1.13% | `java.util.Map.computeIfAbsent(Object, Function) line: 1064` |
| 14 | 1.05% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 15 | 1.05% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 16 | 1.05% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 17 | 1.04% | `net.minecraft.world.entity.Entity.isSprinting() line: 3501` |
| 18 | 0.94% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 549` |
| 19 | 0.87% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |
| 20 | 0.86% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 12` |
| 21 | 0.84% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 22 | 0.84% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 23 | 0.82% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 24 | 0.74% | `java.util.EnumMap.get(Object) line: 247` |
| 25 | 0.66% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2717` |
| 26 | 0.65% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 533` |
| 27 | 0.64% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 28 | 0.57% | `net.minecraft.world.entity.Entity.getGravity() line: 1842` |
| 29 | 0.55% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 30 | 0.53% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 212` |

GC pauses: 0, total 0 ms

