# JVM hotspots (JFR ExecutionSample)

Samples: 15315

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.80% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 2.49% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 3 | 1.77% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 4 | 1.70% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8929` |
| 5 | 1.55% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 6 | 1.49% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8788` |
| 7 | 1.46% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 8 | 1.45% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 9 | 1.29% | `net.minecraft.world.entity.ai.control.LookControl.tick() line: 55` |
| 10 | 1.14% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 11 | 1.08% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 12 | 1.02% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 13 | 0.99% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 14 | 0.93% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 15 | 0.85% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 16 | 0.83% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 17 | 0.81% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 18 | 0.78% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5824` |
| 19 | 0.77% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 20 | 0.71% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 21 | 0.71% | `java.lang.ref.Reference.get() line: 361` |
| 22 | 0.69% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2699` |
| 23 | 0.63% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2661` |
| 24 | 0.62% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 25 | 0.61% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 26 | 0.60% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 27 | 0.60% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 28 | 0.57% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4080` |
| 29 | 0.54% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 30 | 0.54% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4017` |
| 31 | 0.54% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8735` |
| 32 | 0.54% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 6841` |
| 33 | 0.52% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 34 | 0.51% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 35 | 0.50% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 36 | 0.48% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 37 | 0.48% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 4170` |
| 38 | 0.46% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5679` |
| 39 | 0.44% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 40 | 0.43% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 41 | 0.42% | `java.lang.Enum.ordinal() line: 128` |
| 42 | 0.42% | `java.util.concurrent.ConcurrentHashMap.computeIfAbsent(Object, Function) line: 1737` |
| 43 | 0.39% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 44 | 0.39% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 45 | 0.39% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4325` |
| 46 | 0.39% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 47 | 0.37% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 239` |
| 48 | 0.37% | `java.util.EnumMap.get(Object) line: 248` |
| 49 | 0.36% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 100` |
| 50 | 0.36% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 98` |
| 51 | 0.35% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 52 | 0.35% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 53 | 0.35% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 320` |
| 54 | 0.34% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 55 | 0.34% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 56 | 0.33% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 57 | 0.33% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 58 | 0.33% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 59 | 0.33% | `net.minecraft.world.entity.ai.control.LookControl.getYRotD() line: 102` |
| 60 | 0.32% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 1.98% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2043` |
| 2 | 1.82% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 3 | 1.73% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 4 | 1.67% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 5 | 1.58% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 6 | 1.54% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 7 | 1.49% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8801` |
| 8 | 1.48% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 9 | 1.41% | `net.minecraft.world.entity.Mob.serverAiStep() line: 860` |
| 10 | 1.36% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 11 | 1.32% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8776` |
| 12 | 1.22% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 13 | 1.19% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 14 | 1.14% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 15 | 1.10% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 16 | 1.09% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 17 | 1.08% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 18 | 1.06% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 19 | 0.98% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5629` |
| 20 | 0.97% | `net.minecraft.world.entity.Mob.serverAiStep() line: 843` |
| 21 | 0.95% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 22 | 0.95% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 23 | 0.89% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 24 | 0.86% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 25 | 0.85% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 26 | 0.85% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 27 | 0.82% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 28 | 0.78% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2674` |
| 29 | 0.76% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 30 | 0.72% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 6837` |

GC pauses: 0, total 0 ms

