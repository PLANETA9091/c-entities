# JVM hotspots (JFR ExecutionSample)

Samples: 13085

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 2.26% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10216` |
| 2 | 2.20% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 3 | 1.62% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 4 | 1.47% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 1.31% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 6 | 1.20% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 1.16% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9614` |
| 8 | 1.15% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 9 | 1.15% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 10 | 1.12% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6835` |
| 11 | 1.03% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 12 | 1.03% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 0.94% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 14 | 0.90% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 6501` |
| 15 | 0.79% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6903` |
| 16 | 0.79% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 17 | 0.79% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 18 | 0.76% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 19 | 0.74% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 20 | 0.74% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 6679` |
| 21 | 0.72% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 22 | 0.69% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 23 | 0.66% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 24 | 0.66% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4380` |
| 25 | 0.66% | `net.minecraft.util.SimpleBitStorage.get(int) line: 350` |
| 26 | 0.61% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 27 | 0.60% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4689` |
| 28 | 0.60% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4443` |
| 29 | 0.56% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 30 | 0.53% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 31 | 0.52% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 32 | 0.50% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 33 | 0.50% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6328` |
| 34 | 0.46% | `java.util.concurrent.ConcurrentHashMap.computeIfAbsent(Object, Function) line: 1737` |
| 35 | 0.44% | `dev.planeta9091.centities.CeHooks.aiShadowPeek(Object) line: 6528` |
| 36 | 0.44% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7445` |
| 37 | 0.43% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 38 | 0.43% | `java.util.AbstractMap$SimpleImmutableEntry.getKey() line: 774` |
| 39 | 0.41% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 40 | 0.41% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 41 | 0.41% | `java.util.HashMap.getNode(Object) line: 586` |
| 42 | 0.41% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 43 | 0.40% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 44 | 0.39% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 45 | 0.38% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 46 | 0.37% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 47 | 0.37% | `java.lang.ref.Reference.refersToImpl(Object) line: 391` |
| 48 | 0.35% | `java.util.EnumMap.get(Object) line: 248` |
| 49 | 0.34% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 50 | 0.34% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 51 | 0.34% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$hasPlayers() line: 1347` |
| 52 | 0.34% | `dev.planeta9091.centities.CeHooks.getTicksFrozen$(Entity) line: 10227` |
| 53 | 0.34% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 54 | 0.33% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 10128` |
| 55 | 0.33% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 56 | 0.33% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 57 | 0.33% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4769` |
| 58 | 0.32% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getNextAcquire() line: 1837` |
| 59 | 0.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ChunkSlicesRegion.get(int) line: 935` |
| 60 | 0.32% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.18% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 2 | 2.51% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 3 | 2.31% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 4 | 2.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 5 | 1.80% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 6 | 1.66% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 7 | 1.61% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6918` |
| 8 | 1.56% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 9 | 1.47% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 10 | 1.41% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6903` |
| 11 | 1.25% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 12 | 1.25% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9762` |
| 13 | 1.18% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6834` |
| 14 | 1.15% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 15 | 1.09% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 16 | 1.07% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 17 | 1.04% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 18 | 1.00% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 19 | 0.96% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 20 | 0.94% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4298` |
| 21 | 0.91% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 22 | 0.90% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 23 | 0.88% | `dev.planeta9091.centities.CeHooks.goalSelectorTickRunning(GoalSelector, boolean) line: 6991` |
| 24 | 0.78% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 25 | 0.76% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 26 | 0.75% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 27 | 0.70% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 28 | 0.70% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4690` |
| 29 | 0.66% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 30 | 0.61% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 6663` |

GC pauses: 0, total 0 ms

