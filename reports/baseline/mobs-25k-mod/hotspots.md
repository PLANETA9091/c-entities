# JVM hotspots (JFR ExecutionSample)

Samples: 14315

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 1.65% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1331` |
| 2 | 1.61% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 3 | 1.56% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 1.52% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10841` |
| 5 | 1.49% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 6 | 1.45% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 7 | 1.40% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 8 | 0.98% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 9 | 0.95% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 10 | 0.90% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 11 | 0.87% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 12 | 0.87% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 13 | 0.78% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 14 | 0.66% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 7458` |
| 15 | 0.65% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 16 | 0.64% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7627` |
| 17 | 0.64% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 18 | 0.63% | `java.util.HashMap.getNode(Object) line: 586` |
| 19 | 0.63% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 20 | 0.59% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 21 | 0.57% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 22 | 0.56% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 23 | 0.56% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3707` |
| 24 | 0.55% | `dev.planeta9091.centities.CeHooks.stepOnIsNoOp(Block) line: 5041` |
| 25 | 0.54% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 26 | 0.53% | `java.util.concurrent.ConcurrentHashMap.computeIfAbsent(Object, Function) line: 1737` |
| 27 | 0.53% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 8250` |
| 28 | 0.51% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 29 | 0.50% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 7280` |
| 30 | 0.50% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 5066` |
| 31 | 0.45% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 32 | 0.45% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 33 | 0.45% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2731` |
| 34 | 0.43% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 35 | 0.43% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 36 | 0.41% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 37 | 0.41% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 11430` |
| 38 | 0.41% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 39 | 0.41% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 40 | 0.40% | `java.util.EnumMap.get(Object) line: 248` |
| 41 | 0.40% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2821` |
| 42 | 0.39% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7107` |
| 43 | 0.38% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 44 | 0.38% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 5455` |
| 45 | 0.38% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 46 | 0.38% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 47 | 0.38% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 5375` |
| 48 | 0.37% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 49 | 0.36% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 50 | 0.36% | `jdk.internal.misc.Unsafe.getLongUnaligned(Object, long, boolean) line: 3571` |
| 51 | 0.34% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$hasPlayers() line: 1347` |
| 52 | 0.34% | `dev.planeta9091.centities.CeHooks.aiShadowPeek(Object) line: 7307` |
| 53 | 0.34% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 54 | 0.34% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 55 | 0.32% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 56 | 0.31% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 57 | 0.31% | `net.minecraft.core.Holder$Reference.is(TagKey) line: 195` |
| 58 | 0.31% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 59 | 0.31% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 60 | 0.30% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7705` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 2.91% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 2 | 1.75% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 3 | 1.70% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 11065` |
| 4 | 1.69% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 5 | 1.68% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2599` |
| 6 | 1.56% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 1.49% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 8 | 1.41% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3739` |
| 9 | 1.31% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 10 | 1.29% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 11 | 1.29% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 12 | 1.03% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 13 | 1.01% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 14 | 1.00% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 15 | 0.99% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7720` |
| 16 | 0.95% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 17 | 0.93% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 18 | 0.92% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7705` |
| 19 | 0.92% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 20 | 0.87% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 21 | 0.86% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 22 | 0.84% | `dev.planeta9091.centities.CeHooks.goalSelectorTickRunning(GoalSelector, boolean) line: 7796` |
| 23 | 0.78% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7626` |
| 24 | 0.75% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 25 | 0.73% | `java.util.HashMap.get(Object) line: 565` |
| 26 | 0.72% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 27 | 0.70% | `it.unimi.dsi.fastutil.objects.ReferenceCollections$UnmodifiableCollection.contains(Object) line: 344` |
| 28 | 0.65% | `net.minecraft.world.entity.ai.sensing.Sensing.tick()` |
| 29 | 0.64% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 30 | 0.64% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4984` |

GC pauses: 0, total 0 ms

