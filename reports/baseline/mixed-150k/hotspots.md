# JVM hotspots (JFR ExecutionSample)

Samples: 91660

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 30.54% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 6.23% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 3.47% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 3.42% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 5 | 2.08% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 6 | 1.53% | `java.util.HashMap.getNode(Object) line: 585` |
| 7 | 1.37% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 8 | 1.23% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 9 | 1.21% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 10 | 1.14% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.04% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 12 | 0.97% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 13 | 0.91% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 14 | 0.89% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 15 | 0.87% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 16 | 0.86% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 17 | 0.85% | `java.lang.ThreadLocal.get() line: 172` |
| 18 | 0.80% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 19 | 0.79% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 20 | 0.77% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 21 | 0.73% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 22 | 0.71% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 23 | 0.70% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 24 | 0.69% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 61` |
| 25 | 0.69% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 26 | 0.63% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 27 | 0.59% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 28 | 0.59% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 29 | 0.58% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(ResourceLocation) line: 68` |
| 30 | 0.55% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 47` |
| 31 | 0.55% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 32 | 0.45% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 33 | 0.45% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 34 | 0.45% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 35 | 0.42% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 36 | 0.40% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 37 | 0.39% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 38 | 0.37% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 39 | 0.37% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 40 | 0.36% | `java.lang.Enum.ordinal() line: 128` |
| 41 | 0.35% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 42 | 0.32% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 43 | 0.32% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1302` |
| 44 | 0.32% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 45 | 0.31% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 46 | 0.30% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 47 | 0.30% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |
| 48 | 0.29% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1661` |
| 49 | 0.28% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 50 | 0.28% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges() line: 3348` |
| 51 | 0.27% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 52 | 0.27% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1414` |
| 53 | 0.27% | `net.minecraft.world.entity.Entity.getAirSupply() line: 3608` |
| 54 | 0.26% | `java.util.EnumMap.get(Object) line: 247` |
| 55 | 0.25% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.clear() line: 442` |
| 56 | 0.25% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 135` |
| 57 | 0.24% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 58 | 0.24% | `java.util.ArrayDeque.size() line: 648` |
| 59 | 0.24% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 60 | 0.24% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 30.54% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 10.40% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 4.42% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 4 | 2.08% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 5 | 1.90% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 1.57% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 7 | 1.53% | `java.util.HashMap.containsKey(Object) line: 602` |
| 8 | 1.37% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 9 | 1.23% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 10 | 1.05% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 11 | 1.01% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 12 | 0.97% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 13 | 0.93% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 14 | 0.88% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 15 | 0.87% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 16 | 0.86% | `net.minecraft.world.entity.Entity.isSprinting() line: 3501` |
| 17 | 0.80% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 18 | 0.80% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |
| 19 | 0.79% | `net.minecraft.util.profiling.Profiler.get() line: 47` |
| 20 | 0.75% | `java.util.Map.computeIfAbsent(Object, Function) line: 1064` |
| 21 | 0.74% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 22 | 0.70% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 12` |
| 23 | 0.65% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 24 | 0.58% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 533` |
| 25 | 0.55% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 26 | 0.55% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2717` |
| 27 | 0.48% | `java.util.EnumMap.get(Object) line: 247` |
| 28 | 0.46% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 29 | 0.45% | `net.minecraft.world.entity.Mob.serverAiStep() line: 759` |
| 30 | 0.45% | `net.minecraft.world.entity.Entity.getGravity() line: 1842` |

GC pauses: 0, total 0 ms

