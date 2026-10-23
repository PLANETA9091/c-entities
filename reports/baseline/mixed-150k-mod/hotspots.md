# JVM hotspots (JFR ExecutionSample)

Samples: 26540

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 8.50% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 6.01% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 3 | 5.23% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 4.77% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3907` |
| 5 | 3.84% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 6 | 2.91% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 2.89% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 8 | 2.04% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 9 | 1.87% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 10 | 1.80% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 11 | 1.72% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2403` |
| 12 | 1.69% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 13 | 1.61% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 14 | 1.54% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 15 | 1.31% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3868` |
| 16 | 1.19% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 17 | 1.18% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 1655` |
| 18 | 1.14% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 19 | 0.83% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 20 | 0.83% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 21 | 0.74% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 22 | 0.73% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 23 | 0.72% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 24 | 0.68% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 25 | 0.67% | `java.util.EnumMap.get(Object) line: 248` |
| 26 | 0.66% | `net.minecraft.world.entity.Entity.isRemoved() line: 5328` |
| 27 | 0.63% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 28 | 0.63% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 29 | 0.62% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 30 | 0.60% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 31 | 0.59% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 32 | 0.57% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 33 | 0.54% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 34 | 0.53% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 35 | 0.52% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 36 | 0.52% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 37 | 0.50% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 38 | 0.47% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 39 | 0.45% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 40 | 0.43% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1551` |
| 41 | 0.41% | `net.minecraft.world.entity.SteppedInterpolationTracker.updateTracking(Vec3) line: 40` |
| 42 | 0.41% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2093` |
| 43 | 0.41% | `net.minecraft.world.entity.Entity.computeSpeed() line: 960` |
| 44 | 0.40% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 45 | 0.37% | `net.minecraft.world.entity.Entity.getX() line: 5094` |
| 46 | 0.36% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 47 | 0.35% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 48 | 0.35% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 49 | 0.34% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 50 | 0.31% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 51 | 0.31% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1558` |
| 52 | 0.30% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1465` |
| 53 | 0.29% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 54 | 0.28% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 55 | 0.28% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 56 | 0.28% | `java.util.ArrayList.add(Object, Object[], int) line: 483` |
| 57 | 0.28% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 58 | 0.26% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 59 | 0.26% | `net.minecraft.world.entity.LivingEntity$SwingState.tick() line: 5015` |
| 60 | 0.26% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 8.50% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 2 | 8.13% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 7.30% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 4 | 6.03% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 5 | 4.79% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 6 | 1.91% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 7 | 1.88% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 8 | 1.87% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 9 | 1.84% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2120` |
| 10 | 1.76% | `net.minecraft.world.entity.Entity.setOldPos() line: 2395` |
| 11 | 1.75% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 12 | 1.56% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 13 | 1.48% | `dev.planeta9091.centities.CeHooks.arenaRef(long) line: 2187` |
| 14 | 1.30% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1579` |
| 15 | 1.28% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 969` |
| 16 | 1.28% | `net.minecraft.world.entity.Entity.setDeltaMovement(double, double, double) line: 5086` |
| 17 | 1.21% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity)` |
| 18 | 1.18% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 19 | 1.09% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 20 | 0.83% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 21 | 0.80% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 22 | 0.75% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 898` |
| 23 | 0.74% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 24 | 0.72% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 25 | 0.72% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2212` |
| 26 | 0.68% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 27 | 0.65% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 28 | 0.64% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 29 | 0.61% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2195` |
| 30 | 0.56% | `net.minecraft.world.entity.LivingEntity.tick() line: 3481` |

GC pauses: 0, total 0 ms

