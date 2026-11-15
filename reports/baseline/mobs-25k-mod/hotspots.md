# JVM hotspots (JFR ExecutionSample)

Samples: 18479

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 12.18% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1410` |
| 2 | 4.10% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3142` |
| 3 | 1.73% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 4 | 1.72% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 5 | 1.27% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 6 | 1.25% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 62` |
| 7 | 1.20% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 8 | 1.20% | `java.util.ArrayList.add(Object, Object[], int) line: 485` |
| 9 | 1.07% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 10 | 1.04% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3143` |
| 11 | 0.99% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 12 | 0.95% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 8957` |
| 13 | 0.90% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 14 | 0.89% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 15 | 0.83% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 8879` |
| 16 | 0.75% | `java.util.Optional.ifPresent(Consumer) line: 178` |
| 17 | 0.72% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 18 | 0.70% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 19 | 0.68% | `jdk.internal.misc.Unsafe.getLongUnaligned(Object, long, boolean) line: 3571` |
| 20 | 0.65% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2906` |
| 21 | 0.64% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 237` |
| 22 | 0.63% | `java.util.ArrayList.elementData(int) line: 412` |
| 23 | 0.62% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 24 | 0.58% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 8710` |
| 25 | 0.55% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3707` |
| 26 | 0.54% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 5853` |
| 27 | 0.53% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 28 | 0.51% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1411` |
| 29 | 0.49% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 30 | 0.47% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 31 | 0.46% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 5916` |
| 32 | 0.45% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13003` |
| 33 | 0.44% | `java.util.concurrent.ConcurrentHashMap.computeIfAbsent(Object, Function) line: 1737` |
| 34 | 0.41% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 35 | 0.41% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 36 | 0.41% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 37 | 0.40% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 38 | 0.40% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 39 | 0.39% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 40 | 0.38% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 41 | 0.36% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 42 | 0.35% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 43 | 0.35% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 9502` |
| 44 | 0.35% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 320` |
| 45 | 0.34% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 46 | 0.34% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 47 | 0.34% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3125` |
| 48 | 0.32% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 49 | 0.32% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 50 | 0.31% | `java.util.ArrayList.add(Object) line: 497` |
| 51 | 0.31% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 52 | 0.31% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 53 | 0.30% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 13620` |
| 54 | 0.30% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 8532` |
| 55 | 0.30% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1409` |
| 56 | 0.29% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 6245` |
| 57 | 0.28% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 58 | 0.28% | `java.lang.String.hashCode() line: 2430` |
| 59 | 0.27% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 60 | 0.27% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 13.12% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 2 | 7.08% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2906` |
| 3 | 2.48% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 4 | 2.27% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 5 | 2.20% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 8972` |
| 6 | 1.82% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 7 | 1.60% | `java.util.ArrayList.add(Object) line: 497` |
| 8 | 1.59% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 9 | 1.47% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 10 | 1.37% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 11 | 1.37% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3739` |
| 12 | 1.30% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 8957` |
| 13 | 1.15% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 14 | 1.05% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 15 | 0.99% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 16 | 0.89% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 17 | 0.87% | `dev.planeta9091.centities.CeHooks.goalSelectorTickRunning(GoalSelector, boolean) line: 9048` |
| 18 | 0.83% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 19 | 0.73% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13245` |
| 20 | 0.72% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 21 | 0.71% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 22 | 0.70% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 23 | 0.68% | `jdk.internal.misc.ScopedMemoryAccess.getLongUnalignedInternal(MemorySessionImpl, Object, long, boolean) line: 3861` |
| 24 | 0.67% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 25 | 0.67% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 26 | 0.65% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1402` |
| 27 | 0.63% | `java.util.ArrayList.get(int) line: 429` |
| 28 | 0.60% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 6163` |
| 29 | 0.59% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 30 | 0.56% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 3211` |

GC pauses: 0, total 0 ms

