# JVM hotspots (JFR ExecutionSample)

Samples: 51260

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 10.11% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.50% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 2.94% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 4 | 2.69% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 5 | 2.66% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 6 | 2.49% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 2.27% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 8 | 2.19% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 9 | 1.82% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 10 | 1.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 11 | 1.45% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 12 | 1.27% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 13 | 1.21% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 14 | 1.21% | `java.util.HashMap.getNode(Object) line: 585` |
| 15 | 1.09% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 16 | 1.03% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 17 | 1.02% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 18 | 1.01% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 19 | 0.98% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 20 | 0.94% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 21 | 0.88% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 204` |
| 22 | 0.87% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 47` |
| 23 | 0.86% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 24 | 0.84% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 25 | 0.83% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 26 | 0.81% | `it.unimi.dsi.fastutil.objects.ObjectOpenHashSet$SetIterator.<init>(ObjectOpenHashSet, ObjectOpenHashSet$1) line: 534` |
| 27 | 0.79% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 28 | 0.79% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 29 | 0.78% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1215` |
| 30 | 0.75% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 31 | 0.71% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 32 | 0.70% | `java.lang.Enum.ordinal() line: 128` |
| 33 | 0.69% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 34 | 0.67% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 35 | 0.67% | `java.lang.invoke.MethodHandles$Lookup.<init>(Class, Class, int) line: 1644` |
| 36 | 0.65% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 37 | 0.63% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 135` |
| 38 | 0.63% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 39 | 0.62% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 40 | 0.61% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 208` |
| 41 | 0.60% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 42 | 0.58% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 43 | 0.55% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 81` |
| 44 | 0.52% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1213` |
| 45 | 0.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 46 | 0.50% | `net.minecraft.world.entity.Mob.checkDespawn() line: 715` |
| 47 | 0.50% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 48 | 0.48% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 49 | 0.48% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 50 | 0.47% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1414` |
| 51 | 0.44% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges() line: 3348` |
| 52 | 0.43% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 53 | 0.43% | `io.netty.channel.ChannelOutboundBuffer.addFlush() line: 159` |
| 54 | 0.42% | `net.minecraft.world.entity.Entity.getAirSupply() line: 3608` |
| 55 | 0.42% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 56 | 0.42% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 57 | 0.40% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoFluidPushing() line: 1960` |
| 58 | 0.40% | `java.util.ArrayDeque.size() line: 648` |
| 59 | 0.38% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1410` |
| 60 | 0.37% | `dev.planeta9091.centities.CeHooks.arenaRef(long) line: 2014` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 10.11% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 4.27% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 3.94% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 3.50% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 5 | 3.21% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 6 | 2.70% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 7 | 2.67% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 8 | 2.27% | `java.util.stream.AbstractPipeline.wrapAndCopyInto(Sink, Spliterator) line: 499` |
| 9 | 1.82% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 10 | 1.53% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 11 | 1.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 12 | 1.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 13 | 1.35% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1161` |
| 14 | 1.34% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 15 | 1.22% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 16 | 1.21% | `java.util.HashMap.containsKey(Object) line: 602` |
| 17 | 1.17% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 18 | 1.14% | `java.util.EnumMap.get(Object) line: 247` |
| 19 | 1.03% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 20 | 1.03% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |
| 21 | 0.99% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 22 | 0.94% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4724` |
| 23 | 0.93% | `java.util.Map.computeIfAbsent(Object, Function) line: 1064` |
| 24 | 0.93% | `net.minecraft.world.entity.Mob.serverAiStep() line: 759` |
| 25 | 0.91% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 26 | 0.90% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity)` |
| 27 | 0.88% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 28 | 0.87% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2717` |
| 29 | 0.86% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 235` |
| 30 | 0.85% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |

GC pauses: 0, total 0 ms

