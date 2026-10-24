# JVM hotspots (JFR ExecutionSample)

Samples: 28103

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.81% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 4.57% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 3 | 4.56% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3907` |
| 4 | 3.67% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 2.65% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 6 | 2.38% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 7 | 1.74% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 1.49% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 9 | 1.44% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 10 | 1.37% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 11 | 1.14% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 12 | 1.14% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 1.13% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 14 | 1.06% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2403` |
| 15 | 1.06% | `java.util.EnumMap.get(Object) line: 248` |
| 16 | 1.03% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3868` |
| 17 | 1.00% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 18 | 0.99% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 19 | 0.81% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 20 | 0.79% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 21 | 0.75% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 22 | 0.74% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 23 | 0.73% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1420` |
| 24 | 0.72% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 25 | 0.71% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 26 | 0.67% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 27 | 0.66% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 28 | 0.66% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 29 | 0.65% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 30 | 0.62% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 31 | 0.61% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 32 | 0.54% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 33 | 0.51% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 34 | 0.51% | `net.minecraft.world.entity.LivingEntity.updateGlowingStatus() line: 1025` |
| 35 | 0.49% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 36 | 0.49% | `net.minecraft.world.entity.SteppedInterpolationTracker.updateTracking(Vec3) line: 40` |
| 37 | 0.47% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 38 | 0.47% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 39 | 0.44% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 40 | 0.41% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 41 | 0.40% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 42 | 0.40% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 43 | 0.40% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 44 | 0.39% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 45 | 0.39% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 46 | 0.38% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 47 | 0.38% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 48 | 0.37% | `net.minecraft.world.entity.Entity.isRemoved() line: 5328` |
| 49 | 0.35% | `java.util.ArrayDeque.size() line: 647` |
| 50 | 0.33% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 51 | 0.33% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 52 | 0.32% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1851` |
| 53 | 0.32% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 54 | 0.31% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 1955` |
| 55 | 0.31% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 56 | 0.30% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 57 | 0.29% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 58 | 0.29% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 59 | 0.29% | `java.util.concurrent.ConcurrentLinkedQueue.poll() line: 385` |
| 60 | 0.28% | `net.minecraft.world.entity.Entity.getType() line: 633` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.42% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 2 | 5.41% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 4.81% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 4 | 4.58% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 5 | 4.56% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 6 | 1.79% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 7 | 1.72% | `net.minecraft.world.entity.Entity.setDeltaMovement(double, double, double) line: 5086` |
| 8 | 1.54% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 9 | 1.43% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 10 | 1.35% | `dev.planeta9091.centities.CeHooks.arenaRef(long) line: 2487` |
| 11 | 1.23% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2420` |
| 12 | 1.15% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 13 | 1.13% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 14 | 1.08% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 15 | 1.08% | `net.minecraft.world.entity.Entity.setOldPos() line: 2395` |
| 16 | 0.99% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 17 | 0.98% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 18 | 0.97% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 969` |
| 19 | 0.89% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 20 | 0.72% | `java.util.Map.getOrDefault(Object, Object) line: 691` |
| 21 | 0.72% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity)` |
| 22 | 0.70% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2212` |
| 23 | 0.69% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 24 | 0.69% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 25 | 0.62% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 26 | 0.62% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 27 | 0.59% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 28 | 0.57% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 29 | 0.57% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |
| 30 | 0.57% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |

GC pauses: 0, total 0 ms

