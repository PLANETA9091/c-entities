# JVM hotspots (JFR ExecutionSample)

Samples: 16684

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 2.96% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 2.19% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 2.07% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 4 | 2.01% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 5 | 1.72% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 6 | 1.62% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7960` |
| 7 | 1.39% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7819` |
| 8 | 1.27% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 9 | 1.23% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 10 | 1.09% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4879` |
| 11 | 1.05% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 12 | 1.05% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 13 | 1.05% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 14 | 1.04% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 15 | 1.02% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2018` |
| 16 | 0.96% | `java.util.EnumMap.get(Object) line: 248` |
| 17 | 0.91% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 18 | 0.84% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 19 | 0.76% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 20 | 0.76% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 21 | 0.75% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 22 | 0.73% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 23 | 0.65% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |
| 24 | 0.63% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4734` |
| 25 | 0.62% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 26 | 0.62% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 27 | 0.50% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 28 | 0.49% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 29 | 0.47% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 30 | 0.46% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 31 | 0.46% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 32 | 0.46% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 5896` |
| 33 | 0.46% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 34 | 0.44% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 35 | 0.44% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 36 | 0.43% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 100` |
| 37 | 0.43% | `jdk.internal.util.ArraysSupport.hashCodeOfUnsigned(byte[], int, int, int) line: 275` |
| 38 | 0.40% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 39 | 0.40% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1999` |
| 40 | 0.40% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 8027` |
| 41 | 0.40% | `java.lang.ref.Reference.get() line: 361` |
| 42 | 0.39% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 43 | 0.38% | `net.minecraft.world.entity.ai.control.LookControl.tick() line: 55` |
| 44 | 0.37% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 45 | 0.37% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3202` |
| 46 | 0.37% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 47 | 0.36% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 48 | 0.35% | `java.lang.Enum.ordinal() line: 128` |
| 49 | 0.35% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 50 | 0.35% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 51 | 0.34% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 52 | 0.34% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3139` |
| 53 | 0.34% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 54 | 0.33% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7766` |
| 55 | 0.33% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 56 | 0.29% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 57 | 0.29% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 58 | 0.28% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 59 | 0.28% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 60 | 0.28% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 2.43% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1541` |
| 2 | 2.19% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 2.07% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 4 | 2.00% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 1.67% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 6 | 1.66% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 7 | 1.65% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 8 | 1.47% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 9 | 1.31% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7832` |
| 10 | 1.28% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 11 | 1.22% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 12 | 1.20% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 13 | 1.08% | `net.minecraft.world.entity.Mob.serverAiStep() line: 843` |
| 14 | 1.06% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 15 | 1.01% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2003` |
| 16 | 0.98% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 17 | 0.97% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 18 | 0.96% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7807` |
| 19 | 0.91% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 20 | 0.89% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 21 | 0.86% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 22 | 0.86% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 23 | 0.85% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 24 | 0.75% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 25 | 0.73% | `net.minecraft.world.level.EntityGetter.findNearbyPlayer(Entity, double, Predicate) line: 117` |
| 26 | 0.71% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 27 | 0.71% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 28 | 0.71% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 29 | 0.68% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 30 | 0.65% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 196` |

GC pauses: 0, total 0 ms

