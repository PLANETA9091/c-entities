# JVM hotspots (JFR ExecutionSample)

Samples: 24127

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.28% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 4.02% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10097` |
| 3 | 3.62% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6241` |
| 4 | 3.31% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 3.09% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 2.98% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 2.42% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 8 | 1.94% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 9 | 1.74% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 10 | 1.72% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 11 | 1.66% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 12 | 1.50% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 13 | 1.45% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9499` |
| 14 | 1.32% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 15 | 1.32% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 16 | 1.29% | `java.lang.ref.Reference.get() line: 361` |
| 17 | 1.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5473` |
| 18 | 1.14% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 19 | 0.94% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 20 | 0.94% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 21 | 0.93% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4054` |
| 22 | 0.90% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 23 | 0.82% | `java.util.EnumMap.get(Object) line: 248` |
| 24 | 0.82% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 25 | 0.74% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 26 | 0.73% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 27 | 0.70% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 10009` |
| 28 | 0.63% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 29 | 0.60% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 30 | 0.58% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6811` |
| 31 | 0.57% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 32 | 0.55% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 33 | 0.52% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 34 | 0.52% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 35 | 0.51% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 36 | 0.51% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 37 | 0.50% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 38 | 0.50% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 39 | 0.50% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 40 | 0.49% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 41 | 0.49% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 501` |
| 42 | 0.48% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4400` |
| 43 | 0.47% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 44 | 0.46% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7353` |
| 45 | 0.46% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 46 | 0.45% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.addAll(int, Collection) line: 762` |
| 47 | 0.44% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 48 | 0.43% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9517` |
| 49 | 0.42% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6743` |
| 50 | 0.41% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3927` |
| 51 | 0.41% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6251` |
| 52 | 0.40% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 53 | 0.39% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 54 | 0.37% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 10017` |
| 55 | 0.36% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 56 | 0.35% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4337` |
| 57 | 0.34% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 58 | 0.34% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 59 | 0.34% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 6421` |
| 60 | 0.34% | `net.minecraft.world.entity.Entity.computeSpeed() line: 961` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 8.36% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.38% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 4.03% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 4 | 2.98% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 2.96% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9652` |
| 6 | 2.61% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 7 | 2.59% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 8 | 2.53% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 9 | 2.03% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 10 | 2.01% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 11 | 1.94% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 12 | 1.79% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9642` |
| 13 | 1.53% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 14 | 1.46% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 15 | 1.40% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6826` |
| 16 | 1.33% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 17 | 1.32% | `net.minecraft.world.entity.Entity.isEyeInFluid(TagKey) line: 2311` |
| 18 | 1.14% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 19 | 1.08% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 20 | 1.03% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 21 | 0.98% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 22 | 0.97% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3960` |
| 23 | 0.90% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6251` |
| 24 | 0.88% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 25 | 0.87% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 26 | 0.82% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6811` |
| 27 | 0.75% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 6432` |
| 28 | 0.73% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 29 | 0.67% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 30 | 0.67% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 898` |

GC pauses: 0, total 0 ms

