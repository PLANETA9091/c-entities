# JVM hotspots (JFR ExecutionSample)

Samples: 15844

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.91% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.57% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 2.18% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 1.85% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 5 | 1.79% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 1.58% | `java.util.EnumMap.get(Object) line: 248` |
| 7 | 1.37% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 8 | 1.32% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 9 | 1.29% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4604` |
| 10 | 1.18% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7685` |
| 11 | 1.17% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 12 | 1.08% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7544` |
| 13 | 1.07% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |
| 14 | 1.04% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 15 | 1.04% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 16 | 1.02% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 603` |
| 17 | 1.00% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 18 | 0.95% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 19 | 0.88% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1778` |
| 20 | 0.87% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 21 | 0.85% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 22 | 0.83% | `dev.planeta9091.centities.CeHooks.aiPackSel(int[], int, WrappedGoal[], int[], int[], long[], int[], int[], int[], float[], Mob) line: 6578` |
| 23 | 0.80% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 24 | 0.79% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 25 | 0.76% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7491` |
| 26 | 0.76% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 27 | 0.73% | `net.minecraft.world.phys.Vec3.subtract(Vec3) line: 103` |
| 28 | 0.71% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 29 | 0.68% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 2947` |
| 30 | 0.65% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4459` |
| 31 | 0.62% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 32 | 0.62% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 3057` |
| 33 | 0.62% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1797` |
| 34 | 0.61% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 35 | 0.60% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 7752` |
| 36 | 0.59% | `net.minecraft.util.SimpleBitStorage.get(int) line: 350` |
| 37 | 0.57% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 2884` |
| 38 | 0.54% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 39 | 0.54% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7630` |
| 40 | 0.53% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 41 | 0.53% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 42 | 0.51% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 43 | 0.51% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 44 | 0.51% | `net.minecraft.world.level.levelgen.BitRandomSource.nextInt(int) line: 31` |
| 45 | 0.49% | `ca.spottedleaf.moonrise.patches.starlight.light.SWMRNibbleArray.getVisible(int) line: 412` |
| 46 | 0.49% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 47 | 0.48% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 48 | 0.47% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 49 | 0.45% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 50 | 0.44% | `dev.planeta9091.centities.CeHooks.aiMeleeRunning(CeHooks$AiCtx) line: 6774` |
| 51 | 0.43% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 52 | 0.42% | `sun.misc.Unsafe.getLong(Object, long) line: 357` |
| 53 | 0.40% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 54 | 0.37% | `java.util.HashMap.getNode(Object) line: 586` |
| 55 | 0.37% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 56 | 0.37% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 5621` |
| 57 | 0.34% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 58 | 0.33% | `com.google.common.collect.MapMakerInternalMap$Segment.getEntry(Object, int) line: 1416` |
| 59 | 0.33% | `dev.planeta9091.centities.CeHooks.aiClassify(Mob) line: 6307` |
| 60 | 0.33% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap$MapIterator.nextEntry() line: 655` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.91% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 4.35% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 2.44% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 2.19% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1320` |
| 5 | 1.94% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 6 | 1.86% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 7 | 1.55% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7557` |
| 8 | 1.43% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 9 | 1.33% | `com.google.common.collect.MapMakerInternalMap$Segment.getEntry(Object, int) line: 1415` |
| 10 | 1.31% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 11 | 1.20% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 12 | 1.18% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 13 | 1.07% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 196` |
| 14 | 1.07% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 15 | 1.00% | `java.util.concurrent.atomic.AtomicReferenceArray.get(int) line: 103` |
| 16 | 1.00% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 17 | 0.98% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 18 | 0.93% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 19 | 0.91% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 3191` |
| 20 | 0.89% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 21 | 0.86% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1782` |
| 22 | 0.85% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 23 | 0.82% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7540` |
| 24 | 0.81% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 25 | 0.79% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2195` |
| 26 | 0.75% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 27 | 0.74% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4409` |
| 28 | 0.73% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 29 | 0.73% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 30 | 0.72% | `dev.planeta9091.centities.CeHooks.mobAiFloor(Mob) line: 6732` |

GC pauses: 0, total 0 ms

