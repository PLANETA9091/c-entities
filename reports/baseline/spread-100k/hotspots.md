# JVM hotspots (JFR ExecutionSample)

Samples: 104622

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 30.39% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.13% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 3 | 2.05% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 1.78% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 5 | 1.69% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 6 | 1.69% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 1.38% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 8 | 1.25% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 9 | 1.19% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 10 | 1.12% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 11 | 1.08% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 12 | 0.93% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 13 | 0.92% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 14 | 0.86% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 15 | 0.85% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 16 | 0.84% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 17 | 0.82% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 18 | 0.81% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 19 | 0.75% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 20 | 0.66% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 21 | 0.59% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 22 | 0.55% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 23 | 0.54% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 24 | 0.52% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 25 | 0.51% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 26 | 0.50% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 27 | 0.47% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 28 | 0.46% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 29 | 0.46% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 30 | 0.44% | `java.util.EnumMap.get(Object) line: 248` |
| 31 | 0.43% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 32 | 0.42% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 33 | 0.42% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 34 | 0.40% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 35 | 0.39% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 36 | 0.38% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 37 | 0.34% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 38 | 0.33% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 39 | 0.33% | `java.util.HashMap.getNode(Object) line: 586` |
| 40 | 0.32% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 41 | 0.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 42 | 0.32% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 43 | 0.31% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 92` |
| 44 | 0.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 45 | 0.31% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 46 | 0.29% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 323` |
| 47 | 0.29% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 48 | 0.28% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 49 | 0.27% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 50 | 0.27% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 51 | 0.26% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 52 | 0.26% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 53 | 0.26% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 54 | 0.24% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 55 | 0.24% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 56 | 0.24% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 57 | 0.23% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |
| 58 | 0.23% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 59 | 0.23% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 60 | 0.22% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 30.39% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 4.89% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 2.60% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 2.43% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 5 | 2.05% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 6 | 1.82% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 7 | 1.57% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 8 | 1.38% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 9 | 1.17% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 10 | 1.12% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 11 | 1.08% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 12 | 1.04% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 13 | 0.89% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 14 | 0.89% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 15 | 0.86% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 16 | 0.82% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 17 | 0.82% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 18 | 0.79% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 19 | 0.75% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 20 | 0.74% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 21 | 0.66% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |
| 22 | 0.56% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 23 | 0.54% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 24 | 0.53% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 25 | 0.51% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 26 | 0.51% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 27 | 0.51% | `java.util.ArrayList.toArray() line: 370` |
| 28 | 0.50% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 29 | 0.50% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 30 | 0.47% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |

GC pauses: 0, total 0 ms

