# JVM hotspots (JFR ExecutionSample)

Samples: 34449

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.57% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 2.78% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7778` |
| 3 | 2.75% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4838` |
| 4 | 2.75% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.74% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 6 | 2.25% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 2.10% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 1.99% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 9 | 1.97% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4693` |
| 10 | 1.84% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7725` |
| 11 | 1.77% | `java.util.EnumMap$EnumMapIterator.hasNext() line: 524` |
| 12 | 1.50% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 13 | 1.48% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 14 | 1.43% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 15 | 1.34% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 16 | 1.31% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 17 | 1.31% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 18 | 1.25% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 19 | 1.13% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4120` |
| 20 | 1.11% | `java.util.EnumMap.get(Object) line: 248` |
| 21 | 1.10% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 22 | 1.09% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 23 | 1.02% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 24 | 1.00% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 25 | 0.91% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 26 | 0.77% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 27 | 0.77% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 28 | 0.72% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 29 | 0.71% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 30 | 0.71% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 31 | 0.70% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 32 | 0.64% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2816` |
| 33 | 0.64% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 34 | 0.63% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 35 | 0.58% | `net.minecraft.world.entity.Entity.computeSpeed() line: 961` |
| 36 | 0.56% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 37 | 0.55% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 38 | 0.54% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 39 | 0.53% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7864` |
| 40 | 0.51% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3696` |
| 41 | 0.51% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 42 | 0.51% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 43 | 0.50% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 44 | 0.44% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 45 | 0.44% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7739` |
| 46 | 0.42% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 47 | 0.42% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 48 | 0.40% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 5855` |
| 49 | 0.39% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 50 | 0.39% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 51 | 0.38% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 52 | 0.37% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 53 | 0.37% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 54 | 0.36% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2718` |
| 55 | 0.35% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 56 | 0.34% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 57 | 0.33% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3162` |
| 58 | 0.32% | `java.lang.ref.Reference.get() line: 361` |
| 59 | 0.32% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 60 | 0.31% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3099` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.81% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.24% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 2.89% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 2.74% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 5 | 2.48% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7774` |
| 6 | 2.11% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 7 | 2.10% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 1.99% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 9 | 1.78% | `net.minecraft.world.entity.EntityEquipment.tick(Entity) line: 50` |
| 10 | 1.75% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 11 | 1.66% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 12 | 1.54% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 13 | 1.50% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7791` |
| 14 | 1.40% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 15 | 1.31% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 16 | 1.31% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 17 | 1.26% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 18 | 1.22% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 19 | 1.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4157` |
| 20 | 1.16% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 21 | 1.11% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 22 | 1.02% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7766` |
| 23 | 0.93% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 24 | 0.89% | `net.minecraft.world.entity.LivingEntity.detectEquipmentUpdates() line: 3558` |
| 25 | 0.85% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 26 | 0.80% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 27 | 0.77% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 28 | 0.76% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4693` |
| 29 | 0.76% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4840` |
| 30 | 0.76% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |

GC pauses: 0, total 0 ms

