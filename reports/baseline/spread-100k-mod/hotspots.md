# JVM hotspots (JFR ExecutionSample)

Samples: 42733

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.20% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 2 | 2.96% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3907` |
| 3 | 2.95% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 4 | 2.63% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 2.00% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 6 | 1.91% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 1.77% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 8 | 1.38% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 9 | 1.21% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 10 | 1.16% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 11 | 1.14% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 12 | 1.01% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 13 | 0.95% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1420` |
| 14 | 0.90% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 15 | 0.88% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 16 | 0.88% | `java.util.EnumMap.get(Object) line: 248` |
| 17 | 0.86% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 18 | 0.84% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 19 | 0.81% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 20 | 0.78% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 21 | 0.78% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 22 | 0.74% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 23 | 0.72% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3868` |
| 24 | 0.69% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 25 | 0.69% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 26 | 0.66% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 848` |
| 27 | 0.66% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 28 | 0.66% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 29 | 0.62% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 30 | 0.61% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 31 | 0.60% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 32 | 0.60% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 33 | 0.57% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 34 | 0.55% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 35 | 0.55% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 36 | 0.54% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 37 | 0.52% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 38 | 0.51% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 39 | 0.49% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 40 | 0.48% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 311` |
| 41 | 0.47% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 42 | 0.46% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 43 | 0.45% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 44 | 0.45% | `ca.spottedleaf.moonrise.patches.starlight.light.SWMRNibbleArray.getVisible(int) line: 412` |
| 45 | 0.44% | `ca.spottedleaf.moonrise.patches.starlight.light.SWMRNibbleArray.isNullNibbleVisible() line: 280` |
| 46 | 0.43% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 47 | 0.43% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 323` |
| 48 | 0.42% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 49 | 0.41% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 50 | 0.40% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder$ChunkCompletion.chunk() line: 464` |
| 51 | 0.40% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder.getChunkStatus() line: 1112` |
| 52 | 0.40% | `java.util.ArrayDeque.size() line: 647` |
| 53 | 0.39% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 54 | 0.39% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 55 | 0.38% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightInterface.getSkyLightValue(BlockPos, ChunkAccess) line: 252` |
| 56 | 0.37% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 57 | 0.36% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap$MapIterator.nextEntry() line: 655` |
| 58 | 0.35% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 59 | 0.34% | `net.minecraft.world.entity.SteppedInterpolationTracker.updateTracking(Vec3) line: 40` |
| 60 | 0.34% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2403` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 3.52% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 2 | 3.22% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 3 | 2.96% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 4 | 2.96% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 5 | 2.80% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 2.63% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 7 | 1.76% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 8 | 1.35% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 1.20% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 10 | 1.14% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 11 | 1.09% | `net.minecraft.world.entity.Entity.setDeltaMovement(double, double, double) line: 5086` |
| 12 | 1.06% | `dev.planeta9091.centities.CeHooks.arenaRef(long) line: 2487` |
| 13 | 0.99% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 14 | 0.91% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 15 | 0.89% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 16 | 0.83% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 17 | 0.80% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 18 | 0.69% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 19 | 0.67% | `net.minecraft.world.entity.Entity.isInWall() line: 3144` |
| 20 | 0.66% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 969` |
| 21 | 0.66% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 22 | 0.66% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 23 | 0.66% | `net.minecraft.world.entity.Entity.checkInsideBlocks(Vec3, Vec3, InsideBlockEffectApplier$StepBasedCollector, LongSet, int) line: 1882` |
| 24 | 0.66% | `net.minecraft.util.thread.BlockableEventLoop.managedBlock(BooleanSupplier) line: 191` |
| 25 | 0.64% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 26 | 0.64% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 27 | 0.62% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |
| 28 | 0.61% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 29 | 0.61% | `net.minecraft.world.level.chunk.PalettedContainer.readPalette(PalettedContainer$Data, int) line: 79` |
| 30 | 0.60% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |

GC pauses: 0, total 0 ms

