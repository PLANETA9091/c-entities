# JVM hotspots (JFR ExecutionSample)

Samples: 105632

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 31.63% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.08% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 2.76% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 4 | 2.19% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 1.85% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 1.43% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 7 | 1.43% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 8 | 1.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 9 | 1.26% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 10 | 1.18% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.10% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 12 | 1.08% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 13 | 0.86% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 14 | 0.86% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 15 | 0.77% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 16 | 0.73% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 17 | 0.67% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 18 | 0.67% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 19 | 0.61% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 20 | 0.57% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 21 | 0.53% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 22 | 0.52% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 23 | 0.52% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 24 | 0.51% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 25 | 0.48% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 26 | 0.47% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 27 | 0.47% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 28 | 0.46% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 29 | 0.42% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 30 | 0.41% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 31 | 0.40% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 32 | 0.40% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 33 | 0.37% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 34 | 0.35% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 35 | 0.35% | `java.util.HashMap.getNode(Object) line: 586` |
| 36 | 0.35% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 37 | 0.34% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 38 | 0.34% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 39 | 0.34% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 40 | 0.33% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 41 | 0.32% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 42 | 0.32% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 43 | 0.30% | `java.util.EnumMap.get(Object) line: 248` |
| 44 | 0.30% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 45 | 0.29% | `net.minecraft.server.level.ServerChunkCache.pollTask() line: 449` |
| 46 | 0.29% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 47 | 0.28% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 48 | 0.27% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 49 | 0.26% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1484` |
| 50 | 0.25% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |
| 51 | 0.25% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 52 | 0.24% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 53 | 0.24% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 54 | 0.24% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 55 | 0.22% | `java.lang.Enum.ordinal() line: 128` |
| 56 | 0.22% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 57 | 0.22% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 58 | 0.22% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 59 | 0.22% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 60 | 0.21% | `net.minecraft.world.entity.Entity.level() line: 5421` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 31.63% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 5.71% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 4.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 3.03% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 2.19% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 6 | 1.66% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 7 | 1.44% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3803` |
| 8 | 1.29% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 9 | 1.17% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 10 | 0.93% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 11 | 0.81% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 12 | 0.76% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 13 | 0.70% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 14 | 0.69% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 15 | 0.69% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 16 | 0.63% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 17 | 0.56% | `java.util.ArrayList.toArray() line: 370` |
| 18 | 0.50% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 19 | 0.48% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 20 | 0.47% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 21 | 0.47% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 22 | 0.47% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 23 | 0.46% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 24 | 0.46% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 25 | 0.43% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 26 | 0.43% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 27 | 0.42% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 28 | 0.40% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 29 | 0.37% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 30 | 0.37% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |

GC pauses: 0, total 0 ms

