# JVM hotspots (JFR ExecutionSample)

Samples: 12509

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.21% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 2.62% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 3 | 1.99% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9806` |
| 4 | 1.98% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 5 | 1.97% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 6 | 1.86% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 7 | 1.85% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 8 | 1.65% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 1.62% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 10 | 1.52% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 11 | 1.38% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 12 | 0.88% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 13 | 0.88% | `java.lang.ref.Reference.get() line: 361` |
| 14 | 0.88% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 15 | 0.85% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 16 | 0.82% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 17 | 0.76% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 18 | 0.75% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 19 | 0.66% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4311` |
| 20 | 0.66% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4374` |
| 21 | 0.64% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 22 | 0.64% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 23 | 0.60% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 24 | 0.57% | `java.lang.Enum.ordinal() line: 128` |
| 25 | 0.54% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9331` |
| 26 | 0.54% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7233` |
| 27 | 0.51% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 28 | 0.51% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 4464` |
| 29 | 0.50% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 30 | 0.50% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 31 | 0.49% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 32 | 0.45% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 33 | 0.44% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 34 | 0.42% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 35 | 0.42% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 9751` |
| 36 | 0.42% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 37 | 0.41% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 38 | 0.40% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4641` |
| 39 | 0.40% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 40 | 0.38% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 41 | 0.38% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 42 | 0.38% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 43 | 0.38% | `java.util.EnumMap.get(Object) line: 248` |
| 44 | 0.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ChunkSlicesRegion.get(int) line: 935` |
| 45 | 0.37% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 46 | 0.37% | `net.minecraft.core.Holder$Reference.is(TagKey) line: 195` |
| 47 | 0.37% | `java.util.concurrent.ConcurrentHashMap.computeIfAbsent(Object, Function) line: 1737` |
| 48 | 0.36% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 49 | 0.36% | `java.util.HashMap.getNode(Object) line: 586` |
| 50 | 0.36% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 51 | 0.36% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 52 | 0.35% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6203` |
| 53 | 0.35% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 54 | 0.35% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 55 | 0.34% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 56 | 0.34% | `java.util.AbstractMap$SimpleImmutableEntry.getKey() line: 774` |
| 57 | 0.34% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4718` |
| 58 | 0.32% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 59 | 0.30% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 60 | 0.28% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 993` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 2.13% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 2 | 2.11% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 3 | 1.99% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 4 | 1.93% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 5 | 1.79% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 1.65% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 1.54% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 8 | 1.54% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 9 | 1.49% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 10 | 1.45% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 11 | 1.42% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 12 | 1.41% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 13 | 1.34% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9321` |
| 14 | 1.33% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 15 | 1.23% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 16 | 1.18% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 17 | 1.16% | `net.minecraft.world.entity.Mob.serverAiStep() line: 839` |
| 18 | 1.03% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 19 | 0.98% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 20 | 0.91% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 21 | 0.90% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9424` |
| 22 | 0.88% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 4484` |
| 23 | 0.85% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 24 | 0.82% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 25 | 0.75% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 26 | 0.74% | `net.minecraft.world.entity.Mob.serverAiStep() line: 836` |
| 27 | 0.66% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 28 | 0.65% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2094` |
| 29 | 0.64% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |
| 30 | 0.64% | `net.minecraft.world.entity.ai.sensing.Sensing.tick()` |

GC pauses: 0, total 0 ms

