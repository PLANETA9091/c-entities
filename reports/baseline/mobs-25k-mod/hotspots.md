# JVM hotspots (JFR ExecutionSample)

Samples: 12744

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.06% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 2.85% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 3 | 2.06% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 2.06% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 5 | 1.95% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 6 | 1.91% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9126` |
| 7 | 1.57% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9267` |
| 8 | 1.55% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 9 | 1.54% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 10 | 1.49% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 11 | 1.41% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 12 | 1.16% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 1.15% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 14 | 1.09% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 15 | 1.04% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 993` |
| 16 | 0.93% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9073` |
| 17 | 0.87% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4352` |
| 18 | 0.81% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6149` |
| 19 | 0.80% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 20 | 0.80% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 21 | 0.79% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 22 | 0.71% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 23 | 0.70% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 24 | 0.70% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 25 | 0.70% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 26 | 0.64% | `java.lang.ref.Reference.get() line: 361` |
| 27 | 0.64% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5995` |
| 28 | 0.61% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 29 | 0.59% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 30 | 0.56% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 31 | 0.56% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 32 | 0.53% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 100` |
| 33 | 0.50% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4289` |
| 34 | 0.45% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 35 | 0.43% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 36 | 0.43% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 37 | 0.42% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 38 | 0.42% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 39 | 0.42% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 98` |
| 40 | 0.42% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 41 | 0.42% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 42 | 0.42% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 43 | 0.42% | `dev.planeta9091.centities.CeHooks.attrCache(LivingEntity) line: 4442` |
| 44 | 0.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 45 | 0.38% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4619` |
| 46 | 0.37% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 47 | 0.37% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 48 | 0.36% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 49 | 0.35% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 50 | 0.35% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 51 | 0.35% | `java.util.EnumMap.get(Object) line: 248` |
| 52 | 0.35% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 53 | 0.35% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 54 | 0.35% | `net.minecraft.world.level.levelgen.BitRandomSource.nextInt(int) line: 31` |
| 55 | 0.34% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 56 | 0.33% | `java.lang.Enum.ordinal() line: 128` |
| 57 | 0.32% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7179` |
| 58 | 0.31% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 59 | 0.31% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 60 | 0.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 2.14% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9139` |
| 2 | 2.11% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 3 | 2.06% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 2.02% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 5 | 1.96% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 6 | 1.68% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 7 | 1.58% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 8 | 1.57% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 9 | 1.51% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 10 | 1.50% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 11 | 1.44% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 12 | 1.33% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 13 | 1.23% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 14 | 1.19% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 15 | 1.13% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 16 | 1.11% | `net.minecraft.world.entity.Mob.serverAiStep() line: 843` |
| 17 | 1.11% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 18 | 1.11% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 19 | 1.07% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 20 | 1.00% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 21 | 0.98% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9122` |
| 22 | 0.97% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5945` |
| 23 | 0.92% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 24 | 0.90% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 25 | 0.89% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9114` |
| 26 | 0.89% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 27 | 0.80% | `net.minecraft.world.entity.Mob.serverAiStep() line: 836` |
| 28 | 0.79% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 216` |
| 29 | 0.78% | `net.minecraft.world.entity.Mob.serverAiStep() line: 839` |
| 30 | 0.77% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |

GC pauses: 0, total 0 ms

