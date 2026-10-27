# JVM hotspots (JFR ExecutionSample)

Samples: 24974

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.65% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.50% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 3 | 3.19% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7603` |
| 4 | 3.13% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7462` |
| 5 | 2.81% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.43% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4522` |
| 7 | 2.39% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 8 | 2.35% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7409` |
| 9 | 2.06% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 10 | 2.05% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4377` |
| 11 | 1.57% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 12 | 1.35% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 13 | 1.32% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 14 | 1.27% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 15 | 1.02% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 16 | 1.00% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 17 | 0.85% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 18 | 0.82% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1696` |
| 19 | 0.80% | `java.util.EnumMap.get(Object) line: 248` |
| 20 | 0.80% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 21 | 0.79% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 22 | 0.79% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 23 | 0.71% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 24 | 0.70% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 25 | 0.70% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2381` |
| 26 | 0.69% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 27 | 0.68% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 28 | 0.67% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 29 | 0.66% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 30 | 0.64% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 31 | 0.59% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 32 | 0.56% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 33 | 0.54% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 34 | 0.53% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 35 | 0.53% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7423` |
| 36 | 0.52% | `net.minecraft.world.entity.Entity.computeSpeed() line: 961` |
| 37 | 0.51% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 4700` |
| 38 | 0.50% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 39 | 0.50% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 40 | 0.49% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 41 | 0.48% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 42 | 0.47% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1715` |
| 43 | 0.47% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 44 | 0.44% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7556` |
| 45 | 0.42% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 46 | 0.42% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 47 | 0.41% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 48 | 0.40% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 49 | 0.40% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7548` |
| 50 | 0.40% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 51 | 0.40% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 52 | 0.39% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2327` |
| 53 | 0.38% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 54 | 0.38% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 55 | 0.38% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 56 | 0.38% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 57 | 0.37% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 58 | 0.37% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 59 | 0.35% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 5045` |
| 60 | 0.34% | `net.minecraft.world.entity.Entity.getType() line: 633` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 9.14% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.34% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 3.20% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 4 | 3.10% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7458` |
| 5 | 2.78% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 6 | 2.37% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 7 | 2.09% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 8 | 2.03% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 9 | 2.03% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4524` |
| 10 | 1.93% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1307` |
| 11 | 1.91% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 12 | 1.67% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 13 | 1.57% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 14 | 1.55% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 15 | 1.32% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 16 | 1.20% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 17 | 1.07% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 18 | 1.06% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 19 | 1.03% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7475` |
| 20 | 1.03% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 21 | 0.97% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7450` |
| 22 | 0.90% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 898` |
| 23 | 0.86% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 24 | 0.81% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 25 | 0.78% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 5055` |
| 26 | 0.75% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 5416` |
| 27 | 0.71% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2195` |
| 28 | 0.70% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 29 | 0.70% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4377` |
| 30 | 0.69% | `net.minecraft.world.entity.Entity.isEyeInFluid(TagKey) line: 2306` |

GC pauses: 0, total 0 ms

