# JVM hotspots (JFR ExecutionSample)

Samples: 16079

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.78% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 2.79% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 3 | 1.98% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 4 | 1.90% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 1.88% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8588` |
| 6 | 1.78% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 7 | 1.52% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 8 | 1.47% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 9 | 1.43% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 10 | 1.40% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8729` |
| 11 | 1.34% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 12 | 1.21% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 13 | 1.10% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 14 | 1.09% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 15 | 1.07% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5624` |
| 16 | 1.02% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 17 | 0.93% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 18 | 0.88% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 19 | 0.84% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5479` |
| 20 | 0.81% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 21 | 0.79% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 22 | 0.79% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 23 | 0.76% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 24 | 0.75% | `java.lang.ref.Reference.get() line: 361` |
| 25 | 0.75% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3880` |
| 26 | 0.69% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 27 | 0.67% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 28 | 0.66% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 29 | 0.65% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |
| 30 | 0.63% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 31 | 0.57% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8535` |
| 32 | 0.53% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 33 | 0.53% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 34 | 0.50% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 35 | 0.50% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 36 | 0.50% | `java.util.EnumMap.get(Object) line: 248` |
| 37 | 0.49% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3817` |
| 38 | 0.47% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 39 | 0.47% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 40 | 0.47% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 41 | 0.46% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8674` |
| 42 | 0.43% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 3970` |
| 43 | 0.43% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4125` |
| 44 | 0.42% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 6641` |
| 45 | 0.42% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 46 | 0.42% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getAttributesToSync() line: 34` |
| 47 | 0.41% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 48 | 0.40% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 49 | 0.40% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 8796` |
| 50 | 0.39% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 51 | 0.35% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 52 | 0.35% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 53 | 0.34% | `java.lang.Enum.ordinal() line: 128` |
| 54 | 0.34% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 55 | 0.34% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 56 | 0.33% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 57 | 0.32% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 58 | 0.32% | `dev.planeta9091.centities.CeHooks.pushDispatch(Entity) line: 3199` |
| 59 | 0.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 60 | 0.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 1.95% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 2 | 1.95% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8601` |
| 3 | 1.90% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 1.74% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 5 | 1.50% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8576` |
| 6 | 1.49% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 7 | 1.42% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 8 | 1.41% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 9 | 1.38% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 10 | 1.38% | `dev.planeta9091.centities.CeHooks.pushDispatch(Entity) line: 3200` |
| 11 | 1.24% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 12 | 1.22% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 13 | 1.21% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5429` |
| 14 | 1.17% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 15 | 1.16% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 16 | 1.14% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 17 | 1.14% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 18 | 1.11% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 19 | 1.11% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 20 | 1.07% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 21 | 0.98% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 22 | 0.97% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 23 | 0.96% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 24 | 0.89% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 25 | 0.88% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 26 | 0.87% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 27 | 0.85% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 28 | 0.85% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 29 | 0.83% | `net.minecraft.world.entity.Mob.serverAiStep() line: 839` |
| 30 | 0.77% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 216` |

GC pauses: 0, total 0 ms

