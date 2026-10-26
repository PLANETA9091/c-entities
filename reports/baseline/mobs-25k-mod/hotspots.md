# JVM hotspots (JFR ExecutionSample)

Samples: 16182

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.12% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.90% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 2.89% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 2.11% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 5 | 1.80% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 6 | 1.27% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 7 | 1.23% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 8 | 1.22% | `java.util.EnumMap.get(Object) line: 248` |
| 9 | 1.17% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4198` |
| 10 | 1.17% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 11 | 1.17% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 12 | 1.16% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 13 | 1.15% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 14 | 1.07% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 4376` |
| 15 | 1.05% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 16 | 0.95% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1407` |
| 17 | 0.85% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 18 | 0.85% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1426` |
| 19 | 0.83% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 20 | 0.82% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 21 | 0.82% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 22 | 0.78% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 23 | 0.78% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 24 | 0.74% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 25 | 0.70% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 26 | 0.70% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 27 | 0.69% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4721` |
| 28 | 0.68% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4053` |
| 29 | 0.67% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 30 | 0.67% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6444` |
| 31 | 0.65% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 32 | 0.64% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 6546` |
| 33 | 0.64% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 34 | 0.62% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 35 | 0.62% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 36 | 0.61% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 5112` |
| 37 | 0.57% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 38 | 0.57% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 39 | 0.56% | `dev.planeta9091.centities.CeHooks.aiShadowPeek(Object) line: 4410` |
| 40 | 0.55% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 2560` |
| 41 | 0.55% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 42 | 0.54% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 43 | 0.54% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 6497` |
| 44 | 0.54% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 45 | 0.54% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 46 | 0.51% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 47 | 0.48% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1410` |
| 48 | 0.47% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 49 | 0.47% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4664` |
| 50 | 0.46% | `ca.spottedleaf.moonrise.patches.starlight.light.SWMRNibbleArray.getVisible(int) line: 412` |
| 51 | 0.45% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 4365` |
| 52 | 0.45% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 53 | 0.42% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 54 | 0.40% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 55 | 0.40% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 56 | 0.40% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 2497` |
| 57 | 0.40% | `jdk.internal.util.ArraysSupport.hashCodeOfUnsigned(byte[], int, int, int) line: 275` |
| 58 | 0.38% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 2670` |
| 59 | 0.36% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 60 | 0.36% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.12% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 4.02% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 3.46% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1060` |
| 4 | 2.10% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1411` |
| 5 | 1.92% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 6 | 1.83% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 7 | 1.69% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4663` |
| 8 | 1.57% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 9 | 1.54% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 10 | 1.51% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4731` |
| 11 | 1.42% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 12 | 1.34% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6444` |
| 13 | 1.29% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 14 | 1.24% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 15 | 1.17% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4721` |
| 16 | 1.16% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 17 | 1.06% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 18 | 0.92% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 5108` |
| 19 | 0.90% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 20 | 0.90% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 21 | 0.83% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 22 | 0.83% | `net.minecraft.world.entity.ai.sensing.Sensing.tick()` |
| 23 | 0.83% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 4507` |
| 24 | 0.80% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 25 | 0.80% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 26 | 0.76% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 27 | 0.74% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 28 | 0.74% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 29 | 0.73% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 30 | 0.73% | `net.minecraft.server.level.ServerEntity.sendChanges()` |

GC pauses: 0, total 0 ms

