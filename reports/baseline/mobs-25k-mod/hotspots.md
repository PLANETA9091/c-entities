# JVM hotspots (JFR ExecutionSample)

Samples: 18701

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.94% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.13% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 3 | 2.51% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 2.39% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 2.13% | `java.lang.ref.Reference.get() line: 361` |
| 6 | 1.61% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 7 | 1.48% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2005` |
| 8 | 1.45% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7914` |
| 9 | 1.33% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 10 | 1.30% | `java.util.EnumMap.get(Object) line: 248` |
| 11 | 1.23% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7773` |
| 12 | 1.17% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 13 | 1.07% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4833` |
| 14 | 1.04% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 15 | 1.04% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1986` |
| 16 | 0.96% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 17 | 0.91% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 18 | 0.90% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 19 | 0.90% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 20 | 0.87% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 5356` |
| 21 | 0.82% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 22 | 0.80% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 23 | 0.76% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 24 | 0.68% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 25 | 0.67% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 5850` |
| 26 | 0.66% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 27 | 0.66% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 28 | 0.65% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7720` |
| 29 | 0.65% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 30 | 0.64% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 31 | 0.63% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 32 | 0.61% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 33 | 0.60% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 34 | 0.57% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 35 | 0.56% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 5299` |
| 36 | 0.55% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 37 | 0.55% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 38 | 0.55% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 5000` |
| 39 | 0.53% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 40 | 0.51% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3157` |
| 41 | 0.50% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 42 | 0.49% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4688` |
| 43 | 0.49% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 44 | 0.48% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1989` |
| 45 | 0.47% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 46 | 0.47% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 47 | 0.45% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 48 | 0.44% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7859` |
| 49 | 0.44% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 50 | 0.44% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 51 | 0.42% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 52 | 0.42% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 7981` |
| 53 | 0.39% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 54 | 0.36% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 55 | 0.35% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 56 | 0.34% | `ca.spottedleaf.moonrise.patches.starlight.light.SWMRNibbleArray.getVisible(int) line: 412` |
| 57 | 0.34% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1983` |
| 58 | 0.33% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 59 | 0.33% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 990` |
| 60 | 0.33% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.94% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 4.44% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1528` |
| 3 | 3.43% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 3.09% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1990` |
| 5 | 2.25% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 6 | 1.68% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 7 | 1.61% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 8 | 1.49% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 5366` |
| 9 | 1.47% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 10 | 1.39% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 5356` |
| 11 | 1.28% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 12 | 1.17% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 13 | 1.14% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 14 | 1.10% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 5011` |
| 15 | 1.03% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 16 | 0.95% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 17 | 0.94% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7786` |
| 18 | 0.94% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 19 | 0.90% | `net.minecraft.world.entity.ai.sensing.Sensing.tick()` |
| 20 | 0.90% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 21 | 0.90% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 22 | 0.88% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 23 | 0.78% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 24 | 0.78% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 25 | 0.75% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 26 | 0.74% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 27 | 0.73% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 28 | 0.71% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7769` |
| 29 | 0.71% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 30 | 0.71% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |

GC pauses: 0, total 0 ms

