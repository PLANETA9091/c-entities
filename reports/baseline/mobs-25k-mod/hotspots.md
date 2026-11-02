# JVM hotspots (JFR ExecutionSample)

Samples: 12483

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 1.90% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 2 | 1.85% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10099` |
| 3 | 1.82% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 1.59% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 5 | 1.51% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9497` |
| 6 | 1.35% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 7 | 1.33% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 8 | 1.19% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 9 | 1.17% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 10 | 1.12% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6729` |
| 11 | 1.08% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 12 | 1.06% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 13 | 1.01% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 14 | 0.91% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6797` |
| 15 | 0.83% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 16 | 0.79% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 17 | 0.77% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 6395` |
| 18 | 0.75% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 19 | 0.70% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 20 | 0.65% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4353` |
| 21 | 0.64% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 22 | 0.63% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 23 | 0.63% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4416` |
| 24 | 0.60% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 6573` |
| 25 | 0.59% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 26 | 0.57% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 27 | 0.54% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7339` |
| 28 | 0.53% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6222` |
| 29 | 0.50% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 30 | 0.48% | `java.util.EnumMap.get(Object) line: 248` |
| 31 | 0.47% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 32 | 0.47% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 33 | 0.46% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 34 | 0.44% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 35 | 0.43% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 36 | 0.43% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 37 | 0.43% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 38 | 0.41% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 39 | 0.41% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 40 | 0.39% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 41 | 0.39% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 42 | 0.38% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4718` |
| 43 | 0.38% | `dev.planeta9091.centities.CeHooks.stepOnIsNoOp(Block) line: 4328` |
| 44 | 0.38% | `java.lang.Enum.ordinal() line: 128` |
| 45 | 0.38% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4641` |
| 46 | 0.38% | `dev.planeta9091.centities.CeHooks.aiShadowPeek(Object) line: 6422` |
| 47 | 0.36% | `java.util.HashMap.getNode(Object) line: 586` |
| 48 | 0.36% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 49 | 0.35% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 50 | 0.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 51 | 0.34% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 52 | 0.34% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 53 | 0.34% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 54 | 0.34% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 55 | 0.32% | `net.minecraft.core.Holder$Reference.is(TagKey) line: 195` |
| 56 | 0.32% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 10011` |
| 57 | 0.32% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 58 | 0.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 59 | 0.30% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 60 | 0.30% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.25% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 2 | 1.96% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 3 | 1.94% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 4 | 1.87% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 5 | 1.82% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 1.72% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 7 | 1.69% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9645` |
| 8 | 1.67% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 9 | 1.48% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 10 | 1.47% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6812` |
| 11 | 1.44% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 12 | 1.35% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6797` |
| 13 | 1.28% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 14 | 1.23% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 15 | 1.16% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 16 | 1.13% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 17 | 1.07% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6728` |
| 18 | 0.99% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 19 | 0.90% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 20 | 0.87% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 21 | 0.83% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 22 | 0.76% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 23 | 0.75% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 24 | 0.75% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 25 | 0.75% | `dev.planeta9091.centities.CeHooks.goalSelectorTickRunning(GoalSelector, boolean) line: 6885` |
| 26 | 0.69% | `net.minecraft.world.entity.ai.sensing.Sensing.tick()` |
| 27 | 0.66% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 28 | 0.66% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4642` |
| 29 | 0.64% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 6557` |
| 30 | 0.62% | `it.unimi.dsi.fastutil.objects.ReferenceCollections$UnmodifiableCollection.contains(Object) line: 344` |

GC pauses: 0, total 0 ms

