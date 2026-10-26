# JVM hotspots (JFR ExecutionSample)

Samples: 14701

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.91% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.67% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1406` |
| 3 | 2.16% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 4 | 2.05% | `dev.planeta9091.centities.CeHooks.setSharedFlag$(Entity, int, boolean) line: 6307` |
| 5 | 1.99% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 6495` |
| 6 | 1.97% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 7 | 1.85% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 1.80% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 9 | 1.67% | `java.util.EnumMap.get(Object) line: 248` |
| 10 | 1.52% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.46% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 12 | 1.29% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 1.27% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 14 | 1.23% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 15 | 1.18% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1425` |
| 16 | 1.16% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4720` |
| 17 | 1.14% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 18 | 1.07% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 19 | 1.06% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 20 | 1.04% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 4375` |
| 21 | 1.01% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 22 | 0.99% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4197` |
| 23 | 0.97% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 24 | 0.95% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 25 | 0.90% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 26 | 0.88% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 27 | 0.86% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 2559` |
| 28 | 0.84% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 29 | 0.76% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4052` |
| 30 | 0.72% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6418` |
| 31 | 0.71% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 32 | 0.69% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 33 | 0.67% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4663` |
| 34 | 0.64% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 35 | 0.60% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 36 | 0.59% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 2496` |
| 37 | 0.59% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 38 | 0.58% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 39 | 0.56% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 40 | 0.54% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3696` |
| 41 | 0.54% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 4364` |
| 42 | 0.52% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 43 | 0.52% | `dev.planeta9091.centities.CeHooks.aiShadowPeek(Object) line: 4409` |
| 44 | 0.49% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 45 | 0.49% | `jdk.internal.util.ArraysSupport.hashCodeOfUnsigned(byte[], int, int, int) line: 275` |
| 46 | 0.48% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 47 | 0.48% | `java.lang.String.hashCode() line: 2430` |
| 48 | 0.47% | `net.minecraft.world.entity.ai.goal.GoalSelector.getAvailableGoals() line: 134` |
| 49 | 0.45% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3579` |
| 50 | 0.43% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 51 | 0.42% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 4522` |
| 52 | 0.42% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 2669` |
| 53 | 0.42% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 54 | 0.41% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 55 | 0.40% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 56 | 0.39% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 57 | 0.39% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 58 | 0.39% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 59 | 0.35% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 60 | 0.35% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.32% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1059` |
| 2 | 3.91% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 3.35% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 2.47% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4730` |
| 5 | 2.33% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 6 | 2.18% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 7 | 2.17% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 8 | 2.06% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean)` |
| 9 | 1.99% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean)` |
| 10 | 1.95% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1410` |
| 11 | 1.83% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 12 | 1.76% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4662` |
| 13 | 1.54% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 14 | 1.49% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 15 | 1.44% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 16 | 1.22% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 17 | 1.19% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4720` |
| 18 | 1.10% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6418` |
| 19 | 1.05% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 20 | 1.00% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 21 | 0.97% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 22 | 0.92% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 23 | 0.85% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 24 | 0.84% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 25 | 0.80% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 26 | 0.79% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 27 | 0.79% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 4506` |
| 28 | 0.74% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk)` |
| 29 | 0.73% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 30 | 0.71% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |

GC pauses: 0, total 0 ms

