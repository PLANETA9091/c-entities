# JVM hotspots (JFR ExecutionSample)

Samples: 23175

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 41.21% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 2 | 3.58% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1393` |
| 3 | 1.17% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 1.15% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2910` |
| 5 | 0.99% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 12596` |
| 6 | 0.88% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 7 | 0.76% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2674` |
| 8 | 0.75% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 9 | 0.72% | `java.util.ArrayList.add(Object, Object[], int) line: 485` |
| 10 | 0.72% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 11919` |
| 11 | 0.67% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 0.66% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 13 | 0.63% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 14 | 0.53% | `java.util.HashMap.getNode(Object) line: 586` |
| 15 | 0.49% | `dev.planeta9091.centities.CeHooks.pushDispatch(Entity) line: 4374` |
| 16 | 0.47% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 17 | 0.47% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 18 | 0.47% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7416` |
| 19 | 0.46% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 20 | 0.45% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 21 | 0.44% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 22 | 0.39% | `java.lang.ThreadLocal.get(Thread) line: 187` |
| 23 | 0.36% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7960` |
| 24 | 0.36% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 5234` |
| 25 | 0.36% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 26 | 0.35% | `java.util.ArrayList.add(Object) line: 497` |
| 27 | 0.35% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 28 | 0.31% | `java.util.EnumMap.get(Object) line: 248` |
| 29 | 0.31% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 30 | 0.30% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2911` |
| 31 | 0.30% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 7613` |
| 32 | 0.30% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 33 | 0.29% | `java.util.ArrayList.add(Object) line: 496` |
| 34 | 0.29% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 35 | 0.28% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 36 | 0.28% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 8583` |
| 37 | 0.27% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 5297` |
| 38 | 0.27% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 39 | 0.27% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 62` |
| 40 | 0.26% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 41 | 0.26% | `java.util.ArrayList.add(Object, Object[], int) line: 483` |
| 42 | 0.26% | `java.util.ArrayList.add(Object, Object[], int) line: 486` |
| 43 | 0.25% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 44 | 0.25% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 45 | 0.24% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 46 | 0.24% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 47 | 0.24% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1394` |
| 48 | 0.23% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 49 | 0.23% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 50 | 0.22% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2943` |
| 51 | 0.22% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 52 | 0.22% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 7791` |
| 53 | 0.22% | `java.util.ArrayList.elementData(int) line: 412` |
| 54 | 0.21% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 55 | 0.21% | `ca.spottedleaf.moonrise.patches.starlight.light.SWMRNibbleArray.getVisible(int) line: 412` |
| 56 | 0.21% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 5543` |
| 57 | 0.20% | `net.minecraft.world.level.levelgen.BitRandomSource.nextInt(int) line: 31` |
| 58 | 0.20% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 59 | 0.18% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 60 | 0.18% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 41.21% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 2 | 4.09% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 3 | 2.31% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2674` |
| 4 | 1.24% | `java.util.ArrayList.add(Object) line: 497` |
| 5 | 1.17% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 1.16% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 7 | 1.00% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 8 | 0.98% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 9 | 0.91% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12143` |
| 10 | 0.76% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 11 | 0.76% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 12 | 0.76% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1385` |
| 13 | 0.72% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 14 | 0.71% | `net.minecraft.world.entity.LivingEntity.pushEntities()` |
| 15 | 0.71% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 16 | 0.65% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2979` |
| 17 | 0.63% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 18 | 0.63% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 8053` |
| 19 | 0.62% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 20 | 0.62% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3739` |
| 21 | 0.58% | `java.util.HashMap.get(Object) line: 565` |
| 22 | 0.51% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2515` |
| 23 | 0.50% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 24 | 0.50% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 25 | 0.48% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 26 | 0.47% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 8038` |
| 27 | 0.44% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 28 | 0.41% | `java.lang.ThreadLocal.get() line: 171` |
| 29 | 0.41% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7959` |
| 30 | 0.41% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |

GC pauses: 0, total 0 ms

