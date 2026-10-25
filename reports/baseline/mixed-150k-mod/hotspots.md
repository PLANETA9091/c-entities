# JVM hotspots (JFR ExecutionSample)

Samples: 32418

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.90% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.38% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 2.97% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 4 | 2.75% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3624` |
| 5 | 2.62% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 5463` |
| 6 | 2.60% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1496` |
| 7 | 2.43% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 8 | 2.04% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 9 | 1.87% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3806` |
| 10 | 1.82% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 11 | 1.80% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 12 | 1.78% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 5422` |
| 13 | 1.61% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 14 | 1.46% | `dev.planeta9091.centities.CeHooks.setSharedFlag$(Entity, int, boolean) line: 5326` |
| 15 | 1.34% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1406` |
| 16 | 1.30% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3625` |
| 17 | 1.27% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 18 | 1.16% | `java.util.EnumMap.get(Object) line: 248` |
| 19 | 1.11% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 20 | 1.10% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 21 | 1.05% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 22 | 1.02% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1425` |
| 23 | 0.88% | `dev.planeta9091.centities.CeHooks.aiPackSel(int[], int, WrappedGoal[], int[], int[], long[], int[], float[], Mob) line: 4887` |
| 24 | 0.86% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 25 | 0.81% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 26 | 0.80% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 27 | 0.78% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 28 | 0.74% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 29 | 0.68% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 30 | 0.68% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 31 | 0.68% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 32 | 0.66% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 33 | 0.60% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 34 | 0.58% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 35 | 0.54% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 36 | 0.52% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 37 | 0.52% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 38 | 0.52% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 39 | 0.51% | `net.minecraft.world.entity.Entity.computeSpeed() line: 961` |
| 40 | 0.50% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 41 | 0.48% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 42 | 0.46% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2068` |
| 43 | 0.43% | `dev.planeta9091.centities.CeHooks.disabledBacking(GoalSelector) line: 4070` |
| 44 | 0.42% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 45 | 0.42% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 46 | 0.41% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 47 | 0.40% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 48 | 0.39% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 49 | 0.39% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 50 | 0.38% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 51 | 0.37% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 52 | 0.37% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 53 | 0.36% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 603` |
| 54 | 0.36% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 55 | 0.36% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 56 | 0.35% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 57 | 0.33% | `java.util.ArrayDeque.size() line: 647` |
| 58 | 0.32% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 59 | 0.32% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1466` |
| 60 | 0.32% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2014` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 8.87% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.25% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 3 | 3.70% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 4 | 3.05% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1059` |
| 5 | 2.94% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 5422` |
| 6 | 2.64% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 7 | 2.41% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 8 | 2.39% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity) line: 603` |
| 9 | 2.15% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 10 | 2.05% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 11 | 1.80% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3808` |
| 12 | 1.79% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 13 | 1.79% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk)` |
| 14 | 1.48% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean)` |
| 15 | 1.40% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 16 | 1.27% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 17 | 1.20% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 18 | 1.15% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 19 | 1.07% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 20 | 1.05% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 21 | 0.87% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1410` |
| 22 | 0.81% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 23 | 0.72% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 24 | 0.70% | `net.minecraft.world.entity.LivingEntity.detectEquipmentUpdates() line: 3558` |
| 25 | 0.69% | `dev.planeta9091.centities.CeHooks.aiPack(Mob, CeHooks$AiCtx, int, int, int, double, double, double) line: 4874` |
| 26 | 0.68% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 27 | 0.68% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 28 | 0.68% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2195` |
| 29 | 0.64% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 30 | 0.63% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 226` |

GC pauses: 0, total 0 ms

