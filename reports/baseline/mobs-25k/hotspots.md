# JVM hotspots (JFR ExecutionSample)

Samples: 38324

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 25.25% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 7.59% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 3.51% | `java.util.HashMap.getNode(Object) line: 585` |
| 4 | 2.95% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 2.17% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 6 | 2.09% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 7 | 1.61% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 8 | 1.45% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 9 | 1.33% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 10 | 1.18% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 3849` |
| 11 | 0.97% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 12 | 0.94% | `net.minecraft.world.entity.LivingEntity.doPush(Entity) line: 3738` |
| 13 | 0.91% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 14 | 0.87% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 15 | 0.83% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 16 | 0.81% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 17 | 0.80% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 18 | 0.79% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 19 | 0.78% | `java.util.EnumMap.get(Object) line: 247` |
| 20 | 0.78% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 21 | 0.76% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 22 | 0.74% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 23 | 0.69% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 24 | 0.67% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 25 | 0.67% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 26 | 0.63% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1313` |
| 27 | 0.62% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 28 | 0.58% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 29 | 0.57% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 320` |
| 30 | 0.56% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 31 | 0.55% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 32 | 0.53% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4882` |
| 33 | 0.47% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 34 | 0.42% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 70` |
| 35 | 0.39% | `java.util.ArrayDeque.size() line: 648` |
| 36 | 0.39% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 37 | 0.38% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 38 | 0.37% | `net.minecraft.core.Vec3i.getZ() line: 71` |
| 39 | 0.37% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 40 | 0.37% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 41 | 0.37% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 42 | 0.37% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 134` |
| 43 | 0.36% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightInterface.getRawBrightness(BlockPos, int) line: 322` |
| 44 | 0.36% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 45 | 0.36% | `java.lang.ThreadLocal.get() line: 172` |
| 46 | 0.35% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 47 | 0.35% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 48 | 0.33% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 47` |
| 49 | 0.33% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 50 | 0.32% | `java.util.EnumMap.isValidKey(Object) line: 319` |
| 51 | 0.32% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 52 | 0.30% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightInterface.getSkyLightValue(BlockPos, ChunkAccess) line: 256` |
| 53 | 0.30% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 157` |
| 54 | 0.30% | `net.minecraft.world.level.pathfinder.Path.getNextNodePos() line: 89` |
| 55 | 0.30% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1661` |
| 56 | 0.30% | `net.minecraft.util.Mth.packDegrees(float) line: 178` |
| 57 | 0.29% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 58 | 0.29% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 59 | 0.29% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 60 | 0.28% | `net.minecraft.world.entity.Entity.getAirSupply() line: 3608` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 25.25% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 10.43% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 3.81% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 4 | 3.51% | `java.util.HashMap.containsKey(Object) line: 602` |
| 5 | 2.18% | `java.util.stream.AbstractPipeline.wrapAndCopyInto(Sink, Spliterator) line: 499` |
| 6 | 1.61% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 7 | 1.29% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 8 | 1.18% | `net.minecraft.world.entity.EntitySelector.lambda$pushable$9(boolean, Entity, Team, Team$CollisionRule, Entity) line: 58` |
| 9 | 1.11% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 10 | 0.97% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 11 | 0.94% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3693` |
| 12 | 0.85% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 13 | 0.83% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 14 | 0.81% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2721` |
| 15 | 0.80% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2274` |
| 16 | 0.79% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 17 | 0.79% | `net.minecraft.world.entity.Mob.serverAiStep() line: 759` |
| 18 | 0.78% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 78` |
| 19 | 0.78% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 20 | 0.74% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 21 | 0.74% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 22 | 0.73% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |
| 23 | 0.68% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 24 | 0.64% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 25 | 0.63% | `net.minecraft.world.entity.LivingEntity.tick() line: 3315` |
| 26 | 0.62% | `net.minecraft.world.scores.Scoreboard.getPlayersTeam(String) line: 317` |
| 27 | 0.61% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 28 | 0.60% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 29 | 0.59% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 30 | 0.57% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |

GC pauses: 0, total 0 ms

