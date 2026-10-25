# JVM hotspots (JFR ExecutionSample)

Samples: 16598

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.41% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.60% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 5509` |
| 3 | 2.13% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 4 | 2.01% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 5 | 1.95% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1496` |
| 6 | 1.68% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4157` |
| 7 | 1.66% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 1.58% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3817` |
| 9 | 1.50% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 10 | 1.42% | `java.util.EnumMap.get(Object) line: 248` |
| 11 | 1.40% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 12 | 1.38% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 13 | 1.28% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 5480` |
| 14 | 1.20% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 15 | 1.07% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 16 | 1.06% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 3985` |
| 17 | 1.02% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1425` |
| 18 | 0.93% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 19 | 0.93% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 20 | 0.93% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 21 | 0.92% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 22 | 0.86% | `dev.planeta9091.centities.CeHooks.setSharedFlag$(Entity, int, boolean) line: 5343` |
| 23 | 0.81% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 24 | 0.80% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 25 | 0.75% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 26 | 0.75% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 5439` |
| 27 | 0.70% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3671` |
| 28 | 0.70% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1406` |
| 29 | 0.69% | `dev.planeta9091.centities.CeHooks.aiShadowPeek(Object) line: 4019` |
| 30 | 0.65% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 31 | 0.63% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 32 | 0.63% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 33 | 0.61% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 34 | 0.60% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 3974` |
| 35 | 0.58% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 36 | 0.58% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3681` |
| 37 | 0.57% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 38 | 0.57% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 39 | 0.55% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 40 | 0.54% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 41 | 0.52% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 42 | 0.52% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 43 | 0.51% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 44 | 0.51% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1409` |
| 45 | 0.50% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 46 | 0.47% | `java.util.concurrent.atomic.AtomicInteger.get() line: 91` |
| 47 | 0.46% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 48 | 0.45% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 49 | 0.44% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 993` |
| 50 | 0.42% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 51 | 0.41% | `ca.spottedleaf.moonrise.patches.starlight.light.SWMRNibbleArray.getVisible(int) line: 412` |
| 52 | 0.40% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 53 | 0.40% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 54 | 0.39% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 55 | 0.39% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 56 | 0.38% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 57 | 0.38% | `net.minecraft.world.entity.Entity.move(MoverType, Vec3) line: 1170` |
| 58 | 0.37% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 59 | 0.37% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap$MapIterator.nextEntry() line: 655` |
| 60 | 0.37% | `dev.planeta9091.centities.CeHooks.disabledBacking(GoalSelector) line: 4081` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.41% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 3.34% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1059` |
| 3 | 2.62% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean)` |
| 4 | 2.58% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 2.13% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1410` |
| 6 | 2.08% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 7 | 2.01% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 8 | 1.89% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4169` |
| 9 | 1.71% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 10 | 1.65% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 11 | 1.60% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 12 | 1.59% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4157` |
| 13 | 1.30% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 5439` |
| 14 | 1.29% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 15 | 1.28% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 16 | 1.24% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 17 | 1.08% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 18 | 1.07% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 4113` |
| 19 | 1.04% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 20 | 0.99% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 21 | 0.98% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 22 | 0.93% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 23 | 0.90% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4199` |
| 24 | 0.89% | `dev.planeta9091.centities.CeHooks.gateCell(Level, AABB) line: 949` |
| 25 | 0.86% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean)` |
| 26 | 0.80% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 27 | 0.76% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk)` |
| 28 | 0.69% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 29 | 0.67% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 30 | 0.63% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |

GC pauses: 0, total 0 ms

