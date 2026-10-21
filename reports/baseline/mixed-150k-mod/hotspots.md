# JVM hotspots (JFR ExecutionSample)

Samples: 98406

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 32.52% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 6.67% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 2.78% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 4 | 2.75% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 2.44% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 6 | 1.83% | `java.util.HashMap.getNode(Object) line: 585` |
| 7 | 1.52% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 8 | 1.41% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 9 | 1.18% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 10 | 1.14% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 11 | 0.99% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 12 | 0.93% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 0.88% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 14 | 0.87% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 15 | 0.84% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 16 | 0.81% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 17 | 0.77% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 18 | 0.74% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 19 | 0.72% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 20 | 0.71% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 21 | 0.70% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 22 | 0.69% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 23 | 0.69% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 61` |
| 24 | 0.66% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 25 | 0.60% | `it.unimi.dsi.fastutil.objects.ObjectOpenHashSet$SetIterator.<init>(ObjectOpenHashSet, ObjectOpenHashSet$1) line: 534` |
| 26 | 0.59% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 27 | 0.52% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 28 | 0.50% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 29 | 0.50% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 30 | 0.49% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 31 | 0.47% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 32 | 0.46% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 33 | 0.46% | `java.lang.Enum.ordinal() line: 128` |
| 34 | 0.44% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 35 | 0.43% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 36 | 0.41% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 37 | 0.40% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 38 | 0.39% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 39 | 0.39% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 40 | 0.36% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 41 | 0.35% | `net.minecraft.world.item.ItemStack.getMaxStackSize() line: 581` |
| 42 | 0.35% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 43 | 0.32% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 44 | 0.32% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges() line: 3348` |
| 45 | 0.31% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 46 | 0.31% | `net.minecraft.world.item.ItemStack.getDamageValue() line: 597` |
| 47 | 0.31% | `java.util.EnumMap.get(Object) line: 247` |
| 48 | 0.29% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 49 | 0.29% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1302` |
| 50 | 0.27% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |
| 51 | 0.27% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1169` |
| 52 | 0.26% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 53 | 0.26% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 54 | 0.25% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 47` |
| 55 | 0.24% | `java.lang.ThreadLocal.get() line: 172` |
| 56 | 0.24% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 57 | 0.24% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.clear() line: 442` |
| 58 | 0.24% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 59 | 0.24% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 60 | 0.23% | `net.minecraft.util.Mth.floor(double) line: 66` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 32.52% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 8.17% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 3.40% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 4 | 2.78% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 5 | 2.44% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 6 | 1.97% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 7 | 1.83% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 8 | 1.83% | `java.util.HashMap.containsKey(Object) line: 602` |
| 9 | 1.77% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 10 | 1.42% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 11 | 1.13% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 12 | 1.02% | `java.util.Map.computeIfAbsent(Object, Function) line: 1064` |
| 13 | 0.89% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 14 | 0.86% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 15 | 0.82% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 16 | 0.77% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 17 | 0.75% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 18 | 0.74% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 19 | 0.74% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |
| 20 | 0.73% | `net.minecraft.world.entity.Entity.isSprinting() line: 3501` |
| 21 | 0.72% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 22 | 0.70% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 12` |
| 23 | 0.69% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 24 | 0.66% | `java.util.EnumMap.get(Object) line: 247` |
| 25 | 0.65% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 26 | 0.60% | `it.unimi.dsi.fastutil.objects.ObjectOpenHashSet.iterator() line: 654` |
| 27 | 0.53% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 383` |
| 28 | 0.44% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 212` |
| 29 | 0.41% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 30 | 0.39% | `net.minecraft.world.entity.Entity.getGravity() line: 1842` |

GC pauses: 0, total 0 ms

