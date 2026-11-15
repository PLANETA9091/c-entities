# JVM hotspots (JFR ExecutionSample)

Samples: 27487

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 28.11% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1454` |
| 2 | 6.19% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3193` |
| 3 | 4.34% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3194` |
| 4 | 1.66% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |
| 5 | 1.15% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 0.81% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 7 | 0.81% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13538` |
| 8 | 0.70% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 14372` |
| 9 | 0.69% | `java.util.ArrayList.add(Object, Object[], int) line: 485` |
| 10 | 0.66% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 11 | 0.62% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 0.62% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 13 | 0.61% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 14 | 0.58% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8646` |
| 15 | 0.56% | `java.util.ArrayList.add(Object) line: 497` |
| 16 | 0.55% | `java.util.ArrayList.add(Object, Object[], int) line: 483` |
| 17 | 0.53% | `java.util.HashMap.getNode(Object) line: 586` |
| 18 | 0.49% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2957` |
| 19 | 0.47% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 20 | 0.46% | `dev.planeta9091.centities.CeHooks.pushDispatch(Entity) line: 4659` |
| 21 | 0.45% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 22 | 0.45% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 23 | 0.44% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1455` |
| 24 | 0.42% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 25 | 0.40% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 12913` |
| 26 | 0.39% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 27 | 0.38% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3261` |
| 28 | 0.37% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 29 | 0.37% | `java.lang.ThreadLocal.get(Thread) line: 187` |
| 30 | 0.37% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 31 | 0.36% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 32 | 0.35% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 33 | 0.35% | `java.lang.ref.Reference.refersToImpl(Object) line: 391` |
| 34 | 0.33% | `java.util.ArrayList.add(Object, Object[], int) line: 486` |
| 35 | 0.32% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 36 | 0.32% | `java.util.ArrayList.elementData(int) line: 412` |
| 37 | 0.31% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3707` |
| 38 | 0.31% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 39 | 0.31% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 40 | 0.29% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 8843` |
| 41 | 0.26% | `dev.planeta9091.centities.CeHooks.probeDeltaCommon(Entity, double, double, double) line: 1768` |
| 42 | 0.26% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 9190` |
| 43 | 0.26% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 44 | 0.25% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 45 | 0.24% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 46 | 0.24% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 47 | 0.23% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3175` |
| 48 | 0.23% | `ca.spottedleaf.moonrise.patches.starlight.light.SWMRNibbleArray.getVisible(int) line: 412` |
| 49 | 0.23% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 9021` |
| 50 | 0.22% | `java.lang.ThreadLocal.get() line: 171` |
| 51 | 0.22% | `dev.planeta9091.centities.CeHooks.stepOnIsNoOp(Block) line: 5928` |
| 52 | 0.21% | `java.util.ArrayList.add(Object) line: 496` |
| 53 | 0.21% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 54 | 0.21% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder.getChunkStatus() line: 1112` |
| 55 | 0.21% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightInterface.getSkyLightValue(BlockPos, ChunkAccess) line: 252` |
| 56 | 0.21% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 57 | 0.21% | `java.util.ArrayList.grow() line: 245` |
| 58 | 0.20% | `java.util.EnumMap.get(Object) line: 248` |
| 59 | 0.20% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 60 | 0.20% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 28.68% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 2 | 11.84% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2957` |
| 3 | 1.66% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 4 | 1.57% | `java.util.ArrayList.add(Object) line: 497` |
| 5 | 1.15% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 1.02% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13816` |
| 7 | 0.83% | `net.minecraft.world.entity.LivingEntity.pushEntities()` |
| 8 | 0.77% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3262` |
| 9 | 0.76% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 10 | 0.70% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean)` |
| 11 | 0.70% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 12 | 0.65% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 13 | 0.64% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 14 | 0.63% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 15 | 0.62% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 16 | 0.59% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2515` |
| 17 | 0.57% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 18 | 0.57% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 19 | 0.57% | `java.util.HashMap.get(Object) line: 565` |
| 20 | 0.56% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 21 | 0.56% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3739` |
| 22 | 0.53% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 23 | 0.49% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1446` |
| 24 | 0.49% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 9268` |
| 25 | 0.47% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 9283` |
| 26 | 0.46% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 27 | 0.46% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 28 | 0.45% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 29 | 0.44% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 30 | 0.44% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |

GC pauses: 0, total 0 ms

