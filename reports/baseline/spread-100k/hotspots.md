# JVM hotspots (JFR ExecutionSample)

Samples: 74825

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 21.18% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.39% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 3 | 2.31% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 2.23% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 1.69% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 6 | 1.68% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 7 | 1.64% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 8 | 1.60% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 1.36% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 10 | 1.20% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.09% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 12 | 1.01% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 13 | 1.01% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 14 | 0.93% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 15 | 0.90% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 16 | 0.84% | `java.util.ArrayDeque.pollFirst() line: 378` |
| 17 | 0.76% | `java.util.EnumMap.get(Object) line: 248` |
| 18 | 0.74% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 19 | 0.74% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 20 | 0.66% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 21 | 0.62% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 22 | 0.59% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 23 | 0.58% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 24 | 0.57% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 25 | 0.55% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 26 | 0.55% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 27 | 0.53% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 28 | 0.52% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 29 | 0.51% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 30 | 0.49% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 31 | 0.47% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 32 | 0.45% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 33 | 0.44% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 34 | 0.43% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 35 | 0.42% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 36 | 0.41% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 37 | 0.41% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 38 | 0.40% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 39 | 0.38% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 40 | 0.37% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 41 | 0.36% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 42 | 0.34% | `net.minecraft.world.entity.EntitySelector.lambda$pushableBy$0(Entity, Team, Team$CollisionRule, Entity) line: 52` |
| 43 | 0.33% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 44 | 0.33% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 45 | 0.32% | `java.lang.Enum.ordinal() line: 128` |
| 46 | 0.31% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 47 | 0.31% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 48 | 0.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 323` |
| 49 | 0.31% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 50 | 0.29% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 51 | 0.28% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 52 | 0.28% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 53 | 0.28% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 54 | 0.28% | `java.util.HashMap.getNode(Object) line: 586` |
| 55 | 0.27% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 56 | 0.27% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 57 | 0.27% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 58 | 0.26% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |
| 59 | 0.26% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 853` |
| 60 | 0.26% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 21.18% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 3.42% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 3.39% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3803` |
| 4 | 3.19% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 5 | 2.76% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 6 | 2.32% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 7 | 1.87% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 8 | 1.62% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 1.39% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 10 | 1.16% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 11 | 1.06% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 12 | 0.94% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 13 | 0.91% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 14 | 0.88% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 15 | 0.84% | `java.util.ArrayDeque.removeFirst() line: 358` |
| 16 | 0.77% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 17 | 0.74% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 18 | 0.69% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 19 | 0.65% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 20 | 0.63% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 21 | 0.61% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 22 | 0.58% | `java.util.ArrayList.toArray() line: 370` |
| 23 | 0.57% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 24 | 0.53% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 25 | 0.52% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |
| 26 | 0.51% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 27 | 0.50% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |
| 28 | 0.50% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 29 | 0.49% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 30 | 0.49% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |

GC pauses: 0, total 0 ms

