# JVM hotspots (JFR ExecutionSample)

Samples: 12870

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.17% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 2.92% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 2.75% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 2.20% | `java.util.EnumMap.get(Object) line: 248` |
| 5 | 2.07% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 6 | 2.05% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1445` |
| 7 | 1.85% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7317` |
| 8 | 1.77% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 9 | 1.45% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7176` |
| 10 | 1.43% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.21% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 12 | 1.16% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 13 | 1.09% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 14 | 0.96% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 15 | 0.96% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4091` |
| 16 | 0.93% | `dev.planeta9091.centities.CeHooks.aiPackSel(int[], int, WrappedGoal[], int[], int[], long[], int[], int[], int[], float[], Mob) line: 6210` |
| 17 | 0.92% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 18 | 0.88% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 19 | 0.86% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7123` |
| 20 | 0.84% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 5253` |
| 21 | 0.83% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 22 | 0.82% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 23 | 0.78% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 24 | 0.76% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 25 | 0.75% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 26 | 0.74% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 27 | 0.74% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 28 | 0.71% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 2598` |
| 29 | 0.70% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 30 | 0.69% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 31 | 0.68% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 32 | 0.68% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7262` |
| 33 | 0.67% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4236` |
| 34 | 0.66% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 35 | 0.65% | `dev.planeta9091.centities.CeHooks.aiMeleeRunning(CeHooks$AiCtx) line: 6406` |
| 36 | 0.64% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 2535` |
| 37 | 0.62% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 38 | 0.60% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 2708` |
| 39 | 0.59% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 603` |
| 40 | 0.57% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 41 | 0.55% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 42 | 0.54% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 43 | 0.52% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 44 | 0.49% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 45 | 0.48% | `sun.misc.Unsafe.getLong(Object, long) line: 357` |
| 46 | 0.47% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 47 | 0.47% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 7384` |
| 48 | 0.47% | `dev.planeta9091.centities.CeHooks.aiTail(Mob) line: 6697` |
| 49 | 0.45% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 50 | 0.45% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 51 | 0.45% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 52 | 0.44% | `com.google.common.collect.MapMakerInternalMap$Segment.getEntry(Object, int) line: 1416` |
| 53 | 0.44% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 54 | 0.43% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 55 | 0.42% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 56 | 0.41% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 57 | 0.40% | `dev.planeta9091.centities.CeHooks.aiClassify(Mob) line: 5939` |
| 58 | 0.40% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 59 | 0.37% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 60 | 0.36% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.60% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 2.99% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1067` |
| 3 | 2.92% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 2.11% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 5 | 1.85% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 6 | 1.85% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 7 | 1.81% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7189` |
| 8 | 1.58% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 9 | 1.55% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 10 | 1.46% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 11 | 1.43% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 12 | 1.31% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 13 | 1.21% | `com.google.common.collect.MapMakerInternalMap$Segment.getEntry(Object, int) line: 1415` |
| 14 | 1.20% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 15 | 1.09% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1449` |
| 16 | 1.00% | `net.minecraft.world.entity.ai.sensing.Sensing.tick()` |
| 17 | 1.00% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 18 | 0.98% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 19 | 0.94% | `dev.planeta9091.centities.CeHooks.mobAiFloor(Mob) line: 6364` |
| 20 | 0.92% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7172` |
| 21 | 0.90% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 22 | 0.87% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 23 | 0.86% | `dev.planeta9091.centities.CeHooks.aiPack(Mob, CeHooks$AiCtx, int, int, int, double, double, double) line: 6195` |
| 24 | 0.86% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 25 | 0.86% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 2842` |
| 26 | 0.84% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 27 | 0.79% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 28 | 0.78% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 29 | 0.75% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 30 | 0.74% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |

GC pauses: 0, total 0 ms

