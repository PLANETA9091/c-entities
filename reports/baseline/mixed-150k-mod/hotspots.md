# JVM hotspots (JFR ExecutionSample)

Samples: 37640

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.76% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 4.13% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 1.98% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4197` |
| 4 | 1.98% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 5 | 1.97% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 6473` |
| 6 | 1.78% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 1.64% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 8 | 1.55% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1425` |
| 9 | 1.50% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 1.40% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 11 | 1.35% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6420` |
| 12 | 1.30% | `dev.planeta9091.centities.CeHooks.setSharedFlag$(Entity, int, boolean) line: 6309` |
| 13 | 1.28% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 14 | 1.26% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 15 | 1.06% | `java.util.EnumMap.get(Object) line: 248` |
| 16 | 1.04% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 17 | 1.04% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 18 | 0.97% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 19 | 0.86% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4052` |
| 20 | 0.80% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 21 | 0.80% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 22 | 0.78% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 23 | 0.77% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 24 | 0.75% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 25 | 0.74% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1406` |
| 26 | 0.74% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 27 | 0.69% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 28 | 0.66% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 29 | 0.63% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 30 | 0.60% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 31 | 0.59% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 32 | 0.57% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1409` |
| 33 | 0.55% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 34 | 0.52% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 35 | 0.50% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 36 | 0.48% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 4375` |
| 37 | 0.47% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 38 | 0.46% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 39 | 0.43% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 40 | 0.42% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 41 | 0.42% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 42 | 0.42% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 43 | 0.40% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 44 | 0.37% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 45 | 0.37% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2075` |
| 46 | 0.35% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 47 | 0.35% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 48 | 0.34% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 49 | 0.34% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 50 | 0.33% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 51 | 0.33% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 52 | 0.33% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 53 | 0.32% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 54 | 0.32% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 55 | 0.31% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 56 | 0.31% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 57 | 0.31% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 58 | 0.30% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 59 | 0.30% | `java.util.ArrayList.add(Object, Object[], int) line: 483` |
| 60 | 0.30% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.91% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.70% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1059` |
| 3 | 2.58% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6420` |
| 4 | 2.22% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 2.10% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4002` |
| 6 | 1.98% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 7 | 1.98% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 8 | 1.58% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 9 | 1.50% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 10 | 1.46% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 11 | 1.38% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1410` |
| 12 | 1.36% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk)` |
| 13 | 1.35% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 14 | 1.32% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean)` |
| 15 | 1.27% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 16 | 1.26% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 17 | 1.26% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 18 | 1.23% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 19 | 1.11% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4199` |
| 20 | 1.04% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 21 | 0.98% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 22 | 0.85% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 23 | 0.85% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4662` |
| 24 | 0.84% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4730` |
| 25 | 0.83% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 26 | 0.82% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 27 | 0.76% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 28 | 0.74% | `net.minecraft.world.entity.LivingEntity.detectEquipmentUpdates() line: 3558` |
| 29 | 0.74% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 30 | 0.73% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |

GC pauses: 0, total 0 ms

