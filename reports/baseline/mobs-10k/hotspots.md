# JVM hotspots (JFR ExecutionSample)

Samples: 10875

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 14.28% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.89% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 2.43% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 4 | 1.59% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 5 | 1.54% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 6 | 1.54% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 1.40% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 8 | 1.32% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 9 | 1.27% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 10 | 1.16% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 11 | 1.11% | `java.util.HashMap.getNode(Object) line: 585` |
| 12 | 1.03% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 13 | 1.01% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 14 | 1.01% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 15 | 0.91% | `java.util.EnumMap.get(Object) line: 247` |
| 16 | 0.88% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 17 | 0.88% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 18 | 0.88% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 19 | 0.83% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 20 | 0.79% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 21 | 0.78% | `java.lang.ThreadLocal$ThreadLocalMap.getEntry(ThreadLocal) line: 504` |
| 22 | 0.75% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 23 | 0.69% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 24 | 0.68% | `net.minecraft.world.item.ItemStack.getDamageValue() line: 597` |
| 25 | 0.65% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 26 | 0.65% | `net.minecraft.world.entity.Entity.level() line: 5199` |
| 27 | 0.64% | `net.minecraft.util.SimpleBitStorage.get(int) line: 336` |
| 28 | 0.63% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 29 | 0.59% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 30 | 0.59% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1315` |
| 31 | 0.58% | `net.minecraft.core.Vec3i.getZ() line: 71` |
| 32 | 0.58% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 33 | 0.55% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3456` |
| 34 | 0.52% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 35 | 0.51% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 36 | 0.51% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 37 | 0.48% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |
| 38 | 0.47% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 39 | 0.47% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 40 | 0.46% | `java.util.EnumMap.isValidKey(Object) line: 319` |
| 41 | 0.45% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 42 | 0.45% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 43 | 0.45% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 44 | 0.44% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 134` |
| 45 | 0.42% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 46 | 0.41% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 47 | 0.41% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 48 | 0.41% | `java.util.ArrayDeque.size() line: 648` |
| 49 | 0.41% | `java.lang.ThreadLocal.get() line: 172` |
| 50 | 0.40% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 51 | 0.40% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 437` |
| 52 | 0.37% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 53 | 0.36% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 54 | 0.36% | `java.lang.Enum.ordinal() line: 128` |
| 55 | 0.35% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 56 | 0.35% | `net.minecraft.world.level.pathfinder.Path.getNextNodePos() line: 89` |
| 57 | 0.35% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 3849` |
| 58 | 0.33% | `net.minecraft.world.level.chunk.PalettedContainer.readPalette(PalettedContainer$Data, int) line: 96` |
| 59 | 0.32% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 60 | 0.32% | `java.util.EnumMap.unmaskNull(Object) line: 127` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 14.28% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 4.90% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 2.46% | `java.util.stream.AbstractPipeline.wrapAndCopyInto(Sink, Spliterator) line: 499` |
| 4 | 2.16% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 5 | 2.00% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 6 | 1.59% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 78` |
| 7 | 1.55% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 8 | 1.43% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 1.35% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 10 | 1.11% | `java.lang.ThreadLocal.get(Thread) line: 187` |
| 11 | 1.11% | `java.util.HashMap.containsKey(Object) line: 602` |
| 12 | 1.10% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 13 | 0.95% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 14 | 0.93% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 15 | 0.90% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 16 | 0.88% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 17 | 0.81% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2721` |
| 18 | 0.79% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 19 | 0.76% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 20 | 0.74% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 21 | 0.72% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 22 | 0.72% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 23 | 0.71% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 24 | 0.70% | `net.minecraft.world.entity.Entity.collide(Vec3) line: 1516` |
| 25 | 0.70% | `net.minecraft.world.entity.Mob.aiStep() line: 511` |
| 26 | 0.69% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |
| 27 | 0.68% | `net.minecraft.world.entity.monster.Zombie.aiStep() line: 243` |
| 28 | 0.67% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 29 | 0.65% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 30 | 0.63% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |

GC pauses: 0, total 0 ms

