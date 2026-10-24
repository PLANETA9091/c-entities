# JVM hotspots (JFR ExecutionSample)

Samples: 32640

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.83% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 2 | 3.97% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3065` |
| 3 | 3.16% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 2.92% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 2938` |
| 5 | 2.82% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3907` |
| 6 | 2.56% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1462` |
| 7 | 2.29% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 2967` |
| 8 | 1.79% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 9 | 1.74% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 1.73% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.71% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 12 | 1.36% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 1.30% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3067` |
| 14 | 1.19% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 15 | 1.07% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 16 | 1.03% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 17 | 0.99% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 18 | 0.99% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 19 | 0.97% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 20 | 0.89% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 21 | 0.84% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 22 | 0.83% | `jdk.internal.reflect.MethodHandleObjectFieldAccessorImpl.get(Object) line: 57` |
| 23 | 0.82% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3868` |
| 24 | 0.74% | `java.util.EnumMap.get(Object) line: 248` |
| 25 | 0.74% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 26 | 0.70% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2403` |
| 27 | 0.70% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 28 | 0.62% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 29 | 0.59% | `net.minecraft.world.entity.LivingEntity.updateGlowingStatus() line: 1025` |
| 30 | 0.56% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 31 | 0.54% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 32 | 0.51% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 33 | 0.50% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 34 | 0.50% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 35 | 0.48% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 36 | 0.47% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 37 | 0.47% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 38 | 0.46% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 39 | 0.45% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 40 | 0.44% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 41 | 0.44% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2050` |
| 42 | 0.43% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 43 | 0.43% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 44 | 0.41% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 45 | 0.40% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 46 | 0.40% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 47 | 0.39% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1398` |
| 48 | 0.38% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 49 | 0.37% | `net.minecraft.world.entity.Entity.getX() line: 5094` |
| 50 | 0.36% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 51 | 0.36% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 52 | 0.36% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 53 | 0.36% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 54 | 0.35% | `java.util.ArrayList.add(Object, Object[], int) line: 483` |
| 55 | 0.34% | `net.minecraft.world.entity.Entity.isRemoved() line: 5328` |
| 56 | 0.34% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 57 | 0.33% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 58 | 0.33% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 59 | 0.32% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1379` |
| 60 | 0.31% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.84% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 2 | 5.64% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 3 | 5.36% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 4.88% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 2.82% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 6 | 2.49% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 7 | 2.41% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity) line: 594` |
| 8 | 1.89% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 9 | 1.74% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 10 | 1.50% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 11 | 1.27% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1042` |
| 12 | 1.20% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 13 | 1.19% | `dev.planeta9091.centities.CeHooks.arenaRef(long) line: 2582` |
| 14 | 1.13% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 15 | 1.03% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 16 | 0.95% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 17 | 0.92% | `java.lang.reflect.Field.get(Object) line: 440` |
| 18 | 0.92% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 19 | 0.85% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 20 | 0.77% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 21 | 0.77% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 969` |
| 22 | 0.74% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity)` |
| 23 | 0.72% | `net.minecraft.world.entity.Entity.setOldPos() line: 2395` |
| 24 | 0.70% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 25 | 0.63% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 26 | 0.59% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 997` |
| 27 | 0.58% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 898` |
| 28 | 0.56% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 29 | 0.54% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 30 | 0.54% | `net.minecraft.world.entity.Mob.inactiveTick() line: 229` |

GC pauses: 0, total 0 ms

