# JVM hotspots (JFR ExecutionSample)

Samples: 24898

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 8.74% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.54% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 3 | 2.52% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1484` |
| 4 | 2.29% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 1.58% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 6 | 1.46% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 7 | 1.34% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 8 | 1.28% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 9 | 1.21% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 10 | 1.14% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4276` |
| 11 | 1.11% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7357` |
| 12 | 1.08% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1465` |
| 13 | 1.04% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 14 | 1.01% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7216` |
| 15 | 0.98% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 16 | 0.91% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 17 | 0.78% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 18 | 0.77% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 19 | 0.76% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |
| 20 | 0.73% | `java.util.EnumMap.get(Object) line: 248` |
| 21 | 0.71% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 22 | 0.69% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 23 | 0.66% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1468` |
| 24 | 0.65% | `dev.planeta9091.centities.CeHooks.aiPackSel(int[], int, WrappedGoal[], int[], int[], long[], int[], int[], int[], float[], Mob) line: 6250` |
| 25 | 0.65% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 603` |
| 26 | 0.62% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 27 | 0.59% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 28 | 0.57% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 29 | 0.57% | `java.lang.ref.Reference.get() line: 361` |
| 30 | 0.56% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 31 | 0.54% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 32 | 0.50% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 33 | 0.47% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 34 | 0.47% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 35 | 0.46% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 36 | 0.46% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 7424` |
| 37 | 0.45% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 38 | 0.44% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 39 | 0.44% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7302` |
| 40 | 0.42% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 41 | 0.42% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 42 | 0.40% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 43 | 0.39% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 44 | 0.38% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 45 | 0.37% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 46 | 0.37% | `dev.planeta9091.centities.CeHooks.aiMeleeRunning(CeHooks$AiCtx) line: 6446` |
| 47 | 0.36% | `ca.spottedleaf.moonrise.patches.starlight.light.SWMRNibbleArray.getVisible(int) line: 412` |
| 48 | 0.35% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightInterface.getSkyLightValue(BlockPos, ChunkAccess) line: 252` |
| 49 | 0.35% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4131` |
| 50 | 0.35% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 51 | 0.34% | `net.minecraft.world.item.ItemStack.getDamageValue() line: 573` |
| 52 | 0.33% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 53 | 0.33% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 54 | 0.33% | `sun.misc.Unsafe.getLong(Object, long) line: 357` |
| 55 | 0.32% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 56 | 0.31% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 2619` |
| 57 | 0.31% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 58 | 0.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 59 | 0.30% | `java.util.HashMap.getNode(Object) line: 586` |
| 60 | 0.30% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 2556` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 8.74% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 5.49% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1081` |
| 3 | 3.49% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1469` |
| 4 | 2.04% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 1.77% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 6 | 1.68% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 7 | 1.26% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 8 | 1.14% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7229` |
| 9 | 1.11% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 10 | 1.10% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 11 | 1.08% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 12 | 1.07% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 13 | 0.99% | `com.google.common.collect.MapMakerInternalMap$Segment.getEntry(Object, int) line: 1415` |
| 14 | 0.94% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 15 | 0.92% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 16 | 0.87% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 17 | 0.85% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 18 | 0.83% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 19 | 0.83% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7204` |
| 20 | 0.76% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 196` |
| 21 | 0.73% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 22 | 0.69% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 23 | 0.66% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 24 | 0.65% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4081` |
| 25 | 0.64% | `java.util.concurrent.atomic.AtomicReferenceArray.get(int) line: 103` |
| 26 | 0.63% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 27 | 0.60% | `net.minecraft.world.item.enchantment.Enchantment.tick(ServerLevel, int, EnchantedItemInUse, Entity) line: 400` |
| 28 | 0.58% | `net.minecraft.world.level.EntityGetter.findNearbyPlayer(Entity, double, Predicate) line: 117` |
| 29 | 0.57% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 30 | 0.55% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |

GC pauses: 0, total 0 ms

