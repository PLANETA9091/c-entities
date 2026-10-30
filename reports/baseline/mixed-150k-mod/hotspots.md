# JVM hotspots (JFR ExecutionSample)

Samples: 32899

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.71% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8572` |
| 2 | 4.67% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 3 | 4.29% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 3.13% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 3.10% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 2.95% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8431` |
| 7 | 2.85% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5467` |
| 8 | 2.46% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 2.15% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8378` |
| 10 | 2.09% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5322` |
| 11 | 1.78% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 12 | 1.47% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4676` |
| 13 | 1.46% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 14 | 1.40% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 15 | 1.39% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 16 | 1.35% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 17 | 1.31% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 18 | 1.30% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 19 | 1.13% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 20 | 1.11% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 21 | 1.05% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 22 | 0.96% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 23 | 0.90% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 24 | 0.88% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 993` |
| 25 | 0.86% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 26 | 0.81% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 27 | 0.78% | `java.util.EnumMap.get(Object) line: 248` |
| 28 | 0.64% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 29 | 0.63% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 30 | 0.61% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 31 | 0.59% | `jdk.internal.util.ArraysSupport.hashCodeOfUnsigned(byte[], int, int, int) line: 275` |
| 32 | 0.58% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3377` |
| 33 | 0.55% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 34 | 0.55% | `net.minecraft.world.entity.Entity.computeSpeed() line: 961` |
| 35 | 0.53% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 36 | 0.53% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 37 | 0.51% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 38 | 0.48% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 39 | 0.47% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8392` |
| 40 | 0.46% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 41 | 0.46% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 42 | 0.45% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 43 | 0.44% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 44 | 0.44% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3723` |
| 45 | 0.43% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 46 | 0.43% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 47 | 0.42% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 48 | 0.41% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 49 | 0.40% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 374` |
| 50 | 0.40% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 51 | 0.39% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8525` |
| 52 | 0.38% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3269` |
| 53 | 0.36% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 54 | 0.36% | `it.unimi.dsi.fastutil.objects.Object2ObjectArrayMap.get(Object) line: 407` |
| 55 | 0.35% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 6484` |
| 56 | 0.34% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 8074` |
| 57 | 0.34% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 501` |
| 58 | 0.34% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 59 | 0.34% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 60 | 0.34% | `net.minecraft.core.Vec3i.getX() line: 63` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.75% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.71% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 3 | 3.40% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 3.10% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 3.10% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 2.88% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8427` |
| 7 | 2.18% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 8 | 2.16% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 9 | 2.12% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 10 | 1.95% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 11 | 1.78% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 12 | 1.73% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 13 | 1.65% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 14 | 1.65% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 15 | 1.63% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 16 | 1.33% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 17 | 1.30% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 18 | 1.16% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 19 | 1.11% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8419` |
| 20 | 1.06% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 21 | 1.01% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4777` |
| 22 | 0.98% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 23 | 0.94% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 24 | 0.90% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 25 | 0.90% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 26 | 0.88% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3584` |
| 27 | 0.86% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 28 | 0.83% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5469` |
| 29 | 0.81% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5322` |
| 30 | 0.69% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |

GC pauses: 0, total 0 ms

