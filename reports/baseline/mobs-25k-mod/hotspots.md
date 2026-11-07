# JVM hotspots (JFR ExecutionSample)

Samples: 15404

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.19% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2831` |
| 2 | 2.93% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 3 | 1.63% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 4 | 1.57% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10935` |
| 5 | 1.50% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 6 | 1.28% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 7 | 1.24% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 8 | 1.16% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 9 | 1.15% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 0.99% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1312` |
| 11 | 0.97% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 12 | 0.90% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 13 | 0.84% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 14 | 0.82% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 15 | 0.80% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 7245` |
| 16 | 0.80% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7592` |
| 17 | 0.79% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 5031` |
| 18 | 0.73% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 19 | 0.67% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 20 | 0.63% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 21 | 0.63% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3707` |
| 22 | 0.62% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 237` |
| 23 | 0.60% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 7423` |
| 24 | 0.56% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 8215` |
| 25 | 0.56% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 26 | 0.50% | `java.util.HashMap.getNode(Object) line: 586` |
| 27 | 0.50% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 28 | 0.48% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 29 | 0.47% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 30 | 0.46% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.putAll(Map) line: 226` |
| 31 | 0.46% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 32 | 0.45% | `dev.planeta9091.centities.CeHooks.aiShadowPeek(Object) line: 7272` |
| 33 | 0.44% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 34 | 0.43% | `java.util.concurrent.ConcurrentHashMap.computeIfAbsent(Object, Function) line: 1737` |
| 35 | 0.43% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 36 | 0.43% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 5094` |
| 37 | 0.42% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2712` |
| 38 | 0.38% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7670` |
| 39 | 0.38% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 5340` |
| 40 | 0.38% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 41 | 0.38% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 42 | 0.35% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 43 | 0.35% | `jdk.internal.misc.Unsafe.getLongUnaligned(Object, long, boolean) line: 3571` |
| 44 | 0.34% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 11395` |
| 45 | 0.34% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 46 | 0.33% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 5420` |
| 47 | 0.32% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 48 | 0.32% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 49 | 0.32% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 50 | 0.32% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 51 | 0.32% | `java.lang.ThreadLocal$ThreadLocalMap.getEntry(ThreadLocal) line: 486` |
| 52 | 0.30% | `net.minecraft.core.Holder$Reference.is(TagKey) line: 195` |
| 53 | 0.30% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 54 | 0.30% | `dev.planeta9091.centities.CeHooks.stepOnIsNoOp(Block) line: 5006` |
| 55 | 0.29% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 56 | 0.29% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 57 | 0.28% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 58 | 0.28% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 5448` |
| 59 | 0.27% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 60 | 0.27% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.25% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2580` |
| 2 | 3.00% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 3 | 2.43% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2825` |
| 4 | 1.95% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3739` |
| 5 | 1.71% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 6 | 1.62% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 7 | 1.60% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 8 | 1.59% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 9 | 1.58% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 10 | 1.35% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 11 | 1.19% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7670` |
| 12 | 1.15% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 13 | 1.15% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 14 | 1.12% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7591` |
| 15 | 1.06% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 16 | 1.02% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 17 | 1.01% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 18 | 0.95% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7685` |
| 19 | 0.90% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 20 | 0.88% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 21 | 0.84% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 22 | 0.79% | `dev.planeta9091.centities.CeHooks.goalSelectorTickRunning(GoalSelector, boolean) line: 7761` |
| 23 | 0.77% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 24 | 0.75% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 25 | 0.69% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 26 | 0.68% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 7407` |
| 27 | 0.64% | `net.minecraft.world.entity.ai.sensing.Sensing.tick()` |
| 28 | 0.58% | `it.unimi.dsi.fastutil.objects.ReferenceCollections$UnmodifiableCollection.contains(Object) line: 344` |
| 29 | 0.58% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 30 | 0.57% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |

GC pauses: 0, total 0 ms

