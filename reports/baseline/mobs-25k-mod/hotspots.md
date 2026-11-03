# JVM hotspots (JFR ExecutionSample)

Samples: 14337

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 9.77% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2668` |
| 2 | 5.08% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 3 | 4.47% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2399` |
| 4 | 3.15% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getNextAcquire() line: 1837` |
| 5 | 1.82% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 6 | 1.49% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 1.34% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 8 | 1.18% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 9 | 1.11% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 10 | 1.11% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10643` |
| 11 | 1.07% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 12 | 1.05% | `java.util.TimSort.countRunAndMakeAscending(Object[], int, int, Comparator) line: 360` |
| 13 | 1.00% | `dev.planeta9091.centities.CeHooks.ordSortVanilla(ArrayList) line: 2397` |
| 14 | 0.99% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1291` |
| 15 | 0.87% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4832` |
| 16 | 0.85% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7311` |
| 17 | 0.83% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10041` |
| 18 | 0.80% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 19 | 0.75% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 20 | 0.75% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 21 | 0.73% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 22 | 0.68% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 23 | 0.63% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 24 | 0.61% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7243` |
| 25 | 0.50% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 26 | 0.49% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 27 | 0.48% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 28 | 0.47% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 29 | 0.47% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 6909` |
| 30 | 0.47% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4769` |
| 31 | 0.46% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 32 | 0.46% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6736` |
| 33 | 0.45% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 34 | 0.43% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 35 | 0.43% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7853` |
| 36 | 0.42% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 7087` |
| 37 | 0.39% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 38 | 0.36% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 39 | 0.35% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 40 | 0.35% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 41 | 0.34% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 42 | 0.33% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 43 | 0.33% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 44 | 0.33% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 45 | 0.33% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 46 | 0.33% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 47 | 0.33% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2408` |
| 48 | 0.32% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 49 | 0.32% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 50 | 0.32% | `java.lang.ref.Reference.refersToImpl(Object) line: 391` |
| 51 | 0.30% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 52 | 0.29% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 53 | 0.28% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 54 | 0.28% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 55 | 0.27% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 10736` |
| 56 | 0.27% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 57 | 0.27% | `net.minecraft.world.level.levelgen.BitRandomSource.nextInt(int) line: 31` |
| 58 | 0.26% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 59 | 0.26% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 60 | 0.25% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 10.39% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2441` |
| 2 | 5.09% | `java.util.TimSort.countRunAndMakeAscending(Object[], int, int, Comparator) line: 360` |
| 3 | 5.08% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4687` |
| 4 | 3.15% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 247` |
| 5 | 3.06% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 6 | 1.58% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7326` |
| 7 | 1.53% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2662` |
| 8 | 1.49% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 1.47% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 10 | 1.39% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 11 | 1.30% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 12 | 1.18% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 13 | 1.14% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2721` |
| 14 | 1.12% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 15 | 1.11% | `java.util.TimSort.sort(Object[], int, int, Comparator, Object[], int, int) line: 234` |
| 16 | 1.05% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7311` |
| 17 | 1.05% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 18 | 0.91% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10189` |
| 19 | 0.86% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 20 | 0.82% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 21 | 0.80% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 22 | 0.77% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 23 | 0.77% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 24 | 0.75% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 25 | 0.71% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2515` |
| 26 | 0.68% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 27 | 0.63% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7242` |
| 28 | 0.61% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 29 | 0.59% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 30 | 0.55% | `net.minecraft.world.entity.ai.sensing.Sensing.tick()` |

GC pauses: 0, total 0 ms

