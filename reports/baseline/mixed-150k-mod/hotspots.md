# JVM hotspots (JFR ExecutionSample)

Samples: 39485

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.16% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.58% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8271` |
| 3 | 3.30% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 2.84% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.79% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5166` |
| 6 | 2.74% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8130` |
| 7 | 2.02% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 1.81% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8077` |
| 9 | 1.80% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5021` |
| 10 | 1.59% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 11 | 1.30% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 12 | 1.29% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 1.29% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 14 | 1.22% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 15 | 1.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4448` |
| 16 | 1.14% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 17 | 1.10% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 18 | 1.04% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 19 | 0.98% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 20 | 0.97% | `java.util.EnumMap.get(Object) line: 248` |
| 21 | 0.94% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 22 | 0.92% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 993` |
| 23 | 0.91% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 24 | 0.86% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 25 | 0.84% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 26 | 0.83% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 27 | 0.73% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 28 | 0.73% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 29 | 0.72% | `java.util.ImmutableCollections$MapN$MapNIterator.nextIndex() line: 1533` |
| 30 | 0.69% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 31 | 0.68% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 32 | 0.66% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 33 | 0.64% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 34 | 0.62% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 35 | 0.52% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 36 | 0.52% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 37 | 0.52% | `net.minecraft.world.entity.Entity.computeSpeed() line: 961` |
| 38 | 0.52% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3143` |
| 39 | 0.51% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 40 | 0.51% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2256` |
| 41 | 0.50% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 42 | 0.50% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 43 | 0.49% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 44 | 0.48% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 45 | 0.48% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 46 | 0.46% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8216` |
| 47 | 0.45% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 48 | 0.45% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 49 | 0.44% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 50 | 0.43% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8091` |
| 51 | 0.42% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 52 | 0.42% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 53 | 0.40% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2275` |
| 54 | 0.40% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 55 | 0.39% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 56 | 0.39% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 57 | 0.39% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3489` |
| 58 | 0.38% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 59 | 0.38% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 60 | 0.38% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 8338` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.18% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.58% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 3 | 3.30% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 3.24% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 2.85% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 6 | 2.41% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8126` |
| 7 | 1.89% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 8 | 1.82% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 9 | 1.68% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 10 | 1.57% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 11 | 1.52% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 12 | 1.50% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 13 | 1.48% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 14 | 1.29% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 15 | 1.28% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8143` |
| 16 | 1.24% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 17 | 1.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4485` |
| 18 | 1.18% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1748` |
| 19 | 1.16% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 20 | 1.10% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 21 | 1.07% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8118` |
| 22 | 1.04% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 23 | 0.94% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 24 | 0.89% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 25 | 0.87% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 26 | 0.86% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 27 | 0.81% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 28 | 0.78% | `net.minecraft.world.entity.LivingEntity.detectEquipmentUpdates() line: 3558` |
| 29 | 0.76% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 30 | 0.75% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |

GC pauses: 0, total 0 ms

