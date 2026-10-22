# JVM hotspots (JFR ExecutionSample)

Samples: 90129

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 26.29% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 6.11% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 2.84% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 2.10% | `java.util.HashMap.getNode(Object) line: 585` |
| 5 | 2.04% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 6 | 1.88% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 7 | 1.84% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 8 | 1.25% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 9 | 1.25% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 10 | 1.06% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 11 | 0.92% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 12 | 0.89% | `java.lang.ThreadLocal.get() line: 172` |
| 13 | 0.89% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 14 | 0.85% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 15 | 0.85% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 16 | 0.76% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 47` |
| 17 | 0.76% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 18 | 0.75% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 19 | 0.72% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 20 | 0.71% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1010` |
| 21 | 0.65% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 22 | 0.65% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 23 | 0.62% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 24 | 0.62% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 25 | 0.59% | `java.lang.Enum.ordinal() line: 128` |
| 26 | 0.58% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 27 | 0.56% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(ResourceLocation) line: 68` |
| 28 | 0.56% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 61` |
| 29 | 0.56% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 30 | 0.54% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 31 | 0.54% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 32 | 0.51% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 33 | 0.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 34 | 0.50% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 35 | 0.50% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 36 | 0.49% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 37 | 0.45% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 38 | 0.45% | `net.minecraft.world.item.ItemStack.getMaxStackSize() line: 581` |
| 39 | 0.40% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 40 | 0.40% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 41 | 0.38% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 42 | 0.37% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1017` |
| 43 | 0.37% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 119` |
| 44 | 0.36% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 45 | 0.34% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 46 | 0.34% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 47 | 0.33% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges() line: 3348` |
| 48 | 0.32% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 49 | 0.32% | `java.util.EnumMap.get(Object) line: 247` |
| 50 | 0.32% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 51 | 0.28% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 52 | 0.27% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 53 | 0.27% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 54 | 0.27% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 437` |
| 55 | 0.27% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 56 | 0.27% | `net.minecraft.world.entity.LivingEntity.getArrowCount() line: 2528` |
| 57 | 0.26% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 58 | 0.26% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 59 | 0.26% | `net.minecraft.world.entity.Entity.getAirSupply() line: 3608` |
| 60 | 0.25% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 444` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 26.29% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 8.81% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 3.68% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 4 | 2.11% | `java.util.HashMap.containsKey(Object) line: 602` |
| 5 | 1.88% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 6 | 1.85% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 7 | 1.45% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 8 | 1.41% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 9 | 1.29% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 10 | 1.28% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 11 | 1.27% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 12 | 1.13% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 13 | 0.94% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 14 | 0.90% | `java.util.EnumMap.get(Object) line: 247` |
| 15 | 0.87% | `net.minecraft.server.level.ChunkMap.tick() line: 1025` |
| 16 | 0.77% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 17 | 0.76% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2717` |
| 18 | 0.75% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |
| 19 | 0.75% | `java.util.Map.computeIfAbsent(Object, Function) line: 1064` |
| 20 | 0.72% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 21 | 0.68% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 22 | 0.68% | `net.minecraft.util.profiling.Profiler.get() line: 47` |
| 23 | 0.66% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 24 | 0.65% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 25 | 0.62% | `net.minecraft.world.entity.Entity.isSprinting() line: 3501` |
| 26 | 0.56% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 533` |
| 27 | 0.55% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 12` |
| 28 | 0.55% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 29 | 0.52% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 30 | 0.49% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 549` |

GC pauses: 0, total 0 ms

