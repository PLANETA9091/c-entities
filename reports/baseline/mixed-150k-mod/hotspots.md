# JVM hotspots (JFR ExecutionSample)

Samples: 28474

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.19% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.55% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9269` |
| 3 | 3.51% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6166` |
| 4 | 3.31% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9146` |
| 5 | 2.95% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.88% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 2.71% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 2.41% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 9 | 2.27% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9605` |
| 10 | 1.94% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 11 | 1.64% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 12 | 1.63% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 13 | 1.57% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 14 | 1.56% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 15 | 1.56% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5427` |
| 16 | 1.45% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 17 | 1.40% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 18 | 1.38% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4735` |
| 19 | 1.30% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 20 | 1.17% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 21 | 1.11% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 22 | 1.08% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 23 | 0.90% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 24 | 0.88% | `java.util.EnumMap.get(Object) line: 248` |
| 25 | 0.78% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 26 | 0.77% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 27 | 0.74% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9183` |
| 28 | 0.71% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 29 | 0.70% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4020` |
| 30 | 0.65% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 31 | 0.63% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 32 | 0.59% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 33 | 0.58% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9228` |
| 34 | 0.57% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 35 | 0.57% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 36 | 0.52% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 37 | 0.52% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 38 | 0.50% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4366` |
| 39 | 0.48% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 40 | 0.47% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3893` |
| 41 | 0.47% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 9558` |
| 42 | 0.46% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 43 | 0.46% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 44 | 0.46% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 45 | 0.45% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 501` |
| 46 | 0.44% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 47 | 0.44% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 9550` |
| 48 | 0.42% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6179` |
| 49 | 0.41% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 50 | 0.41% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 51 | 0.41% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 52 | 0.39% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 53 | 0.38% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 54 | 0.38% | `net.minecraft.world.entity.ai.control.LookControl.getYRotD() line: 102` |
| 55 | 0.37% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 56 | 0.37% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7196` |
| 57 | 0.37% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 58 | 0.35% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 59 | 0.34% | `net.minecraft.world.entity.Entity.computeSpeed() line: 961` |
| 60 | 0.33% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4303` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.90% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 5.04% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9265` |
| 3 | 4.45% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 3.62% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 2.88% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 2.42% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 7 | 2.41% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 8 | 2.28% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 9 | 2.27% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 10 | 2.05% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 11 | 2.02% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 12 | 1.89% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 13 | 1.57% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 14 | 1.45% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 15 | 1.44% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5465` |
| 16 | 1.25% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 17 | 1.22% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 18 | 1.14% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9257` |
| 19 | 1.11% | `net.minecraft.world.entity.Entity.isEyeInFluid(TagKey) line: 2311` |
| 20 | 1.09% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 21 | 1.01% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 22 | 1.00% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 23 | 0.95% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 24 | 0.92% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 25 | 0.91% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 26 | 0.88% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6176` |
| 27 | 0.78% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2195` |
| 28 | 0.78% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 29 | 0.76% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 30 | 0.74% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |

GC pauses: 0, total 0 ms

