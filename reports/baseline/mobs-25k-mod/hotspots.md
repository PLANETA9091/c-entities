# JVM hotspots (JFR ExecutionSample)

Samples: 13973

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.49% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.58% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 5716` |
| 3 | 2.51% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4364` |
| 4 | 2.36% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 5 | 2.23% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1406` |
| 6 | 2.07% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.03% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 8 | 1.77% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 9 | 1.75% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 5687` |
| 10 | 1.46% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 11 | 1.40% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1496` |
| 12 | 1.31% | `java.util.EnumMap.get(Object) line: 248` |
| 13 | 1.27% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 14 | 1.25% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 15 | 1.15% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 16 | 1.10% | `dev.planeta9091.centities.CeHooks.setSharedFlag$(Entity, int, boolean) line: 5550` |
| 17 | 1.08% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 18 | 1.04% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 2552` |
| 19 | 0.94% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 20 | 0.92% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 21 | 0.90% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 22 | 0.90% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4024` |
| 23 | 0.89% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 24 | 0.88% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 25 | 0.85% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 26 | 0.82% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 27 | 0.82% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 28 | 0.77% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 29 | 0.75% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 30 | 0.71% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$hasPlayers() line: 1347` |
| 31 | 0.69% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 2489` |
| 32 | 0.69% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 5646` |
| 33 | 0.68% | `java.util.concurrent.atomic.AtomicInteger.get() line: 91` |
| 34 | 0.67% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 35 | 0.67% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 36 | 0.65% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 37 | 0.63% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 38 | 0.62% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 39 | 0.62% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 40 | 0.61% | `dev.planeta9091.centities.CeHooks.aiShadowPeek(Object) line: 4226` |
| 41 | 0.59% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3888` |
| 42 | 0.57% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 43 | 0.56% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 44 | 0.52% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 45 | 0.51% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3860` |
| 46 | 0.49% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1425` |
| 47 | 0.48% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 48 | 0.48% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 4192` |
| 49 | 0.47% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 50 | 0.47% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 51 | 0.44% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 4336` |
| 52 | 0.44% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 53 | 0.43% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 54 | 0.42% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 55 | 0.42% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 56 | 0.42% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 57 | 0.39% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 58 | 0.39% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 993` |
| 59 | 0.37% | `dev.planeta9091.centities.CeHooks.disabledBacking(GoalSelector) line: 4288` |
| 60 | 0.36% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 3.49% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 3.32% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 3.28% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1059` |
| 4 | 2.98% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 5 | 2.59% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean)` |
| 6 | 2.37% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 7 | 2.26% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4364` |
| 8 | 2.23% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 9 | 1.97% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 10 | 1.90% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 11 | 1.85% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 12 | 1.75% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 13 | 1.52% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 14 | 1.36% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 15 | 1.32% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 5646` |
| 16 | 1.27% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4406` |
| 17 | 1.24% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 18 | 1.23% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1410` |
| 19 | 1.13% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 20 | 1.12% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 21 | 1.11% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean)` |
| 22 | 0.95% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 4320` |
| 23 | 0.94% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 24 | 0.90% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 25 | 0.85% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 26 | 0.82% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 27 | 0.82% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4376` |
| 28 | 0.79% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity) line: 603` |
| 29 | 0.76% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 30 | 0.76% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |

GC pauses: 0, total 0 ms

