# JVM hotspots (JFR ExecutionSample)

Samples: 17101

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.96% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.08% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1484` |
| 3 | 1.72% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 3872` |
| 4 | 1.48% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 5 | 1.39% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 6 | 1.32% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 3517` |
| 7 | 1.30% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 8 | 1.25% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 9 | 1.25% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 10 | 1.19% | `java.util.EnumMap.get(Object) line: 248` |
| 11 | 1.12% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 1.06% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 13 | 0.96% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 14 | 0.96% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 15 | 0.81% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4706` |
| 16 | 0.80% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3868` |
| 17 | 0.78% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1394` |
| 18 | 0.76% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 19 | 0.76% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3190` |
| 20 | 0.75% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 21 | 0.72% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 22 | 0.71% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 23 | 0.70% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 24 | 0.70% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3053` |
| 25 | 0.69% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 26 | 0.68% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 27 | 0.66% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 3345` |
| 28 | 0.66% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 29 | 0.62% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 30 | 0.61% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 31 | 0.56% | `dev.planeta9091.centities.CeHooks.aiShadowPeek(Object) line: 3379` |
| 32 | 0.55% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3696` |
| 33 | 0.54% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 34 | 0.51% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 35 | 0.51% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 36 | 0.49% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 37 | 0.49% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 3818` |
| 38 | 0.48% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 39 | 0.48% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 3334` |
| 40 | 0.46% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 41 | 0.43% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 42 | 0.43% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 43 | 0.43% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 44 | 0.43% | `java.util.concurrent.atomic.AtomicInteger.get() line: 91` |
| 45 | 0.42% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 46 | 0.41% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 47 | 0.41% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1413` |
| 48 | 0.40% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 49 | 0.39% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 50 | 0.38% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 51 | 0.37% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 52 | 0.37% | `dev.planeta9091.centities.CeHooks.disabledBacking(GoalSelector) line: 3441` |
| 53 | 0.36% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 54 | 0.36% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 55 | 0.36% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 56 | 0.35% | `java.util.ArrayDeque.size() line: 647` |
| 57 | 0.35% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 58 | 0.33% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 59 | 0.33% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3063` |
| 60 | 0.33% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 3.96% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 2.36% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1047` |
| 3 | 1.82% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 1.81% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 5 | 1.75% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 6 | 1.73% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean)` |
| 7 | 1.45% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1398` |
| 8 | 1.44% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity) line: 599` |
| 9 | 1.42% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 10 | 1.35% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 11 | 1.27% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 3529` |
| 12 | 1.25% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 13 | 1.05% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 14 | 1.04% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 3473` |
| 15 | 1.01% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 16 | 0.96% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 17 | 0.96% | `dev.planeta9091.centities.CeHooks$RefField.getObj(Object) line: 2296` |
| 18 | 0.88% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 3517` |
| 19 | 0.88% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 20 | 0.84% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 3559` |
| 21 | 0.81% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 22 | 0.80% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3798` |
| 23 | 0.80% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 24 | 0.78% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 25 | 0.77% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 26 | 0.74% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 27 | 0.71% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 28 | 0.70% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 29 | 0.65% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 30 | 0.65% | `java.util.ArrayList.toArray() line: 370` |

GC pauses: 0, total 0 ms

