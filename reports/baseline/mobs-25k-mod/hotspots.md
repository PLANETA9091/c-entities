# JVM hotspots (JFR ExecutionSample)

Samples: 14941

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.92% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 2 | 1.63% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10854` |
| 3 | 1.62% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 4 | 1.50% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 5 | 1.47% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 6 | 1.29% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 7 | 1.12% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 8 | 1.08% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 0.95% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 10 | 0.92% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 11 | 0.91% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 12 | 0.84% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7640` |
| 13 | 0.84% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 7471` |
| 14 | 0.75% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 243` |
| 15 | 0.74% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 16 | 0.74% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 17 | 0.74% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 18 | 0.72% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3707` |
| 19 | 0.72% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 20 | 0.62% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 21 | 0.62% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 7293` |
| 22 | 0.61% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 5079` |
| 23 | 0.58% | `java.util.EnumMap.get(Object) line: 248` |
| 24 | 0.56% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 25 | 0.56% | `java.util.concurrent.ConcurrentHashMap.computeIfAbsent(Object, Function) line: 1737` |
| 26 | 0.56% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 27 | 0.54% | `dev.planeta9091.centities.CeHooks.aiShadowPeek(Object) line: 7320` |
| 28 | 0.52% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 29 | 0.52% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 30 | 0.51% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 31 | 0.50% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 32 | 0.50% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 33 | 0.48% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 8263` |
| 34 | 0.47% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 35 | 0.46% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2744` |
| 36 | 0.45% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 37 | 0.44% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 38 | 0.44% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 5142` |
| 39 | 0.41% | `net.minecraft.core.Holder$Reference.is(TagKey) line: 195` |
| 40 | 0.41% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 41 | 0.39% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1313` |
| 42 | 0.39% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 5388` |
| 43 | 0.38% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 44 | 0.37% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 45 | 0.37% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 11443` |
| 46 | 0.36% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 47 | 0.36% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 48 | 0.35% | `com.google.common.collect.RegularImmutableMap.get(Object, ImmutableMapEntry[], int) line: 273` |
| 49 | 0.35% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 50 | 0.33% | `dev.planeta9091.centities.CeHooks.stepOnIsNoOp(Block) line: 5054` |
| 51 | 0.33% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 52 | 0.32% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 53 | 0.31% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 54 | 0.31% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 55 | 0.31% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 5471` |
| 56 | 0.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 57 | 0.29% | `jdk.internal.misc.Unsafe.getLongUnaligned(Object, long, boolean) line: 3571` |
| 58 | 0.29% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 59 | 0.29% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 60 | 0.28% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7718` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 3.92% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 2 | 2.72% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 3 | 1.93% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3739` |
| 4 | 1.83% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 11078` |
| 5 | 1.82% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 6 | 1.77% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 7 | 1.69% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 8 | 1.46% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 9 | 1.28% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2612` |
| 10 | 1.26% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 11 | 1.22% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 12 | 1.18% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7718` |
| 13 | 1.16% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7733` |
| 14 | 1.09% | `dev.planeta9091.centities.CeHooks.goalSelectorTickRunning(GoalSelector, boolean) line: 7809` |
| 15 | 1.08% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 16 | 1.01% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 17 | 1.01% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 18 | 0.98% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 19 | 0.92% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 20 | 0.90% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7639` |
| 21 | 0.90% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 22 | 0.84% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 23 | 0.79% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 24 | 0.74% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 7455` |
| 25 | 0.74% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 26 | 0.72% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |
| 27 | 0.71% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 28 | 0.68% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 29 | 0.67% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 30 | 0.63% | `com.google.common.collect.RegularImmutableMap.get(Object) line: 262` |

GC pauses: 0, total 0 ms

