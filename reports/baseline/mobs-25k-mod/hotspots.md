# JVM hotspots (JFR ExecutionSample)

Samples: 22686

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 41.68% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 4011` |
| 2 | 4.93% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 4083` |
| 3 | 1.83% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3993` |
| 4 | 1.36% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 4025` |
| 5 | 1.02% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 6 | 1.00% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 0.83% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 8 | 0.83% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 9 | 0.82% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 4010` |
| 10 | 0.82% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 11 | 0.73% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 62` |
| 12 | 0.69% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 0.63% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 11449` |
| 14 | 0.62% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 16241` |
| 15 | 0.60% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 16 | 0.59% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 17 | 0.52% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 11102` |
| 18 | 0.49% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 7353` |
| 19 | 0.49% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 11527` |
| 20 | 0.46% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 21 | 0.45% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 7290` |
| 22 | 0.42% | `java.util.HashMap.getNode(Object) line: 586` |
| 23 | 0.42% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3708` |
| 24 | 0.38% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 25 | 0.38% | `java.util.EnumMap.get(Object) line: 248` |
| 26 | 0.37% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 27 | 0.37% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10841` |
| 28 | 0.34% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 4001` |
| 29 | 0.33% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 11280` |
| 30 | 0.33% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 31 | 0.32% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 32 | 0.32% | `dev.planeta9091.centities.CeHooks.pushDispatch(Entity) line: 5544` |
| 33 | 0.31% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 34 | 0.30% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 35 | 0.30% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 12072` |
| 36 | 0.30% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 37 | 0.30% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 38 | 0.28% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 39 | 0.27% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3597` |
| 40 | 0.27% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$hasPlayers() line: 1347` |
| 41 | 0.26% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 42 | 0.26% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 43 | 0.26% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 44 | 0.24% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 7667` |
| 45 | 0.23% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 46 | 0.23% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 47 | 0.23% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 48 | 0.22% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 49 | 0.22% | `java.lang.String.hashCode() line: 2430` |
| 50 | 0.21% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 4043` |
| 51 | 0.21% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 52 | 0.20% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 53 | 0.20% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 54 | 0.20% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 55 | 0.19% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 56 | 0.19% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 57 | 0.19% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 58 | 0.19% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 59 | 0.19% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 60 | 0.19% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 51.43% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 2113` |
| 2 | 1.41% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 11542` |
| 3 | 1.38% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 4 | 1.38% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 1.10% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 6 | 1.06% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2822` |
| 7 | 0.98% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 8 | 0.86% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 11527` |
| 9 | 0.82% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 10 | 0.79% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 16521` |
| 11 | 0.76% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 12 | 0.74% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3740` |
| 13 | 0.71% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 11448` |
| 14 | 0.58% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 15 | 0.50% | `net.minecraft.world.entity.LivingEntity.pushEntities()` |
| 16 | 0.47% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 17 | 0.45% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 18 | 0.44% | `java.util.HashMap.get(Object) line: 565` |
| 19 | 0.43% | `dev.planeta9091.centities.CeHooks.goalSelectorTickRunning(GoalSelector, boolean) line: 11618` |
| 20 | 0.43% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 21 | 0.39% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 22 | 0.39% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 23 | 0.38% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 24 | 0.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 25 | 0.37% | `net.minecraft.world.entity.ai.sensing.Sensing.tick()` |
| 26 | 0.32% | `jdk.internal.reflect.DirectMethodHandleAccessor.invoke(Object, Object[]) line: 104` |
| 27 | 0.32% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 28 | 0.30% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2515` |
| 29 | 0.30% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 30 | 0.29% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |

GC pauses: 0, total 0 ms

