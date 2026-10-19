# JVM hotspots (JFR ExecutionSample)

Samples: 12547

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 13.60% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.63% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 2.65% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 2.49% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 5 | 2.05% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 6 | 1.94% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 7 | 1.86% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 1.75% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 9 | 1.54% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.39% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 11 | 1.36% | `java.lang.ThreadLocal.get() line: 172` |
| 12 | 1.35% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 13 | 1.25% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 14 | 1.21% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 15 | 1.19% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 61` |
| 16 | 1.15% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 17 | 1.12% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 18 | 0.96% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 19 | 0.95% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 20 | 0.92% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 21 | 0.92% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 22 | 0.91% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3459` |
| 23 | 0.88% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 24 | 0.87% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 25 | 0.85% | `net.minecraft.world.entity.InterpolationHandler.hasActiveInterpolation() line: 69` |
| 26 | 0.85% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 27 | 0.76% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 28 | 0.73% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 29 | 0.72% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 30 | 0.67% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 31 | 0.65% | `java.util.EnumMap.get(Object) line: 247` |
| 32 | 0.65% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 33 | 0.64% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(ResourceLocation) line: 68` |
| 34 | 0.62% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 35 | 0.60% | `java.util.EnumMap.isValidKey(Object) line: 319` |
| 36 | 0.59% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 37 | 0.59% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 38 | 0.57% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 69` |
| 39 | 0.54% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1661` |
| 40 | 0.54% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 41 | 0.52% | `java.util.HashMap.getNode(Object) line: 585` |
| 42 | 0.50% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 43 | 0.49% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1215` |
| 44 | 0.49% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 45 | 0.48% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartB() line: 938` |
| 46 | 0.48% | `net.minecraft.world.entity.Entity.level() line: 5199` |
| 47 | 0.46% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 48 | 0.42% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 49 | 0.41% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 50 | 0.40% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 51 | 0.39% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 52 | 0.38% | `net.minecraft.world.entity.Entity.getAirSupply() line: 3608` |
| 53 | 0.37% | `it.unimi.dsi.fastutil.objects.AbstractObjectList.ensureIndex(int) line: 54` |
| 54 | 0.37% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 55 | 0.37% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 56 | 0.36% | `java.util.ArrayDeque.size() line: 648` |
| 57 | 0.36% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 47` |
| 58 | 0.36% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 59 | 0.35% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 60 | 0.34% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 13.60% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 5.39% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 3.63% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 4 | 3.25% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 5 | 2.65% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 2.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 7 | 2.30% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 8 | 2.06% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 9 | 2.04% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 10 | 1.53% | `java.util.Map.computeIfAbsent(Object, Function) line: 1064` |
| 11 | 1.34% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 12 | 1.32% | `net.minecraft.util.profiling.Profiler.get() line: 47` |
| 13 | 1.20% | `net.minecraft.world.entity.Entity.isSprinting() line: 3501` |
| 14 | 1.20% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 12` |
| 15 | 0.97% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |
| 16 | 0.96% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 17 | 0.92% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 18 | 0.92% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 904` |
| 19 | 0.91% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 20 | 0.88% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 235` |
| 21 | 0.85% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3343` |
| 22 | 0.80% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 23 | 0.77% | `net.minecraft.world.entity.LivingEntity.tick() line: 3267` |
| 24 | 0.77% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 25 | 0.77% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 26 | 0.72% | `java.util.EnumMap.get(Object) line: 247` |
| 27 | 0.71% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 28 | 0.69% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 29 | 0.66% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 157` |
| 30 | 0.65% | `net.minecraft.world.entity.Entity.setDeltaMovement(double, double, double) line: 4891` |

GC pauses: 0, total 0 ms

