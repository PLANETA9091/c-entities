# JVM hotspots (JFR ExecutionSample)

Samples: 54448

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 2.94% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 2 | 2.62% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 3 | 2.44% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 2.04% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 5 | 1.91% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 208` |
| 6 | 1.90% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 7 | 1.62% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 8 | 1.43% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 9 | 1.38% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 10 | 1.34% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.26% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 12 | 1.24% | `java.lang.Enum.ordinal() line: 128` |
| 13 | 1.19% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 196` |
| 14 | 1.09% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 15 | 1.07% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 16 | 1.06% | `net.minecraft.server.level.ServerChunkCache.getChunkNow(int, int) line: 337` |
| 17 | 1.06% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1215` |
| 18 | 1.05% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 19 | 1.04% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 20 | 1.01% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1213` |
| 21 | 0.99% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 22 | 0.99% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 23 | 0.95% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 24 | 0.94% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 25 | 0.94% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 26 | 0.93% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.get(int) line: 102` |
| 27 | 0.88% | `java.util.HashMap.getNode(Object) line: 585` |
| 28 | 0.87% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 29 | 0.85% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 30 | 0.77% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 31 | 0.76% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 32 | 0.74% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 61` |
| 33 | 0.69% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 34 | 0.66% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 135` |
| 35 | 0.62% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 323` |
| 36 | 0.58% | `java.util.EnumMap.get(Object) line: 247` |
| 37 | 0.54% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 38 | 0.54% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 39 | 0.54% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 40 | 0.53% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 41 | 0.52% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 42 | 0.50% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 43 | 0.49% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(ResourceLocation) line: 68` |
| 44 | 0.48% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 45 | 0.47% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 46 | 0.47% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 81` |
| 47 | 0.46% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1010` |
| 48 | 0.45% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 3849` |
| 49 | 0.44% | `java.lang.ThreadLocal.get() line: 172` |
| 50 | 0.44% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 51 | 0.42% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 52 | 0.42% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 47` |
| 53 | 0.40% | `net.minecraft.world.level.Level.isOutsideBuildHeight(int) line: 781` |
| 54 | 0.39% | `ca.spottedleaf.moonrise.common.misc.NearbyPlayers$TrackedChunk.getPlayers(NearbyPlayers$NearbyMapType) line: 181` |
| 55 | 0.38% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 56 | 0.37% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 57 | 0.37% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges() line: 3348` |
| 58 | 0.36% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 324` |
| 59 | 0.36% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 60 | 0.35% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 3.72% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 2 | 2.70% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 3 | 2.61% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 4 | 2.45% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 5 | 2.44% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 6 | 2.27% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 7 | 2.15% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1161` |
| 8 | 2.04% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 9 | 1.96% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 10 | 1.62% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 11 | 1.62% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 12 | 1.47% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 13 | 1.44% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 14 | 1.39% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 15 | 1.32% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4724` |
| 16 | 1.08% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 17 | 1.06% | `net.minecraft.server.level.ServerChunkCache.hasChunk(int, int) line: 419` |
| 18 | 1.00% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 19 | 0.97% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 904` |
| 20 | 0.93% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.get(int) line: 37` |
| 21 | 0.89% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |
| 22 | 0.89% | `java.util.HashMap.containsKey(Object) line: 602` |
| 23 | 0.86% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 24 | 0.86% | `java.util.EnumMap.get(Object) line: 247` |
| 25 | 0.84% | `net.minecraft.server.level.FullChunkStatus.isOrAfter(FullChunkStatus) line: 10` |
| 26 | 0.82% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 27 | 0.82% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 28 | 0.76% | `net.minecraft.server.level.ChunkMap.tick() line: 1025` |
| 29 | 0.74% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 12` |
| 30 | 0.71% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoFluidPushing() line: 1963` |

GC pauses: 0, total 0 ms

