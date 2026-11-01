# JVM hotspots (JFR ExecutionSample)

Samples: 31692

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.31% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 4.17% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9080` |
| 3 | 3.20% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5962` |
| 4 | 2.91% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8939` |
| 5 | 2.88% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 6 | 2.80% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 2.80% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 2.53% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8886` |
| 9 | 2.41% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 2.33% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 11 | 2.11% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5808` |
| 12 | 1.95% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 13 | 1.50% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 14 | 1.49% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 15 | 1.43% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 16 | 1.28% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 17 | 1.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5226` |
| 18 | 1.17% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 19 | 1.15% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 20 | 1.03% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 21 | 0.96% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 22 | 0.93% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 23 | 0.82% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 24 | 0.81% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 25 | 0.80% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 26 | 0.77% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 27 | 0.74% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3819` |
| 28 | 0.74% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 29 | 0.74% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 30 | 0.73% | `java.util.EnumMap.get(Object) line: 248` |
| 31 | 0.72% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 32 | 0.67% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 33 | 0.59% | `net.minecraft.world.entity.Entity.computeSpeed() line: 961` |
| 34 | 0.58% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 35 | 0.54% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 36 | 0.52% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 37 | 0.52% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 38 | 0.51% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 39 | 0.50% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 40 | 0.49% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 41 | 0.47% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 42 | 0.46% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 43 | 0.44% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 44 | 0.44% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 45 | 0.42% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 46 | 0.41% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 9025` |
| 47 | 0.41% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 48 | 0.40% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 49 | 0.39% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3692` |
| 50 | 0.39% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 51 | 0.39% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4165` |
| 52 | 0.38% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 53 | 0.37% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 54 | 0.37% | `java.lang.String.equals(Object) line: 1926` |
| 55 | 0.37% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 9033` |
| 56 | 0.36% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 501` |
| 57 | 0.36% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 58 | 0.36% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 59 | 0.35% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5972` |
| 60 | 0.35% | `net.minecraft.world.entity.Entity.getType() line: 633` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.73% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.19% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 3 | 3.80% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 3.05% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 3.04% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8935` |
| 6 | 2.80% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 2.28% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 8 | 2.16% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 9 | 1.95% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 10 | 1.92% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 11 | 1.85% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 12 | 1.63% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 13 | 1.59% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 14 | 1.51% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 15 | 1.28% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8952` |
| 16 | 1.27% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 17 | 1.24% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 18 | 1.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5263` |
| 19 | 1.15% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 20 | 1.03% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 21 | 1.02% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8927` |
| 22 | 0.96% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 23 | 0.91% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 24 | 0.90% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 25 | 0.88% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 26 | 0.85% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 27 | 0.82% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 28 | 0.79% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 29 | 0.77% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3725` |
| 30 | 0.77% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5972` |

GC pauses: 0, total 0 ms

