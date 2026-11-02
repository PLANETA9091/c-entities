# JVM hotspots (JFR ExecutionSample)

Samples: 92996

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 33.38% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 5.20% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 3 | 2.24% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 1.81% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 5 | 1.47% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 6 | 1.45% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 1.39% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 1.18% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 9 | 1.15% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 10 | 1.03% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 11 | 0.99% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 12 | 0.91% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 13 | 0.89% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 14 | 0.87% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 15 | 0.83% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 16 | 0.81% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 17 | 0.77% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 18 | 0.75% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 19 | 0.66% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 20 | 0.64% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 21 | 0.62% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 22 | 0.59% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 23 | 0.59% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 24 | 0.50% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 25 | 0.45% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 26 | 0.45% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 27 | 0.43% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 28 | 0.41% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 29 | 0.41% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 30 | 0.41% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 31 | 0.40% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 32 | 0.40% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 33 | 0.39% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 34 | 0.37% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 35 | 0.34% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 36 | 0.33% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 37 | 0.33% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 38 | 0.32% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 39 | 0.30% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 40 | 0.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 323` |
| 41 | 0.29% | `java.util.EnumMap.get(Object) line: 248` |
| 42 | 0.27% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 43 | 0.27% | `net.minecraft.world.entity.LivingEntity$SwingState.tick() line: 5015` |
| 44 | 0.27% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 45 | 0.26% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 46 | 0.26% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 47 | 0.26% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 48 | 0.25% | `java.util.HashMap.getNode(Object) line: 586` |
| 49 | 0.25% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 320` |
| 50 | 0.25% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 51 | 0.25% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 52 | 0.25% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 53 | 0.25% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 54 | 0.24% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 92` |
| 55 | 0.23% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 56 | 0.22% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 57 | 0.22% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 58 | 0.22% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 59 | 0.21% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 60 | 0.21% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 33.38% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 5.75% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 3.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 2.25% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 2.24% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 6 | 1.82% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 7 | 1.40% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 8 | 1.19% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 9 | 1.09% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 10 | 1.03% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 11 | 1.01% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 12 | 1.00% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 13 | 0.94% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 14 | 0.87% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 15 | 0.83% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 16 | 0.82% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 17 | 0.74% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 18 | 0.67% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 19 | 0.59% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 20 | 0.59% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |
| 21 | 0.50% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 22 | 0.48% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 23 | 0.46% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 24 | 0.41% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |
| 25 | 0.41% | `java.util.Collections$UnmodifiableCollection.iterator() line: 1076` |
| 26 | 0.40% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 27 | 0.39% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 28 | 0.38% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 29 | 0.38% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 30 | 0.35% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |

GC pauses: 0, total 0 ms

