# JVM hotspots (JFR ExecutionSample)

Samples: 14865

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.84% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.75% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 2.44% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 6709` |
| 4 | 2.29% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 5 | 1.98% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3810` |
| 6 | 1.92% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 1.65% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 8 | 1.37% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 9 | 1.23% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 10 | 1.23% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4202` |
| 11 | 1.17% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 4380` |
| 12 | 1.16% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 13 | 1.10% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 14 | 1.08% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 15 | 0.98% | `java.util.EnumMap.get(Object) line: 248` |
| 16 | 0.91% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 17 | 0.85% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1411` |
| 18 | 0.85% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 19 | 0.81% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 20 | 0.75% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4725` |
| 21 | 0.73% | `dev.planeta9091.centities.CeHooks.setSharedFlag$(Entity, int, boolean) line: 6496` |
| 22 | 0.72% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 23 | 0.71% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 24 | 0.69% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1430` |
| 25 | 0.69% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 26 | 0.69% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 27 | 0.69% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 28 | 0.67% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6607` |
| 29 | 0.65% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 30 | 0.65% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 31 | 0.65% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 32 | 0.64% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 33 | 0.63% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 34 | 0.63% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4057` |
| 35 | 0.61% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 36 | 0.61% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 2564` |
| 37 | 0.56% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 38 | 0.55% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4668` |
| 39 | 0.54% | `java.lang.String.hashCode() line: 2433` |
| 40 | 0.54% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 41 | 0.54% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 4369` |
| 42 | 0.53% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 43 | 0.51% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 44 | 0.51% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 45 | 0.50% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 46 | 0.48% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 47 | 0.48% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 48 | 0.47% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1414` |
| 49 | 0.44% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 50 | 0.43% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 51 | 0.43% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 52 | 0.40% | `dev.planeta9091.centities.CeHooks.aiShadowPeek(Object) line: 4414` |
| 53 | 0.40% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 54 | 0.38% | `java.lang.String.hashCode() line: 2430` |
| 55 | 0.38% | `java.util.HashMap.getNode(Object) line: 586` |
| 56 | 0.36% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 57 | 0.36% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 2674` |
| 58 | 0.35% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 2501` |
| 59 | 0.34% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 60 | 0.34% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.84% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 3.15% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1064` |
| 3 | 3.02% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 2.60% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 5 | 2.44% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean)` |
| 6 | 2.25% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1415` |
| 7 | 1.89% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 8 | 1.80% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4667` |
| 9 | 1.71% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 10 | 1.68% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 11 | 1.66% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4735` |
| 12 | 1.62% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 13 | 1.53% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6607` |
| 14 | 1.41% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 15 | 1.26% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 16 | 1.08% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 17 | 1.02% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4725` |
| 18 | 0.94% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 19 | 0.91% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 20 | 0.85% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 21 | 0.85% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 22 | 0.83% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 23 | 0.81% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 5122` |
| 24 | 0.77% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 25 | 0.77% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 26 | 0.75% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 27 | 0.73% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean)` |
| 28 | 0.72% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 29 | 0.71% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 30 | 0.68% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |

GC pauses: 0, total 0 ms

