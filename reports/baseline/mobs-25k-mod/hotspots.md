# JVM hotspots (JFR ExecutionSample)

Samples: 15048

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 1.61% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 2 | 1.50% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 3 | 1.35% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10836` |
| 4 | 1.35% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |
| 5 | 1.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 6 | 1.32% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 7 | 1.18% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 8 | 1.08% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 9 | 1.02% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 10 | 0.95% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 11 | 0.95% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 12 | 0.95% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7622` |
| 13 | 0.92% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2726` |
| 14 | 0.87% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1326` |
| 15 | 0.83% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 16 | 0.82% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 17 | 0.80% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 7453` |
| 18 | 0.77% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 19 | 0.66% | `jdk.internal.misc.Unsafe.getLongUnaligned(Object, long, boolean) line: 3571` |
| 20 | 0.59% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3707` |
| 21 | 0.56% | `java.util.HashMap.getNode(Object) line: 586` |
| 22 | 0.56% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 23 | 0.55% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 24 | 0.55% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 5061` |
| 25 | 0.54% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 11425` |
| 26 | 0.53% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 27 | 0.52% | `java.util.concurrent.ConcurrentHashMap.computeIfAbsent(Object, Function) line: 1737` |
| 28 | 0.52% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 29 | 0.52% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 30 | 0.52% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 8245` |
| 31 | 0.50% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 32 | 0.49% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 7275` |
| 33 | 0.49% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 34 | 0.47% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 35 | 0.46% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 36 | 0.45% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 37 | 0.45% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 38 | 0.45% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 5124` |
| 39 | 0.44% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 40 | 0.43% | `jdk.internal.misc.Unsafe.convEndian(boolean, long) line: 3835` |
| 41 | 0.43% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 42 | 0.41% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 43 | 0.39% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 44 | 0.39% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 45 | 0.39% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 46 | 0.38% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 47 | 0.38% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 48 | 0.38% | `java.lang.Enum.ordinal() line: 128` |
| 49 | 0.37% | `java.util.EnumMap.get(Object) line: 248` |
| 50 | 0.37% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7700` |
| 51 | 0.37% | `java.util.AbstractMap$SimpleImmutableEntry.getKey() line: 774` |
| 52 | 0.35% | `java.lang.ref.Reference.refersToImpl(Object) line: 391` |
| 53 | 0.35% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 54 | 0.35% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 55 | 0.32% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 56 | 0.32% | `net.minecraft.core.Holder$Reference.is(TagKey) line: 195` |
| 57 | 0.32% | `dev.planeta9091.centities.CeHooks.aiShadowPeek(Object) line: 7302` |
| 58 | 0.31% | `dev.planeta9091.centities.CeHooks.stepOnIsNoOp(Block) line: 5036` |
| 59 | 0.31% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 239` |
| 60 | 0.30% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 2.67% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 2 | 2.16% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 3 | 1.66% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2594` |
| 4 | 1.65% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3739` |
| 5 | 1.55% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 6 | 1.51% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 11060` |
| 7 | 1.42% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 8 | 1.35% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 9 | 1.30% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 10 | 1.30% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 11 | 1.24% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 12 | 1.16% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7715` |
| 13 | 1.10% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 14 | 1.10% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7700` |
| 15 | 1.04% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 16 | 1.01% | `dev.planeta9091.centities.CeHooks.goalSelectorTickRunning(GoalSelector, boolean) line: 7791` |
| 17 | 0.98% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 18 | 0.95% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 19 | 0.95% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 20 | 0.94% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 21 | 0.93% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 22 | 0.92% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 23 | 0.84% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 24 | 0.84% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 25 | 0.83% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 26 | 0.83% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7621` |
| 27 | 0.72% | `java.util.HashMap.get(Object) line: 565` |
| 28 | 0.68% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 29 | 0.66% | `jdk.internal.misc.ScopedMemoryAccess.getLongUnalignedInternal(MemorySessionImpl, Object, long, boolean) line: 3861` |
| 30 | 0.66% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |

GC pauses: 0, total 0 ms

