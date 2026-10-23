# JVM hotspots (JFR ExecutionSample)

Samples: 45693

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 11.37% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.64% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 2.71% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 4 | 2.39% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 5 | 2.13% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 6 | 2.03% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 7 | 1.92% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 8 | 1.81% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 9 | 1.78% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 10 | 1.36% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 11 | 1.33% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 12 | 1.29% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 13 | 1.25% | `java.util.HashMap.getNode(Object) line: 585` |
| 14 | 1.18% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 15 | 1.11% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1215` |
| 16 | 1.10% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 17 | 1.02% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 18 | 0.97% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 61` |
| 19 | 0.95% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 20 | 0.95% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 21 | 0.94% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 22 | 0.91% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 23 | 0.89% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 24 | 0.88% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1213` |
| 25 | 0.81% | `io.netty.channel.ChannelOutboundBuffer.addFlush() line: 159` |
| 26 | 0.76% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 27 | 0.75% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(ResourceLocation) line: 68` |
| 28 | 0.74% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 29 | 0.71% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 30 | 0.69% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 81` |
| 31 | 0.69% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 32 | 0.69% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 33 | 0.67% | `it.unimi.dsi.fastutil.objects.ObjectOpenHashSet$SetIterator.<init>(ObjectOpenHashSet, ObjectOpenHashSet$1) line: 534` |
| 34 | 0.66% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 35 | 0.65% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 36 | 0.63% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 37 | 0.62% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 38 | 0.62% | `java.lang.invoke.MethodHandles$Lookup.<init>(Class, Class, int) line: 1644` |
| 39 | 0.61% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 208` |
| 40 | 0.60% | `java.lang.Enum.ordinal() line: 128` |
| 41 | 0.60% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 42 | 0.58% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 135` |
| 43 | 0.56% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 44 | 0.53% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 45 | 0.53% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 46 | 0.49% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 47` |
| 47 | 0.47% | `net.minecraft.server.level.ServerChunkCache.getChunkNow(int, int) line: 337` |
| 48 | 0.43% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges() line: 3348` |
| 49 | 0.41% | `net.minecraft.world.entity.Entity.getAirSupply() line: 3608` |
| 50 | 0.40% | `dev.planeta9091.centities.CeHooks.arenaRef(long) line: 2023` |
| 51 | 0.38% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 196` |
| 52 | 0.38% | `java.lang.ThreadLocal.get() line: 172` |
| 53 | 0.37% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 54 | 0.37% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 55 | 0.37% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 56 | 0.36% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 57 | 0.34% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1414` |
| 58 | 0.34% | `net.minecraft.world.entity.Entity.getType() line: 605` |
| 59 | 0.34% | `net.minecraft.world.entity.Mob.checkDespawn() line: 715` |
| 60 | 0.32% | `net.minecraft.util.Mth.floor(double) line: 67` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 11.37% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 4.82% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 4.00% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 3.64% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 5 | 3.14% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 6 | 2.40% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 7 | 2.13% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 8 | 2.06% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1161` |
| 9 | 1.45% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 10 | 1.42% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 11 | 1.33% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 12 | 1.29% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 13 | 1.26% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 14 | 1.25% | `java.util.HashMap.containsKey(Object) line: 602` |
| 15 | 1.21% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 16 | 1.17% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1432` |
| 17 | 1.16% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |
| 18 | 0.98% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 19 | 0.97% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 12` |
| 20 | 0.95% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 21 | 0.94% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 22 | 0.87% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 23 | 0.85% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 24 | 0.84% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 25 | 0.83% | `net.minecraft.world.entity.Entity.isSprinting() line: 3501` |
| 26 | 0.82% | `io.netty.channel.AbstractChannel$AbstractUnsafe.flush() line: 892` |
| 27 | 0.79% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 28 | 0.75% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 533` |
| 29 | 0.69% | `java.util.EnumMap.get(Object) line: 247` |
| 30 | 0.69% | `java.util.Map.computeIfAbsent(Object, Function) line: 1064` |

GC pauses: 0, total 0 ms

