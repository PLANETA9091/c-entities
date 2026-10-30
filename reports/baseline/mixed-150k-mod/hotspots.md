# JVM hotspots (JFR ExecutionSample)

Samples: 29266

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.83% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 2 | 4.76% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8423` |
| 3 | 4.34% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 3.11% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8282` |
| 5 | 3.08% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5318` |
| 6 | 2.87% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.45% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 2.26% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8229` |
| 9 | 2.11% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5173` |
| 10 | 2.05% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 11 | 2.00% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 12 | 1.69% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 1.52% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 14 | 1.50% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 15 | 1.40% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 16 | 1.38% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 17 | 1.36% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 18 | 1.34% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4594` |
| 19 | 1.16% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 20 | 1.06% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 21 | 1.01% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 22 | 1.00% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 23 | 0.90% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 24 | 0.84% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 25 | 0.80% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 26 | 0.72% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 27 | 0.71% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 28 | 0.66% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3289` |
| 29 | 0.66% | `java.util.EnumMap.get(Object) line: 248` |
| 30 | 0.61% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 31 | 0.60% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 32 | 0.60% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 33 | 0.58% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 34 | 0.58% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 35 | 0.54% | `jdk.internal.util.ArraysSupport.hashCodeOfUnsigned(byte[], int, int, int) line: 275` |
| 36 | 0.54% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 37 | 0.54% | `net.minecraft.world.entity.Entity.computeSpeed() line: 961` |
| 38 | 0.49% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 39 | 0.49% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 40 | 0.47% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3635` |
| 41 | 0.46% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 42 | 0.46% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3181` |
| 43 | 0.46% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 44 | 0.46% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 45 | 0.45% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 46 | 0.45% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8243` |
| 47 | 0.42% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8368` |
| 48 | 0.40% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 49 | 0.39% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 6335` |
| 50 | 0.39% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 51 | 0.38% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 52 | 0.37% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 501` |
| 53 | 0.36% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 54 | 0.35% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 55 | 0.35% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 56 | 0.34% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 57 | 0.33% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3572` |
| 58 | 0.33% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 59 | 0.33% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 60 | 0.32% | `net.minecraft.core.Vec3i.getX() line: 63` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.79% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.77% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 3 | 3.68% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 3.24% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 2.98% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8278` |
| 6 | 2.39% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 7 | 2.26% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 8 | 2.20% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 9 | 2.15% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 10 | 2.05% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 11 | 1.99% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 12 | 1.77% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 13 | 1.72% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 14 | 1.62% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 15 | 1.62% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 16 | 1.24% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 17 | 1.21% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 18 | 1.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4631` |
| 19 | 1.10% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8270` |
| 20 | 1.07% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 21 | 1.07% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 22 | 1.03% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 23 | 0.92% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 24 | 0.89% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 25 | 0.86% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5320` |
| 26 | 0.82% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3584` |
| 27 | 0.79% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 28 | 0.74% | `net.minecraft.world.entity.LivingEntity.detectEquipmentUpdates() line: 3558` |
| 29 | 0.72% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5173` |
| 30 | 0.68% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |

GC pauses: 0, total 0 ms

