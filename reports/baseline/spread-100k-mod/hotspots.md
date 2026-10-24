# JVM hotspots (JFR ExecutionSample)

Samples: 30565

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.53% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.95% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3907` |
| 3 | 3.85% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 4 | 3.39% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 2.40% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 6 | 2.25% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 7 | 2.05% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 8 | 1.79% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 1.74% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 10 | 1.73% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.37% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 12 | 1.15% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 13 | 1.01% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 14 | 0.98% | `java.util.EnumMap.get(Object) line: 248` |
| 15 | 0.96% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 16 | 0.96% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 17 | 0.95% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3868` |
| 18 | 0.88% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 19 | 0.79% | `net.minecraft.world.entity.LivingEntity.updateGlowingStatus() line: 1025` |
| 20 | 0.75% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 21 | 0.72% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 22 | 0.71% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 23 | 0.70% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 24 | 0.70% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2403` |
| 25 | 0.70% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 26 | 0.64% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 27 | 0.61% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 28 | 0.60% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 29 | 0.57% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 30 | 0.55% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 31 | 0.54% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 32 | 0.54% | `java.util.ArrayDeque.pollFirst() line: 378` |
| 33 | 0.54% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 34 | 0.52% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 35 | 0.51% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 36 | 0.51% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 37 | 0.49% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 1999` |
| 38 | 0.48% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1432` |
| 39 | 0.47% | `net.minecraft.world.entity.Entity.getX() line: 5094` |
| 40 | 0.46% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 41 | 0.46% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 42 | 0.44% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 43 | 0.43% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 44 | 0.42% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 45 | 0.41% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 46 | 0.41% | `dev.planeta9091.centities.CeHooks.cellOfBox(AABB) line: 813` |
| 47 | 0.39% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 48 | 0.37% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 49 | 0.37% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 50 | 0.34% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4706` |
| 51 | 0.34% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1358` |
| 52 | 0.34% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 53 | 0.34% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 54 | 0.33% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 55 | 0.33% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 56 | 0.33% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 57 | 0.30% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1361` |
| 58 | 0.30% | `net.minecraft.world.entity.Entity.computeSpeed() line: 960` |
| 59 | 0.28% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 60 | 0.28% | `net.minecraft.world.entity.Entity.isRemoved() line: 5328` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.53% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 2 | 5.11% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 4.95% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 4 | 4.77% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 5 | 3.86% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 6 | 2.40% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 7 | 1.79% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 1.68% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 9 | 1.43% | `net.minecraft.world.entity.Entity.setDeltaMovement(double, double, double) line: 5086` |
| 10 | 1.22% | `dev.planeta9091.centities.CeHooks.arenaRef(long) line: 2531` |
| 11 | 1.11% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 12 | 1.10% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2464` |
| 13 | 0.95% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1032` |
| 14 | 0.93% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 15 | 0.91% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 969` |
| 16 | 0.83% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity)` |
| 17 | 0.79% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 997` |
| 18 | 0.76% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 19 | 0.72% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 20 | 0.72% | `net.minecraft.world.entity.Entity.setOldPos() line: 2395` |
| 21 | 0.70% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 22 | 0.68% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 23 | 0.66% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |
| 24 | 0.66% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 25 | 0.64% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 26 | 0.62% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 898` |
| 27 | 0.61% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 28 | 0.60% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 29 | 0.57% | `java.util.Map.getOrDefault(Object, Object) line: 691` |
| 30 | 0.56% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |

GC pauses: 0, total 0 ms

