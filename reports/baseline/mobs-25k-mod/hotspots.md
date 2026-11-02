# JVM hotspots (JFR ExecutionSample)

Samples: 13494

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.61% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 2.19% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 3 | 2.06% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9267` |
| 4 | 1.93% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 5 | 1.81% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 6 | 1.79% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9126` |
| 7 | 1.76% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 8 | 1.65% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 9 | 1.44% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 10 | 1.36% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 11 | 1.32% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 12 | 1.27% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 13 | 1.26% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 14 | 1.16% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 15 | 0.96% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 16 | 0.92% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 17 | 0.83% | `java.lang.ref.Reference.get() line: 361` |
| 18 | 0.82% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 19 | 0.79% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 20 | 0.70% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 21 | 0.69% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 22 | 0.65% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 23 | 0.60% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 24 | 0.56% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5995` |
| 25 | 0.54% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 26 | 0.53% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4696` |
| 27 | 0.49% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 28 | 0.49% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4352` |
| 29 | 0.49% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 30 | 0.47% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7179` |
| 31 | 0.47% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 32 | 0.46% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4289` |
| 33 | 0.46% | `net.minecraft.world.entity.ai.control.LookControl.getYRotD() line: 102` |
| 34 | 0.45% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 35 | 0.45% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 36 | 0.44% | `java.lang.Enum.ordinal() line: 128` |
| 37 | 0.44% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 38 | 0.42% | `java.util.EnumMap.get(Object) line: 248` |
| 39 | 0.42% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9073` |
| 40 | 0.41% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 4442` |
| 41 | 0.41% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 42 | 0.40% | `java.util.concurrent.ConcurrentHashMap.computeIfAbsent(Object, Function) line: 1737` |
| 43 | 0.40% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6149` |
| 44 | 0.39% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 45 | 0.36% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 46 | 0.36% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4619` |
| 47 | 0.36% | `java.lang.ref.Reference.refersToImpl(Object) line: 391` |
| 48 | 0.35% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 49 | 0.35% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 50 | 0.34% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 51 | 0.34% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 52 | 0.33% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 53 | 0.33% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 9212` |
| 54 | 0.33% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 55 | 0.32% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 56 | 0.31% | `dev.planeta9091.centities.CeHooks.stepOnIsNoOp(Block) line: 4264` |
| 57 | 0.30% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 58 | 0.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 59 | 0.29% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 60 | 0.28% | `java.util.AbstractMap$SimpleImmutableEntry.getKey() line: 774` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 2.21% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9139` |
| 2 | 2.19% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 3 | 2.07% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 4 | 2.07% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 5 | 1.83% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 6 | 1.70% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 7 | 1.57% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 8 | 1.45% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 9 | 1.33% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9114` |
| 10 | 1.27% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 11 | 1.27% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 12 | 1.22% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 13 | 1.16% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 14 | 1.08% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 15 | 1.07% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 16 | 1.07% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 17 | 1.05% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 18 | 1.03% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 19 | 0.98% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5945` |
| 20 | 0.92% | `net.minecraft.world.entity.Mob.serverAiStep() line: 839` |
| 21 | 0.90% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 22 | 0.89% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 23 | 0.87% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 24 | 0.83% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 4462` |
| 25 | 0.80% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 26 | 0.79% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 27 | 0.78% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4620` |
| 28 | 0.77% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 29 | 0.77% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 30 | 0.74% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 216` |

GC pauses: 0, total 0 ms

