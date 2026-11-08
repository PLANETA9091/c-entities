# JVM hotspots (JFR ExecutionSample)

Samples: 14856

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 1.61% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 2 | 1.58% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 3 | 1.51% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 4 | 1.40% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 5 | 1.37% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10841` |
| 6 | 1.25% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 7 | 1.20% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1331` |
| 8 | 1.11% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 1.08% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 10 | 1.05% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3707` |
| 11 | 1.01% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7627` |
| 12 | 0.84% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 13 | 0.80% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 7458` |
| 14 | 0.75% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 15 | 0.67% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 16 | 0.66% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 17 | 0.66% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 18 | 0.63% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 7280` |
| 19 | 0.63% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 20 | 0.62% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 5066` |
| 21 | 0.60% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 22 | 0.58% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 237` |
| 23 | 0.55% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 24 | 0.55% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 25 | 0.53% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 11430` |
| 26 | 0.53% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.<init>(Map, float) line: 124` |
| 27 | 0.52% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 28 | 0.48% | `java.util.EnumMap.get(Object) line: 248` |
| 29 | 0.48% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 30 | 0.48% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 8250` |
| 31 | 0.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 32 | 0.45% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 33 | 0.44% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 34 | 0.44% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 35 | 0.44% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 36 | 0.43% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 37 | 0.43% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 38 | 0.42% | `java.util.concurrent.ConcurrentHashMap.computeIfAbsent(Object, Function) line: 1737` |
| 39 | 0.42% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2731` |
| 40 | 0.41% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 41 | 0.41% | `jdk.internal.misc.Unsafe.getLongUnaligned(Object, long, boolean) line: 3571` |
| 42 | 0.40% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 43 | 0.40% | `dev.planeta9091.centities.CeHooks.stepOnIsNoOp(Block) line: 5041` |
| 44 | 0.39% | `java.lang.ref.Reference.refersToImpl(Object) line: 391` |
| 45 | 0.38% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 46 | 0.37% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 47 | 0.37% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 48 | 0.36% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2821` |
| 49 | 0.36% | `dev.planeta9091.centities.CeHooks.aiShadowPeek(Object) line: 7307` |
| 50 | 0.36% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 5455` |
| 51 | 0.34% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 52 | 0.34% | `net.minecraft.core.Holder$Reference.boundTags() line: 186` |
| 53 | 0.34% | `net.minecraft.core.Holder$Reference.is(TagKey) line: 195` |
| 54 | 0.34% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 55 | 0.34% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 5129` |
| 56 | 0.32% | `net.minecraft.util.SimpleBitStorage.get(int) line: 350` |
| 57 | 0.32% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 58 | 0.32% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 59 | 0.32% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 60 | 0.32% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7107` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 2.65% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 2 | 1.86% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 3 | 1.74% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3739` |
| 4 | 1.74% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 5 | 1.60% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2599` |
| 6 | 1.56% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 11065` |
| 7 | 1.51% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 8 | 1.44% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 9 | 1.32% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 10 | 1.31% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7705` |
| 11 | 1.27% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 12 | 1.24% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7720` |
| 13 | 1.23% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 14 | 1.16% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 15 | 1.11% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 16 | 1.11% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 17 | 1.10% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 18 | 1.01% | `dev.planeta9091.centities.CeHooks.goalSelectorTickRunning(GoalSelector, boolean) line: 7796` |
| 19 | 0.97% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 20 | 0.95% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7626` |
| 21 | 0.92% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 22 | 0.85% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 23 | 0.85% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 24 | 0.74% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 25 | 0.74% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 26 | 0.69% | `it.unimi.dsi.fastutil.objects.ReferenceCollections$UnmodifiableCollection.contains(Object) line: 344` |
| 27 | 0.67% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |
| 28 | 0.67% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 29 | 0.63% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 5376` |
| 30 | 0.60% | `dev.planeta9091.centities.CeHooks.mirrorTyped(Level, Class, AABB, Predicate) line: 1416` |

GC pauses: 0, total 0 ms

