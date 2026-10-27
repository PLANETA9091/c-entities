# JVM hotspots (JFR ExecutionSample)

Samples: 63910

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 12.71% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.02% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 2.48% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 4 | 2.20% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 2.18% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 6 | 1.65% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 7 | 1.61% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 1.33% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 9 | 1.31% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 10 | 1.30% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 11 | 1.28% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 12 | 1.10% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 13 | 1.10% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 14 | 1.01% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 15 | 1.00% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 16 | 0.98% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 17 | 0.96% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 18 | 0.92% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 19 | 0.92% | `java.util.EnumMap.get(Object) line: 248` |
| 20 | 0.79% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 21 | 0.79% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 22 | 0.77% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 23 | 0.76% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 24 | 0.76% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 25 | 0.73% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 26 | 0.72% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 27 | 0.68% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 28 | 0.68% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 29 | 0.65% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 30 | 0.64% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 31 | 0.62% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 32 | 0.58% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 33 | 0.53% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 34 | 0.52% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 35 | 0.51% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 36 | 0.50% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 37 | 0.45% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 38 | 0.45% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 39 | 0.38% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 40 | 0.38% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 41 | 0.37% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 42 | 0.37% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 43 | 0.36% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 44 | 0.36% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 45 | 0.35% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 46 | 0.34% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 47 | 0.33% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 48 | 0.33% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 49 | 0.33% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 50 | 0.33% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 323` |
| 51 | 0.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 52 | 0.31% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 53 | 0.31% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 54 | 0.31% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 55 | 0.30% | `java.util.HashMap.getNode(Object) line: 586` |
| 56 | 0.29% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 57 | 0.29% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 58 | 0.29% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 59 | 0.28% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 60 | 0.28% | `org.spigotmc.WatchdogThread.run() line: 72` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 12.71% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 4.63% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 3.70% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 3.42% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 5 | 2.48% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 6 | 2.31% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 7 | 2.18% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 8 | 1.81% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 1.73% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 10 | 1.32% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 11 | 1.31% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 12 | 1.05% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 13 | 0.99% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 14 | 0.87% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 15 | 0.79% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 16 | 0.77% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 17 | 0.74% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 18 | 0.73% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 19 | 0.72% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 20 | 0.71% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 21 | 0.70% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 22 | 0.67% | `java.util.ArrayList.toArray() line: 370` |
| 23 | 0.65% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 125` |
| 24 | 0.64% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |
| 25 | 0.64% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 26 | 0.62% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 27 | 0.60% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 28 | 0.59% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 29 | 0.53% | `net.minecraft.world.item.enchantment.Enchantment.tick(ServerLevel, int, EnchantedItemInUse, Entity) line: 400` |
| 30 | 0.52% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |

GC pauses: 0, total 0 ms

