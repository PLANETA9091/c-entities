# JVM hotspots (JFR ExecutionSample)

Samples: 40924

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.84% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 3844` |
| 2 | 3.67% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.11% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1484` |
| 4 | 2.84% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3190` |
| 5 | 2.42% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 6 | 2.36% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 1.78% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 1.43% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3053` |
| 9 | 1.41% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3063` |
| 10 | 1.38% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 3818` |
| 11 | 1.38% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 12 | 1.34% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1413` |
| 13 | 1.32% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 14 | 1.26% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 15 | 1.09% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 16 | 1.07% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 17 | 1.00% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 18 | 0.97% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3868` |
| 19 | 0.87% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 20 | 0.75% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2403` |
| 21 | 0.74% | `java.util.EnumMap.get(Object) line: 248` |
| 22 | 0.74% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 23 | 0.73% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1394` |
| 24 | 0.72% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 25 | 0.69% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 26 | 0.67% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 898` |
| 27 | 0.64% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 28 | 0.62% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 29 | 0.61% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 30 | 0.61% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 31 | 0.60% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 3517` |
| 32 | 0.57% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 33 | 0.56% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1397` |
| 34 | 0.55% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 35 | 0.53% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 36 | 0.52% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 37 | 0.51% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 38 | 0.49% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 39 | 0.48% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 40 | 0.46% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 41 | 0.45% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 42 | 0.44% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 43 | 0.42% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 44 | 0.41% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 3345` |
| 45 | 0.40% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 46 | 0.38% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 47 | 0.36% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 48 | 0.36% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 49 | 0.35% | `net.minecraft.world.entity.Entity.level() line: 5415` |
| 50 | 0.33% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2079` |
| 51 | 0.32% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 52 | 0.32% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 53 | 0.32% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 54 | 0.32% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 55 | 0.32% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 56 | 0.32% | `java.util.ArrayList.add(Object, Object[], int) line: 483` |
| 57 | 0.31% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 58 | 0.31% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 59 | 0.31% | `dev.planeta9091.centities.CeHooks.aiShadowPeek(Object) line: 3379` |
| 60 | 0.30% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.45% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.86% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 3 | 3.18% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1047` |
| 4 | 2.93% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 2.92% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 6 | 2.82% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity) line: 599` |
| 7 | 2.36% | `dev.planeta9091.centities.CeHooks$RefField.getObj(Object) line: 2296` |
| 8 | 1.99% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 9 | 1.52% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 10 | 1.49% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 11 | 1.41% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk)` |
| 12 | 1.38% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 13 | 1.29% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1398` |
| 14 | 1.18% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 15 | 1.15% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 16 | 1.13% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 17 | 1.12% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 18 | 0.94% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 19 | 0.91% | `dev.planeta9091.centities.CeHooks.arenaRef(long) line: 2692` |
| 20 | 0.88% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 969` |
| 21 | 0.87% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 22 | 0.82% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 23 | 0.77% | `net.minecraft.world.entity.Entity.setOldPos() line: 2395` |
| 24 | 0.76% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 3529` |
| 25 | 0.73% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2212` |
| 26 | 0.69% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3190` |
| 27 | 0.66% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 28 | 0.65% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 29 | 0.63% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 30 | 0.62% | `net.minecraft.world.entity.LivingEntity.detectEquipmentUpdates() line: 3558` |

GC pauses: 0, total 0 ms

