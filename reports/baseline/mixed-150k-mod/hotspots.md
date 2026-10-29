# JVM hotspots (JFR ExecutionSample)

Samples: 31394

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.47% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8383` |
| 2 | 4.05% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.21% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 4 | 3.03% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 3.00% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8242` |
| 6 | 2.94% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5278` |
| 7 | 2.75% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 2.59% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 2.16% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8189` |
| 10 | 2.14% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5133` |
| 11 | 1.92% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 1.70% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 13 | 1.65% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 14 | 1.47% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 15 | 1.34% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 16 | 1.28% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 17 | 1.22% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 18 | 1.15% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 19 | 1.13% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 20 | 1.04% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4560` |
| 21 | 1.02% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 22 | 0.95% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 23 | 0.89% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 24 | 0.83% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 25 | 0.82% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 26 | 0.78% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 27 | 0.78% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 28 | 0.74% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 29 | 0.71% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 30 | 0.68% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 31 | 0.68% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3255` |
| 32 | 0.66% | `java.lang.String.hashCode() line: 2433` |
| 33 | 0.62% | `java.util.EnumMap.get(Object) line: 248` |
| 34 | 0.60% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8328` |
| 35 | 0.60% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 36 | 0.59% | `net.minecraft.world.entity.Entity.computeSpeed() line: 961` |
| 37 | 0.59% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 38 | 0.55% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 39 | 0.52% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 40 | 0.49% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 41 | 0.49% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8203` |
| 42 | 0.49% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 43 | 0.46% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 44 | 0.46% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 45 | 0.46% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 46 | 0.46% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 47 | 0.44% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 48 | 0.42% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 6295` |
| 49 | 0.40% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3601` |
| 50 | 0.39% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 51 | 0.38% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3147` |
| 52 | 0.37% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 53 | 0.37% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 501` |
| 54 | 0.37% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 55 | 0.36% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 56 | 0.35% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 57 | 0.35% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 993` |
| 58 | 0.33% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 59 | 0.33% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 60 | 0.33% | `net.minecraft.core.Vec3i.getX() line: 63` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.64% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.47% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 3 | 3.48% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 3.13% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 2.86% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8238` |
| 6 | 2.75% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 2.39% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 8 | 2.19% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 9 | 2.18% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 10 | 2.08% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 11 | 1.70% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 12 | 1.58% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8255` |
| 13 | 1.58% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 14 | 1.56% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 15 | 1.55% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3584` |
| 16 | 1.50% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 17 | 1.25% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 18 | 1.20% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 19 | 1.18% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 20 | 1.15% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 21 | 1.11% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8230` |
| 22 | 1.02% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 23 | 0.89% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 24 | 0.86% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 25 | 0.84% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4597` |
| 26 | 0.82% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5133` |
| 27 | 0.80% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 28 | 0.78% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 29 | 0.78% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 30 | 0.76% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |

GC pauses: 0, total 0 ms

