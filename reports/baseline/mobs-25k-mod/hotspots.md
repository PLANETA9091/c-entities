# JVM hotspots (JFR ExecutionSample)

Samples: 16791

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.48% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 2.98% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 2.39% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 4 | 1.73% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 5 | 1.64% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8593` |
| 6 | 1.52% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 7 | 1.46% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 8 | 1.45% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8734` |
| 9 | 1.27% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5629` |
| 10 | 1.17% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 11 | 1.11% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 12 | 1.05% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 13 | 0.96% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 14 | 0.94% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 15 | 0.89% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 16 | 0.87% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 17 | 0.86% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 18 | 0.83% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 19 | 0.81% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 20 | 0.80% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 21 | 0.73% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |
| 22 | 0.71% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 993` |
| 23 | 0.71% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 24 | 0.68% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 25 | 0.66% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 26 | 0.63% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 27 | 0.62% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 28 | 0.61% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 29 | 0.55% | `java.lang.ref.Reference.get() line: 361` |
| 30 | 0.55% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3885` |
| 31 | 0.51% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 32 | 0.51% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 33 | 0.51% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3822` |
| 34 | 0.51% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8540` |
| 35 | 0.49% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 320` |
| 36 | 0.48% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 37 | 0.48% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 38 | 0.46% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 39 | 0.44% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 40 | 0.42% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 41 | 0.42% | `net.minecraft.world.level.levelgen.BitRandomSource.nextInt(int) line: 31` |
| 42 | 0.40% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 8801` |
| 43 | 0.39% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 44 | 0.39% | `dev.planeta9091.centities.CeHooks.pushDispatch(Entity) line: 3204` |
| 45 | 0.38% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 46 | 0.38% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 98` |
| 47 | 0.37% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 48 | 0.37% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 49 | 0.37% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 50 | 0.36% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 3975` |
| 51 | 0.34% | `java.util.EnumMap.get(Object) line: 248` |
| 52 | 0.34% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5484` |
| 53 | 0.33% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 100` |
| 54 | 0.33% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4130` |
| 55 | 0.32% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 56 | 0.32% | `java.util.concurrent.ConcurrentHashMap.computeIfAbsent(Object, Function) line: 1737` |
| 57 | 0.32% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 58 | 0.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 59 | 0.30% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8679` |
| 60 | 0.29% | `java.lang.ThreadLocal.get() line: 171` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 2.98% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 1.87% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 3 | 1.77% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8606` |
| 4 | 1.69% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 1.58% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8581` |
| 6 | 1.56% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 7 | 1.45% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 8 | 1.44% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 9 | 1.40% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 10 | 1.39% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 11 | 1.30% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 12 | 1.30% | `dev.planeta9091.centities.CeHooks.pushDispatch(Entity) line: 3205` |
| 13 | 1.19% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 14 | 1.13% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 15 | 1.13% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 16 | 1.11% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 17 | 1.04% | `net.minecraft.world.entity.Mob.serverAiStep() line: 843` |
| 18 | 1.04% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 19 | 1.02% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 20 | 0.92% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 21 | 0.91% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5434` |
| 22 | 0.89% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 23 | 0.85% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 24 | 0.83% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 25 | 0.82% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2510` |
| 26 | 0.80% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 27 | 0.78% | `net.minecraft.world.entity.LivingEntity.pushEntities()` |
| 28 | 0.77% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 29 | 0.77% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 30 | 0.76% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |

GC pauses: 0, total 0 ms

