# JVM hotspots (JFR ExecutionSample)

Samples: 68820

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 11.67% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.46% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 3 | 3.39% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 4 | 3.33% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 5 | 3.04% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 6 | 2.57% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 7 | 2.07% | `dev.planeta9091.centities.CeHooks.arenaRef(long) line: 2023` |
| 8 | 2.02% | `java.util.HashMap.getNode(Object) line: 585` |
| 9 | 2.02% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 10 | 2.01% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 11 | 1.92% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1000` |
| 12 | 1.78% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 13 | 1.57% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 14 | 1.12% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 15 | 1.11% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 16 | 1.09% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 17 | 1.00% | `net.minecraft.world.entity.Entity.getX() line: 4899` |
| 18 | 0.99% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 19 | 0.95% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 20 | 0.95% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 21 | 0.91% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 22 | 0.91% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 23 | 0.88% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 24 | 0.87% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 25 | 0.83% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 26 | 0.82% | `java.lang.invoke.MethodHandles$Lookup.<init>(Class, Class, int) line: 1644` |
| 27 | 0.79% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 28 | 0.76% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 29 | 0.71% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 30 | 0.67% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 31 | 0.67% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 32 | 0.64% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 33 | 0.63% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 34 | 0.62% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 35 | 0.61% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 36 | 0.60% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 135` |
| 37 | 0.60% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 38 | 0.60% | `it.unimi.dsi.fastutil.objects.ObjectOpenHashSet$SetIterator.<init>(ObjectOpenHashSet, ObjectOpenHashSet$1) line: 534` |
| 39 | 0.55% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1302` |
| 40 | 0.52% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 41 | 0.49% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 42 | 0.46% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 43 | 0.46% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 44 | 0.46% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3690` |
| 45 | 0.45% | `java.lang.Enum.ordinal() line: 128` |
| 46 | 0.42% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges() line: 3348` |
| 47 | 0.41% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1414` |
| 48 | 0.40% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4882` |
| 49 | 0.38% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 50 | 0.37% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 51 | 0.37% | `java.util.EnumMap.get(Object) line: 247` |
| 52 | 0.36% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 53 | 0.36% | `org.spigotmc.AsyncCatcher.catchOp(String) line: 8` |
| 54 | 0.36% | `net.minecraft.world.entity.Entity.getType() line: 605` |
| 55 | 0.35% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 81` |
| 56 | 0.35% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 57 | 0.34% | `net.minecraft.server.level.ServerLevel.lambda$tick$4(TickRateManager, ProfilerFiller, Entity) line: 795` |
| 58 | 0.33% | `io.netty.channel.ChannelOutboundBuffer.addFlush() line: 159` |
| 59 | 0.33% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1169` |
| 60 | 0.31% | `net.minecraft.util.Mth.wrapDegrees(float) line: 221` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 11.67% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 5.34% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 3.46% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 4 | 3.41% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 5 | 3.26% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 6 | 3.04% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 7 | 2.87% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 8 | 2.08% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 1924` |
| 9 | 2.03% | `dev.planeta9091.centities.CeHooks.getEntities(Level, Entity, AABB, Predicate) line: 460` |
| 10 | 2.01% | `java.util.HashMap.containsKey(Object) line: 602` |
| 11 | 1.78% | `java.util.stream.AbstractPipeline.wrapAndCopyInto(Sink, Spliterator) line: 499` |
| 12 | 1.58% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 13 | 1.41% | `java.util.Map.computeIfAbsent(Object, Function) line: 1064` |
| 14 | 1.38% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 15 | 1.29% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1432` |
| 16 | 1.12% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 17 | 1.05% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 18 | 1.04% | `net.minecraft.world.entity.Entity.isSprinting() line: 3501` |
| 19 | 1.02% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 20 | 0.99% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 21 | 0.98% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1286` |
| 22 | 0.91% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 23 | 0.91% | `net.minecraft.world.entity.Entity.setDeltaMovement(double, double, double) line: 4891` |
| 24 | 0.89% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 25 | 0.86% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 26 | 0.85% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 27 | 0.82% | `java.lang.invoke.MethodHandles$Lookup.<init>(Class) line: 1637` |
| 28 | 0.80% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |
| 29 | 0.74% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 235` |
| 30 | 0.68% | `java.util.EnumMap.get(Object) line: 247` |

GC pauses: 0, total 0 ms

