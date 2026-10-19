# JVM hotspots (JFR ExecutionSample)

Samples: 15165

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 17.45% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.95% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 3.09% | `java.util.HashMap.getNode(Object) line: 585` |
| 4 | 2.49% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 1.80% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 6 | 1.46% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 7 | 1.36% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 8 | 1.29% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 9 | 1.26% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 10 | 1.24% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 11 | 1.19% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 12 | 0.98% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 13 | 0.95% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 14 | 0.90% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 15 | 0.86% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 16 | 0.81% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 17 | 0.76% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 18 | 0.76% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 19 | 0.75% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 20 | 0.72% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 21 | 0.70% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 22 | 0.69% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 23 | 0.69% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 24 | 0.64% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 25 | 0.63% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 26 | 0.61% | `java.lang.Enum.ordinal() line: 128` |
| 27 | 0.59% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 28 | 0.58% | `java.lang.ThreadLocal.get() line: 172` |
| 29 | 0.55% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 30 | 0.55% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 31 | 0.54% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 32 | 0.53% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 33 | 0.52% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1169` |
| 34 | 0.51% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 35 | 0.51% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 36 | 0.50% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 37 | 0.49% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 38 | 0.48% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.getDouble(Object) line: 431` |
| 39 | 0.45% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 40 | 0.45% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 41 | 0.41% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 42 | 0.40% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 43 | 0.40% | `java.util.ArrayDeque.size() line: 648` |
| 44 | 0.38% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 45 | 0.38% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 46 | 0.36% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 47 | 0.36% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 48 | 0.36% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(ResourceLocation) line: 68` |
| 49 | 0.36% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 50 | 0.35% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 51 | 0.34% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 47` |
| 52 | 0.34% | `net.minecraft.world.phys.AABB.<init>(double, double, double, double, double, double) line: 23` |
| 53 | 0.33% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 54 | 0.32% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 161` |
| 55 | 0.28% | `net.minecraft.world.entity.Entity.getType() line: 605` |
| 56 | 0.28% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |
| 57 | 0.28% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 58 | 0.27% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 81` |
| 59 | 0.27% | `java.util.EnumMap.get(Object) line: 247` |
| 60 | 0.26% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 17.45% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.09% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 3.09% | `java.util.HashMap.containsKey(Object) line: 602` |
| 4 | 2.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 5 | 2.49% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 6 | 2.28% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 7 | 1.64% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 8 | 1.56% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 9 | 1.33% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 10 | 1.26% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 11 | 1.06% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 12 | 0.98% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 13 | 0.96% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 14 | 0.91% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 15 | 0.90% | `java.util.stream.AbstractPipeline.wrapAndCopyInto(Sink, Spliterator) line: 499` |
| 16 | 0.84% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 17 | 0.82% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 18 | 0.81% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 19 | 0.78% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 20 | 0.78% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 21 | 0.78% | `net.minecraft.world.entity.Entity.isSprinting() line: 3501` |
| 22 | 0.76% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 904` |
| 23 | 0.69% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 24 | 0.69% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 25 | 0.69% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 26 | 0.62% | `java.util.EnumMap.get(Object) line: 247` |
| 27 | 0.61% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 28 | 0.59% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1277` |
| 29 | 0.57% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 30 | 0.57% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4724` |

GC pauses: 0, total 0 ms

