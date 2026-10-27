# JVM hotspots (JFR ExecutionSample)

Samples: 67588

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 14.66% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.82% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 2.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 4 | 2.13% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 2.04% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 1.85% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 7 | 1.78% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 1.57% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 9 | 1.52% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 10 | 1.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 11 | 1.28% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 12 | 1.24% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 13 | 1.16% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 14 | 1.00% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 15 | 0.99% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 16 | 0.95% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 17 | 0.93% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 18 | 0.77% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 19 | 0.77% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 20 | 0.76% | `java.util.EnumMap.get(Object) line: 248` |
| 21 | 0.71% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 22 | 0.70% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 23 | 0.69% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 24 | 0.68% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 25 | 0.59% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 26 | 0.52% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 27 | 0.52% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 28 | 0.52% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 29 | 0.50% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 30 | 0.47% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 31 | 0.45% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 32 | 0.44% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 33 | 0.44% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 34 | 0.43% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 35 | 0.42% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 36 | 0.40% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 37 | 0.40% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 38 | 0.40% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 39 | 0.40% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 40 | 0.40% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 41 | 0.38% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 42 | 0.38% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 43 | 0.38% | `java.util.HashMap.getNode(Object) line: 586` |
| 44 | 0.38% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 45 | 0.37% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 46 | 0.37% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 47 | 0.37% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 48 | 0.35% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 49 | 0.34% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 50 | 0.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 51 | 0.33% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 323` |
| 52 | 0.33% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 53 | 0.33% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 54 | 0.32% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 55 | 0.31% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 56 | 0.30% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 57 | 0.30% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 58 | 0.29% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 92` |
| 59 | 0.29% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 60 | 0.29% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 14.66% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 4.59% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 3.87% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 3.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 5 | 2.36% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 6 | 2.13% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 7 | 1.85% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 8 | 1.77% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 1.45% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 10 | 1.43% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 11 | 1.28% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 12 | 1.17% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 13 | 0.93% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 14 | 0.83% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 15 | 0.79% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 16 | 0.72% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 17 | 0.72% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 18 | 0.71% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 19 | 0.67% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 20 | 0.66% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 21 | 0.62% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 22 | 0.62% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 23 | 0.62% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 24 | 0.60% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 25 | 0.56% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 26 | 0.53% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 27 | 0.52% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 28 | 0.51% | `java.util.ArrayList.toArray() line: 370` |
| 29 | 0.50% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 30 | 0.46% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |

GC pauses: 0, total 0 ms

