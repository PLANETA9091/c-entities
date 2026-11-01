# JVM hotspots (JFR ExecutionSample)

Samples: 13410

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 2.30% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.24% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 3 | 2.20% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 4 | 2.16% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 5 | 1.89% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 6 | 1.79% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 7 | 1.70% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 1.67% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8911` |
| 9 | 1.51% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9105` |
| 10 | 1.39% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5833` |
| 11 | 1.35% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 12 | 1.34% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 13 | 1.33% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 14 | 1.28% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 15 | 1.19% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 16 | 1.18% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8964` |
| 17 | 1.10% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 993` |
| 18 | 1.07% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 19 | 1.07% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 20 | 1.01% | `java.util.Optional.ifPresent(Consumer) line: 178` |
| 21 | 0.88% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 22 | 0.84% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 23 | 0.83% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 24 | 0.82% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5987` |
| 25 | 0.78% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 26 | 0.77% | `java.lang.ref.Reference.get() line: 361` |
| 27 | 0.71% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 28 | 0.69% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 29 | 0.68% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 30 | 0.68% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |
| 31 | 0.63% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4190` |
| 32 | 0.58% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7017` |
| 33 | 0.58% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4127` |
| 34 | 0.55% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 35 | 0.54% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 36 | 0.52% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 37 | 0.51% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 38 | 0.51% | `net.minecraft.world.entity.ai.control.LookControl.getYRotD() line: 102` |
| 39 | 0.50% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 40 | 0.46% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 41 | 0.44% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 42 | 0.43% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 43 | 0.41% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 44 | 0.40% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 45 | 0.40% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 46 | 0.39% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 237` |
| 47 | 0.38% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 48 | 0.38% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 49 | 0.38% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 9050` |
| 50 | 0.37% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 51 | 0.37% | `net.minecraft.world.entity.Mob.serverAiStep() line: 860` |
| 52 | 0.37% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 53 | 0.36% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 54 | 0.34% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 55 | 0.34% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 4280` |
| 56 | 0.34% | `net.minecraft.world.entity.ai.control.LookControl.tick() line: 55` |
| 57 | 0.31% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 58 | 0.31% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 59 | 0.31% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 60 | 0.31% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 2.61% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 2 | 2.30% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 2.20% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 4 | 2.08% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8977` |
| 5 | 1.92% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 6 | 1.89% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 7 | 1.73% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8960` |
| 8 | 1.69% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 9 | 1.54% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 10 | 1.46% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 11 | 1.45% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 12 | 1.44% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 13 | 1.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 14 | 1.37% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 15 | 1.37% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 16 | 1.37% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 17 | 1.21% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 18 | 1.18% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 19 | 1.17% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 20 | 1.07% | `net.minecraft.world.entity.Mob.serverAiStep() line: 843` |
| 21 | 1.05% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 22 | 1.01% | `net.minecraft.world.entity.ai.control.LookControl.tick() line: 54` |
| 23 | 0.98% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 24 | 0.92% | `net.minecraft.world.entity.Mob.serverAiStep() line: 836` |
| 25 | 0.85% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 26 | 0.81% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 216` |
| 27 | 0.80% | `net.minecraft.world.entity.ai.sensing.Sensing.tick()` |
| 28 | 0.79% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 29 | 0.77% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 4300` |
| 30 | 0.75% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |

GC pauses: 0, total 0 ms

