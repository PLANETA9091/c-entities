# JVM hotspots (JFR ExecutionSample)

Samples: 9132

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 2.83% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 2 | 2.30% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 1.97% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 4 | 1.71% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 5 | 1.62% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9597` |
| 6 | 1.52% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 7 | 1.45% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 8 | 1.39% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 9 | 1.38% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 10 | 1.36% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10199` |
| 11 | 1.25% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6311` |
| 12 | 1.18% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6886` |
| 13 | 1.14% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 14 | 1.13% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 15 | 1.07% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 16 | 1.04% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 17 | 0.93% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4380` |
| 18 | 0.92% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 19 | 0.92% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6818` |
| 20 | 0.90% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4443` |
| 21 | 0.84% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 22 | 0.83% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 23 | 0.74% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3695` |
| 24 | 0.74% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 6484` |
| 25 | 0.73% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 26 | 0.70% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 27 | 0.69% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 28 | 0.65% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 29 | 0.61% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 30 | 0.57% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 31 | 0.55% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 6662` |
| 32 | 0.54% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 33 | 0.53% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$hasPlayers() line: 1347` |
| 34 | 0.51% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 35 | 0.50% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 36 | 0.49% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 37 | 0.49% | `net.minecraft.world.level.levelgen.BitRandomSource.nextInt(int) line: 31` |
| 38 | 0.47% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 39 | 0.47% | `java.util.EnumMap.get(Object) line: 248` |
| 40 | 0.47% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 41 | 0.47% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7428` |
| 42 | 0.46% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 43 | 0.43% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 44 | 0.41% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 45 | 0.38% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 46 | 0.38% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 4686` |
| 47 | 0.37% | `net.minecraft.world.phys.Vec3.subtract(Vec3) line: 103` |
| 48 | 0.36% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 49 | 0.35% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 50 | 0.34% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 10111` |
| 51 | 0.33% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 320` |
| 52 | 0.33% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 53 | 0.33% | `java.lang.ref.Reference.refersToImpl(Object) line: 391` |
| 54 | 0.33% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 55 | 0.32% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 228` |
| 56 | 0.32% | `net.minecraft.world.entity.ai.goal.GoalSelector.getAvailableGoals() line: 134` |
| 57 | 0.32% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 10292` |
| 58 | 0.32% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 59 | 0.32% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 60 | 0.31% | `java.lang.String.hashCode() line: 2430` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 3.85% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 2 | 2.96% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6901` |
| 3 | 2.30% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 2.18% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 5 | 2.01% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 6 | 1.93% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 7 | 1.77% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 8 | 1.76% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9745` |
| 9 | 1.69% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 10 | 1.52% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4298` |
| 11 | 1.49% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6886` |
| 12 | 1.36% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 13 | 1.31% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 14 | 1.28% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 15 | 1.22% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 16 | 1.15% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 17 | 1.13% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 18 | 1.10% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6817` |
| 19 | 1.07% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 20 | 0.99% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 21 | 0.99% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2515` |
| 22 | 0.96% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 23 | 0.91% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 24 | 0.84% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 25 | 0.79% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 26 | 0.78% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 27 | 0.77% | `net.minecraft.world.entity.Entity.checkSupportingBlock(boolean, Vec3) line: 1128` |
| 28 | 0.76% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 29 | 0.70% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 30 | 0.69% | `dev.planeta9091.centities.CeHooks.goalSelectorTickRunning(GoalSelector, boolean) line: 6974` |

GC pauses: 0, total 0 ms

