# JVM hotspots (JFR ExecutionSample)

Samples: 14108

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.45% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 2.59% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 3 | 2.26% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8840` |
| 4 | 2.02% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 2.01% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 6 | 1.81% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 7 | 1.69% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 8 | 1.69% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 9 | 1.62% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8699` |
| 10 | 1.55% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 11 | 1.22% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 12 | 1.11% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 13 | 1.07% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 14 | 1.03% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 15 | 0.96% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 16 | 0.82% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 17 | 0.72% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 18 | 0.71% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 19 | 0.69% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 20 | 0.66% | `java.lang.ref.Reference.get() line: 361` |
| 21 | 0.64% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3991` |
| 22 | 0.61% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 23 | 0.60% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 24 | 0.59% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5590` |
| 25 | 0.58% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5735` |
| 26 | 0.55% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 27 | 0.53% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 28 | 0.53% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3928` |
| 29 | 0.53% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.<init>(Map, float) line: 124` |
| 30 | 0.52% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 6752` |
| 31 | 0.50% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4236` |
| 32 | 0.49% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 33 | 0.49% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 34 | 0.47% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 35 | 0.46% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 237` |
| 36 | 0.45% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 37 | 0.44% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 4081` |
| 38 | 0.40% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 39 | 0.40% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 40 | 0.40% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 41 | 0.39% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8646` |
| 42 | 0.37% | `java.util.EnumMap.get(Object) line: 248` |
| 43 | 0.36% | `java.util.AbstractMap$SimpleImmutableEntry.getKey() line: 774` |
| 44 | 0.35% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 45 | 0.34% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 46 | 0.34% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 47 | 0.33% | `java.util.concurrent.ConcurrentHashMap.computeIfAbsent(Object, Function) line: 1710` |
| 48 | 0.33% | `net.minecraft.core.Holder$Reference.is(TagKey) line: 195` |
| 49 | 0.33% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 50 | 0.33% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getAttributesToSync() line: 34` |
| 51 | 0.32% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 52 | 0.32% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 53 | 0.32% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 54 | 0.31% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 55 | 0.31% | `dev.planeta9091.centities.CeHooks.stepOnIsNoOp(Block) line: 3903` |
| 56 | 0.31% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8785` |
| 57 | 0.31% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 58 | 0.30% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 59 | 0.30% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 60 | 0.30% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 2.29% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 2 | 2.02% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 1.89% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8712` |
| 4 | 1.71% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 5 | 1.67% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 6 | 1.64% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 7 | 1.57% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 8 | 1.46% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 9 | 1.36% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 10 | 1.30% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 11 | 1.24% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 12 | 1.23% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8687` |
| 13 | 1.23% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 14 | 1.11% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 15 | 1.08% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 16 | 1.07% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 17 | 1.02% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 18 | 0.98% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 19 | 0.93% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 216` |
| 20 | 0.91% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5540` |
| 21 | 0.89% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 22 | 0.88% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 23 | 0.88% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 24 | 0.87% | `net.minecraft.world.entity.Mob.serverAiStep() line: 836` |
| 25 | 0.84% | `net.minecraft.world.entity.Mob.serverAiStep() line: 839` |
| 26 | 0.82% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 27 | 0.82% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 28 | 0.80% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 29 | 0.76% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 30 | 0.72% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |

GC pauses: 0, total 0 ms

