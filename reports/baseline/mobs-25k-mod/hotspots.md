# JVM hotspots (JFR ExecutionSample)

Samples: 21006

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.19% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.51% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 1.86% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 1.68% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 5 | 1.52% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 6 | 1.33% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7863` |
| 7 | 1.22% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |
| 8 | 1.19% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 9 | 1.18% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1963` |
| 10 | 1.17% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 11 | 1.16% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 12 | 1.08% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7722` |
| 13 | 0.98% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1944` |
| 14 | 0.96% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 15 | 0.85% | `dev.planeta9091.centities.CeHooks.aiPackSel(int[], int, WrappedGoal[], int[], int[], long[], int[], int[], int[], float[], Mob) line: 6756` |
| 16 | 0.84% | `java.util.EnumMap.get(Object) line: 248` |
| 17 | 0.80% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 18 | 0.80% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 19 | 0.78% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 20 | 0.77% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 21 | 0.76% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4782` |
| 22 | 0.71% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 23 | 0.68% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 24 | 0.66% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 25 | 0.65% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 26 | 0.60% | `net.minecraft.util.SimpleBitStorage.get(int) line: 350` |
| 27 | 0.57% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 28 | 0.54% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 29 | 0.54% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 603` |
| 30 | 0.53% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1947` |
| 31 | 0.50% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 32 | 0.50% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 33 | 0.47% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 34 | 0.46% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4637` |
| 35 | 0.45% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap$MapIterator.nextEntry() line: 655` |
| 36 | 0.44% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 37 | 0.44% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7808` |
| 38 | 0.43% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 39 | 0.42% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 40 | 0.42% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3115` |
| 41 | 0.41% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 7930` |
| 42 | 0.40% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 43 | 0.40% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 44 | 0.39% | `sun.misc.Unsafe.getLong(Object, long) line: 357` |
| 45 | 0.39% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 46 | 0.39% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 47 | 0.39% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 48 | 0.39% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 49 | 0.38% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 50 | 0.38% | `dev.planeta9091.centities.CeHooks.aiMeleeRunning(CeHooks$AiCtx) line: 6952` |
| 51 | 0.38% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 52 | 0.35% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 53 | 0.35% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 3225` |
| 54 | 0.34% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 55 | 0.34% | `ca.spottedleaf.moonrise.patches.starlight.light.SWMRNibbleArray.getVisible(int) line: 412` |
| 56 | 0.34% | `dev.planeta9091.centities.CeHooks.disabledBacking(GoalSelector) line: 5056` |
| 57 | 0.33% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 58 | 0.32% | `net.minecraft.world.level.levelgen.BitRandomSource.nextInt(int) line: 31` |
| 59 | 0.32% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 60 | 0.30% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightInterface.getSkyLightValue(BlockPos, ChunkAccess) line: 252` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.19% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 3.99% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1486` |
| 3 | 2.82% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 1.73% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 5 | 1.62% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 6 | 1.62% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1948` |
| 7 | 1.34% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 8 | 1.28% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 9 | 1.22% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 196` |
| 10 | 1.21% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 11 | 1.15% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 12 | 1.13% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 13 | 1.03% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7735` |
| 14 | 1.00% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 15 | 0.96% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 16 | 0.90% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 17 | 0.84% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 18 | 0.83% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 19 | 0.82% | `com.google.common.collect.MapMakerInternalMap$Segment.getEntry(Object, int) line: 1415` |
| 20 | 0.81% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 21 | 0.78% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 22 | 0.75% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 23 | 0.75% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 24 | 0.73% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 25 | 0.70% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 26 | 0.70% | `net.minecraft.world.item.enchantment.ItemEnchantments.entrySet() line: 101` |
| 27 | 0.69% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7710` |
| 28 | 0.67% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4587` |
| 29 | 0.66% | `dev.planeta9091.centities.CeHooks.aiPack(Mob, CeHooks$AiCtx, int, int, int, double, double, double) line: 6739` |
| 30 | 0.60% | `dev.planeta9091.centities.CeHooks.mobAiFloor(Mob) line: 6922` |

GC pauses: 0, total 0 ms

