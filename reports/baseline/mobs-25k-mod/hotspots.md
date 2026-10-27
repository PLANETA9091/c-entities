# JVM hotspots (JFR ExecutionSample)

Samples: 15464

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.71% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.34% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 2.37% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 1.91% | `java.util.EnumMap.get(Object) line: 248` |
| 5 | 1.67% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 6 | 1.45% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 1.44% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 8 | 1.30% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7511` |
| 9 | 1.18% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 10 | 1.16% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4571` |
| 11 | 1.14% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 12 | 1.09% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 13 | 1.03% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 14 | 0.96% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 15 | 0.94% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 16 | 0.94% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 17 | 0.93% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7458` |
| 18 | 0.89% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 19 | 0.89% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1745` |
| 20 | 0.83% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 21 | 0.82% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 22 | 0.77% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4426` |
| 23 | 0.73% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 603` |
| 24 | 0.72% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 25 | 0.71% | `net.minecraft.world.phys.Vec3.subtract(Vec3) line: 103` |
| 26 | 0.71% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 27 | 0.70% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 28 | 0.70% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 29 | 0.70% | `net.minecraft.world.level.levelgen.BitRandomSource.nextInt(int) line: 31` |
| 30 | 0.68% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 31 | 0.67% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 2914` |
| 32 | 0.66% | `dev.planeta9091.centities.CeHooks.aiMeleeRunning(CeHooks$AiCtx) line: 6741` |
| 33 | 0.66% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7597` |
| 34 | 0.65% | `dev.planeta9091.centities.CeHooks.aiPackSel(int[], int, WrappedGoal[], int[], int[], long[], int[], int[], int[], float[], Mob) line: 6545` |
| 35 | 0.63% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 36 | 0.62% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 37 | 0.62% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 2851` |
| 38 | 0.60% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 39 | 0.55% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 40 | 0.48% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 5588` |
| 41 | 0.48% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 42 | 0.47% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3696` |
| 43 | 0.47% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 7719` |
| 44 | 0.47% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 45 | 0.47% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 46 | 0.47% | `sun.misc.Unsafe.getLong(Object, long) line: 357` |
| 47 | 0.47% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 48 | 0.45% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 49 | 0.45% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3579` |
| 50 | 0.45% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 51 | 0.45% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 3024` |
| 52 | 0.44% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 53 | 0.41% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 54 | 0.41% | `jdk.internal.util.ArraysSupport.hashCodeOfUnsigned(byte[], int, int, int) line: 275` |
| 55 | 0.39% | `it.unimi.dsi.fastutil.objects.Object2IntAVLTreeMap.isEmpty() line: 942` |
| 56 | 0.38% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 57 | 0.38% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 58 | 0.37% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 59 | 0.37% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1764` |
| 60 | 0.36% | `java.lang.String.hashCode() line: 2430` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.79% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.71% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 2.07% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 1.73% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1316` |
| 5 | 1.71% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 6 | 1.60% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 7 | 1.58% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7524` |
| 8 | 1.47% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 9 | 1.47% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 10 | 1.38% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 11 | 1.36% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 12 | 1.29% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 13 | 1.14% | `com.google.common.collect.MapMakerInternalMap$Segment.getEntry(Object, int) line: 1415` |
| 14 | 1.11% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 15 | 1.02% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7507` |
| 16 | 0.98% | `dev.planeta9091.centities.CeHooks.mobAiFloor(Mob) line: 6699` |
| 17 | 0.94% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 18 | 0.91% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 19 | 0.88% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1749` |
| 20 | 0.85% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 21 | 0.83% | `net.minecraft.world.item.enchantment.ItemEnchantments.entrySet() line: 101` |
| 22 | 0.82% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 23 | 0.80% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 24 | 0.80% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 25 | 0.73% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 3158` |
| 26 | 0.73% | `java.util.concurrent.atomic.AtomicReferenceArray.get(int) line: 103` |
| 27 | 0.72% | `dev.planeta9091.centities.CeHooks.aiPack(Mob, CeHooks$AiCtx, int, int, int, double, double, double) line: 6530` |
| 28 | 0.72% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2195` |
| 29 | 0.71% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 30 | 0.71% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |

GC pauses: 0, total 0 ms

