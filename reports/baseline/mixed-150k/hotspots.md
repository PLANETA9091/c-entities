# JVM hotspots (JFR ExecutionSample)

Samples: 86460

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 26.72% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 5.76% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 3.86% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 4 | 3.79% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 2.33% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 6 | 2.09% | `java.util.HashMap.getNode(Object) line: 585` |
| 7 | 1.61% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 8 | 1.43% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 9 | 1.35% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.28% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 11 | 1.07% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 12 | 1.07% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 13 | 1.05% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 14 | 0.90% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 15 | 0.87% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 16 | 0.85% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 17 | 0.84% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 18 | 0.82% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 19 | 0.82% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 20 | 0.80% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 21 | 0.79% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 22 | 0.70% | `it.unimi.dsi.fastutil.objects.ObjectOpenHashSet$SetIterator.<init>(ObjectOpenHashSet, ObjectOpenHashSet$1) line: 534` |
| 23 | 0.70% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 61` |
| 24 | 0.67% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 47` |
| 25 | 0.65% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 26 | 0.63% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 27 | 0.62% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 28 | 0.61% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 29 | 0.54% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 30 | 0.53% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 31 | 0.51% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(ResourceLocation) line: 68` |
| 32 | 0.46% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 33 | 0.46% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 34 | 0.46% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 35 | 0.44% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 36 | 0.44% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 37 | 0.41% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 38 | 0.41% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 39 | 0.39% | `java.lang.Enum.ordinal() line: 128` |
| 40 | 0.37% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 41 | 0.36% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 42 | 0.36% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 43 | 0.35% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 44 | 0.34% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 45 | 0.34% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges() line: 3348` |
| 46 | 0.33% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 47 | 0.33% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 48 | 0.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 49 | 0.31% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |
| 50 | 0.31% | `net.minecraft.world.entity.Entity.isEyeInFluid(TagKey) line: 2077` |
| 51 | 0.30% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1169` |
| 52 | 0.29% | `java.util.ArrayDeque.size() line: 648` |
| 53 | 0.28% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 54 | 0.28% | `net.minecraft.world.item.ItemStack.getDamageValue() line: 597` |
| 55 | 0.28% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 56 | 0.27% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 57 | 0.27% | `java.util.EnumMap.get(Object) line: 247` |
| 58 | 0.26% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1414` |
| 59 | 0.26% | `net.minecraft.world.entity.LivingEntity.getArrowCount() line: 2528` |
| 60 | 0.25% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1661` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 26.72% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 10.14% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 4.39% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 4 | 2.40% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 2.33% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 6 | 2.10% | `java.util.HashMap.containsKey(Object) line: 602` |
| 7 | 1.67% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 8 | 1.63% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 9 | 1.53% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 10 | 1.43% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 11 | 1.19% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 12 | 0.95% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 13 | 0.94% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 14 | 0.87% | `java.util.Map.computeIfAbsent(Object, Function) line: 1064` |
| 15 | 0.85% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 16 | 0.84% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 17 | 0.82% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 549` |
| 18 | 0.79% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 19 | 0.78% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |
| 20 | 0.74% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 21 | 0.74% | `java.util.EnumMap.get(Object) line: 247` |
| 22 | 0.71% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 23 | 0.71% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 12` |
| 24 | 0.70% | `it.unimi.dsi.fastutil.objects.ObjectOpenHashSet.iterator() line: 654` |
| 25 | 0.66% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2717` |
| 26 | 0.61% | `net.minecraft.world.entity.Entity.isSprinting() line: 3501` |
| 27 | 0.53% | `net.minecraft.world.entity.Entity.getGravity() line: 1842` |
| 28 | 0.51% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 533` |
| 29 | 0.51% | `net.minecraft.world.entity.Mob.serverAiStep() line: 759` |
| 30 | 0.50% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |

GC pauses: 0, total 0 ms

