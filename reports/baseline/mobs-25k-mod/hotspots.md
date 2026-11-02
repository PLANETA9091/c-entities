# JVM hotspots (JFR ExecutionSample)

Samples: 13326

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 2.36% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10182` |
| 2 | 1.69% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 3 | 1.58% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 4 | 1.34% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9580` |
| 5 | 1.34% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 1.32% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 1.31% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 8 | 1.27% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 9 | 1.25% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 10 | 1.19% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 11 | 1.15% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 12 | 0.98% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6812` |
| 13 | 0.97% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6880` |
| 14 | 0.93% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 15 | 0.85% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 16 | 0.77% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 17 | 0.76% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 18 | 0.75% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 19 | 0.69% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 6478` |
| 20 | 0.67% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 6656` |
| 21 | 0.67% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4437` |
| 22 | 0.65% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7422` |
| 23 | 0.64% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 24 | 0.63% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 25 | 0.61% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 26 | 0.58% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 27 | 0.56% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 28 | 0.50% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 29 | 0.48% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 30 | 0.47% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 10094` |
| 31 | 0.47% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 32 | 0.45% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6305` |
| 33 | 0.45% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4374` |
| 34 | 0.44% | `net.minecraft.core.Holder$Reference.is(TagKey) line: 195` |
| 35 | 0.43% | `java.util.EnumMap.get(Object) line: 248` |
| 36 | 0.43% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 37 | 0.43% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 38 | 0.43% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 239` |
| 39 | 0.42% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 990` |
| 40 | 0.41% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4760` |
| 41 | 0.40% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 42 | 0.39% | `java.lang.Enum.ordinal() line: 128` |
| 43 | 0.39% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap$MapIterator.nextEntry() line: 655` |
| 44 | 0.38% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 45 | 0.38% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4680` |
| 46 | 0.38% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 47 | 0.38% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 48 | 0.38% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 49 | 0.36% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 50 | 0.36% | `dev.planeta9091.centities.CeHooks.aiShadowPeek(Object) line: 6505` |
| 51 | 0.35% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 52 | 0.35% | `java.util.concurrent.ConcurrentHashMap.computeIfAbsent(Object, Function) line: 1737` |
| 53 | 0.35% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 54 | 0.35% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 55 | 0.35% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 56 | 0.34% | `java.lang.ref.Reference.refersToImpl(Object) line: 391` |
| 57 | 0.34% | `net.minecraft.world.entity.ai.goal.Goal.getFlags() line: 60` |
| 58 | 0.34% | `java.util.AbstractMap$SimpleImmutableEntry.getKey() line: 774` |
| 59 | 0.33% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 60 | 0.33% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 228` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.04% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 2 | 2.39% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 3 | 2.11% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 4 | 1.90% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6880` |
| 5 | 1.82% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 6 | 1.67% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 7 | 1.66% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6895` |
| 8 | 1.61% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 9 | 1.52% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 10 | 1.52% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9728` |
| 11 | 1.43% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 12 | 1.43% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 13 | 1.34% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 14 | 1.21% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 15 | 1.04% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6811` |
| 16 | 1.03% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 17 | 0.98% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 18 | 0.98% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 19 | 0.90% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 20 | 0.86% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 21 | 0.86% | `dev.planeta9091.centities.CeHooks.goalSelectorTickRunning(GoalSelector, boolean) line: 6968` |
| 22 | 0.85% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 23 | 0.83% | `net.minecraft.world.entity.ai.sensing.Sensing.tick()` |
| 24 | 0.83% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 25 | 0.79% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 26 | 0.72% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3584` |
| 27 | 0.68% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4681` |
| 28 | 0.67% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 29 | 0.63% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 30 | 0.63% | `net.minecraft.world.item.enchantment.Enchantment.tick(ServerLevel, int, EnchantedItemInUse, Entity) line: 400` |

GC pauses: 0, total 0 ms

