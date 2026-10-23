# JVM hotspots (JFR ExecutionSample)

Samples: 37942

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 17.15% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 5.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 4.67% | `java.util.HashMap.getNode(Object) line: 585` |
| 4 | 1.85% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 1.54% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 6 | 1.44% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 61` |
| 7 | 1.35% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 8 | 1.24% | `net.minecraft.world.item.ItemStack.getDamageValue() line: 597` |
| 9 | 1.15% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 10 | 1.12% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 11 | 1.12% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 12 | 1.10% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 13 | 1.09% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 14 | 1.05% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 15 | 1.05% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 16 | 1.03% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 17 | 0.91% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 18 | 0.82% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 19 | 0.82% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 20 | 0.81% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3690` |
| 21 | 0.81% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 22 | 0.79% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 23 | 0.77% | `io.netty.channel.ChannelOutboundBuffer.addFlush() line: 159` |
| 24 | 0.74% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 25 | 0.72% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 26 | 0.72% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 27 | 0.68% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1414` |
| 28 | 0.68% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 29 | 0.65% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1313` |
| 30 | 0.65% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 31 | 0.61% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 32 | 0.59% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 33 | 0.57% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 34 | 0.57% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 35 | 0.56% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 36 | 0.55% | `java.util.ArrayDeque.size() line: 648` |
| 37 | 0.54% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 38 | 0.53% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 70` |
| 39 | 0.50% | `net.minecraft.network.syncher.SynchedEntityData.packDirty() line: 83` |
| 40 | 0.47% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 41 | 0.46% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 134` |
| 42 | 0.46% | `java.lang.invoke.VarHandleLongs$FieldInstanceReadWrite.weakCompareAndSetRelease(VarHandle, Object, long, long) line: 241` |
| 43 | 0.45% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 44 | 0.43% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 45 | 0.43% | `java.lang.ThreadLocal.get() line: 172` |
| 46 | 0.41% | `io.netty.channel.unix.IovArray.processMessage(Object) line: 227` |
| 47 | 0.40% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3459` |
| 48 | 0.37% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4882` |
| 49 | 0.36% | `sun.misc.Unsafe.compareAndSwapLong(Object, long, long, long) line: 938` |
| 50 | 0.35% | `net.minecraft.world.level.pathfinder.Path.getNextNodePos() line: 89` |
| 51 | 0.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 52 | 0.34% | `net.minecraft.core.Vec3i.getZ() line: 71` |
| 53 | 0.33% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 54 | 0.32% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 175` |
| 55 | 0.32% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 56 | 0.31% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 57 | 0.31% | `io.netty.channel.ChannelOutboundBuffer.forEachFlushedMessage(ChannelOutboundBuffer$MessageProcessor) line: 806` |
| 58 | 0.31% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 59 | 0.30% | `java.lang.Enum.ordinal() line: 128` |
| 60 | 0.30% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 17.15% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 7.69% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 4.67% | `java.util.HashMap.containsKey(Object) line: 602` |
| 4 | 2.76% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 5 | 1.46% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 12` |
| 6 | 1.44% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 7 | 1.35% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 8 | 1.19% | `net.minecraft.world.entity.Mob.serverAiStep() line: 759` |
| 9 | 1.14% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 10 | 1.12% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 11 | 1.10% | `net.minecraft.world.item.ItemStack.isBroken() line: 609` |
| 12 | 1.10% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 549` |
| 13 | 1.09% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 78` |
| 14 | 1.07% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 15 | 0.95% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2274` |
| 16 | 0.94% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 17 | 0.90% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 18 | 0.83% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3571` |
| 19 | 0.83% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 20 | 0.82% | `io.netty.channel.AbstractChannel$AbstractUnsafe.flush() line: 892` |
| 21 | 0.79% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2721` |
| 22 | 0.78% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 23 | 0.74% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 24 | 0.72% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 25 | 0.71% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 26 | 0.70% | `net.minecraft.world.entity.Entity.getBlockPosBelowThatAffectsMyMovement() line: 1406` |
| 27 | 0.67% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 212` |
| 28 | 0.66% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 29 | 0.65% | `net.minecraft.world.entity.LivingEntity.tick() line: 3315` |
| 30 | 0.65% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |

GC pauses: 0, total 0 ms

