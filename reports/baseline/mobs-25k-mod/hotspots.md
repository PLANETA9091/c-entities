# JVM hotspots (JFR ExecutionSample)

Samples: 15536

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.70% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 2.47% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 3 | 2.14% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 1.93% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 5 | 1.85% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 6 | 1.83% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9129` |
| 7 | 1.58% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 1.44% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8988` |
| 9 | 1.43% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 10 | 1.34% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 11 | 1.13% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 1.10% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 13 | 1.07% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 14 | 0.95% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 15 | 0.95% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 993` |
| 16 | 0.91% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 17 | 0.87% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 18 | 0.84% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 19 | 0.76% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 20 | 0.69% | `java.lang.ref.Reference.get() line: 361` |
| 21 | 0.68% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4151` |
| 22 | 0.68% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 23 | 0.67% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 24 | 0.65% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |
| 25 | 0.64% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 26 | 0.62% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 27 | 0.58% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 28 | 0.57% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8935` |
| 29 | 0.57% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6011` |
| 30 | 0.51% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4214` |
| 31 | 0.50% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 32 | 0.48% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 33 | 0.48% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7041` |
| 34 | 0.48% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5857` |
| 35 | 0.46% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 36 | 0.44% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4558` |
| 37 | 0.43% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 98` |
| 38 | 0.41% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 39 | 0.41% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 4304` |
| 40 | 0.40% | `java.lang.Enum.ordinal() line: 128` |
| 41 | 0.40% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4481` |
| 42 | 0.40% | `java.util.EnumMap.get(Object) line: 248` |
| 43 | 0.38% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 44 | 0.38% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 45 | 0.37% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 46 | 0.36% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 100` |
| 47 | 0.35% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 48 | 0.34% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getAttributesToSync() line: 34` |
| 49 | 0.33% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 50 | 0.33% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 51 | 0.33% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 52 | 0.32% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 53 | 0.31% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 54 | 0.30% | `java.util.AbstractMap$SimpleImmutableEntry.getKey() line: 774` |
| 55 | 0.30% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 9196` |
| 56 | 0.30% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 57 | 0.30% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 446` |
| 58 | 0.29% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 59 | 0.29% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 60 | 0.28% | `net.minecraft.util.SimpleBitStorage.get(int) line: 350` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 2.14% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 1.97% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 3 | 1.91% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 4 | 1.85% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 5 | 1.84% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 6 | 1.83% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 7 | 1.76% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9001` |
| 8 | 1.66% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 9 | 1.49% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 10 | 1.44% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 11 | 1.40% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8976` |
| 12 | 1.35% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 13 | 1.15% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 14 | 1.05% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 15 | 1.04% | `net.minecraft.world.entity.Mob.serverAiStep() line: 843` |
| 16 | 1.02% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 17 | 1.01% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5807` |
| 18 | 0.98% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 19 | 0.98% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 20 | 0.97% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 21 | 0.95% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 22 | 0.93% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 23 | 0.90% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 24 | 0.88% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 25 | 0.87% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 26 | 0.82% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 27 | 0.80% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 28 | 0.77% | `net.minecraft.world.entity.Mob.serverAiStep() line: 836` |
| 29 | 0.75% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 30 | 0.74% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |

GC pauses: 0, total 0 ms

