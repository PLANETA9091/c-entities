# JVM hotspots (JFR ExecutionSample)

Samples: 26732

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.22% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 4.31% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 3.66% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6473` |
| 4 | 3.41% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 5 | 3.12% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 6 | 2.91% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 2.91% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 8 | 1.90% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9759` |
| 9 | 1.73% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 10 | 1.72% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 11 | 1.69% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 12 | 1.55% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 13 | 1.51% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 14 | 1.41% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 15 | 1.38% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 16 | 1.38% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 17 | 1.33% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3584` |
| 18 | 1.26% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 19 | 1.18% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 20 | 1.01% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 21 | 1.00% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 22 | 0.91% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 23 | 0.86% | `java.util.EnumMap.get(Object) line: 248` |
| 24 | 0.83% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 25 | 0.83% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 26 | 0.79% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 27 | 0.76% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4204` |
| 28 | 0.73% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6980` |
| 29 | 0.70% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 30 | 0.65% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 31 | 0.61% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3696` |
| 32 | 0.56% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7048` |
| 33 | 0.51% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4072` |
| 34 | 0.50% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 35 | 0.48% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 36 | 0.46% | `dev.planeta9091.centities.CeHooks.equipUpdates$(LivingEntity) line: 10169` |
| 37 | 0.45% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 38 | 0.43% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 39 | 0.42% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7590` |
| 40 | 0.42% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 41 | 0.41% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 42 | 0.40% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 6824` |
| 43 | 0.39% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 10273` |
| 44 | 0.39% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 45 | 0.39% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4570` |
| 46 | 0.37% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9730` |
| 47 | 0.37% | `net.minecraft.world.entity.Entity.computeSpeed() line: 961` |
| 48 | 0.36% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 49 | 0.36% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 50 | 0.36% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 51 | 0.36% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 52 | 0.35% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 53 | 0.35% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 54 | 0.34% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6483` |
| 55 | 0.34% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 56 | 0.34% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 501` |
| 57 | 0.33% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9777` |
| 58 | 0.33% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 59 | 0.33% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5710` |
| 60 | 0.33% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9733` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 8.13% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.39% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 4.31% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 3.70% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9907` |
| 5 | 3.05% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 6 | 2.86% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 7 | 2.71% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 8 | 2.66% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 9 | 2.45% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 10 | 1.78% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 11 | 1.71% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4424` |
| 12 | 1.55% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 13 | 1.54% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 14 | 1.51% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7063` |
| 15 | 1.43% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 16 | 1.38% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 17 | 1.35% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3685` |
| 18 | 1.20% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 19 | 1.20% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 20 | 1.04% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 21 | 1.04% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 22 | 0.94% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6483` |
| 23 | 0.92% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 24 | 0.88% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7048` |
| 25 | 0.85% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5675` |
| 26 | 0.84% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 27 | 0.79% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4105` |
| 28 | 0.78% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 898` |
| 29 | 0.76% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 30 | 0.71% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |

GC pauses: 0, total 0 ms

