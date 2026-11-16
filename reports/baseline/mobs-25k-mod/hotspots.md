# JVM hotspots (JFR ExecutionSample)

Samples: 15303

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 9.89% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1581` |
| 2 | 4.32% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3367` |
| 3 | 2.84% | `java.util.ArrayList.elementData(int) line: 412` |
| 4 | 2.44% | `java.util.ArrayList.add(Object, Object[], int) line: 485` |
| 5 | 1.89% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 1.84% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 7 | 1.56% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 8 | 1.42% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 9 | 1.41% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 10 | 1.22% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 11 | 1.21% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 3088` |
| 12 | 1.16% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 62` |
| 13 | 1.08% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1582` |
| 14 | 1.02% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3368` |
| 15 | 1.01% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 9489` |
| 16 | 0.96% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 6260` |
| 17 | 0.95% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3350` |
| 18 | 0.91% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 19 | 0.86% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13865` |
| 20 | 0.86% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 21 | 0.85% | `java.lang.ThreadLocal.get(Thread) line: 187` |
| 22 | 0.84% | `java.util.ArrayList.add(Object) line: 496` |
| 23 | 0.77% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 9567` |
| 24 | 0.74% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3707` |
| 25 | 0.74% | `java.util.ArrayList.add(Object) line: 497` |
| 26 | 0.73% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 6197` |
| 27 | 0.72% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 10112` |
| 28 | 0.71% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 29 | 0.69% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 30 | 0.69% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 9142` |
| 31 | 0.66% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 32 | 0.65% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3435` |
| 33 | 0.65% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 34 | 0.62% | `java.util.ArrayList.grow() line: 245` |
| 35 | 0.61% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 36 | 0.58% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 9320` |
| 37 | 0.57% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 14518` |
| 38 | 0.51% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 39 | 0.46% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3596` |
| 40 | 0.46% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 41 | 0.45% | `java.util.ArrayList.add(Object, Object[], int) line: 483` |
| 42 | 0.44% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 43 | 0.43% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 44 | 0.42% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3584` |
| 45 | 0.42% | `java.util.EnumMap.get(Object) line: 248` |
| 46 | 0.42% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 47 | 0.42% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 48 | 0.41% | `dev.planeta9091.centities.CeHooks.aiShadowPeek(Object) line: 9169` |
| 49 | 0.41% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 50 | 0.39% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 51 | 0.39% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 52 | 0.35% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 53 | 0.35% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$hasPlayers() line: 1347` |
| 54 | 0.35% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 55 | 0.32% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 6506` |
| 56 | 0.32% | `java.util.ArrayList.add(Object, Object[], int) line: 486` |
| 57 | 0.30% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 58 | 0.30% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 59 | 0.30% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8945` |
| 60 | 0.30% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 11.06% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 2 | 8.10% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 3088` |
| 3 | 3.21% | `java.util.ArrayList.add(Object) line: 497` |
| 4 | 2.84% | `java.util.ArrayList.get(int) line: 429` |
| 5 | 2.44% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 9582` |
| 6 | 2.37% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 7 | 2.32% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 8 | 1.89% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 9 | 1.78% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 10 | 1.61% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3739` |
| 11 | 1.57% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3436` |
| 12 | 1.52% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 9567` |
| 13 | 1.49% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 14 | 1.40% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 15 | 1.33% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14143` |
| 16 | 1.22% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 17 | 1.21% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1573` |
| 18 | 0.92% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 9488` |
| 19 | 0.91% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 20 | 0.88% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 21 | 0.88% | `java.lang.ThreadLocal.get() line: 171` |
| 22 | 0.87% | `net.minecraft.world.entity.ai.sensing.Sensing.tick()` |
| 23 | 0.87% | `dev.planeta9091.centities.CeHooks.goalSelectorTickRunning(GoalSelector, boolean) line: 9658` |
| 24 | 0.71% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 25 | 0.70% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 26 | 0.70% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 27 | 0.67% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 28 | 0.66% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 29 | 0.65% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 30 | 0.62% | `java.util.ArrayList.add(Object, Object[], int) line: 484` |

GC pauses: 0, total 0 ms

