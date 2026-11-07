# JVM hotspots (JFR ExecutionSample)

Samples: 15449

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.66% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2797` |
| 2 | 2.16% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 3 | 1.64% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 4 | 1.32% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 5 | 1.24% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10701` |
| 6 | 1.24% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 7 | 1.20% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 8 | 1.09% | `dev.planeta9091.centities.CeHooks.ordSortVanilla(ArrayList) line: 2488` |
| 9 | 1.09% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 10 | 1.05% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 11 | 1.02% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 12 | 0.89% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7487` |
| 13 | 0.88% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 14 | 0.83% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 15 | 0.71% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3707` |
| 16 | 0.68% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 17 | 0.68% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 7318` |
| 18 | 0.68% | `dev.planeta9091.centities.CeHooks.ordKey(Entity, int) line: 2343` |
| 19 | 0.67% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 20 | 0.65% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 21 | 0.65% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 22 | 0.63% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 23 | 0.62% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4926` |
| 24 | 0.56% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2690` |
| 25 | 0.54% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 26 | 0.53% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 27 | 0.53% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 28 | 0.52% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 29 | 0.50% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 7140` |
| 30 | 0.48% | `jdk.internal.misc.Unsafe.getLongUnaligned(Object, long, boolean) line: 3571` |
| 31 | 0.47% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 5235` |
| 32 | 0.46% | `java.util.HashMap.getNode(Object) line: 586` |
| 33 | 0.45% | `java.util.EnumMap.get(Object) line: 248` |
| 34 | 0.43% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 35 | 0.43% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 36 | 0.42% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 37 | 0.41% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 8110` |
| 38 | 0.40% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$hasPlayers() line: 1347` |
| 39 | 0.39% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 11290` |
| 40 | 0.39% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 41 | 0.39% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 42 | 0.39% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 43 | 0.38% | `java.util.concurrent.ConcurrentHashMap.computeIfAbsent(Object, Function) line: 1737` |
| 44 | 0.38% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 45 | 0.38% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 46 | 0.38% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 228` |
| 47 | 0.37% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 48 | 0.37% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 49 | 0.37% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4989` |
| 50 | 0.36% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 51 | 0.36% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 52 | 0.36% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 53 | 0.36% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 54 | 0.35% | `net.minecraft.core.Holder$Reference.is(TagKey) line: 195` |
| 55 | 0.33% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 56 | 0.32% | `jdk.internal.misc.Unsafe.convEndian(boolean, long) line: 3835` |
| 57 | 0.31% | `java.lang.ref.Reference.refersToImpl(Object) line: 391` |
| 58 | 0.30% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 59 | 0.30% | `net.minecraft.world.entity.Entity.moonrise$getSectionZ() line: 463` |
| 60 | 0.28% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ChunkSlicesRegion.get(int) line: 935` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.04% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2558` |
| 2 | 2.47% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 3 | 1.82% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2791` |
| 4 | 1.73% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3739` |
| 5 | 1.66% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 6 | 1.39% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10925` |
| 7 | 1.39% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2850` |
| 8 | 1.38% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 9 | 1.31% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 10 | 1.30% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 11 | 1.25% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 12 | 1.23% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7580` |
| 13 | 1.10% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 14 | 1.09% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7565` |
| 15 | 1.06% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 16 | 1.06% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 17 | 1.01% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 18 | 0.95% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 19 | 0.93% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 20 | 0.89% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7486` |
| 21 | 0.88% | `dev.planeta9091.centities.CeHooks.goalSelectorTickRunning(GoalSelector, boolean) line: 7656` |
| 22 | 0.83% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 23 | 0.82% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 24 | 0.81% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 25 | 0.74% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 26 | 0.74% | `dev.planeta9091.centities.CeHooks.ordSortVanilla(ArrayList) line: 2489` |
| 27 | 0.70% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 28 | 0.68% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 29 | 0.68% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3595` |
| 30 | 0.67% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |

GC pauses: 0, total 0 ms

