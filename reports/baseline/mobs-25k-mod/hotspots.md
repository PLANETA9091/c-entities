# JVM hotspots (JFR ExecutionSample)

Samples: 12427

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 2.19% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10202` |
| 2 | 2.07% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 3 | 1.63% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 4 | 1.59% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 5 | 1.59% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 6 | 1.55% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 7 | 1.42% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9600` |
| 8 | 1.38% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 1.33% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 10 | 1.29% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 11 | 1.26% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 12 | 1.21% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 13 | 1.04% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6821` |
| 14 | 0.96% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6889` |
| 15 | 0.88% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 16 | 0.87% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 17 | 0.80% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 18 | 0.80% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 19 | 0.78% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 6487` |
| 20 | 0.75% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 6665` |
| 21 | 0.75% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 22 | 0.65% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 23 | 0.60% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 24 | 0.59% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4443` |
| 25 | 0.58% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 26 | 0.57% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 27 | 0.57% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4380` |
| 28 | 0.51% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7431` |
| 29 | 0.46% | `java.util.EnumMap.get(Object) line: 248` |
| 30 | 0.46% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6314` |
| 31 | 0.45% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 32 | 0.45% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 33 | 0.44% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 34 | 0.43% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 35 | 0.43% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 36 | 0.42% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 37 | 0.42% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4689` |
| 38 | 0.41% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 39 | 0.40% | `java.util.concurrent.ConcurrentHashMap.computeIfAbsent(Object, Function) line: 1737` |
| 40 | 0.39% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 41 | 0.38% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 42 | 0.38% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6905` |
| 43 | 0.38% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 44 | 0.37% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 45 | 0.37% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 46 | 0.35% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 47 | 0.35% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 10114` |
| 48 | 0.35% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 49 | 0.35% | `dev.planeta9091.centities.CeHooks.aiShadowPeek(Object) line: 6514` |
| 50 | 0.34% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 51 | 0.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ChunkSlicesRegion.get(int) line: 935` |
| 52 | 0.31% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 53 | 0.31% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 54 | 0.31% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4769` |
| 55 | 0.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 56 | 0.30% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap$MapIterator.nextEntry() line: 655` |
| 57 | 0.30% | `java.util.HashMap.getNode(Object) line: 586` |
| 58 | 0.29% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 59 | 0.29% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 60 | 0.28% | `java.lang.Enum.ordinal() line: 128` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.10% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 2 | 2.28% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 3 | 2.25% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 4 | 2.18% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 5 | 1.69% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6904` |
| 6 | 1.67% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 7 | 1.63% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 8 | 1.62% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 9 | 1.62% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 10 | 1.53% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9748` |
| 11 | 1.45% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6889` |
| 12 | 1.38% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 13 | 1.32% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 14 | 1.29% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 15 | 1.11% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 16 | 1.10% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 17 | 1.06% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6820` |
| 18 | 1.01% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 19 | 0.96% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 20 | 0.91% | `dev.planeta9091.centities.CeHooks.goalSelectorTickRunning(GoalSelector, boolean) line: 6977` |
| 21 | 0.88% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4298` |
| 22 | 0.87% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 23 | 0.86% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 24 | 0.85% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 25 | 0.83% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 26 | 0.77% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 27 | 0.77% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 28 | 0.66% | `it.unimi.dsi.fastutil.objects.ReferenceCollections$UnmodifiableCollection.contains(Object) line: 344` |
| 29 | 0.65% | `net.minecraft.world.entity.ai.sensing.Sensing.tick()` |
| 30 | 0.64% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |

GC pauses: 0, total 0 ms

