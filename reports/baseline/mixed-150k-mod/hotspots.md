# JVM hotspots (JFR ExecutionSample)

Samples: 48853

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.39% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 2 | 3.23% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 2.92% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1462` |
| 4 | 2.82% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3907` |
| 5 | 2.66% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 2938` |
| 6 | 2.42% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 7 | 2.27% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3065` |
| 8 | 1.71% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 2967` |
| 9 | 1.47% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.38% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 11 | 1.28% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3868` |
| 12 | 1.26% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1398` |
| 13 | 1.22% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 14 | 1.22% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 15 | 1.12% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 16 | 1.11% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 17 | 1.04% | `java.util.EnumMap.get(Object) line: 248` |
| 18 | 1.02% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 19 | 1.02% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 20 | 0.93% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 21 | 0.91% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 22 | 0.87% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 23 | 0.85% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 24 | 0.78% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 25 | 0.78% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3067` |
| 26 | 0.77% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 27 | 0.74% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1382` |
| 28 | 0.71% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 29 | 0.69% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 30 | 0.66% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2403` |
| 31 | 0.64% | `jdk.internal.reflect.MethodHandleObjectFieldAccessorImpl.get(Object) line: 57` |
| 32 | 0.63% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 33 | 0.63% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 34 | 0.60% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 35 | 0.60% | `net.minecraft.world.entity.Entity.isRemoved() line: 5328` |
| 36 | 0.55% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 37 | 0.53% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 38 | 0.51% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 39 | 0.50% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 40 | 0.48% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1379` |
| 41 | 0.48% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 42 | 0.45% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 43 | 0.41% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 44 | 0.41% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 45 | 0.40% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 46 | 0.40% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 47 | 0.40% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 237` |
| 48 | 0.40% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 49 | 0.37% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 50 | 0.36% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 51 | 0.35% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 52 | 0.34% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 53 | 0.34% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 54 | 0.34% | `java.util.ArrayList.add(Object, Object[], int) line: 483` |
| 55 | 0.33% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 56 | 0.33% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2050` |
| 57 | 0.31% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 58 | 0.31% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 59 | 0.31% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 60 | 0.30% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4706` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.69% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.64% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 3 | 4.39% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 4 | 3.19% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 3.15% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1042` |
| 6 | 2.82% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 7 | 2.65% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity) line: 594` |
| 8 | 2.14% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 9 | 1.46% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 10 | 1.40% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 11 | 1.23% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 969` |
| 12 | 1.22% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 13 | 1.17% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1383` |
| 14 | 1.17% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 15 | 1.13% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 16 | 1.12% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 17 | 1.06% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 18 | 1.06% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 19 | 1.05% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 20 | 1.02% | `dev.planeta9091.centities.CeHooks.arenaRef(long) line: 2582` |
| 21 | 0.91% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 22 | 0.89% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 23 | 0.87% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 24 | 0.80% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2212` |
| 25 | 0.70% | `java.lang.reflect.Field.get(Object) line: 440` |
| 26 | 0.69% | `net.minecraft.world.entity.Entity.setOldPos() line: 2395` |
| 27 | 0.69% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 28 | 0.67% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 29 | 0.65% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 30 | 0.61% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |

GC pauses: 0, total 0 ms

