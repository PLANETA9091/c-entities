# JVM hotspots (JFR ExecutionSample)

Samples: 49262

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.07% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 4.50% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 3 | 3.53% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 4394` |
| 4 | 3.34% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 5 | 3.23% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1495` |
| 6 | 2.61% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 7 | 2.31% | `net.minecraft.world.entity.Entity.level() line: 5415` |
| 8 | 2.14% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 9 | 1.97% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3467` |
| 10 | 1.68% | `java.util.HashMap.getNode(Object) line: 586` |
| 11 | 1.58% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 12 | 1.49% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3613` |
| 13 | 1.39% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 14 | 1.39% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(BlockPos) line: 15` |
| 15 | 1.26% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 16 | 1.22% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 17 | 1.14% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 4363` |
| 18 | 1.05% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 19 | 1.05% | `java.util.EnumMap.get(Object) line: 248` |
| 20 | 1.04% | `dev.planeta9091.centities.CeHooks.setSharedFlag$(Entity, int, boolean) line: 4277` |
| 21 | 0.98% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 22 | 0.94% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 23 | 0.92% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 3953` |
| 24 | 0.85% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3926` |
| 25 | 0.85% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 26 | 0.79% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2403` |
| 27 | 0.78% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3477` |
| 28 | 0.75% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 29 | 0.71% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity) line: 617` |
| 30 | 0.69% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 31 | 0.64% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 32 | 0.64% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 33 | 0.60% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1181` |
| 34 | 0.59% | `java.util.ArrayDeque.size() line: 647` |
| 35 | 0.58% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 36 | 0.57% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 37 | 0.56% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 38 | 0.55% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 39 | 0.51% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1574` |
| 40 | 0.49% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 41 | 0.48% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 42 | 0.47% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 43 | 0.44% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 44 | 0.43% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 45 | 0.43% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 46 | 0.42% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 47 | 0.41% | `net.minecraft.world.entity.Entity.computeSpeed() line: 960` |
| 48 | 0.39% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 49 | 0.38% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 50 | 0.37% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 51 | 0.36% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 52 | 0.35% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 53 | 0.34% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 54 | 0.33% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 4089` |
| 55 | 0.33% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2059` |
| 56 | 0.32% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 57 | 0.31% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 58 | 0.31% | `net.minecraft.world.entity.LivingEntity.isPushable() line: 4073` |
| 59 | 0.30% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 60 | 0.30% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 8.42% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.41% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1182` |
| 3 | 3.53% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 4 | 2.83% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity) line: 602` |
| 5 | 2.80% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 6 | 2.14% | `dev.planeta9091.centities.CeHooks$RefField.getObj(Object) line: 2647` |
| 7 | 2.04% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 8 | 1.98% | `net.minecraft.world.entity.LivingEntity.isPushable() line: 4073` |
| 9 | 1.93% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 10 | 1.79% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 11 | 1.76% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 12 | 1.72% | `java.util.HashMap.containsKey(Object) line: 603` |
| 13 | 1.45% | `net.minecraft.server.level.ServerLevel.isPositionEntityTicking(BlockPos) line: 2956` |
| 14 | 1.39% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 15 | 1.35% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3615` |
| 16 | 1.28% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 17 | 1.21% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2195` |
| 18 | 1.15% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk)` |
| 19 | 1.12% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 20 | 1.06% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean)` |
| 21 | 1.05% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 22 | 0.97% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 23 | 0.96% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 24 | 0.93% | `dev.planeta9091.centities.CeHooks.getEntities(Level, Entity, AABB, Predicate) line: 480` |
| 25 | 0.90% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3810` |
| 26 | 0.87% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 27 | 0.86% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 28 | 0.82% | `net.minecraft.world.entity.Entity.setOldPos() line: 2395` |
| 29 | 0.80% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 30 | 0.79% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 3953` |

GC pauses: 0, total 0 ms

