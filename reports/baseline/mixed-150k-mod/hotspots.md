# JVM hotspots (JFR ExecutionSample)

Samples: 29299

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.76% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8439` |
| 2 | 3.61% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.59% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8633` |
| 4 | 2.84% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 5 | 2.61% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.31% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5528` |
| 7 | 2.31% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5383` |
| 8 | 2.21% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8492` |
| 9 | 1.86% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 1.82% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 11 | 1.53% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 12 | 1.46% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 13 | 1.44% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 14 | 1.37% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4737` |
| 15 | 1.28% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 16 | 1.21% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 17 | 1.16% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 18 | 1.14% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 19 | 1.07% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 20 | 1.06% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 21 | 1.01% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 22 | 0.89% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 23 | 0.89% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 24 | 0.86% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 25 | 0.85% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 26 | 0.77% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5530` |
| 27 | 0.72% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 28 | 0.71% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 29 | 0.62% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 30 | 0.60% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 31 | 0.58% | `java.util.EnumMap.get(Object) line: 248` |
| 32 | 0.56% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 33 | 0.55% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 34 | 0.53% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 35 | 0.52% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 36 | 0.51% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 37 | 0.51% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 38 | 0.50% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 39 | 0.49% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 40 | 0.49% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 41 | 0.48% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8578` |
| 42 | 0.47% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 43 | 0.47% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 44 | 0.46% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 374` |
| 45 | 0.45% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 46 | 0.44% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 47 | 0.41% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 48 | 0.40% | `java.util.Optional.ifPresent(Consumer) line: 178` |
| 49 | 0.40% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 50 | 0.40% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5533` |
| 51 | 0.40% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 52 | 0.40% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3330` |
| 53 | 0.39% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 54 | 0.39% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 55 | 0.39% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 56 | 0.38% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3438` |
| 57 | 0.38% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3784` |
| 58 | 0.38% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 59 | 0.34% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 60 | 0.34% | `net.minecraft.world.entity.ai.control.LookControl.getYRotD() line: 102` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.05% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.01% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8488` |
| 3 | 3.59% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 4 | 3.56% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 2.36% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 6 | 2.34% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 7 | 1.86% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 1.77% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4838` |
| 9 | 1.73% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 10 | 1.70% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 11 | 1.67% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8505` |
| 12 | 1.61% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 13 | 1.51% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 14 | 1.35% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 15 | 1.19% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 16 | 1.18% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 17 | 1.16% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5383` |
| 18 | 1.08% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 19 | 1.02% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3584` |
| 20 | 1.00% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 21 | 0.98% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 22 | 0.94% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 23 | 0.90% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 24 | 0.89% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 25 | 0.86% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 26 | 0.85% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 27 | 0.78% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 28 | 0.76% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 6422` |
| 29 | 0.73% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 30 | 0.70% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 226` |

GC pauses: 0, total 0 ms

