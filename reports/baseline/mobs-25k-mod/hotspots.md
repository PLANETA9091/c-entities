# JVM hotspots (JFR ExecutionSample)

Samples: 13164

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 1.79% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 2 | 1.69% | `net.minecraft.world.entity.ConversionTracker.setConverting(boolean) line: 89` |
| 3 | 1.64% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 4 | 1.59% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3907` |
| 5 | 1.52% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1420` |
| 6 | 1.41% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 7 | 1.32% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |
| 8 | 1.29% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 9 | 1.26% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 10 | 1.19% | `com.google.common.collect.AbstractIterator.tryToComputeNext() line: 141` |
| 11 | 1.16% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 12 | 1.15% | `java.util.EnumMap.get(Object) line: 248` |
| 13 | 1.14% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 14 | 1.12% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 15 | 1.09% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 16 | 1.05% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 17 | 0.89% | `jdk.internal.util.ArraysSupport.hashCodeOfUnsigned(byte[], int, int, int) line: 275` |
| 18 | 0.87% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 19 | 0.85% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 20 | 0.81% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 21 | 0.74% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 22 | 0.71% | `net.minecraft.util.SimpleBitStorage.get(int) line: 350` |
| 23 | 0.71% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 24 | 0.71% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 25 | 0.68% | `java.util.ArrayDeque.size() line: 647` |
| 26 | 0.67% | `com.google.common.collect.AbstractIterator.hasNext() line: 129` |
| 27 | 0.67% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 28 | 0.64% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 29 | 0.62% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 30 | 0.62% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.add(long) line: 402` |
| 31 | 0.61% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 32 | 0.58% | `net.minecraft.world.item.ItemStack.getDamageValue() line: 573` |
| 33 | 0.55% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 34 | 0.53% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 35 | 0.53% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 36 | 0.52% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 37 | 0.51% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 38 | 0.50% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1866` |
| 39 | 0.49% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 40 | 0.49% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3868` |
| 41 | 0.47% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 42 | 0.46% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 43 | 0.46% | `net.minecraft.world.level.BlockGetter.forEachBlockIntersectedBetween(Vec3, Vec3, AABB, BlockGetter$BlockStepVisitor) line: 222` |
| 44 | 0.46% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 45 | 0.46% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 46 | 0.43% | `net.minecraft.core.SectionPos.blockToSectionCoord(int) line: 90` |
| 47 | 0.40% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 48 | 0.40% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 49 | 0.40% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 50 | 0.40% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 51 | 0.40% | `net.minecraft.core.BlockPos$7.computeNext() line: 602` |
| 52 | 0.40% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 53 | 0.40% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 54 | 0.39% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 55 | 0.39% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 56 | 0.39% | `ca.spottedleaf.moonrise.patches.starlight.light.SWMRNibbleArray.getVisible(int) line: 412` |
| 57 | 0.39% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 58 | 0.39% | `net.minecraft.world.level.BlockGetter.forEachBlockIntersectedBetween(Vec3, Vec3, AABB, BlockGetter$BlockStepVisitor) line: 223` |
| 59 | 0.38% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 60 | 0.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 323` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 2.46% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 2 | 2.26% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 3 | 1.85% | `net.minecraft.world.entity.Entity.checkInsideBlocks(Vec3, Vec3, InsideBlockEffectApplier$StepBasedCollector, LongSet, int) line: 1882` |
| 4 | 1.72% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 5 | 1.69% | `net.minecraft.world.entity.ConversionTracker.tick() line: 79` |
| 6 | 1.59% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 7 | 1.40% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 8 | 1.34% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 9 | 1.32% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 196` |
| 10 | 1.27% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 11 | 1.19% | `com.google.common.collect.AbstractIterator.hasNext() line: 136` |
| 12 | 1.19% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 13 | 1.14% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 14 | 1.06% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 15 | 1.01% | `java.util.ArrayList.toArray() line: 370` |
| 16 | 1.00% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1475` |
| 17 | 0.97% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 18 | 0.89% | `java.lang.StringLatin1.hashCode(byte[]) line: 195` |
| 19 | 0.87% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 20 | 0.81% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 21 | 0.81% | `dev.planeta9091.centities.CeHooks.pushDispatch(Entity) line: 1703` |
| 22 | 0.79% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 23 | 0.73% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 24 | 0.71% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 25 | 0.71% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 26 | 0.70% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 27 | 0.70% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 28 | 0.67% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 29 | 0.65% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 30 | 0.64% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |

GC pauses: 0, total 0 ms

