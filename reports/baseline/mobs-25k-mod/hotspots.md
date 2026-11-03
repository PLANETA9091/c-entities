# JVM hotspots (JFR ExecutionSample)

Samples: 8804

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.15% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getNextAcquire() line: 1837` |
| 2 | 2.31% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 1.57% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 4 | 1.43% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7048` |
| 5 | 1.41% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 6 | 1.27% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10361` |
| 7 | 1.25% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 8 | 1.18% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 9 | 1.12% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 10 | 1.04% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 11 | 1.00% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 12 | 0.93% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6980` |
| 13 | 0.90% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9759` |
| 14 | 0.90% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 15 | 0.85% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 16 | 0.82% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 17 | 0.81% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6473` |
| 18 | 0.81% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 19 | 0.75% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 20 | 0.75% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 21 | 0.73% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 22 | 0.72% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 23 | 0.69% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 24 | 0.67% | `jdk.internal.reflect.MethodHandleBooleanFieldAccessorImpl.setBoolean(Object, boolean) line: 132` |
| 25 | 0.66% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 6824` |
| 26 | 0.66% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 6646` |
| 27 | 0.61% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 28 | 0.53% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 29 | 0.52% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4569` |
| 30 | 0.52% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4506` |
| 31 | 0.51% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 32 | 0.50% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7590` |
| 33 | 0.48% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 34 | 0.48% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 35 | 0.43% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 36 | 0.42% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 37 | 0.42% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 38 | 0.42% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2835` |
| 39 | 0.41% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 40 | 0.37% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 41 | 0.36% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 42 | 0.36% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 43 | 0.35% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 44 | 0.35% | `dev.planeta9091.centities.CeHooks.stepOnIsNoOp(Block) line: 4481` |
| 45 | 0.35% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7064` |
| 46 | 0.34% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 47 | 0.33% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 48 | 0.33% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 49 | 0.32% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 228` |
| 50 | 0.31% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 51 | 0.31% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 52 | 0.30% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 320` |
| 53 | 0.30% | `dev.planeta9091.centities.CeHooks.aiShadowPeek(Object) line: 6673` |
| 54 | 0.28% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7063` |
| 55 | 0.28% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 56 | 0.28% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 10454` |
| 57 | 0.27% | `net.minecraft.world.entity.ai.goal.Goal.getFlags() line: 60` |
| 58 | 0.27% | `java.lang.ref.Reference.refersToImpl(Object) line: 391` |
| 59 | 0.27% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 60 | 0.27% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.65% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 2 | 3.15% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 247` |
| 3 | 2.31% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 1.70% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7063` |
| 5 | 1.57% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4424` |
| 6 | 1.45% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 7 | 1.33% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 8 | 1.31% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 9 | 1.27% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 10 | 1.23% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 11 | 1.23% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 12 | 1.16% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 13 | 1.16% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 14 | 1.12% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 15 | 1.12% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9907` |
| 16 | 1.11% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7048` |
| 17 | 1.08% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 18 | 1.00% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2217` |
| 19 | 0.95% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6979` |
| 20 | 0.95% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 21 | 0.92% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 22 | 0.89% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 23 | 0.84% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 24 | 0.84% | `dev.planeta9091.centities.CeHooks.goalSelectorTickRunning(GoalSelector, boolean) line: 7136` |
| 25 | 0.82% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 26 | 0.77% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 27 | 0.75% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 28 | 0.75% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 29 | 0.75% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 30 | 0.70% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |

GC pauses: 0, total 0 ms

