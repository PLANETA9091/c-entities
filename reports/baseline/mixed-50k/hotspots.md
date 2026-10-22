# JVM hotspots (JFR ExecutionSample)

Samples: 14081

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 16.25% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.95% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 3.71% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 3.67% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 5 | 2.33% | `java.util.HashMap.getNode(Object) line: 585` |
| 6 | 2.00% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 7 | 1.77% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 8 | 1.43% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 9 | 1.40% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.39% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 11 | 1.38% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 12 | 1.29% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 13 | 1.24% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 14 | 1.24% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 15 | 1.17% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 16 | 1.09% | `java.lang.ThreadLocal.get() line: 172` |
| 17 | 1.02% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 18 | 0.96% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 19 | 0.94% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 20 | 0.86% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 21 | 0.82% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 22 | 0.81% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 23 | 0.80% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 24 | 0.78% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 25 | 0.75% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 26 | 0.72% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 27 | 0.70% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 28 | 0.66% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 29 | 0.62% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 30 | 0.59% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 47` |
| 31 | 0.58% | `java.lang.Enum.ordinal() line: 128` |
| 32 | 0.55% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 33 | 0.55% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 34 | 0.52% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 35 | 0.49% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 36 | 0.48% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 37 | 0.48% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 38 | 0.48% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 39 | 0.47% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 40 | 0.43% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1414` |
| 41 | 0.43% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 42 | 0.40% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1661` |
| 43 | 0.40% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 44 | 0.39% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 45 | 0.39% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 46 | 0.38% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 47 | 0.38% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 48 | 0.37% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 49 | 0.36% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1169` |
| 50 | 0.36% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.clear() line: 442` |
| 51 | 0.36% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 86` |
| 52 | 0.34% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 53 | 0.33% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 54 | 0.33% | `java.util.EnumMap.get(Object) line: 247` |
| 55 | 0.33% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 81` |
| 56 | 0.33% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 69` |
| 57 | 0.32% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 58 | 0.31% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.getDouble(Object) line: 431` |
| 59 | 0.31% | `net.minecraft.world.entity.Entity.getType() line: 605` |
| 60 | 0.31% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1302` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 16.25% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 4.84% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 3.71% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 4 | 3.67% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 5 | 2.84% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 6 | 2.41% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 7 | 2.35% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 8 | 2.33% | `java.util.HashMap.containsKey(Object) line: 602` |
| 9 | 1.68% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 10 | 1.67% | `java.util.Map.computeIfAbsent(Object, Function) line: 1064` |
| 11 | 1.62% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 12 | 1.41% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 13 | 1.38% | `net.minecraft.world.entity.Entity.isSprinting() line: 3501` |
| 14 | 1.35% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 15 | 1.19% | `java.util.stream.AbstractPipeline.wrapAndCopyInto(Sink, Spliterator) line: 499` |
| 16 | 1.16% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 17 | 1.07% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 18 | 0.97% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 19 | 0.95% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |
| 20 | 0.90% | `net.minecraft.util.profiling.Profiler.get() line: 47` |
| 21 | 0.90% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 22 | 0.89% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 23 | 0.87% | `java.util.EnumMap.get(Object) line: 247` |
| 24 | 0.81% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 25 | 0.80% | `net.minecraft.world.entity.Entity.getGravity() line: 1842` |
| 26 | 0.78% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 27 | 0.77% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 28 | 0.69% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 29 | 0.59% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2717` |
| 30 | 0.58% | `net.minecraft.world.entity.Mob.serverAiStep() line: 774` |

GC pauses: 0, total 0 ms

