# JVM hotspots (JFR ExecutionSample)

Samples: 11679

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 12.95% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.83% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 2.44% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 2.01% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 5 | 1.62% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 6 | 1.53% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 7 | 1.38% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 8 | 1.33% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1215` |
| 9 | 1.29% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.28% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 11 | 1.21% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 12 | 1.19% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 13 | 1.14% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 14 | 1.12% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 15 | 1.11% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 16 | 1.05% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 17 | 1.02% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 18 | 0.98% | `net.minecraft.world.entity.InterpolationHandler.hasActiveInterpolation() line: 69` |
| 19 | 0.96% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 20 | 0.95% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 21 | 0.92% | `java.lang.ThreadLocal.get() line: 172` |
| 22 | 0.91% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 61` |
| 23 | 0.82% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 47` |
| 24 | 0.80% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 25 | 0.80% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 26 | 0.76% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 27 | 0.75% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(ResourceLocation) line: 68` |
| 28 | 0.74% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 29 | 0.68% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4882` |
| 30 | 0.68% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 31 | 0.65% | `java.util.EnumMap.get(Object) line: 247` |
| 32 | 0.64% | `java.util.EnumMap.isValidKey(Object) line: 319` |
| 33 | 0.63% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 34 | 0.63% | `net.minecraft.world.level.chunk.PalettedContainer$Data.moonrise$getPalette() line: 458` |
| 35 | 0.59% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 36 | 0.57% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.getDouble(Object) line: 431` |
| 37 | 0.57% | `net.minecraft.world.entity.Entity.setOldRot(float, float) line: 2189` |
| 38 | 0.57% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 39 | 0.57% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 115` |
| 40 | 0.57% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3459` |
| 41 | 0.55% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 42 | 0.53% | `net.minecraft.world.entity.Entity.level() line: 5199` |
| 43 | 0.53% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 44 | 0.52% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 45 | 0.52% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 46 | 0.52% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 47 | 0.50% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 69` |
| 48 | 0.49% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 135` |
| 49 | 0.48% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 50 | 0.46% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 51 | 0.45% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 52 | 0.45% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 53 | 0.44% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 54 | 0.44% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 55 | 0.44% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 56 | 0.41% | `net.minecraft.util.profiling.Profiler.get() line: 47` |
| 57 | 0.39% | `net.minecraft.world.entity.Mob.checkDespawn() line: 715` |
| 58 | 0.39% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 59 | 0.38% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 60 | 0.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 12.95% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 3.77% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 2.83% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 4 | 2.41% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 2.23% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 6 | 2.05% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 7 | 2.00% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 8 | 1.93% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 9 | 1.51% | `net.minecraft.world.entity.Entity.isSprinting() line: 3501` |
| 10 | 1.45% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 11 | 1.40% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 12 | 1.33% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1154` |
| 13 | 1.22% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 14 | 1.03% | `net.minecraft.util.profiling.Profiler.get() line: 47` |
| 15 | 1.02% | `java.util.Map.computeIfAbsent(Object, Function) line: 1064` |
| 16 | 0.99% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 235` |
| 17 | 0.98% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3343` |
| 18 | 0.96% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 19 | 0.91% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 12` |
| 20 | 0.90% | `net.minecraft.world.entity.Entity.setDeltaMovement(double, double, double) line: 4891` |
| 21 | 0.82% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2717` |
| 22 | 0.80% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 23 | 0.80% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 212` |
| 24 | 0.75% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 533` |
| 25 | 0.75% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 26 | 0.74% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 27 | 0.72% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 28 | 0.71% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 29 | 0.69% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 157` |
| 30 | 0.68% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 78` |

GC pauses: 0, total 0 ms

