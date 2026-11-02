# JVM hotspots (JFR ExecutionSample)

Samples: 9617

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 2.81% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.08% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 3 | 1.93% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 4 | 1.85% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10150` |
| 5 | 1.68% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9548` |
| 6 | 1.60% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 7 | 1.58% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6273` |
| 8 | 1.54% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 9 | 1.39% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 10 | 1.33% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 11 | 1.29% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 12 | 1.28% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 13 | 1.12% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6780` |
| 14 | 1.10% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4417` |
| 15 | 1.06% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 16 | 1.05% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6848` |
| 17 | 0.98% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 18 | 0.88% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 19 | 0.85% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 20 | 0.82% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 6446` |
| 21 | 0.82% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 22 | 0.80% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 23 | 0.75% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 24 | 0.72% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 25 | 0.67% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 26 | 0.66% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 27 | 0.64% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7390` |
| 28 | 0.62% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 29 | 0.60% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 30 | 0.60% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 31 | 0.58% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 32 | 0.58% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 33 | 0.57% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4354` |
| 34 | 0.53% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 35 | 0.52% | `net.minecraft.world.level.levelgen.BitRandomSource.nextInt(int) line: 31` |
| 36 | 0.51% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 37 | 0.49% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 10243` |
| 38 | 0.48% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 39 | 0.48% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 40 | 0.47% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$hasPlayers() line: 1347` |
| 41 | 0.47% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 42 | 0.46% | `java.util.EnumMap.get(Object) line: 248` |
| 43 | 0.45% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 44 | 0.44% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 45 | 0.44% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 6624` |
| 46 | 0.44% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 47 | 0.43% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 48 | 0.43% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 49 | 0.41% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 50 | 0.40% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 51 | 0.36% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 52 | 0.35% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 53 | 0.35% | `net.minecraft.world.entity.ai.goal.GoalSelector.getAvailableGoals() line: 134` |
| 54 | 0.34% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 55 | 0.34% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4660` |
| 56 | 0.33% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 57 | 0.32% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 10062` |
| 58 | 0.30% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 59 | 0.30% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6864` |
| 60 | 0.29% | `net.minecraft.world.entity.Entity.getDeltaMovement() line: 5074` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 3.90% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 2 | 2.81% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 2.11% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 4 | 2.08% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6863` |
| 5 | 2.02% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 1.90% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9696` |
| 7 | 1.86% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 8 | 1.78% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 9 | 1.75% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 10 | 1.46% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2515` |
| 11 | 1.41% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 12 | 1.40% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 13 | 1.39% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6848` |
| 14 | 1.30% | `dev.planeta9091.centities.CeHooks.pushDispatch(Entity) line: 3595` |
| 15 | 1.28% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 16 | 1.03% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6779` |
| 17 | 1.00% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 18 | 1.00% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 19 | 0.92% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 20 | 0.90% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 21 | 0.85% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 22 | 0.80% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 23 | 0.79% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 24 | 0.77% | `net.minecraft.world.entity.ai.sensing.Sensing.tick()` |
| 25 | 0.75% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 26 | 0.75% | `net.minecraft.world.level.EntityGetter.findNearbyPlayer(Entity, double, Predicate) line: 117` |
| 27 | 0.74% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4661` |
| 28 | 0.73% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 29 | 0.72% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 59` |
| 30 | 0.71% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |

GC pauses: 0, total 0 ms

