# JVM hotspots (JFR ExecutionSample)

Samples: 17910

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 31.55% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 2 | 2.52% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1393` |
| 3 | 2.50% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getNextAcquire() line: 1837` |
| 4 | 1.51% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 5 | 1.27% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 6 | 1.17% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 7 | 1.04% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 8 | 1.04% | `net.minecraft.world.entity.Entity.checkInsideBlocks(Vec3, Vec3, InsideBlockEffectApplier$StepBasedCollector, LongSet, int) line: 1883` |
| 9 | 1.02% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3118` |
| 10 | 0.94% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 11 | 0.84% | `java.util.ArrayList.add(Object, Object[], int) line: 485` |
| 12 | 0.78% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 8361` |
| 13 | 0.74% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 14 | 0.71% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12320` |
| 15 | 0.70% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 16 | 0.68% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 62` |
| 17 | 0.66% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 18 | 0.64% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 8014` |
| 19 | 0.60% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 5566` |
| 20 | 0.59% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 21 | 0.58% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 22 | 0.57% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 23 | 0.55% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 24 | 0.55% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 8192` |
| 25 | 0.55% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 8439` |
| 26 | 0.54% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 13090` |
| 27 | 0.54% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 5503` |
| 28 | 0.54% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2882` |
| 29 | 0.45% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7817` |
| 30 | 0.44% | `java.util.HashMap.getNode(Object) line: 586` |
| 31 | 0.42% | `java.util.ArrayList.elementData(int) line: 412` |
| 32 | 0.39% | `dev.planeta9091.centities.CeHooks.pushDispatch(Entity) line: 4584` |
| 33 | 0.37% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 34 | 0.36% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 35 | 0.35% | `java.util.EnumMap.get(Object) line: 248` |
| 36 | 0.35% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 37 | 0.33% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 38 | 0.32% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1394` |
| 39 | 0.32% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3596` |
| 40 | 0.32% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 41 | 0.31% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 42 | 0.30% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 43 | 0.30% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 237` |
| 44 | 0.30% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 45 | 0.28% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 12909` |
| 46 | 0.28% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 47 | 0.28% | `java.lang.String.hashCode() line: 2430` |
| 48 | 0.28% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 49 | 0.28% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 50 | 0.27% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 8984` |
| 51 | 0.26% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 52 | 0.26% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 53 | 0.26% | `dev.planeta9091.centities.CeHooks.aiShadowPeek(Object) line: 8041` |
| 54 | 0.25% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3186` |
| 55 | 0.24% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 56 | 0.23% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 57 | 0.23% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3119` |
| 58 | 0.22% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 59 | 0.22% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 60 | 0.22% | `java.util.ArrayList.add(Object) line: 497` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 31.55% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 2 | 2.89% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 3 | 2.62% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2882` |
| 4 | 2.50% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 247` |
| 5 | 1.61% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 6 | 1.56% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 7 | 1.55% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 8454` |
| 8 | 1.32% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 9 | 1.25% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 8439` |
| 10 | 1.25% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 11 | 1.04% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1854` |
| 12 | 1.04% | `java.util.ArrayList.add(Object) line: 497` |
| 13 | 0.94% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 14 | 0.89% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12544` |
| 15 | 0.88% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 16 | 0.87% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3739` |
| 17 | 0.84% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 8360` |
| 18 | 0.74% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 19 | 0.74% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 20 | 0.73% | `dev.planeta9091.centities.CeHooks.goalSelectorTickRunning(GoalSelector, boolean) line: 8530` |
| 21 | 0.61% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 22 | 0.60% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 23 | 0.60% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 24 | 0.59% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 25 | 0.54% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean)` |
| 26 | 0.54% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1385` |
| 27 | 0.52% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 28 | 0.47% | `net.minecraft.world.entity.LivingEntity.pushEntities()` |
| 29 | 0.45% | `java.util.HashMap.get(Object) line: 565` |
| 30 | 0.43% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |

GC pauses: 0, total 0 ms

