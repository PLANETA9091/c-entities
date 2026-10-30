# JVM hotspots (JFR ExecutionSample)

Samples: 31345

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.99% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.18% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8699` |
| 3 | 3.11% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8558` |
| 4 | 2.98% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5594` |
| 5 | 2.97% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.84% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 2.75% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 8 | 2.52% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 9 | 2.38% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 10 | 2.31% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 2.15% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8505` |
| 12 | 1.95% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5449` |
| 13 | 1.54% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 14 | 1.42% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 15 | 1.39% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 16 | 1.37% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 17 | 1.31% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 18 | 1.26% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4803` |
| 19 | 1.23% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 20 | 1.19% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 21 | 1.13% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 22 | 1.10% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 23 | 1.07% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 24 | 1.02% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 25 | 0.87% | `java.util.EnumMap.get(Object) line: 248` |
| 26 | 0.78% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 27 | 0.77% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 28 | 0.77% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 29 | 0.74% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 30 | 0.73% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 31 | 0.71% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3504` |
| 32 | 0.70% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 33 | 0.70% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 34 | 0.63% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 35 | 0.59% | `net.minecraft.world.entity.Entity.computeSpeed() line: 961` |
| 36 | 0.57% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 37 | 0.57% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 38 | 0.53% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 39 | 0.52% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 40 | 0.49% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 41 | 0.49% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8644` |
| 42 | 0.47% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 43 | 0.46% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 44 | 0.45% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3396` |
| 45 | 0.44% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 46 | 0.42% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3850` |
| 47 | 0.41% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 48 | 0.40% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 49 | 0.39% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 50 | 0.39% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 51 | 0.38% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 52 | 0.38% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8519` |
| 53 | 0.35% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 6611` |
| 54 | 0.35% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 55 | 0.33% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 56 | 0.32% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5596` |
| 57 | 0.32% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 58 | 0.32% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 59 | 0.30% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 60 | 0.29% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.30% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.49% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 3.23% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 3.18% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 5 | 2.84% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 2.76% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8554` |
| 7 | 2.37% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 8 | 2.32% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 9 | 2.09% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 10 | 2.06% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 11 | 1.99% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 12 | 1.75% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8571` |
| 13 | 1.58% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 14 | 1.54% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 15 | 1.49% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 16 | 1.45% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 17 | 1.24% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 18 | 1.20% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3584` |
| 19 | 1.19% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8546` |
| 20 | 1.11% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 21 | 1.10% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 22 | 1.09% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 23 | 0.96% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4904` |
| 24 | 0.93% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 25 | 0.87% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5596` |
| 26 | 0.84% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 27 | 0.82% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 28 | 0.79% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 29 | 0.79% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 30 | 0.77% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |

GC pauses: 0, total 0 ms

