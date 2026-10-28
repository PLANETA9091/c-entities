# JVM hotspots (JFR ExecutionSample)

Samples: 13457

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.05% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.76% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 2.70% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 2.10% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 5 | 1.78% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7862` |
| 6 | 1.69% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7721` |
| 7 | 1.66% | `java.util.EnumMap.get(Object) line: 248` |
| 8 | 1.58% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1943` |
| 9 | 1.54% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.39% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |
| 11 | 1.39% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 1.39% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 13 | 1.31% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 14 | 1.06% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 15 | 1.01% | `dev.planeta9091.centities.CeHooks.aiPackSel(int[], int, WrappedGoal[], int[], int[], long[], int[], int[], int[], float[], Mob) line: 6755` |
| 16 | 0.97% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3051` |
| 17 | 0.97% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7668` |
| 18 | 0.96% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 19 | 0.94% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3114` |
| 20 | 0.90% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4781` |
| 21 | 0.88% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 22 | 0.85% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7807` |
| 23 | 0.84% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 24 | 0.80% | `net.minecraft.world.level.levelgen.BitRandomSource.nextInt(int) line: 31` |
| 25 | 0.80% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 26 | 0.79% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 27 | 0.78% | `net.minecraft.world.phys.Vec3.subtract(Vec3) line: 103` |
| 28 | 0.72% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4636` |
| 29 | 0.71% | `dev.planeta9091.centities.CeHooks.aiMeleeRunning(CeHooks$AiCtx) line: 6951` |
| 30 | 0.70% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 31 | 0.68% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 5798` |
| 32 | 0.66% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 33 | 0.65% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 603` |
| 34 | 0.64% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 35 | 0.62% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 36 | 0.60% | `java.lang.String.hashCode() line: 2430` |
| 37 | 0.59% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 38 | 0.59% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 39 | 0.57% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 40 | 0.56% | `dev.planeta9091.centities.CeHooks.aiClassify(Mob) line: 6484` |
| 41 | 0.54% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 42 | 0.51% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 3224` |
| 43 | 0.51% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 44 | 0.51% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 45 | 0.51% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 46 | 0.48% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 7929` |
| 47 | 0.48% | `dev.planeta9091.centities.CeHooks.disabledBacking(GoalSelector) line: 5055` |
| 48 | 0.48% | `sun.misc.Unsafe.getLong(Object, long) line: 357` |
| 49 | 0.45% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 50 | 0.45% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 51 | 0.45% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 52 | 0.44% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 53 | 0.42% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 54 | 0.42% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 55 | 0.42% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 56 | 0.36% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 57 | 0.35% | `it.unimi.dsi.fastutil.objects.Object2IntAVLTreeMap.isEmpty() line: 942` |
| 58 | 0.34% | `com.google.common.collect.MapMakerInternalMap$Segment.getEntry(Object, int) line: 1416` |
| 59 | 0.33% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 60 | 0.33% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.29% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.05% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 2.62% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1485` |
| 4 | 2.14% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 5 | 2.00% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 6 | 1.81% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 7 | 1.79% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 8 | 1.77% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7734` |
| 9 | 1.77% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 10 | 1.51% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 11 | 1.50% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 12 | 1.39% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 196` |
| 13 | 1.35% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1947` |
| 14 | 1.32% | `com.google.common.collect.MapMakerInternalMap$Segment.getEntry(Object, int) line: 1415` |
| 15 | 1.23% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 16 | 1.20% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 17 | 1.03% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7717` |
| 18 | 0.99% | `dev.planeta9091.centities.CeHooks.mobAiFloor(Mob) line: 6909` |
| 19 | 0.96% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 20 | 0.96% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 21 | 0.94% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 22 | 0.87% | `net.minecraft.world.entity.ai.sensing.Sensing.tick()` |
| 23 | 0.85% | `dev.planeta9091.centities.CeHooks.aiPack(Mob, CeHooks$AiCtx, int, int, int, double, double, double) line: 6740` |
| 24 | 0.84% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 25 | 0.80% | `net.minecraft.world.entity.Mob.baseTick() line: 353` |
| 26 | 0.78% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 27 | 0.77% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 28 | 0.76% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 3358` |
| 29 | 0.74% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 30 | 0.74% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |

GC pauses: 0, total 0 ms

