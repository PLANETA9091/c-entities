# JVM hotspots (JFR ExecutionSample)

Samples: 71315

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 19.92% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.63% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 2.54% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 2.45% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 5 | 1.96% | `dev.planeta9091.centities.CeHooks.cellOfBox(AABB) line: 754` |
| 6 | 1.86% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 7 | 1.52% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 8 | 1.46% | `java.util.HashMap.getNode(Object) line: 585` |
| 9 | 1.41% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 10 | 1.39% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 11 | 1.05% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 12 | 0.99% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 13 | 0.95% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 14 | 0.92% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1215` |
| 15 | 0.89% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 16 | 0.89% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 17 | 0.85% | `net.minecraft.server.level.ServerChunkCache.getChunkNow(int, int) line: 337` |
| 18 | 0.85% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 19 | 0.84% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1156` |
| 20 | 0.81% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1213` |
| 21 | 0.79% | `java.lang.Enum.ordinal() line: 128` |
| 22 | 0.78% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 23 | 0.74% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 24 | 0.73% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 25 | 0.72% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 208` |
| 26 | 0.67% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 61` |
| 27 | 0.66% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 28 | 0.64% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 29 | 0.64% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1010` |
| 30 | 0.62% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 31 | 0.59% | `java.lang.ThreadLocal.get() line: 172` |
| 32 | 0.57% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 33 | 0.55% | `net.minecraft.world.item.ItemStack.getMaxStackSize() line: 581` |
| 34 | 0.52% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 35 | 0.47% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 36 | 0.47% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 37 | 0.45% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 196` |
| 38 | 0.45% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 444` |
| 39 | 0.44% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 40 | 0.42% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 41 | 0.42% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 42 | 0.41% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 119` |
| 43 | 0.41% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 47` |
| 44 | 0.39% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 45 | 0.39% | `it.unimi.dsi.fastutil.objects.ObjectOpenHashSet$SetIterator.<init>(ObjectOpenHashSet, ObjectOpenHashSet$1) line: 534` |
| 46 | 0.39% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 47 | 0.38% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 48 | 0.37% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 81` |
| 49 | 0.37% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 50 | 0.36% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 51 | 0.35% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 52 | 0.33% | `ca.spottedleaf.moonrise.common.misc.NearbyPlayers$TrackedChunk.getPlayers(NearbyPlayers$NearbyMapType) line: 181` |
| 53 | 0.33% | `io.netty.channel.ChannelOutboundBuffer.addFlush() line: 159` |
| 54 | 0.33% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 55 | 0.33% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges() line: 3348` |
| 56 | 0.32% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 57 | 0.32% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 58 | 0.29% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 437` |
| 59 | 0.29% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 60 | 0.28% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 19.92% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 3.96% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 2.54% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 4 | 2.45% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 5 | 2.12% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 6 | 1.96% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1165` |
| 7 | 1.86% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 8 | 1.77% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1161` |
| 9 | 1.76% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 10 | 1.63% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 11 | 1.43% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 12 | 1.43% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 13 | 1.39% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 14 | 1.34% | `java.util.HashMap.containsKey(Object) line: 602` |
| 15 | 1.31% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 954` |
| 16 | 1.20% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 17 | 1.17% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 18 | 1.00% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 19 | 0.91% | `net.minecraft.server.level.ChunkMap.tick() line: 1025` |
| 20 | 0.85% | `net.minecraft.server.level.ServerChunkCache.hasChunk(int, int) line: 419` |
| 21 | 0.80% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 22 | 0.74% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 23 | 0.68% | `java.util.EnumMap.get(Object) line: 247` |
| 24 | 0.66% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 12` |
| 25 | 0.65% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4724` |
| 26 | 0.60% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 27 | 0.58% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 904` |
| 28 | 0.55% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 282` |
| 29 | 0.53% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 30 | 0.51% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 212` |

GC pauses: 0, total 0 ms

