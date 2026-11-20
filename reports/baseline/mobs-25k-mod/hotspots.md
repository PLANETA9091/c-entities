# JVM hotspots (JFR ExecutionSample)

Samples: 22790

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 40.13% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3881` |
| 2 | 5.62% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3953` |
| 3 | 1.95% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3863` |
| 4 | 1.53% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3895` |
| 5 | 1.41% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 6 | 1.24% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3880` |
| 7 | 0.99% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 8 | 0.93% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 0.90% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 10 | 0.90% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 11 | 0.80% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 12 | 0.66% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 62` |
| 13 | 0.65% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 10694` |
| 14 | 0.61% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 6917` |
| 15 | 0.60% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 10347` |
| 16 | 0.55% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 17 | 0.53% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 18 | 0.51% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 19 | 0.48% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3707` |
| 20 | 0.47% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15283` |
| 21 | 0.47% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 22 | 0.43% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3879` |
| 23 | 0.42% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 10772` |
| 24 | 0.41% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 11317` |
| 25 | 0.41% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 26 | 0.39% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 27 | 0.38% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 6854` |
| 28 | 0.38% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3871` |
| 29 | 0.37% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 10525` |
| 30 | 0.37% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 31 | 0.34% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 32 | 0.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 33 | 0.30% | `java.util.HashMap.getNode(Object) line: 586` |
| 34 | 0.30% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 35 | 0.29% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 36 | 0.27% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3596` |
| 37 | 0.27% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10067` |
| 38 | 0.26% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 16062` |
| 39 | 0.26% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 40 | 0.26% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 41 | 0.25% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 42 | 0.25% | `dev.planeta9091.centities.CeHooks.pushDispatch(Entity) line: 5388` |
| 43 | 0.25% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 44 | 0.25% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 45 | 0.25% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 46 | 0.25% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 47 | 0.24% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 48 | 0.24% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 49 | 0.23% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3733` |
| 50 | 0.23% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 51 | 0.22% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 52 | 0.22% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 7163` |
| 53 | 0.22% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 54 | 0.21% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 55 | 0.20% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 237` |
| 56 | 0.19% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 57 | 0.19% | `dev.planeta9091.centities.CeHooks.aiShadowPeek(Object) line: 10374` |
| 58 | 0.19% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 59 | 0.18% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 60 | 0.18% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 51.87% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 2010` |
| 2 | 1.45% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 3 | 1.42% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 4 | 1.42% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 10787` |
| 5 | 1.24% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 1.04% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 7 | 1.04% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 8 | 0.93% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 0.92% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 10 | 0.88% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 10772` |
| 11 | 0.79% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3739` |
| 12 | 0.78% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 10693` |
| 13 | 0.56% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 14 | 0.51% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 15 | 0.50% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 16 | 0.50% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 15563` |
| 17 | 0.50% | `net.minecraft.world.entity.ai.sensing.Sensing.tick()` |
| 18 | 0.50% | `dev.planeta9091.centities.CeHooks.goalSelectorTickRunning(GoalSelector, boolean) line: 10863` |
| 19 | 0.48% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 20 | 0.41% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 21 | 0.41% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 22 | 0.40% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 23 | 0.37% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 24 | 0.36% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 7164` |
| 25 | 0.34% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 26 | 0.34% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 27 | 0.34% | `java.util.HashMap.get(Object) line: 565` |
| 28 | 0.33% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 29 | 0.32% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 30 | 0.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |

GC pauses: 0, total 0 ms

