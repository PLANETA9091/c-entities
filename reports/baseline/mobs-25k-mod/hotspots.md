# JVM hotspots (JFR ExecutionSample)

Samples: 10141

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.90% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getNextAcquire() line: 1837` |
| 2 | 2.47% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 2.23% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 4 | 2.04% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10239` |
| 5 | 1.74% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 6 | 1.54% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 7 | 1.50% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 8 | 1.46% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 9 | 1.44% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9637` |
| 10 | 1.28% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 11 | 1.25% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 12 | 1.21% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6926` |
| 13 | 1.00% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 14 | 0.93% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6351` |
| 15 | 0.92% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 16 | 0.88% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6858` |
| 17 | 0.87% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 18 | 0.85% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4447` |
| 19 | 0.76% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 20 | 0.68% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 21 | 0.67% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 22 | 0.65% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 23 | 0.65% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 24 | 0.64% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 243` |
| 25 | 0.64% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 26 | 0.61% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 27 | 0.55% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4384` |
| 28 | 0.55% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 29 | 0.54% | `java.util.EnumMap.get(Object) line: 248` |
| 30 | 0.54% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7468` |
| 31 | 0.53% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 32 | 0.53% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 6524` |
| 33 | 0.51% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 6702` |
| 34 | 0.48% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 35 | 0.46% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 36 | 0.46% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 37 | 0.44% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 38 | 0.44% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 10332` |
| 39 | 0.43% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 40 | 0.43% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 41 | 0.43% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 42 | 0.43% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 43 | 0.41% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 44 | 0.39% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 45 | 0.39% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 46 | 0.38% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 47 | 0.37% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6941` |
| 48 | 0.35% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 228` |
| 49 | 0.35% | `net.minecraft.world.level.levelgen.BitRandomSource.nextInt(int) line: 31` |
| 50 | 0.35% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 51 | 0.35% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 52 | 0.35% | `java.lang.Enum.ordinal() line: 128` |
| 53 | 0.34% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 54 | 0.34% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap$MapIterator.nextEntry() line: 655` |
| 55 | 0.29% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 56 | 0.28% | `java.lang.ref.Reference.refersToImpl(Object) line: 391` |
| 57 | 0.28% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 58 | 0.27% | `java.lang.String.hashCode() line: 2430` |
| 59 | 0.26% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 60 | 0.26% | `net.minecraft.core.Holder$Reference.is(TagKey) line: 195` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.39% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 2 | 3.90% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 247` |
| 3 | 2.47% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 2.14% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 5 | 2.04% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 6 | 1.86% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6941` |
| 7 | 1.77% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 8 | 1.72% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9785` |
| 9 | 1.68% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 10 | 1.67% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 11 | 1.50% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 12 | 1.50% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4302` |
| 13 | 1.49% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 14 | 1.48% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6926` |
| 15 | 1.39% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 16 | 1.10% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 17 | 1.08% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 18 | 0.98% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 19 | 0.94% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 20 | 0.91% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 21 | 0.90% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 22 | 0.85% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 23 | 0.83% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2515` |
| 24 | 0.79% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6857` |
| 25 | 0.76% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 26 | 0.69% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 27 | 0.68% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |
| 28 | 0.66% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 29 | 0.64% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.canUse() line: 54` |
| 30 | 0.63% | `net.minecraft.world.entity.ai.sensing.Sensing.tick()` |

GC pauses: 0, total 0 ms

