# JVM hotspots (JFR ExecutionSample)

Samples: 82583

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 31.34% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 6.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 2.54% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 4 | 2.50% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 2.48% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 6 | 1.61% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 7 | 1.37% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 8 | 1.36% | `java.util.HashMap.getNode(Object) line: 585` |
| 9 | 1.29% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 10 | 1.11% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 11 | 1.02% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 0.93% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 13 | 0.84% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 14 | 0.81% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 15 | 0.80% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 16 | 0.79% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 61` |
| 17 | 0.73% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 18 | 0.68% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 19 | 0.68% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 20 | 0.67% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 21 | 0.67% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 22 | 0.66% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 23 | 0.64% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 24 | 0.64% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 25 | 0.64% | `it.unimi.dsi.fastutil.objects.ObjectOpenHashSet$SetIterator.<init>(ObjectOpenHashSet, ObjectOpenHashSet$1) line: 534` |
| 26 | 0.62% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 27 | 0.61% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 47` |
| 28 | 0.58% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 29 | 0.57% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 30 | 0.57% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 31 | 0.55% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 32 | 0.54% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(ResourceLocation) line: 68` |
| 33 | 0.52% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 34 | 0.51% | `java.lang.Enum.ordinal() line: 128` |
| 35 | 0.48% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 36 | 0.47% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 37 | 0.42% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1061` |
| 38 | 0.40% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 39 | 0.40% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 40 | 0.36% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 41 | 0.35% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 42 | 0.34% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 43 | 0.33% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 44 | 0.32% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges() line: 3348` |
| 45 | 0.32% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1302` |
| 46 | 0.32% | `net.minecraft.world.item.ItemStack.getMaxStackSize() line: 581` |
| 47 | 0.31% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 48 | 0.30% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 49 | 0.30% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 50 | 0.29% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 51 | 0.29% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.clear() line: 442` |
| 52 | 0.28% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 119` |
| 53 | 0.28% | `java.lang.ThreadLocal.get() line: 172` |
| 54 | 0.28% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 135` |
| 55 | 0.27% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 56 | 0.26% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1414` |
| 57 | 0.26% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoFluidPushing() line: 1960` |
| 58 | 0.25% | `java.util.ArrayDeque.size() line: 648` |
| 59 | 0.25% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 81` |
| 60 | 0.23% | `net.minecraft.core.Vec3i.getZ() line: 71` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 31.34% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 8.29% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 3.14% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 4 | 2.54% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 5 | 2.48% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 6 | 2.12% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 7 | 1.72% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 8 | 1.70% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 9 | 1.63% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 10 | 1.36% | `java.util.HashMap.containsKey(Object) line: 602` |
| 11 | 0.94% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 12 | 0.93% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 13 | 0.91% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 14 | 0.90% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 15 | 0.89% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 16 | 0.89% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 17 | 0.85% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 18 | 0.82% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 19 | 0.79% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 12` |
| 20 | 0.73% | `java.util.EnumMap.get(Object) line: 247` |
| 21 | 0.66% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 22 | 0.65% | `java.util.Map.computeIfAbsent(Object, Function) line: 1064` |
| 23 | 0.64% | `it.unimi.dsi.fastutil.objects.ObjectOpenHashSet.iterator() line: 654` |
| 24 | 0.62% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 25 | 0.61% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2717` |
| 26 | 0.58% | `net.minecraft.world.entity.Entity.isSprinting() line: 3501` |
| 27 | 0.54% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 533` |
| 28 | 0.48% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 212` |
| 29 | 0.46% | `net.minecraft.world.entity.item.ItemEntity.tick()` |
| 30 | 0.43% | `net.minecraft.world.entity.Entity.setDeltaMovement(double, double, double) line: 4891` |

GC pauses: 0, total 0 ms

