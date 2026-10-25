# JVM hotspots (JFR ExecutionSample)

Samples: 34068

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.37% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 2.83% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 3 | 2.78% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 2.42% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3314` |
| 5 | 2.25% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 6 | 2.22% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 7 | 2.00% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 8 | 1.99% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3460` |
| 9 | 1.87% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 4210` |
| 10 | 1.77% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 11 | 1.68% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1494` |
| 12 | 1.66% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3578` |
| 13 | 1.63% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 14 | 1.40% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3696` |
| 15 | 1.36% | `dev.planeta9091.centities.CeHooks.setSharedFlag$(Entity, int, boolean) line: 4124` |
| 16 | 1.28% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 17 | 1.27% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1404` |
| 18 | 1.22% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3324` |
| 19 | 1.14% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 20 | 1.11% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2403` |
| 21 | 1.07% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 22 | 1.00% | `java.util.EnumMap.get(Object) line: 248` |
| 23 | 1.00% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 3800` |
| 24 | 0.96% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 25 | 0.95% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 26 | 0.88% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 27 | 0.86% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1423` |
| 28 | 0.85% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 29 | 0.81% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 30 | 0.73% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 31 | 0.73% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 32 | 0.65% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 33 | 0.62% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 34 | 0.61% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 35 | 0.61% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 36 | 0.61% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 37 | 0.54% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 38 | 0.51% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 39 | 0.50% | `net.minecraft.world.entity.Entity.computeSpeed() line: 960` |
| 40 | 0.50% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 41 | 0.50% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 42 | 0.49% | `net.minecraft.world.entity.Entity.level() line: 5415` |
| 43 | 0.47% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 44 | 0.47% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 45 | 0.47% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 46 | 0.45% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 47 | 0.44% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2058` |
| 48 | 0.43% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 49 | 0.43% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 50 | 0.42% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 51 | 0.42% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 52 | 0.40% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 53 | 0.40% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 54 | 0.38% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 55 | 0.38% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 56 | 0.37% | `java.util.ArrayDeque.size() line: 647` |
| 57 | 0.34% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 58 | 0.33% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 59 | 0.33% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1428` |
| 60 | 0.32% | `net.minecraft.core.Vec3i.getX() line: 63` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 8.20% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.74% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 3 | 2.89% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1057` |
| 4 | 2.78% | `dev.planeta9091.centities.CeHooks$RefField.getObj(Object) line: 2494` |
| 5 | 2.41% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 6 | 2.36% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 7 | 2.26% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 8 | 2.10% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 9 | 2.02% | `net.minecraft.world.entity.Entity.setDeltaMovement(double, double, double) line: 5086` |
| 10 | 1.89% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk)` |
| 11 | 1.77% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 12 | 1.69% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3462` |
| 13 | 1.67% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3685` |
| 14 | 1.44% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity) line: 601` |
| 15 | 1.44% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 16 | 1.36% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean)` |
| 17 | 1.30% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 18 | 1.23% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 19 | 1.22% | `net.minecraft.world.entity.LivingEntity.tick() line: 3432` |
| 20 | 1.18% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2212` |
| 21 | 1.17% | `net.minecraft.world.entity.Entity.setOldPos() line: 2395` |
| 22 | 1.11% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 23 | 1.07% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 24 | 1.05% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 25 | 1.03% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 3800` |
| 26 | 0.95% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 27 | 0.86% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 28 | 0.75% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 29 | 0.75% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 30 | 0.69% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |

GC pauses: 0, total 0 ms

