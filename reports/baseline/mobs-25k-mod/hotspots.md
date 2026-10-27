# JVM hotspots (JFR ExecutionSample)

Samples: 13477

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.78% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 3.08% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 2.62% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 6972` |
| 4 | 2.29% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 2.16% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1412` |
| 6 | 1.95% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 7 | 1.83% | `java.util.EnumMap.get(Object) line: 248` |
| 8 | 1.51% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 6923` |
| 9 | 1.41% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 10 | 1.35% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 11 | 1.28% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.22% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 13 | 1.16% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 14 | 1.05% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 15 | 1.04% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 16 | 1.02% | `dev.planeta9091.centities.CeHooks.setSharedFlag$(Entity, int, boolean) line: 6759` |
| 17 | 1.02% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4203` |
| 18 | 0.96% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 2565` |
| 19 | 0.93% | `dev.planeta9091.centities.CeHooks.aiPackSel(int[], int, WrappedGoal[], int[], int[], long[], int[], int[], int[], float[], Mob) line: 6079` |
| 20 | 0.93% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 21 | 0.88% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4058` |
| 22 | 0.88% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 23 | 0.86% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 24 | 0.85% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 25 | 0.82% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 26 | 0.82% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 2502` |
| 27 | 0.76% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 28 | 0.74% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 29 | 0.73% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 30 | 0.73% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 31 | 0.70% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 32 | 0.70% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6870` |
| 33 | 0.68% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 34 | 0.64% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 35 | 0.62% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 603` |
| 36 | 0.62% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 37 | 0.59% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 5127` |
| 38 | 0.59% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 39 | 0.58% | `dev.planeta9091.centities.CeHooks.aiMeleeRunning(CeHooks$AiCtx) line: 6275` |
| 40 | 0.55% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 41 | 0.54% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 42 | 0.52% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 43 | 0.51% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 2675` |
| 44 | 0.49% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 45 | 0.48% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 46 | 0.48% | `sun.misc.Unsafe.getLong(Object, long) line: 357` |
| 47 | 0.47% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 48 | 0.47% | `dev.planeta9091.centities.CeHooks.disabledBacking(GoalSelector) line: 4477` |
| 49 | 0.46% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 50 | 0.42% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 51 | 0.41% | `java.lang.String.hashCode() line: 2430` |
| 52 | 0.40% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 53 | 0.39% | `dev.planeta9091.centities.CeHooks.aiClassify(Mob) line: 5808` |
| 54 | 0.36% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 55 | 0.36% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 56 | 0.36% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 57 | 0.35% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 58 | 0.35% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 59 | 0.34% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 60 | 0.33% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 3.57% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.22% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1065` |
| 3 | 3.08% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 2.62% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean)` |
| 5 | 2.05% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 6 | 1.85% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 7 | 1.71% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 8 | 1.65% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 9 | 1.51% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 10 | 1.47% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 11 | 1.46% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 12 | 1.42% | `com.google.common.collect.MapMakerInternalMap$Segment.getEntry(Object, int) line: 1415` |
| 13 | 1.25% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6870` |
| 14 | 1.20% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 15 | 1.19% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 16 | 1.16% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1416` |
| 17 | 1.16% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 18 | 1.05% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 19 | 1.05% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 20 | 1.02% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean)` |
| 21 | 0.98% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 22 | 0.92% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 23 | 0.85% | `net.minecraft.world.item.enchantment.ItemEnchantments.entrySet() line: 101` |
| 24 | 0.85% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 25 | 0.83% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 26 | 0.82% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 27 | 0.79% | `dev.planeta9091.centities.CeHooks.mobAiFloor(Mob) line: 6233` |
| 28 | 0.79% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 29 | 0.79% | `dev.planeta9091.centities.CeHooks.aiPack(Mob, CeHooks$AiCtx, int, int, int, double, double, double) line: 6064` |
| 30 | 0.76% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |

GC pauses: 0, total 0 ms

