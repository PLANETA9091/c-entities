# JVM hotspots (JFR ExecutionSample)

Samples: 84501

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 26.35% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 6.68% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 3.18% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 2.32% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 1.68% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 6 | 1.53% | `java.util.HashMap.getNode(Object) line: 585` |
| 7 | 1.53% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 8 | 1.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 9 | 1.38% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 10 | 1.30% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.11% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 12 | 1.06% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 13 | 0.97% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 14 | 0.94% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 15 | 0.92% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 16 | 0.88% | `java.lang.ThreadLocal.get() line: 172` |
| 17 | 0.87% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 18 | 0.83% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 19 | 0.83% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 20 | 0.82% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 21 | 0.81% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 22 | 0.80% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 23 | 0.78% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 24 | 0.77% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 25 | 0.70% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 61` |
| 26 | 0.70% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 27 | 0.70% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 28 | 0.66% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 29 | 0.65% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(ResourceLocation) line: 68` |
| 30 | 0.61% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 47` |
| 31 | 0.50% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 32 | 0.50% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 33 | 0.49% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 34 | 0.48% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 35 | 0.48% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 36 | 0.48% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 37 | 0.44% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 38 | 0.43% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1010` |
| 39 | 0.42% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 40 | 0.41% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |
| 41 | 0.38% | `net.minecraft.world.entity.Entity.isEyeInFluid(TagKey) line: 2077` |
| 42 | 0.37% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 43 | 0.37% | `java.lang.Enum.ordinal() line: 128` |
| 44 | 0.37% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 45 | 0.35% | `java.util.ArrayDeque.size() line: 648` |
| 46 | 0.34% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 47 | 0.33% | `net.minecraft.world.item.ItemStack.getMaxStackSize() line: 581` |
| 48 | 0.33% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 49 | 0.33% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 50 | 0.32% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges() line: 3348` |
| 51 | 0.31% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 52 | 0.30% | `net.minecraft.world.entity.LivingEntity.getArrowCount() line: 2528` |
| 53 | 0.29% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 54 | 0.29% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.clear() line: 442` |
| 55 | 0.28% | `net.minecraft.world.entity.Entity.getAirSupply() line: 3608` |
| 56 | 0.28% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1661` |
| 57 | 0.28% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 58 | 0.28% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1414` |
| 59 | 0.27% | `java.util.EnumMap.get(Object) line: 247` |
| 60 | 0.27% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 135` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 26.35% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 8.54% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 4.04% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 4 | 2.32% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 5 | 2.27% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 1.77% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 7 | 1.70% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 8 | 1.54% | `java.util.HashMap.containsKey(Object) line: 602` |
| 9 | 1.38% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 10 | 1.26% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 11 | 1.21% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 12 | 1.16% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 13 | 0.93% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 14 | 0.92% | `net.minecraft.world.entity.Entity.isSprinting() line: 3501` |
| 15 | 0.90% | `java.util.Map.computeIfAbsent(Object, Function) line: 1064` |
| 16 | 0.89% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 17 | 0.88% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 18 | 0.87% | `net.minecraft.util.profiling.Profiler.get() line: 47` |
| 19 | 0.87% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |
| 20 | 0.83% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 21 | 0.81% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 22 | 0.70% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 12` |
| 23 | 0.70% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 24 | 0.69% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 25 | 0.69% | `java.util.EnumMap.get(Object) line: 247` |
| 26 | 0.65% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 533` |
| 27 | 0.61% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2717` |
| 28 | 0.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 383` |
| 29 | 0.50% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 30 | 0.50% | `net.minecraft.world.entity.Mob.serverAiStep() line: 759` |

GC pauses: 0, total 0 ms

