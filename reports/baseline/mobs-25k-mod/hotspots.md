# JVM hotspots (JFR ExecutionSample)

Samples: 14568

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.55% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 2.10% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 3 | 2.00% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 4 | 1.89% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 1.85% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 6 | 1.80% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9127` |
| 7 | 1.73% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 8 | 1.72% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8986` |
| 9 | 1.63% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 10 | 1.59% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 11 | 1.57% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 12 | 1.46% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 13 | 1.37% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 14 | 1.32% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 15 | 0.97% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 16 | 0.95% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8933` |
| 17 | 0.91% | `java.lang.ref.Reference.get() line: 361` |
| 18 | 0.83% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 19 | 0.80% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 20 | 0.75% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 21 | 0.73% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 22 | 0.71% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 23 | 0.69% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6009` |
| 24 | 0.64% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 25 | 0.62% | `java.util.EnumMap.get(Object) line: 248` |
| 26 | 0.58% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 27 | 0.54% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 28 | 0.54% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 4302` |
| 29 | 0.52% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 30 | 0.52% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 31 | 0.50% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7039` |
| 32 | 0.50% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 33 | 0.50% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5855` |
| 34 | 0.48% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4149` |
| 35 | 0.47% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 36 | 0.47% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 37 | 0.45% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4212` |
| 38 | 0.41% | `java.lang.Enum.ordinal() line: 128` |
| 39 | 0.40% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 40 | 0.40% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 41 | 0.39% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 42 | 0.39% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 43 | 0.38% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 44 | 0.38% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 45 | 0.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 46 | 0.37% | `java.util.concurrent.ConcurrentHashMap.computeIfAbsent(Object, Function) line: 1737` |
| 47 | 0.36% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 48 | 0.35% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getAttributesToSync() line: 34` |
| 49 | 0.34% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4479` |
| 50 | 0.34% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 51 | 0.32% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 52 | 0.32% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 9072` |
| 53 | 0.31% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 54 | 0.30% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 55 | 0.30% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 56 | 0.29% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 57 | 0.28% | `dev.planeta9091.centities.CeHooks.stepOnIsNoOp(Block) line: 4124` |
| 58 | 0.28% | `net.minecraft.world.level.levelgen.BitRandomSource.nextInt(int) line: 31` |
| 59 | 0.28% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 60 | 0.27% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 2.48% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8999` |
| 2 | 1.89% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 1.81% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 4 | 1.77% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 5 | 1.74% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 6 | 1.67% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 7 | 1.57% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 8 | 1.43% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 9 | 1.42% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8974` |
| 10 | 1.38% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 11 | 1.33% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 12 | 1.33% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 13 | 1.24% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 14 | 1.16% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 15 | 1.15% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 16 | 1.13% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 17 | 1.11% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 18 | 1.04% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 19 | 1.02% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8982` |
| 20 | 0.99% | `net.minecraft.world.entity.Mob.serverAiStep() line: 839` |
| 21 | 0.98% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 22 | 0.98% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 23 | 0.97% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5805` |
| 24 | 0.91% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 4322` |
| 25 | 0.87% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 26 | 0.82% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 27 | 0.81% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 28 | 0.78% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 29 | 0.77% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 216` |
| 30 | 0.76% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2079` |

GC pauses: 0, total 0 ms

