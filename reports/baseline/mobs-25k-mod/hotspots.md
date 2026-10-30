# JVM hotspots (JFR ExecutionSample)

Samples: 15203

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.64% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 2.60% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 3 | 2.07% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 4 | 1.91% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 5 | 1.86% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 1.65% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8837` |
| 7 | 1.61% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8696` |
| 8 | 1.47% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 9 | 1.47% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 10 | 1.45% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 11 | 1.31% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |
| 12 | 1.24% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 13 | 1.12% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 14 | 1.09% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 15 | 0.94% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 16 | 0.93% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 17 | 0.86% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 18 | 0.85% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 19 | 0.82% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 20 | 0.79% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 21 | 0.76% | `java.lang.ref.Reference.get() line: 361` |
| 22 | 0.75% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5732` |
| 23 | 0.67% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 24 | 0.66% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 25 | 0.62% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3988` |
| 26 | 0.62% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3925` |
| 27 | 0.57% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 28 | 0.54% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 6749` |
| 29 | 0.51% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 30 | 0.50% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 31 | 0.50% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 4078` |
| 32 | 0.49% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5587` |
| 33 | 0.49% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 34 | 0.47% | `java.util.EnumMap.get(Object) line: 248` |
| 35 | 0.46% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 36 | 0.45% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 37 | 0.43% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8643` |
| 38 | 0.42% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8782` |
| 39 | 0.39% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 40 | 0.39% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 41 | 0.39% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 42 | 0.39% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 43 | 0.39% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 44 | 0.39% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 45 | 0.38% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 46 | 0.37% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 47 | 0.37% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 48 | 0.36% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 49 | 0.36% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 50 | 0.36% | `java.util.concurrent.ConcurrentHashMap.computeIfAbsent(Object, Function) line: 1737` |
| 51 | 0.35% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 52 | 0.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 53 | 0.34% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 54 | 0.33% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 55 | 0.32% | `net.minecraft.world.level.levelgen.BitRandomSource.nextInt(int) line: 31` |
| 56 | 0.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 57 | 0.30% | `net.minecraft.core.Holder$Reference.is(TagKey) line: 195` |
| 58 | 0.29% | `dev.planeta9091.centities.CeHooks.stepOnIsNoOp(Block) line: 3900` |
| 59 | 0.28% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap$MapIterator.nextEntry() line: 655` |
| 60 | 0.28% | `net.minecraft.world.entity.ai.goal.target.NearestAttackableTargetGoal.canUse() line: 50` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 1.91% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8709` |
| 2 | 1.86% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 1.68% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 4 | 1.68% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 1.62% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 6 | 1.49% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 7 | 1.45% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 8 | 1.42% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 9 | 1.42% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 10 | 1.39% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8684` |
| 11 | 1.34% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 12 | 1.34% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 13 | 1.31% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 196` |
| 14 | 1.25% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 15 | 1.11% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 16 | 1.11% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 17 | 1.05% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 18 | 1.03% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5537` |
| 19 | 1.03% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 20 | 1.03% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 21 | 1.00% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 22 | 0.95% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 23 | 0.88% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 24 | 0.85% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 25 | 0.84% | `net.minecraft.world.entity.Mob.serverAiStep() line: 839` |
| 26 | 0.83% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 27 | 0.80% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 28 | 0.76% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 4098` |
| 29 | 0.73% | `net.minecraft.world.entity.Mob.serverAiStep() line: 836` |
| 30 | 0.71% | `net.minecraft.world.entity.Mob.serverAiStep() line: 843` |

GC pauses: 0, total 0 ms

