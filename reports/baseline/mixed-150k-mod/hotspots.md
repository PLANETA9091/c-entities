# JVM hotspots (JFR ExecutionSample)

Samples: 29910

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.84% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8933` |
| 2 | 3.33% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.17% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9127` |
| 4 | 3.15% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 2.76% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 6 | 2.72% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6009` |
| 7 | 2.43% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 2.29% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5855` |
| 9 | 2.18% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8986` |
| 10 | 1.44% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.42% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 12 | 1.30% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 1.23% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5273` |
| 14 | 1.21% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 15 | 1.20% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 16 | 1.11% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 17 | 1.07% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 18 | 0.97% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 19 | 0.95% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 20 | 0.88% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 21 | 0.88% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 22 | 0.85% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 23 | 0.80% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 24 | 0.79% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 25 | 0.74% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 26 | 0.69% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 27 | 0.63% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6019` |
| 28 | 0.62% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 29 | 0.60% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 30 | 0.57% | `java.util.EnumMap.get(Object) line: 248` |
| 31 | 0.55% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 32 | 0.54% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 33 | 0.51% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 34 | 0.50% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 35 | 0.50% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 36 | 0.48% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 37 | 0.48% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 38 | 0.48% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 9072` |
| 39 | 0.48% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 40 | 0.47% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 41 | 0.47% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 42 | 0.46% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 43 | 0.46% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3866` |
| 44 | 0.46% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 45 | 0.45% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 46 | 0.44% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 47 | 0.44% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 48 | 0.43% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 49 | 0.39% | `java.lang.ref.Reference.get() line: 361` |
| 50 | 0.38% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1466` |
| 51 | 0.38% | `jdk.internal.util.ArraysSupport.hashCodeOfUnsigned(byte[], int, int, int) line: 275` |
| 52 | 0.38% | `net.minecraft.world.entity.Mob.serverAiStep() line: 860` |
| 53 | 0.38% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 54 | 0.37% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 55 | 0.37% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 56 | 0.37% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 57 | 0.35% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 58 | 0.35% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 59 | 0.35% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getAttributesToSync() line: 34` |
| 60 | 0.35% | `net.minecraft.world.entity.Entity.getType() line: 633` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.77% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.18% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8982` |
| 3 | 3.62% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 3.18% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 5 | 3.15% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 2.36% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 7 | 2.32% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 8 | 1.91% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5310` |
| 9 | 1.84% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 10 | 1.66% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 11 | 1.50% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 12 | 1.44% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8999` |
| 13 | 1.40% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 14 | 1.32% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 15 | 1.21% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5855` |
| 16 | 1.16% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 17 | 1.09% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 18 | 1.03% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3584` |
| 19 | 1.00% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 20 | 0.98% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 21 | 0.86% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 22 | 0.84% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 23 | 0.84% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 24 | 0.82% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 25 | 0.68% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8974` |
| 26 | 0.66% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 27 | 0.65% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 28 | 0.65% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 226` |
| 29 | 0.64% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 30 | 0.61% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 6916` |

GC pauses: 0, total 0 ms

