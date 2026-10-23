# JVM hotspots (JFR ExecutionSample)

Samples: 36978

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.78% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 3.30% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 3 | 3.17% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 2.89% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3907` |
| 5 | 2.55% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 6 | 2.03% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 1.82% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 8 | 1.70% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.67% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 10 | 1.21% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 11 | 1.19% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 12 | 1.18% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 13 | 1.14% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 14 | 1.01% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 15 | 0.95% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3868` |
| 16 | 0.93% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 17 | 0.92% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 18 | 0.83% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 19 | 0.80% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 20 | 0.78% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 21 | 0.78% | `java.util.EnumMap.get(Object) line: 248` |
| 22 | 0.75% | `java.lang.String.hashCode() line: 2435` |
| 23 | 0.70% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 24 | 0.70% | `dev.planeta9091.centities.CeHooks.cellOfBox(AABB) line: 720` |
| 25 | 0.68% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 26 | 0.68% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 27 | 0.67% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 28 | 0.65% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 29 | 0.63% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 30 | 0.62% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 31 | 0.60% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 32 | 0.59% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 33 | 0.58% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 34 | 0.57% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 35 | 0.56% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2403` |
| 36 | 0.54% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 37 | 0.52% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 38 | 0.52% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 39 | 0.51% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1254` |
| 40 | 0.50% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 41 | 0.47% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 42 | 0.46% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 43 | 0.45% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 44 | 0.45% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 45 | 0.45% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 46 | 0.44% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1251` |
| 47 | 0.42% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 48 | 0.42% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 1655` |
| 49 | 0.42% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 50 | 0.40% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4706` |
| 51 | 0.39% | `net.minecraft.world.entity.SteppedInterpolationTracker.updateTracking(Vec3) line: 40` |
| 52 | 0.38% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 53 | 0.38% | `net.minecraft.world.entity.LivingEntity.updateGlowingStatus() line: 1025` |
| 54 | 0.37% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 55 | 0.37% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 56 | 0.36% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 57 | 0.36% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 58 | 0.34% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1574` |
| 59 | 0.34% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1325` |
| 60 | 0.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.87% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.01% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 3 | 3.78% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 4 | 3.30% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 5 | 2.89% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 6 | 2.63% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 7 | 2.03% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 1.57% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 937` |
| 9 | 1.35% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 10 | 1.16% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 11 | 1.13% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 12 | 1.03% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 13 | 1.01% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 14 | 0.98% | `dev.planeta9091.centities.CeHooks.arenaRef(long) line: 2187` |
| 15 | 0.97% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 16 | 0.91% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 969` |
| 17 | 0.88% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2120` |
| 18 | 0.82% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 19 | 0.81% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 20 | 0.80% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 21 | 0.78% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1255` |
| 22 | 0.78% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 23 | 0.72% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity)` |
| 24 | 0.71% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 25 | 0.70% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 26 | 0.70% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 27 | 0.70% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1264` |
| 28 | 0.68% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 29 | 0.67% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 30 | 0.63% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |

GC pauses: 0, total 0 ms

