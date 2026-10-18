# JVM hotspots (JFR ExecutionSample)

Samples: 11034

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 13.82% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.94% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 2.62% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 4 | 2.01% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 5 | 1.98% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 6 | 1.49% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 7 | 1.48% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 1.44% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 9 | 1.34% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 10 | 1.33% | `java.util.HashMap.getNode(Object) line: 585` |
| 11 | 1.21% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 12 | 1.11% | `java.util.EnumMap.get(Object) line: 247` |
| 13 | 1.05% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 14 | 1.02% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 15 | 0.98% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 16 | 0.97% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 17 | 0.90% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 18 | 0.88% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 19 | 0.79% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 20 | 0.79% | `net.minecraft.world.item.ItemStack.getDamageValue() line: 597` |
| 21 | 0.78% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1313` |
| 22 | 0.76% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 23 | 0.70% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 24 | 0.69% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 25 | 0.68% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3456` |
| 26 | 0.68% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 27 | 0.65% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 28 | 0.63% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 29 | 0.63% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1422` |
| 30 | 0.59% | `java.lang.ThreadLocal.get() line: 172` |
| 31 | 0.56% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 32 | 0.56% | `net.minecraft.core.Vec3i.getZ() line: 71` |
| 33 | 0.54% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 34 | 0.53% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 35 | 0.51% | `java.util.EnumMap.isValidKey(Object) line: 319` |
| 36 | 0.49% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 37 | 0.48% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 320` |
| 38 | 0.47% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 47` |
| 39 | 0.46% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 40 | 0.45% | `java.util.ArrayDeque.size() line: 648` |
| 41 | 0.44% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 42 | 0.42% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 43 | 0.42% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 44 | 0.41% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 45 | 0.41% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 46 | 0.40% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 47 | 0.40% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 48 | 0.37% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 49 | 0.37% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |
| 50 | 0.35% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 437` |
| 51 | 0.35% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 70` |
| 52 | 0.34% | `net.minecraft.world.level.pathfinder.Path.isDone() line: 38` |
| 53 | 0.34% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 54 | 0.33% | `net.minecraft.util.SimpleBitStorage.get(int) line: 334` |
| 55 | 0.33% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 56 | 0.33% | `net.minecraft.world.entity.LivingEntity.doPush(Entity) line: 3738` |
| 57 | 0.32% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 58 | 0.30% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 59 | 0.30% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 147` |
| 60 | 0.29% | `java.lang.Enum.ordinal() line: 128` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 13.82% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 5.08% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 2.63% | `java.util.stream.AbstractPipeline.wrapAndCopyInto(Sink, Spliterator) line: 499` |
| 4 | 1.98% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 5 | 1.95% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 6 | 1.55% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 7 | 1.34% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 78` |
| 8 | 1.34% | `java.util.HashMap.containsKey(Object) line: 602` |
| 9 | 1.28% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 10 | 1.22% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 11 | 1.14% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 12 | 1.08% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 13 | 1.02% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 14 | 1.01% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 15 | 1.00% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 16 | 0.93% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 17 | 0.92% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 18 | 0.87% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2721` |
| 19 | 0.85% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 20 | 0.82% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 21 | 0.79% | `net.minecraft.world.entity.monster.Zombie.aiStep() line: 243` |
| 22 | 0.77% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 23 | 0.77% | `net.minecraft.world.entity.LivingEntity.tick() line: 3315` |
| 24 | 0.76% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 25 | 0.75% | `net.minecraft.world.entity.Entity.collide(Vec3) line: 1516` |
| 26 | 0.75% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges() line: 3348` |
| 27 | 0.75% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 28 | 0.74% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 29 | 0.73% | `java.util.EnumMap.get(Object) line: 247` |
| 30 | 0.72% | `net.minecraft.world.entity.Mob.aiStep() line: 511` |

GC pauses: 0, total 0 ms

