# JVM hotspots (JFR ExecutionSample)

Samples: 17504

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 19.94% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.23% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 1.98% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 4 | 1.31% | `java.util.HashMap.getNode(Object) line: 585` |
| 5 | 1.15% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 6 | 1.12% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 7 | 1.05% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 61` |
| 8 | 1.04% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 0.97% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 10 | 0.91% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 11 | 0.91% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 12 | 0.81% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 13 | 0.80% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 14 | 0.75% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 15 | 0.70% | `java.util.EnumMap.get(Object) line: 247` |
| 16 | 0.66% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 17 | 0.62% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 18 | 0.59% | `net.minecraft.core.Vec3i.getZ() line: 71` |
| 19 | 0.59% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 20 | 0.59% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 21 | 0.57% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 22 | 0.56% | `java.util.stream.AbstractPipeline.copyInto(Sink, Spliterator) line: 509` |
| 23 | 0.56% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 24 | 0.55% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 25 | 0.55% | `java.lang.Enum.ordinal() line: 128` |
| 26 | 0.54% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 27 | 0.49% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 28 | 0.48% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3459` |
| 29 | 0.45% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 30 | 0.44% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 31 | 0.44% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 32 | 0.43% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 33 | 0.43% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 34 | 0.43% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1414` |
| 35 | 0.43% | `io.netty.channel.ChannelOutboundBuffer.addFlush() line: 159` |
| 36 | 0.42% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 437` |
| 37 | 0.42% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 38 | 0.42% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(ResourceLocation) line: 68` |
| 39 | 0.41% | `java.lang.ThreadLocal.get() line: 172` |
| 40 | 0.39% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 41 | 0.38% | `java.util.ArrayDeque.size() line: 648` |
| 42 | 0.38% | `net.minecraft.world.item.ItemStack.getDamageValue() line: 597` |
| 43 | 0.38% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 44 | 0.37% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 45 | 0.37% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 46 | 0.33% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 47 | 0.33% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 48 | 0.33% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 49 | 0.33% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 50 | 0.32% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 47` |
| 51 | 0.31% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 52 | 0.31% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 134` |
| 53 | 0.31% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 175` |
| 54 | 0.30% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 3849` |
| 55 | 0.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 56 | 0.28% | `net.minecraft.world.entity.InterpolationHandler.hasActiveInterpolation() line: 69` |
| 57 | 0.27% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.size() line: 422` |
| 58 | 0.26% | `net.minecraft.core.BlockPos$4.<init>(int, int, int, int, int, int) line: 460` |
| 59 | 0.26% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1429` |
| 60 | 0.25% | `java.lang.ThreadLocal$ThreadLocalMap.getEntry(ThreadLocal) line: 499` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 19.94% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 4.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 2.03% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 4 | 1.46% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 5 | 1.33% | `java.util.HashMap.containsKey(Object) line: 602` |
| 6 | 1.09% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 12` |
| 7 | 1.00% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 8 | 0.92% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 9 | 0.91% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 78` |
| 10 | 0.91% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 11 | 0.87% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 12 | 0.81% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 13 | 0.77% | `net.minecraft.world.entity.Mob.serverAiStep() line: 759` |
| 14 | 0.75% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 15 | 0.74% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 16 | 0.74% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 17 | 0.67% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 18 | 0.67% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 19 | 0.66% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 20 | 0.64% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.is(TagKey) line: 961` |
| 21 | 0.63% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2721` |
| 22 | 0.63% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 23 | 0.61% | `net.minecraft.world.entity.Mob.aiStep() line: 511` |
| 24 | 0.59% | `java.lang.ThreadLocal.get(Thread) line: 187` |
| 25 | 0.56% | `java.util.stream.AbstractPipeline.wrapAndCopyInto(Sink, Spliterator) line: 499` |
| 26 | 0.53% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 27 | 0.52% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 28 | 0.51% | `net.minecraft.world.entity.Entity.getBlockZ() line: 4931` |
| 29 | 0.50% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 30 | 0.50% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |

GC pauses: 0, total 0 ms

