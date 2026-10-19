# JVM hotspots (JFR ExecutionSample)

Samples: 42793

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 26.21% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 5.35% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 2.90% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 2.59% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 5 | 2.54% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 6 | 1.78% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 7 | 1.20% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 8 | 1.10% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 9 | 1.10% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 61` |
| 10 | 1.09% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 11 | 1.03% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 320` |
| 12 | 0.91% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 3849` |
| 13 | 0.84% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 14 | 0.82% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 15 | 0.78% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 16 | 0.77% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 17 | 0.74% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 18 | 0.73% | `java.util.HashMap.getNode(Object) line: 585` |
| 19 | 0.72% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 20 | 0.72% | `java.lang.ThreadLocal$ThreadLocalMap.getEntry(ThreadLocal) line: 504` |
| 21 | 0.67% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 22 | 0.66% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 23 | 0.63% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 24 | 0.61% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3690` |
| 25 | 0.60% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 26 | 0.59% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 27 | 0.59% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 70` |
| 28 | 0.57% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 29 | 0.55% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 30 | 0.55% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 31 | 0.50% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 32 | 0.50% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 33 | 0.49% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 34 | 0.49% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 35 | 0.47% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 36 | 0.43% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 37 | 0.43% | `java.util.ArrayDeque.size() line: 648` |
| 38 | 0.42% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 39 | 0.42% | `java.util.EnumMap.isValidKey(Object) line: 319` |
| 40 | 0.42% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 41 | 0.41% | `it.unimi.dsi.fastutil.objects.ObjectOpenHashSet$SetIterator.<init>(ObjectOpenHashSet, ObjectOpenHashSet$1) line: 534` |
| 42 | 0.41% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges() line: 3348` |
| 43 | 0.39% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 44 | 0.39% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 45 | 0.36% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 46 | 0.34% | `net.minecraft.core.Vec3i.getZ() line: 71` |
| 47 | 0.34% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 48 | 0.33% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 49 | 0.33% | `java.util.EnumMap.get(Object) line: 247` |
| 50 | 0.32% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1422` |
| 51 | 0.32% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 52 | 0.30% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 53 | 0.30% | `net.minecraft.world.item.ItemStack.getDamageValue() line: 597` |
| 54 | 0.29% | `net.minecraft.util.Mth.packDegrees(float) line: 178` |
| 55 | 0.29% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 56 | 0.28% | `net.minecraft.world.entity.Entity.level() line: 5199` |
| 57 | 0.26% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 58 | 0.26% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 437` |
| 59 | 0.26% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1661` |
| 60 | 0.26% | `net.minecraft.world.entity.Entity.getAirSupply() line: 3608` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 26.21% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 8.95% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 4.01% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 4 | 2.59% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 5 | 1.80% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 6 | 1.30% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 7 | 1.09% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 12` |
| 8 | 1.06% | `net.minecraft.world.scores.Scoreboard.getPlayersTeam(String) line: 317` |
| 9 | 1.06% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 10 | 1.05% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 11 | 0.97% | `net.minecraft.world.entity.EntitySelector.lambda$pushable$9(boolean, Entity, Team, Team$CollisionRule, Entity) line: 58` |
| 12 | 0.91% | `java.lang.ThreadLocal.get(Thread) line: 187` |
| 13 | 0.81% | `net.minecraft.world.entity.Mob.serverAiStep() line: 759` |
| 14 | 0.77% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 549` |
| 15 | 0.75% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 16 | 0.74% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 78` |
| 17 | 0.74% | `java.util.HashMap.containsKey(Object) line: 602` |
| 18 | 0.69% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 19 | 0.68% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2274` |
| 20 | 0.67% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 21 | 0.66% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 22 | 0.65% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 23 | 0.65% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 24 | 0.64% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3571` |
| 25 | 0.62% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 26 | 0.61% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 119` |
| 27 | 0.60% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 28 | 0.54% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 29 | 0.51% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 30 | 0.50% | `net.minecraft.world.entity.Mob.serverAiStep() line: 764` |

GC pauses: 0, total 0 ms

