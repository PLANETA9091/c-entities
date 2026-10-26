# JVM hotspots (JFR ExecutionSample)

Samples: 17974

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.14% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.74% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 1.80% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 4 | 1.72% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 6970` |
| 5 | 1.70% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 6921` |
| 6 | 1.43% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 1.34% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 8 | 1.26% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1412` |
| 9 | 1.07% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 10 | 1.04% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 11 | 0.98% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 12 | 0.95% | `java.util.EnumMap.get(Object) line: 248` |
| 13 | 0.88% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 14 | 0.86% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 15 | 0.85% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 16 | 0.83% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4203` |
| 17 | 0.81% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 18 | 0.80% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 19 | 0.80% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 20 | 0.78% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 21 | 0.73% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 22 | 0.69% | `dev.planeta9091.centities.CeHooks.setSharedFlag$(Entity, int, boolean) line: 6757` |
| 23 | 0.68% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 24 | 0.67% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1431` |
| 25 | 0.66% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 4381` |
| 26 | 0.63% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 27 | 0.61% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4058` |
| 28 | 0.57% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 29 | 0.57% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 30 | 0.56% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 31 | 0.55% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 32 | 0.55% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 5127` |
| 33 | 0.53% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 34 | 0.51% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 35 | 0.51% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 36 | 0.50% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 37 | 0.49% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 38 | 0.48% | `dev.planeta9091.centities.CeHooks.aiShadowPeek(Object) line: 4415` |
| 39 | 0.48% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 40 | 0.47% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 41 | 0.47% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 4370` |
| 42 | 0.46% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 43 | 0.45% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 44 | 0.45% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4669` |
| 45 | 0.45% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4726` |
| 46 | 0.45% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 47 | 0.43% | `jdk.internal.util.ArraysSupport.hashCodeOfUnsigned(byte[], int, int, int) line: 275` |
| 48 | 0.43% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 2565` |
| 49 | 0.42% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6868` |
| 50 | 0.40% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 51 | 0.40% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 52 | 0.38% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 53 | 0.36% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 54 | 0.36% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1415` |
| 55 | 0.36% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 56 | 0.35% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 57 | 0.34% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 58 | 0.34% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap$MapIterator.nextEntry() line: 655` |
| 59 | 0.33% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 2502` |
| 60 | 0.32% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.14% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 3.27% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1065` |
| 3 | 2.16% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 1.74% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1416` |
| 5 | 1.72% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean)` |
| 6 | 1.71% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 7 | 1.60% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 8 | 1.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 9 | 1.40% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 10 | 1.30% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4736` |
| 11 | 1.27% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4668` |
| 12 | 1.17% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 13 | 1.13% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 14 | 1.13% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 15 | 0.95% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 16 | 0.93% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 17 | 0.92% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 18 | 0.86% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 19 | 0.86% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6868` |
| 20 | 0.85% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 21 | 0.85% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 22 | 0.84% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 23 | 0.83% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4726` |
| 24 | 0.81% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 5123` |
| 25 | 0.77% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 26 | 0.77% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 4512` |
| 27 | 0.74% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 28 | 0.72% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 29 | 0.69% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean)` |
| 30 | 0.67% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |

GC pauses: 0, total 0 ms

