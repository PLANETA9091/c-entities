# JVM hotspots (JFR ExecutionSample)

Samples: 18739

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 20.63% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.50% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 3.21% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 1.60% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 5 | 1.35% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 1.32% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 7 | 1.30% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 8 | 1.26% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 9 | 1.21% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 10 | 1.01% | `net.minecraft.world.entity.Entity.getX() line: 4899` |
| 11 | 0.99% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 12 | 0.97% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 0.96% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 14 | 0.96% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 15 | 0.94% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 16 | 0.93% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 17 | 0.83% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 18 | 0.83% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 19 | 0.76% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 20 | 0.71% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 21 | 0.71% | `java.lang.ThreadLocal.get() line: 172` |
| 22 | 0.69% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.add(Object) line: 367` |
| 23 | 0.66% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 24 | 0.66% | `java.lang.Enum.ordinal() line: 128` |
| 25 | 0.64% | `java.lang.ThreadLocal$ThreadLocalMap.getEntry(ThreadLocal) line: 504` |
| 26 | 0.62% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 27 | 0.60% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3459` |
| 28 | 0.60% | `net.minecraft.world.entity.InterpolationHandler.hasActiveInterpolation() line: 69` |
| 29 | 0.59% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 30 | 0.59% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 31 | 0.58% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 61` |
| 32 | 0.53% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 33 | 0.51% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 34 | 0.51% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 35 | 0.51% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 36 | 0.50% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 37 | 0.49% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(ResourceLocation) line: 68` |
| 38 | 0.48% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 39 | 0.45% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 40 | 0.45% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1169` |
| 41 | 0.45% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.getDouble(Object) line: 431` |
| 42 | 0.44% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 43 | 0.42% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 44 | 0.41% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 45 | 0.40% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 46 | 0.39% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 47 | 0.38% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartB() line: 938` |
| 48 | 0.38% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4882` |
| 49 | 0.36% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 50 | 0.35% | `java.util.ArrayDeque.size() line: 648` |
| 51 | 0.35% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 52 | 0.35% | `java.util.EnumMap.get(Object) line: 247` |
| 53 | 0.34% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 54 | 0.34% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 55 | 0.33% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 56 | 0.33% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 57 | 0.31% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 58 | 0.30% | `org.spigotmc.AsyncCatcher.catchOp(String) line: 8` |
| 59 | 0.30% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 81` |
| 60 | 0.30% | `net.minecraft.core.Vec3i.getZ() line: 71` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 20.63% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 5.42% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 3.21% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 4 | 2.04% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 5 | 2.01% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 6 | 1.62% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 7 | 1.41% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 8 | 1.39% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 9 | 1.26% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 10 | 1.21% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 11 | 1.14% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 12 | 1.13% | `java.util.Map.computeIfAbsent(Object, Function) line: 1064` |
| 13 | 1.06% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 14 | 1.01% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 15 | 0.89% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 904` |
| 16 | 0.86% | `net.minecraft.world.entity.Entity.isSprinting() line: 3501` |
| 17 | 0.84% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 18 | 0.79% | `java.lang.ThreadLocal.get(Thread) line: 187` |
| 19 | 0.79% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 20 | 0.78% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 21 | 0.78% | `net.minecraft.util.profiling.Profiler.get() line: 47` |
| 22 | 0.77% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 23 | 0.76% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1310` |
| 24 | 0.69% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 25 | 0.68% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 26 | 0.62% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1286` |
| 27 | 0.61% | `java.util.EnumMap.get(Object) line: 247` |
| 28 | 0.60% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3343` |
| 29 | 0.58% | `net.minecraft.world.entity.Entity.setDeltaMovement(double, double, double) line: 4891` |
| 30 | 0.58% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 12` |

GC pauses: 0, total 0 ms

