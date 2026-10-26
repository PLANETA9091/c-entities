# JVM hotspots (JFR ExecutionSample)

Samples: 15421

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.72% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.16% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 2.77% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1411` |
| 4 | 2.22% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 6633` |
| 5 | 2.18% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 6 | 1.88% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 7 | 1.71% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 8 | 1.66% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 6584` |
| 9 | 1.47% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 10 | 1.45% | `dev.planeta9091.centities.CeHooks.aiPackSel(int[], int, WrappedGoal[], int[], int[], long[], int[], float[], Mob) line: 5906` |
| 11 | 1.45% | `java.util.EnumMap.get(Object) line: 248` |
| 12 | 1.24% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1430` |
| 13 | 1.21% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 14 | 1.19% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 15 | 1.16% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 16 | 1.12% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 17 | 1.10% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 18 | 1.01% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 19 | 0.97% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4202` |
| 20 | 0.88% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 21 | 0.86% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 22 | 0.86% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 23 | 0.84% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 603` |
| 24 | 0.82% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 25 | 0.77% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 26 | 0.77% | `java.util.HashMap.getNode(Object) line: 586` |
| 27 | 0.76% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 2564` |
| 28 | 0.73% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 29 | 0.73% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 30 | 0.71% | `dev.planeta9091.centities.CeHooks.setSharedFlag$(Entity, int, boolean) line: 6420` |
| 31 | 0.70% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4057` |
| 32 | 0.69% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 2501` |
| 33 | 0.67% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 5126` |
| 34 | 0.67% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 35 | 0.65% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 36 | 0.62% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6531` |
| 37 | 0.53% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 38 | 0.50% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 39 | 0.50% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 40 | 0.49% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 41 | 0.49% | `dev.planeta9091.centities.CeHooks.disabledBacking(GoalSelector) line: 4476` |
| 42 | 0.49% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 43 | 0.48% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 44 | 0.47% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 45 | 0.47% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 46 | 0.47% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 47 | 0.46% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 48 | 0.46% | `java.lang.String.hashCode() line: 2430` |
| 49 | 0.43% | `jdk.internal.util.ArraysSupport.hashCodeOfUnsigned(byte[], int, int, int) line: 275` |
| 50 | 0.40% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 51 | 0.38% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 2674` |
| 52 | 0.38% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 53 | 0.37% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 54 | 0.36% | `ca.spottedleaf.moonrise.patches.starlight.light.SWMRNibbleArray.getVisible(int) line: 412` |
| 55 | 0.32% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 4380` |
| 56 | 0.31% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 57 | 0.31% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 58 | 0.31% | `com.google.common.collect.MapMakerInternalMap$Segment.getEntry(Object, int) line: 1416` |
| 59 | 0.30% | `dev.planeta9091.centities.CeHooks.aiClassify(Mob) line: 5734` |
| 60 | 0.30% | `dev.planeta9091.centities.CeHooks.aiPack(Mob, CeHooks$AiCtx, int, int, int, double, double, double) line: 5874` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.00% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1064` |
| 2 | 4.72% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 2.33% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 2.22% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean)` |
| 5 | 2.18% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 6 | 1.86% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1415` |
| 7 | 1.84% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 8 | 1.67% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 9 | 1.67% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 10 | 1.56% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 11 | 1.36% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 12 | 1.34% | `dev.planeta9091.centities.CeHooks.aiPack(Mob, CeHooks$AiCtx, int, int, int, double, double, double) line: 5893` |
| 13 | 1.28% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 14 | 1.22% | `com.google.common.collect.MapMakerInternalMap$Segment.getEntry(Object, int) line: 1415` |
| 15 | 1.19% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 16 | 1.17% | `dev.planeta9091.centities.CeHooks.aiRun(Mob, CeHooks$AiCtx) line: 6051` |
| 17 | 1.12% | `dev.planeta9091.centities.CeHooks.aiPack(Mob, CeHooks$AiCtx, int, int, int, double, double, double) line: 5891` |
| 18 | 1.04% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 19 | 0.95% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6531` |
| 20 | 0.91% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 21 | 0.91% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 22 | 0.88% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 23 | 0.84% | `java.util.concurrent.atomic.AtomicReferenceArray.get(int) line: 103` |
| 24 | 0.82% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 25 | 0.79% | `dev.planeta9091.centities.CeHooks.aiRun(Mob, CeHooks$AiCtx) line: 6004` |
| 26 | 0.77% | `java.util.HashMap.containsKey(Object) line: 603` |
| 27 | 0.75% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 28 | 0.73% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 29 | 0.73% | `net.minecraft.world.entity.ai.sensing.Sensing.tick()` |
| 30 | 0.72% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4007` |

GC pauses: 0, total 0 ms

