# JVM hotspots (JFR ExecutionSample)

Samples: 11928

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 10.66% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.59% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 3.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 2.17% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 5 | 2.17% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 6 | 1.84% | `java.util.HashMap.getNode(Object) line: 585` |
| 7 | 1.69% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 1.59% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 9 | 1.59% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 10 | 1.58% | `java.lang.ThreadLocal.get() line: 172` |
| 11 | 1.51% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 12 | 1.49% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 13 | 1.39% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 14 | 1.39% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 15 | 1.37% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 16 | 1.34% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 17 | 1.19% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 18 | 1.12% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 19 | 1.10% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 20 | 1.06% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3459` |
| 21 | 0.92% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 22 | 0.90% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 23 | 0.86% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 24 | 0.85% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 47` |
| 25 | 0.84% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 26 | 0.71% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 27 | 0.71% | `net.minecraft.world.entity.InterpolationHandler.hasActiveInterpolation() line: 69` |
| 28 | 0.70% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 29 | 0.70% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 30 | 0.69% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(ResourceLocation) line: 68` |
| 31 | 0.69% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 32 | 0.66% | `net.minecraft.world.entity.Entity.isEyeInFluid(TagKey) line: 2077` |
| 33 | 0.64% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 34 | 0.62% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 35 | 0.62% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 36 | 0.60% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 37 | 0.56% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 38 | 0.54% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 39 | 0.53% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 40 | 0.53% | `java.lang.Enum.ordinal() line: 128` |
| 41 | 0.51% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1414` |
| 42 | 0.49% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 43 | 0.48% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges() line: 3348` |
| 44 | 0.47% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.clear() line: 442` |
| 45 | 0.45% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 46 | 0.44% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 47 | 0.43% | `java.util.EnumMap.get(Object) line: 247` |
| 48 | 0.42% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1302` |
| 49 | 0.39% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 50 | 0.39% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 51 | 0.38% | `net.minecraft.world.entity.Entity.getAirSupply() line: 3608` |
| 52 | 0.36% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 69` |
| 53 | 0.35% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 54 | 0.34% | `java.util.ArrayDeque.size() line: 648` |
| 55 | 0.34% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1169` |
| 56 | 0.34% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |
| 57 | 0.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 58 | 0.33% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.getDouble(Object) line: 431` |
| 59 | 0.33% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 135` |
| 60 | 0.32% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 10.66% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 4.53% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 3.59% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 4 | 3.09% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 3.06% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 6 | 2.68% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 7 | 2.02% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 8 | 1.88% | `java.util.HashMap.containsKey(Object) line: 602` |
| 9 | 1.79% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 10 | 1.63% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 11 | 1.48% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 12 | 1.38% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 13 | 1.37% | `net.minecraft.util.profiling.Profiler.get() line: 47` |
| 14 | 1.36% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 15 | 1.32% | `net.minecraft.world.entity.Entity.isSprinting() line: 3501` |
| 16 | 1.24% | `java.util.Map.computeIfAbsent(Object, Function) line: 1064` |
| 17 | 1.10% | `java.util.stream.AbstractPipeline.wrapAndCopyInto(Sink, Spliterator) line: 499` |
| 18 | 1.02% | `java.util.EnumMap.get(Object) line: 247` |
| 19 | 0.98% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |
| 20 | 0.94% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 21 | 0.94% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 22 | 0.91% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 23 | 0.86% | `net.minecraft.world.entity.Entity.getGravity() line: 1842` |
| 24 | 0.86% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 25 | 0.85% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2717` |
| 26 | 0.84% | `net.minecraft.world.entity.LivingEntity.tick() line: 3267` |
| 27 | 0.75% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 28 | 0.71% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3343` |
| 29 | 0.69% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 533` |
| 30 | 0.69% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 157` |

GC pauses: 0, total 0 ms

