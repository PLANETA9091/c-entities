# JVM hotspots (JFR ExecutionSample)

Samples: 15062

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.84% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 2.53% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 2.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 4 | 2.10% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 5 | 2.04% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8929` |
| 6 | 1.75% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 7 | 1.67% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 8 | 1.67% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 9 | 1.61% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 10 | 1.57% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8788` |
| 11 | 1.55% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 12 | 1.43% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 13 | 1.12% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 14 | 1.09% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 15 | 0.84% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 16 | 0.79% | `java.lang.ref.Reference.get() line: 361` |
| 17 | 0.77% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 18 | 0.72% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5824` |
| 19 | 0.71% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 20 | 0.71% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 21 | 0.71% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 22 | 0.70% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 23 | 0.62% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5679` |
| 24 | 0.62% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 4170` |
| 25 | 0.60% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 26 | 0.59% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 27 | 0.58% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4080` |
| 28 | 0.56% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 6841` |
| 29 | 0.52% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8735` |
| 30 | 0.51% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 31 | 0.50% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 32 | 0.50% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 33 | 0.49% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4017` |
| 34 | 0.46% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 35 | 0.44% | `java.util.EnumMap.get(Object) line: 248` |
| 36 | 0.44% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 37 | 0.43% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 38 | 0.42% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 39 | 0.41% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 40 | 0.40% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 41 | 0.40% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4325` |
| 42 | 0.40% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 43 | 0.39% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 44 | 0.38% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 45 | 0.37% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 46 | 0.37% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 47 | 0.37% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 48 | 0.35% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 49 | 0.35% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 50 | 0.34% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 51 | 0.34% | `java.lang.Enum.ordinal() line: 128` |
| 52 | 0.31% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 53 | 0.31% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 54 | 0.30% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 8996` |
| 55 | 0.29% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 56 | 0.29% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 57 | 0.29% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8874` |
| 58 | 0.28% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 59 | 0.28% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2699` |
| 60 | 0.28% | `java.util.concurrent.ConcurrentHashMap.computeIfAbsent(Object, Function) line: 1737` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 2.53% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 2.50% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 3 | 2.26% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8801` |
| 4 | 2.10% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 5 | 1.95% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 6 | 1.65% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 7 | 1.61% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 8 | 1.51% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8776` |
| 9 | 1.49% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 10 | 1.30% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 11 | 1.27% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 12 | 1.25% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 13 | 1.16% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 14 | 1.14% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 15 | 1.10% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 16 | 1.05% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 17 | 1.02% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5629` |
| 18 | 0.97% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 19 | 0.93% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 20 | 0.91% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 21 | 0.90% | `net.minecraft.world.entity.Mob.serverAiStep() line: 839` |
| 22 | 0.87% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 23 | 0.87% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4324` |
| 24 | 0.86% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 25 | 0.82% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 26 | 0.79% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 27 | 0.79% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 4190` |
| 28 | 0.78% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 29 | 0.78% | `net.minecraft.world.entity.Mob.serverAiStep() line: 836` |
| 30 | 0.76% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |

GC pauses: 0, total 0 ms

