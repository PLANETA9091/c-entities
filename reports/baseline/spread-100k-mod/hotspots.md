# JVM hotspots (JFR ExecutionSample)

Samples: 21578

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.44% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 5.16% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.64% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 6584` |
| 4 | 3.25% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 5 | 2.42% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6531` |
| 6 | 2.31% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 7 | 2.19% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4202` |
| 8 | 1.77% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4057` |
| 9 | 1.69% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 10 | 1.39% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 11 | 1.32% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 12 | 1.27% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 13 | 1.19% | `dev.planeta9091.centities.CeHooks.setSharedFlag$(Entity, int, boolean) line: 6420` |
| 14 | 1.19% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 15 | 1.08% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 16 | 1.08% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 17 | 0.91% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 18 | 0.87% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 19 | 0.87% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 20 | 0.83% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 21 | 0.79% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 22 | 0.79% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 23 | 0.78% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 24 | 0.76% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 25 | 0.76% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 2911` |
| 26 | 0.74% | `java.util.EnumMap.get(Object) line: 248` |
| 27 | 0.71% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 28 | 0.71% | `dev.planeta9091.centities.CeHooks.aiPackSel(int[], int, WrappedGoal[], int[], int[], long[], int[], float[], Mob) line: 5906` |
| 29 | 0.71% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1411` |
| 30 | 0.66% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 31 | 0.65% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 32 | 0.64% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 33 | 0.57% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2080` |
| 34 | 0.56% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4207` |
| 35 | 0.56% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 36 | 0.55% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 37 | 0.55% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 4380` |
| 38 | 0.53% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 39 | 0.51% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 40 | 0.50% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 41 | 0.49% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 42 | 0.49% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 43 | 0.46% | `net.minecraft.world.entity.Entity.computeSpeed() line: 961` |
| 44 | 0.46% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 45 | 0.45% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 46 | 0.45% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 47 | 0.44% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 48 | 0.44% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 49 | 0.41% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 50 | 0.40% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 51 | 0.40% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2026` |
| 52 | 0.39% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4725` |
| 53 | 0.38% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 54 | 0.35% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 55 | 0.35% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getAttributesToSync() line: 34` |
| 56 | 0.35% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 57 | 0.34% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 58 | 0.34% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4668` |
| 59 | 0.33% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 60 | 0.32% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 8.41% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.64% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 3 | 3.12% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 2.83% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6531` |
| 5 | 2.45% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk)` |
| 6 | 2.30% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 7 | 2.27% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 8 | 2.23% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4007` |
| 9 | 2.05% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4204` |
| 10 | 1.93% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 11 | 1.90% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 12 | 1.65% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 13 | 1.62% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 14 | 1.39% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 15 | 1.36% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 16 | 1.32% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1064` |
| 17 | 1.27% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 18 | 1.20% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean)` |
| 19 | 1.12% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 20 | 1.08% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 21 | 0.93% | `dev.planeta9091.centities.CeHooks.entityDataValue(Entity, Object) line: 5005` |
| 22 | 0.93% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 23 | 0.92% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 24 | 0.90% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 898` |
| 25 | 0.88% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4735` |
| 26 | 0.88% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 27 | 0.87% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 28 | 0.82% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 29 | 0.76% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 30 | 0.73% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 226` |

GC pauses: 0, total 0 ms

