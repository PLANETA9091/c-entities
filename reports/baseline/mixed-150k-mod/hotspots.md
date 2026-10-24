# JVM hotspots (JFR ExecutionSample)

Samples: 44518

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.37% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 2 | 3.98% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3907` |
| 3 | 3.39% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 2.73% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3025` |
| 5 | 2.60% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 2898` |
| 6 | 2.32% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 7 | 2.07% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 8 | 1.67% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 1.64% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 2927` |
| 10 | 1.61% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.59% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 1.24% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 13 | 1.21% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 14 | 1.21% | `dev.planeta9091.centities.CeHooks.cellOfBox(AABB) line: 817` |
| 15 | 1.11% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 16 | 1.07% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3868` |
| 17 | 1.04% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 18 | 0.93% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 19 | 0.86% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3027` |
| 20 | 0.74% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 21 | 0.73% | `java.util.EnumMap.get(Object) line: 248` |
| 22 | 0.73% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 23 | 0.73% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1372` |
| 24 | 0.73% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 25 | 0.72% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 26 | 0.71% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 27 | 0.70% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 28 | 0.69% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 29 | 0.65% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2403` |
| 30 | 0.62% | `jdk.internal.reflect.MethodHandleObjectFieldAccessorImpl.get(Object) line: 57` |
| 31 | 0.62% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 32 | 0.60% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 33 | 0.60% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 34 | 0.58% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 35 | 0.55% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1369` |
| 36 | 0.53% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1443` |
| 37 | 0.52% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 38 | 0.52% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 39 | 0.44% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 40 | 0.43% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 41 | 0.43% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 42 | 0.42% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 43 | 0.41% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 44 | 0.40% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 45 | 0.40% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 46 | 0.39% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 47 | 0.37% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 48 | 0.35% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 49 | 0.35% | `net.minecraft.world.entity.Entity.isRemoved() line: 5328` |
| 50 | 0.33% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 51 | 0.33% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 52 | 0.33% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4706` |
| 53 | 0.33% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 54 | 0.33% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 55 | 0.32% | `java.util.ArrayList.add(Object, Object[], int) line: 483` |
| 56 | 0.31% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 57 | 0.30% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 58 | 0.29% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 59 | 0.29% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 60 | 0.29% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.99% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.54% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 3 | 4.38% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 4 | 3.98% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 5 | 3.64% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 6 | 2.00% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 7 | 1.85% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1036` |
| 8 | 1.67% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 1.65% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 10 | 1.52% | `net.minecraft.world.entity.Entity.setDeltaMovement(double, double, double) line: 5086` |
| 11 | 1.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 12 | 1.31% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 13 | 1.28% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1382` |
| 14 | 1.25% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 15 | 1.19% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1373` |
| 16 | 1.15% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 17 | 1.13% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 18 | 1.00% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 969` |
| 19 | 0.93% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 20 | 0.89% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 21 | 0.88% | `dev.planeta9091.centities.CeHooks.arenaRef(long) line: 2542` |
| 22 | 0.79% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2212` |
| 23 | 0.76% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2475` |
| 24 | 0.74% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 25 | 0.71% | `java.lang.reflect.Field.get(Object) line: 440` |
| 26 | 0.68% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 27 | 0.67% | `net.minecraft.world.entity.Entity.setOldPos() line: 2395` |
| 28 | 0.60% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 29 | 0.60% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 30 | 0.60% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity)` |

GC pauses: 0, total 0 ms

