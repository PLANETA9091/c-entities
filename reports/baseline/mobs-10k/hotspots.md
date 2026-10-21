# JVM hotspots (JFR ExecutionSample)

Samples: 11256

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 13.28% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.57% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 3.50% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 4 | 2.88% | `java.util.HashMap.getNode(Object) line: 585` |
| 5 | 1.82% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 6 | 1.75% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 7 | 1.36% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 8 | 1.35% | `java.util.EnumMap.get(Object) line: 247` |
| 9 | 1.29% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 10 | 1.24% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 11 | 1.23% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 1.18% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 13 | 1.04% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1414` |
| 14 | 1.02% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 15 | 1.01% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 16 | 1.01% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 17 | 0.95% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 18 | 0.92% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 19 | 0.85% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 20 | 0.84% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 21 | 0.76% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 22 | 0.76% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 23 | 0.73% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 24 | 0.72% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 25 | 0.69% | `java.util.ArrayDeque.size() line: 648` |
| 26 | 0.67% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1313` |
| 27 | 0.67% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 28 | 0.63% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 29 | 0.60% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 30 | 0.60% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 31 | 0.60% | `java.lang.ThreadLocal.get() line: 172` |
| 32 | 0.59% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 134` |
| 33 | 0.58% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 34 | 0.56% | `io.netty.channel.ChannelOutboundBuffer.addFlush() line: 159` |
| 35 | 0.55% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 36 | 0.54% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 37 | 0.54% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 38 | 0.54% | `net.minecraft.core.Vec3i.getZ() line: 71` |
| 39 | 0.54% | `sun.misc.Unsafe.compareAndSwapLong(Object, long, long, long) line: 938` |
| 40 | 0.52% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 41 | 0.51% | `java.lang.Enum.ordinal() line: 128` |
| 42 | 0.51% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3459` |
| 43 | 0.48% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 44 | 0.47% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 45 | 0.46% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 46 | 0.39% | `net.minecraft.world.entity.Entity.getAirSupply() line: 3608` |
| 47 | 0.39% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 48 | 0.36% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartB() line: 938` |
| 49 | 0.36% | `java.lang.invoke.VarHandleLongs$FieldInstanceReadWrite.weakCompareAndSetRelease(VarHandle, Object, long, long) line: 241` |
| 50 | 0.36% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 51 | 0.36% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 175` |
| 52 | 0.35% | `net.minecraft.world.level.pathfinder.Path.getNextNodePos() line: 89` |
| 53 | 0.35% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 54 | 0.33% | `net.minecraft.world.level.pathfinder.Path.isDone() line: 38` |
| 55 | 0.33% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |
| 56 | 0.31% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3690` |
| 57 | 0.30% | `java.util.EnumMap.isValidKey(Object) line: 319` |
| 58 | 0.30% | `net.minecraft.world.item.ItemStack.getDamageValue() line: 597` |
| 59 | 0.30% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 60 | 0.30% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 13.28% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 4.93% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 3.50% | `java.util.stream.AbstractPipeline.wrapAndCopyInto(Sink, Spliterator) line: 499` |
| 4 | 2.35% | `java.util.HashMap.containsKey(Object) line: 602` |
| 5 | 1.75% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 6 | 1.66% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 7 | 1.36% | `net.minecraft.world.entity.Mob.serverAiStep() line: 759` |
| 8 | 1.33% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 9 | 1.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 10 | 1.18% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 78` |
| 11 | 1.14% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 12 | 1.12% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 13 | 1.09% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 14 | 1.08% | `net.minecraft.world.entity.Entity.getBlockPosBelowThatAffectsMyMovement() line: 1406` |
| 15 | 1.01% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 16 | 1.01% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 17 | 1.00% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges() line: 3348` |
| 18 | 1.00% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 19 | 0.95% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 20 | 0.84% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 21 | 0.84% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2721` |
| 22 | 0.81% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 23 | 0.80% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 212` |
| 24 | 0.79% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 25 | 0.78% | `net.minecraft.world.entity.Mob.serverAiStep() line: 764` |
| 26 | 0.72% | `net.minecraft.world.entity.Mob.serverAiStep() line: 774` |
| 27 | 0.69% | `net.minecraft.world.entity.Entity.addMovementThisTick(Entity$Movement) line: 1282` |
| 28 | 0.68% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 235` |
| 29 | 0.68% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 30 | 0.68% | `net.minecraft.world.entity.LivingEntity.tick() line: 3315` |

GC pauses: 0, total 0 ms

