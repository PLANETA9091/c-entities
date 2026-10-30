# JVM hotspots (JFR ExecutionSample)

Samples: 33891

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.35% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 4.30% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8643` |
| 3 | 4.07% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8837` |
| 4 | 2.95% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.68% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5732` |
| 6 | 2.54% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8696` |
| 7 | 2.46% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5587` |
| 8 | 2.41% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 9 | 1.91% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 10 | 1.86% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 11 | 1.76% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 1.65% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 13 | 1.56% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 14 | 1.49% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4941` |
| 15 | 1.36% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 16 | 1.21% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 17 | 1.20% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 18 | 1.19% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 19 | 1.15% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 20 | 1.14% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 21 | 1.08% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 22 | 1.05% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 23 | 0.92% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 24 | 0.87% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 25 | 0.84% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 26 | 0.82% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 27 | 0.78% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 28 | 0.74% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 29 | 0.73% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 30 | 0.71% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8782` |
| 31 | 0.61% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 32 | 0.60% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 33 | 0.57% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 34 | 0.57% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 35 | 0.52% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 36 | 0.51% | `java.util.EnumMap.get(Object) line: 248` |
| 37 | 0.49% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 389` |
| 38 | 0.49% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 39 | 0.48% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 40 | 0.48% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 41 | 0.48% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3642` |
| 42 | 0.48% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 43 | 0.46% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 44 | 0.45% | `java.lang.ref.Reference.get() line: 361` |
| 45 | 0.44% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3988` |
| 46 | 0.43% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 47 | 0.42% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 48 | 0.42% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getAttributesToSync() line: 34` |
| 49 | 0.42% | `java.util.Optional.ifPresent(Consumer) line: 178` |
| 50 | 0.41% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 51 | 0.40% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 52 | 0.39% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 53 | 0.37% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 54 | 0.37% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 55 | 0.37% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 56 | 0.36% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 57 | 0.36% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 58 | 0.35% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 59 | 0.35% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 60 | 0.35% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3925` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.91% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.52% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8692` |
| 3 | 4.07% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 4 | 3.12% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 2.70% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 6 | 2.48% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 7 | 1.90% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 8 | 1.86% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 1.81% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 10 | 1.69% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8709` |
| 11 | 1.53% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 12 | 1.52% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5042` |
| 13 | 1.47% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 14 | 1.44% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 15 | 1.31% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 16 | 1.28% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 17 | 1.21% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 18 | 1.16% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 19 | 1.09% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 20 | 1.08% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5587` |
| 21 | 1.03% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 6626` |
| 22 | 1.00% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 23 | 0.95% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 24 | 0.93% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 25 | 0.92% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 26 | 0.91% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 27 | 0.88% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 28 | 0.87% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 29 | 0.83% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3584` |
| 30 | 0.80% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |

GC pauses: 0, total 0 ms

