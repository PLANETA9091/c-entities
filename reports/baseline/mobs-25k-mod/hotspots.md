# JVM hotspots (JFR ExecutionSample)

Samples: 16090

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.60% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 2.24% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 1.93% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 4 | 1.72% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 5 | 1.68% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 6 | 1.68% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8929` |
| 7 | 1.66% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 8 | 1.62% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8788` |
| 9 | 1.47% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 10 | 1.45% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 11 | 1.34% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 12 | 1.15% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 13 | 0.96% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 14 | 0.94% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 15 | 0.91% | `java.lang.ref.Reference.get() line: 361` |
| 16 | 0.89% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 17 | 0.88% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 18 | 0.85% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5824` |
| 19 | 0.85% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 20 | 0.83% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 993` |
| 21 | 0.78% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 22 | 0.74% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 23 | 0.70% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 24 | 0.63% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 25 | 0.62% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5679` |
| 26 | 0.62% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4080` |
| 27 | 0.57% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 28 | 0.57% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 6841` |
| 29 | 0.55% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 30 | 0.53% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 4170` |
| 31 | 0.52% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 100` |
| 32 | 0.52% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8735` |
| 33 | 0.51% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 34 | 0.50% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 35 | 0.50% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 36 | 0.47% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 37 | 0.45% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 38 | 0.44% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4017` |
| 39 | 0.43% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 40 | 0.42% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 41 | 0.42% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 98` |
| 42 | 0.39% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 43 | 0.39% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 44 | 0.37% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 45 | 0.37% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 46 | 0.36% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 47 | 0.35% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8874` |
| 48 | 0.35% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 49 | 0.34% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 50 | 0.34% | `java.lang.Enum.ordinal() line: 128` |
| 51 | 0.34% | `net.minecraft.world.entity.ai.control.LookControl.tick() line: 55` |
| 52 | 0.33% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 53 | 0.33% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 54 | 0.33% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 55 | 0.33% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 56 | 0.32% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 8996` |
| 57 | 0.32% | `net.minecraft.core.Holder$Reference.is(TagKey) line: 195` |
| 58 | 0.32% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4427` |
| 59 | 0.30% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 60 | 0.30% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4325` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 2.37% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8801` |
| 2 | 2.24% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 2.01% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 4 | 1.86% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 5 | 1.82% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 6 | 1.72% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 7 | 1.70% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 8 | 1.68% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 9 | 1.68% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 10 | 1.50% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 11 | 1.41% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 12 | 1.35% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8776` |
| 13 | 1.24% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 14 | 1.17% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 15 | 1.15% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 16 | 1.11% | `net.minecraft.world.entity.Mob.serverAiStep() line: 843` |
| 17 | 1.07% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 18 | 0.99% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 19 | 0.93% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 20 | 0.93% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 21 | 0.91% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 4190` |
| 22 | 0.89% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 23 | 0.87% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5629` |
| 24 | 0.83% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 25 | 0.79% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 26 | 0.74% | `net.minecraft.world.entity.Mob.serverAiStep() line: 836` |
| 27 | 0.73% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 28 | 0.73% | `dev.planeta9091.centities.CeHooks.pushDispatch(Entity) line: 3376` |
| 29 | 0.72% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 30 | 0.70% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 216` |

GC pauses: 0, total 0 ms

