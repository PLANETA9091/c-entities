# JVM hotspots (JFR ExecutionSample)

Samples: 14153

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 10.80% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.87% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 2.09% | `java.util.HashMap.getNode(Object) line: 585` |
| 4 | 1.33% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 5 | 1.27% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 1.24% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 7 | 1.20% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 8 | 1.09% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 61` |
| 9 | 1.09% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3599` |
| 10 | 0.95% | `io.netty.util.internal.shaded.org.jctools.util.UnsafeRefArrayAccess.soRefElement(Object[], long, Object) line: 63` |
| 11 | 0.88% | `java.util.EnumMap.get(Object) line: 247` |
| 12 | 0.87% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 13 | 0.81% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processPendingFullUpdate() line: 1501` |
| 14 | 0.79% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 15 | 0.78% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 16 | 0.78% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 17 | 0.75% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 18 | 0.74% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 19 | 0.74% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 20 | 0.73% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 21 | 0.66% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 201` |
| 22 | 0.64% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1414` |
| 23 | 0.60% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 24 | 0.60% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 25 | 0.55% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 26 | 0.52% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(ResourceLocation) line: 68` |
| 27 | 0.52% | `java.lang.Enum.ordinal() line: 128` |
| 28 | 0.52% | `net.minecraft.world.item.ItemStack.getDamageValue() line: 597` |
| 29 | 0.50% | `net.minecraft.world.entity.InterpolationHandler.hasActiveInterpolation() line: 69` |
| 30 | 0.49% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 31 | 0.49% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 32 | 0.48% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 33 | 0.48% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 34 | 0.48% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 35 | 0.48% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 36 | 0.48% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 37 | 0.47% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 38 | 0.46% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |
| 39 | 0.45% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 437` |
| 40 | 0.45% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1313` |
| 41 | 0.44% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 42 | 0.43% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3459` |
| 43 | 0.43% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 44 | 0.42% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 45 | 0.40% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 46 | 0.40% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 47 | 0.40% | `net.minecraft.core.Vec3i.getZ() line: 71` |
| 48 | 0.37% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 49 | 0.37% | `io.netty.channel.ChannelOutboundBuffer.addFlush() line: 159` |
| 50 | 0.37% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 51 | 0.36% | `ca.spottedleaf.concurrentutil.util.HashUtil.mix(long) line: 92` |
| 52 | 0.36% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 53 | 0.36% | `java.util.ArrayDeque.size() line: 648` |
| 54 | 0.35% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 47` |
| 55 | 0.35% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 56 | 0.31% | `java.util.EnumMap.isValidKey(Object) line: 319` |
| 57 | 0.31% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 134` |
| 58 | 0.31% | `io.netty.channel.ChannelOutboundBuffer.removeBytes(long) line: 369` |
| 59 | 0.31% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |
| 60 | 0.30% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 10.80% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 4.50% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 2.10% | `java.util.HashMap.containsKey(Object) line: 602` |
| 4 | 1.72% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 5 | 1.69% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 6 | 1.09% | `net.minecraft.world.entity.Entity.setSwimming(boolean) line: 3528` |
| 7 | 1.07% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 12` |
| 8 | 1.04% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 0.95% | `io.netty.util.internal.shaded.org.jctools.queues.BaseMpscLinkedArrayQueue.offer(Object) line: 280` |
| 10 | 0.91% | `net.minecraft.world.entity.Mob.serverAiStep() line: 759` |
| 11 | 0.89% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 12 | 0.87% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 78` |
| 13 | 0.82% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 14 | 0.81% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1462` |
| 15 | 0.78% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 16 | 0.77% | `java.util.EnumMap.get(Object) line: 247` |
| 17 | 0.76% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 18 | 0.73% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 285` |
| 19 | 0.71% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 20 | 0.71% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |
| 21 | 0.69% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 22 | 0.69% | `net.minecraft.world.entity.Entity.getBlockPosBelowThatAffectsMyMovement() line: 1406` |
| 23 | 0.67% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 24 | 0.67% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 25 | 0.67% | `net.minecraft.world.entity.monster.Zombie.aiStep() line: 243` |
| 26 | 0.66% | `net.minecraft.world.entity.Entity.collide(Vec3) line: 1516` |
| 27 | 0.64% | `net.minecraft.world.entity.Mob.aiStep() line: 511` |
| 28 | 0.64% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 29 | 0.63% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2721` |
| 30 | 0.61% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |

GC pauses: 0, total 0 ms

