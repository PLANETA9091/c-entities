# JVM hotspots (JFR ExecutionSample)

Samples: 46966

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 10.57% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.80% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 2.38% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 4 | 1.91% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 5 | 1.88% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 6 | 1.87% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 7 | 1.77% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 1.54% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 9 | 1.48% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 10 | 1.42% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 11 | 1.40% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 12 | 1.23% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1215` |
| 13 | 1.13% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 14 | 1.04% | `java.util.HashMap.getNode(Object) line: 585` |
| 15 | 1.02% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 16 | 0.99% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 17 | 0.98% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 18 | 0.90% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 19 | 0.87% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 20 | 0.83% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 21 | 0.80% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 22 | 0.80% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 23 | 0.78% | `dev.planeta9091.centities.CeHooks.arenaRef(long) line: 2023` |
| 24 | 0.78% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 25 | 0.75% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 26 | 0.75% | `java.lang.Enum.ordinal() line: 128` |
| 27 | 0.73% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 28 | 0.72% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 29 | 0.71% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 30 | 0.68% | `java.lang.invoke.MethodHandles$Lookup.<init>(Class, Class, int) line: 1644` |
| 31 | 0.65% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 32 | 0.63% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 204` |
| 33 | 0.63% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 135` |
| 34 | 0.61% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 35 | 0.60% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 208` |
| 36 | 0.57% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 37 | 0.57% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(ResourceLocation) line: 68` |
| 38 | 0.53% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 39 | 0.53% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 40 | 0.51% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 47` |
| 41 | 0.51% | `it.unimi.dsi.fastutil.objects.ObjectOpenHashSet$SetIterator.<init>(ObjectOpenHashSet, ObjectOpenHashSet$1) line: 534` |
| 42 | 0.50% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 43 | 0.49% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 44 | 0.49% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 81` |
| 45 | 0.48% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 46 | 0.48% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 47 | 0.46% | `java.lang.ThreadLocal.get() line: 172` |
| 48 | 0.46% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 49 | 0.45% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 50 | 0.44% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 51 | 0.44% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1206` |
| 52 | 0.42% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 53 | 0.38% | `java.util.EnumMap.isValidKey(Object) line: 319` |
| 54 | 0.38% | `java.util.EnumMap.get(Object) line: 247` |
| 55 | 0.38% | `io.netty.channel.ChannelOutboundBuffer.addFlush() line: 159` |
| 56 | 0.37% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 196` |
| 57 | 0.35% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges() line: 3348` |
| 58 | 0.33% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 59 | 0.32% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 173` |
| 60 | 0.32% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 10.57% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 3.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 3.15% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 2.97% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 5 | 2.81% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 6 | 1.93% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 7 | 1.88% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 8 | 1.84% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 9 | 1.83% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 10 | 1.70% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 11 | 1.41% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 12 | 1.40% | `java.util.stream.AbstractPipeline.wrapAndCopyInto(Sink, Spliterator) line: 499` |
| 13 | 1.33% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 14 | 1.28% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1161` |
| 15 | 1.04% | `java.util.HashMap.containsKey(Object) line: 602` |
| 16 | 1.01% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 17 | 0.99% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 937` |
| 18 | 0.99% | `java.util.EnumMap.get(Object) line: 247` |
| 19 | 0.89% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 20 | 0.85% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 21 | 0.82% | `net.minecraft.world.entity.Entity.isSprinting() line: 3501` |
| 22 | 0.81% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 23 | 0.80% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 78` |
| 24 | 0.78% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |
| 25 | 0.77% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 1924` |
| 26 | 0.75% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 27 | 0.73% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 212` |
| 28 | 0.70% | `java.util.Map.computeIfAbsent(Object, Function) line: 1064` |
| 29 | 0.69% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 30 | 0.68% | `java.lang.invoke.MethodHandles$Lookup.<init>(Class) line: 1637` |

GC pauses: 0, total 0 ms

