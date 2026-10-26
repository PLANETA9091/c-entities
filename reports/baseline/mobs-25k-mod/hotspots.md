# JVM hotspots (JFR ExecutionSample)

Samples: 16193

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.13% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.98% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 1.52% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 4 | 1.46% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 1.43% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 6 | 1.36% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 6507` |
| 7 | 1.33% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 6556` |
| 8 | 1.12% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 9 | 1.08% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1407` |
| 10 | 1.05% | `java.util.EnumMap.get(Object) line: 248` |
| 11 | 1.00% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 12 | 0.96% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 13 | 0.88% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 14 | 0.86% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 15 | 0.82% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 16 | 0.77% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 4376` |
| 17 | 0.75% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 18 | 0.74% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 19 | 0.73% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 20 | 0.72% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4198` |
| 21 | 0.70% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 22 | 0.70% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 23 | 0.67% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 24 | 0.65% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4721` |
| 25 | 0.64% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 26 | 0.64% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 27 | 0.62% | `dev.planeta9091.centities.CeHooks.setSharedFlag$(Entity, int, boolean) line: 6343` |
| 28 | 0.60% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4053` |
| 29 | 0.58% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 5122` |
| 30 | 0.57% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 31 | 0.54% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 32 | 0.53% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 33 | 0.52% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 34 | 0.52% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 35 | 0.52% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 36 | 0.51% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1426` |
| 37 | 0.50% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 38 | 0.49% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 39 | 0.48% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6454` |
| 40 | 0.45% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 41 | 0.45% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 42 | 0.44% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 43 | 0.44% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 44 | 0.44% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4664` |
| 45 | 0.43% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 46 | 0.43% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 47 | 0.40% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 48 | 0.38% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 2560` |
| 49 | 0.38% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 50 | 0.38% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 51 | 0.38% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap$MapIterator.nextEntry() line: 655` |
| 52 | 0.37% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 53 | 0.36% | `dev.planeta9091.centities.CeHooks.aiShadowPeek(Object) line: 4410` |
| 54 | 0.36% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 55 | 0.36% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |
| 56 | 0.36% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 57 | 0.35% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 58 | 0.35% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 59 | 0.35% | `java.util.HashMap.getNode(Object) line: 586` |
| 60 | 0.35% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.13% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 3.12% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1060` |
| 3 | 1.91% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 1.61% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 5 | 1.57% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 6 | 1.46% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 7 | 1.40% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1411` |
| 8 | 1.38% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4731` |
| 9 | 1.36% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 10 | 1.33% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean)` |
| 11 | 1.26% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6454` |
| 12 | 1.25% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 13 | 1.16% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 14 | 1.14% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 15 | 1.14% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4663` |
| 16 | 1.04% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4721` |
| 17 | 0.99% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 18 | 0.96% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 19 | 0.90% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 20 | 0.85% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 21 | 0.82% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 22 | 0.81% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 23 | 0.81% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 24 | 0.79% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 25 | 0.70% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 26 | 0.70% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 27 | 0.70% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 28 | 0.69% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 29 | 0.69% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 30 | 0.68% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 5118` |

GC pauses: 0, total 0 ms

