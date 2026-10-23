# JVM hotspots (JFR ExecutionSample)

Samples: 58688

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 11.45% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.21% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 3 | 2.89% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 2.50% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 5 | 2.05% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1376` |
| 6 | 1.97% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2182` |
| 7 | 1.71% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3636` |
| 8 | 1.41% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 9 | 1.40% | `java.util.HashMap.getNode(Object) line: 585` |
| 10 | 1.29% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.29% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 12 | 1.25% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 13 | 1.21% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 14 | 1.19% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1269` |
| 15 | 1.08% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1215` |
| 16 | 1.08% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 542` |
| 17 | 1.08% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 18 | 1.06% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 987` |
| 19 | 0.96% | `net.minecraft.world.entity.Entity.getSharedFlag(int) line: 3591` |
| 20 | 0.94% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 940` |
| 21 | 0.85% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 237` |
| 22 | 0.83% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1213` |
| 23 | 0.81% | `net.minecraft.core.component.PatchedDataComponentMap.get(DataComponentType) line: 61` |
| 24 | 0.77% | `net.minecraft.world.entity.LivingEntity.getSleepingPos() line: 4378` |
| 25 | 0.76% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(ResourceLocation) line: 68` |
| 26 | 0.74% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 27 | 0.74% | `it.unimi.dsi.fastutil.objects.ObjectOpenHashSet$SetIterator.<init>(ObjectOpenHashSet, ObjectOpenHashSet$1) line: 534` |
| 28 | 0.67% | `java.lang.Enum.ordinal() line: 128` |
| 29 | 0.67% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 204` |
| 30 | 0.65% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 31 | 0.64% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 32 | 0.64% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 208` |
| 33 | 0.64% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet.iterator() line: 1115` |
| 34 | 0.62% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 35 | 0.62% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 47` |
| 36 | 0.61% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4733` |
| 37 | 0.60% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 419` |
| 38 | 0.60% | `net.minecraft.util.Mth.floor(double) line: 67` |
| 39 | 0.59% | `net.minecraft.world.entity.Entity.isNoGravity() line: 1830` |
| 40 | 0.56% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 41 | 0.52% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.put(Object, double) line: 485` |
| 42 | 0.52% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 43 | 0.50% | `java.util.EnumMap.unmaskNull(Object) line: 127` |
| 44 | 0.49% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4387` |
| 45 | 0.49% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 81` |
| 46 | 0.46% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 47 | 0.43% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges() line: 3348` |
| 48 | 0.40% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 60` |
| 49 | 0.40% | `net.minecraft.util.Mth.floor(double) line: 66` |
| 50 | 0.39% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 51 | 0.39% | `java.util.EnumMap.get(Object) line: 247` |
| 52 | 0.39% | `java.lang.ThreadLocal.get() line: 172` |
| 53 | 0.38% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 54 | 0.37% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 55 | 0.36% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 196` |
| 56 | 0.35% | `java.util.Arrays.copyOf(Object[], int) line: 3482` |
| 57 | 0.33% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3690` |
| 58 | 0.33% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 275` |
| 59 | 0.33% | `io.netty.channel.ChannelOutboundBuffer.addFlush() line: 159` |
| 60 | 0.33% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getBlock() line: 641` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 11.45% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 4.63% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 3 | 2.89% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 4 | 2.41% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 5 | 2.37% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 2.08% | `net.minecraft.world.entity.Entity.setOldPos() line: 2174` |
| 7 | 2.05% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2152` |
| 8 | 1.96% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1161` |
| 9 | 1.91% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 10 | 1.74% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 11 | 1.71% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3553` |
| 12 | 1.69% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 545` |
| 13 | 1.42% | `net.minecraft.world.entity.Entity.updateInWaterStateAndDoWaterCurrentPushing() line: 1970` |
| 14 | 1.40% | `java.util.HashMap.containsKey(Object) line: 602` |
| 15 | 1.33% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4750` |
| 16 | 1.25% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 17 | 1.12% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 18 | 0.96% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 513` |
| 19 | 0.89% | `net.minecraft.world.entity.Entity.isSprinting() line: 3501` |
| 20 | 0.85% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 21 | 0.81% | `net.minecraft.core.component.DataComponentHolder.get(DataComponentType) line: 12` |
| 22 | 0.78% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3542` |
| 23 | 0.78% | `net.minecraft.world.entity.LivingEntity.isSleeping() line: 4390` |
| 24 | 0.77% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4724` |
| 25 | 0.76% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 533` |
| 26 | 0.75% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 256` |
| 27 | 0.74% | `java.util.EnumMap.get(Object) line: 247` |
| 28 | 0.74% | `it.unimi.dsi.fastutil.objects.ObjectOpenHashSet.iterator() line: 654` |
| 29 | 0.66% | `net.minecraft.world.entity.Entity.updateFluidHeightAndDoFluidPushing(TagKey, double) line: 4788` |
| 30 | 0.65% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 279` |

GC pauses: 0, total 0 ms

