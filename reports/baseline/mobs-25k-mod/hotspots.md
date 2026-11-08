# JVM hotspots (JFR ExecutionSample)

Samples: 16860

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.18% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2831` |
| 2 | 4.12% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 3 | 3.16% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getNextAcquire() line: 1837` |
| 4 | 1.39% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 5 | 1.32% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 6 | 1.25% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 7 | 1.22% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10806` |
| 8 | 1.08% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 1.01% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 10 | 0.91% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1312` |
| 11 | 0.90% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 12 | 0.87% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 13 | 0.87% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 14 | 0.84% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 15 | 0.80% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7592` |
| 16 | 0.70% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 5031` |
| 17 | 0.67% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3707` |
| 18 | 0.64% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 19 | 0.63% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 243` |
| 20 | 0.63% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 7423` |
| 21 | 0.62% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 22 | 0.60% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 23 | 0.57% | `java.util.HashMap.getNode(Object) line: 586` |
| 24 | 0.57% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 25 | 0.57% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 26 | 0.51% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 27 | 0.51% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 7245` |
| 28 | 0.50% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 29 | 0.48% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 8215` |
| 30 | 0.46% | `java.util.EnumMap.get(Object) line: 248` |
| 31 | 0.46% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 32 | 0.43% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7072` |
| 33 | 0.43% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 34 | 0.42% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 35 | 0.42% | `jdk.internal.misc.Unsafe.getLongUnaligned(Object, long, boolean) line: 3571` |
| 36 | 0.41% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 37 | 0.41% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 5094` |
| 38 | 0.39% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 39 | 0.39% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 40 | 0.39% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 41 | 0.38% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 42 | 0.38% | `dev.planeta9091.centities.CeHooks.stepOnIsNoOp(Block) line: 5006` |
| 43 | 0.37% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 44 | 0.37% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2712` |
| 45 | 0.37% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 46 | 0.36% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 47 | 0.36% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 48 | 0.34% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 49 | 0.34% | `java.util.concurrent.ConcurrentHashMap.computeIfAbsent(Object, Function) line: 1710` |
| 50 | 0.34% | `dev.planeta9091.centities.CeHooks.aiShadowPeek(Object) line: 7272` |
| 51 | 0.31% | `java.util.ArrayList.elementData(int) line: 412` |
| 52 | 0.31% | `java.lang.ref.Reference.refersToImpl(Object) line: 391` |
| 53 | 0.31% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 11395` |
| 54 | 0.30% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 55 | 0.29% | `java.lang.Enum.ordinal() line: 128` |
| 56 | 0.29% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 5423` |
| 57 | 0.28% | `net.minecraft.core.Holder$Reference.is(TagKey) line: 195` |
| 58 | 0.28% | `java.util.AbstractMap$SimpleImmutableEntry.getKey() line: 774` |
| 59 | 0.28% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 60 | 0.27% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2799` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.32% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2580` |
| 2 | 3.55% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2825` |
| 3 | 3.16% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 247` |
| 4 | 1.61% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 5 | 1.48% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 6 | 1.45% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3739` |
| 7 | 1.45% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 11030` |
| 8 | 1.45% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 9 | 1.30% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 10 | 1.25% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 11 | 1.24% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 12 | 1.08% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 13 | 1.06% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7685` |
| 14 | 0.95% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 15 | 0.95% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7670` |
| 16 | 0.91% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 17 | 0.87% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 18 | 0.82% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 19 | 0.79% | `dev.planeta9091.centities.CeHooks.goalSelectorTickRunning(GoalSelector, boolean) line: 7761` |
| 20 | 0.79% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 21 | 0.77% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 22 | 0.72% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7591` |
| 23 | 0.69% | `it.unimi.dsi.fastutil.objects.ReferenceCollections$UnmodifiableCollection.contains(Object) line: 344` |
| 24 | 0.69% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 25 | 0.68% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 26 | 0.66% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 27 | 0.66% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 28 | 0.65% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |
| 29 | 0.64% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 30 | 0.62% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |

GC pauses: 0, total 0 ms

