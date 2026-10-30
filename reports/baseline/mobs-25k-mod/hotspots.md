# JVM hotspots (JFR ExecutionSample)

Samples: 15507

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.62% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 2.09% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 3 | 1.85% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 1.84% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8633` |
| 5 | 1.71% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 6 | 1.62% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8492` |
| 7 | 1.41% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 1.39% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 9 | 1.32% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 10 | 1.32% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 11 | 1.29% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 1.29% | `net.minecraft.world.entity.ai.control.LookControl.tick() line: 55` |
| 13 | 1.27% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 14 | 1.16% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 15 | 0.99% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 16 | 0.94% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 17 | 0.92% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5528` |
| 18 | 0.91% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 19 | 0.85% | `java.lang.ref.Reference.get() line: 361` |
| 20 | 0.75% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 21 | 0.74% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 993` |
| 22 | 0.72% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 23 | 0.64% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 24 | 0.59% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 237` |
| 25 | 0.57% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 26 | 0.57% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 27 | 0.57% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3784` |
| 28 | 0.56% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 29 | 0.55% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3721` |
| 30 | 0.54% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 31 | 0.51% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 3874` |
| 32 | 0.50% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 33 | 0.47% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 34 | 0.46% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 35 | 0.46% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 36 | 0.45% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 98` |
| 37 | 0.45% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 38 | 0.44% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8578` |
| 39 | 0.44% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 40 | 0.44% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8439` |
| 41 | 0.43% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 42 | 0.43% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 6545` |
| 43 | 0.41% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 44 | 0.40% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5383` |
| 45 | 0.39% | `java.util.EnumMap.get(Object) line: 248` |
| 46 | 0.39% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 47 | 0.39% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 48 | 0.39% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 49 | 0.37% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 50 | 0.36% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 8700` |
| 51 | 0.34% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 52 | 0.33% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 53 | 0.33% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 54 | 0.33% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 55 | 0.33% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 100` |
| 56 | 0.32% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 57 | 0.32% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4106` |
| 58 | 0.32% | `java.util.concurrent.ConcurrentHashMap.computeIfAbsent(Object, Function) line: 1737` |
| 59 | 0.30% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 60 | 0.30% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2450` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 2.01% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8505` |
| 2 | 1.87% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 3 | 1.85% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 4 | 1.85% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 1.79% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 6 | 1.71% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 7 | 1.63% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 8 | 1.55% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 9 | 1.53% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 10 | 1.52% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 11 | 1.41% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8480` |
| 12 | 1.40% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 13 | 1.40% | `net.minecraft.world.entity.Mob.serverAiStep() line: 860` |
| 14 | 1.32% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 15 | 1.28% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 16 | 1.26% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 17 | 1.22% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 18 | 1.05% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 19 | 1.01% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 20 | 0.99% | `net.minecraft.world.entity.Mob.serverAiStep() line: 843` |
| 21 | 0.97% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 22 | 0.95% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5333` |
| 23 | 0.85% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 24 | 0.85% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 3894` |
| 25 | 0.82% | `dev.planeta9091.centities.CeHooks.pushDispatch(Entity) line: 3130` |
| 26 | 0.81% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 27 | 0.81% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 28 | 0.75% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 29 | 0.75% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 30 | 0.75% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |

GC pauses: 0, total 0 ms

