# JVM hotspots (JFR ExecutionSample)

Samples: 42179

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 26.34% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 5.68% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 3.23% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 2.85% | `java.util.HashMap.getNode(Object) line: 585` |
| 5 | 2.74% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 6 | 1.35% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 7 | 1.30% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 8 | 1.19% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 9 | 1.04% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 61` |
| 10 | 1.03% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 11 | 1.00% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 12 | 0.89% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 13 | 0.85% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 3849` |
| 14 | 0.84% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 15 | 0.78% | `java.util.EnumMap.get(Object) line: 247` |
| 16 | 0.73% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 17 | 0.72% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 18 | 0.68% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 19 | 0.68% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 20 | 0.64% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 21 | 0.63% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3690` |
| 22 | 0.62% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 23 | 0.61% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 24 | 0.60% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 25 | 0.60% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 26 | 0.58% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 27 | 0.57% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1313` |
| 28 | 0.56% | `java.util.EnumMap.isValidKey(Object) line: 319` |
| 29 | 0.55% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 30 | 0.55% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 31 | 0.54% | `net.minecraft.world.entity.Entity.level() line: 5199` |
| 32 | 0.53% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 320` |
| 33 | 0.51% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 34 | 0.50% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 35 | 0.50% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 36 | 0.47% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 37 | 0.46% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3456` |
| 38 | 0.44% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 70` |
| 39 | 0.43% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 40 | 0.41% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 41 | 0.39% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 42 | 0.38% | `net.minecraft.core.Vec3i.getZ() line: 71` |
| 43 | 0.37% | `java.util.ArrayDeque.size() line: 648` |
| 44 | 0.36% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 45 | 0.35% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 46 | 0.34% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 134` |
| 47 | 0.32% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 48 | 0.31% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 437` |
| 49 | 0.30% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 50 | 0.29% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 51 | 0.29% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 52 | 0.28% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 53 | 0.28% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 54 | 0.27% | `net.minecraft.world.item.ItemStack.setDamageValue(int) line: 601` |
| 55 | 0.27% | `java.lang.ThreadLocal.get() line: 172` |
| 56 | 0.27% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 57 | 0.26% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 58 | 0.26% | `net.minecraft.world.level.chunk.PalettedContainer.readPalette(PalettedContainer$Data, int) line: 96` |
| 59 | 0.26% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 444` |
| 60 | 0.25% | `net.minecraft.world.entity.Entity.getAirSupply() line: 3608` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 26.34% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 9.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 4.24% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 4 | 2.84% | `java.util.HashMap.containsKey(Object) line: 602` |
| 5 | 1.42% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 6 | 1.30% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 7 | 1.03% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 12` |
| 8 | 1.00% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 78` |
| 9 | 0.89% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 10 | 0.87% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 11 | 0.85% | `net.minecraft.world.entity.EntitySelector.lambda$pushable$9(boolean, Entity, Team, Team$CollisionRule, Entity) line: 58` |
| 12 | 0.80% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 13 | 0.79% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 14 | 0.75% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2274` |
| 15 | 0.73% | `java.util.stream.AbstractPipeline.wrapAndCopyInto(Sink, Spliterator) line: 499` |
| 16 | 0.72% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 17 | 0.71% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 18 | 0.67% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 19 | 0.67% | `net.minecraft.world.entity.Mob.serverAiStep() line: 759` |
| 20 | 0.67% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3571` |
| 21 | 0.63% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 22 | 0.63% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2721` |
| 23 | 0.63% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 24 | 0.61% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 25 | 0.59% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 26 | 0.57% | `net.minecraft.world.entity.LivingEntity.tick() line: 3315` |
| 27 | 0.56% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 28 | 0.54% | `java.util.EnumMap.get(Object) line: 246` |
| 29 | 0.54% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |
| 30 | 0.53% | `net.minecraft.world.scores.Scoreboard.getPlayersTeam(String) line: 317` |

GC pauses: 0, total 0 ms

