# JVM hotspots (JFR ExecutionSample)

Samples: 105582

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 34.23% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.26% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 2.81% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 4 | 2.44% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 1.88% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 6 | 1.76% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 7 | 1.55% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 1.41% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 9 | 1.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 10 | 1.24% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 11 | 1.02% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 12 | 0.99% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 0.91% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 14 | 0.80% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 15 | 0.77% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 16 | 0.76% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 17 | 0.75% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 18 | 0.70% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 19 | 0.65% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 20 | 0.63% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 21 | 0.58% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 22 | 0.54% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 23 | 0.53% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 24 | 0.52% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 25 | 0.48% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 26 | 0.48% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 27 | 0.47% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 28 | 0.47% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 29 | 0.43% | `java.util.EnumMap.get(Object) line: 248` |
| 30 | 0.40% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 31 | 0.38% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 32 | 0.36% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 92` |
| 33 | 0.33% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 34 | 0.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 35 | 0.31% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 36 | 0.30% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 37 | 0.30% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 38 | 0.29% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 39 | 0.27% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 40 | 0.27% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 41 | 0.26% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 42 | 0.26% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 43 | 0.26% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 44 | 0.25% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |
| 45 | 0.25% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 46 | 0.25% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 47 | 0.25% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 48 | 0.25% | `java.lang.Enum.ordinal() line: 128` |
| 49 | 0.25% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 50 | 0.24% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 51 | 0.24% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1484` |
| 52 | 0.24% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |
| 53 | 0.23% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 54 | 0.23% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 55 | 0.23% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 56 | 0.22% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 57 | 0.22% | `java.util.HashMap.getNode(Object) line: 586` |
| 58 | 0.22% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 59 | 0.22% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 60 | 0.21% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 34.23% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 5.99% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 5.46% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 2.54% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 2.44% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 6 | 1.88% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 7 | 1.57% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 8 | 1.16% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 1.02% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 10 | 0.83% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 11 | 0.80% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 12 | 0.80% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 13 | 0.77% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 14 | 0.75% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 15 | 0.64% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 16 | 0.60% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 17 | 0.59% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 18 | 0.56% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 19 | 0.50% | `java.util.ArrayList.toArray() line: 370` |
| 20 | 0.49% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 21 | 0.47% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 22 | 0.46% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 23 | 0.46% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 24 | 0.43% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 25 | 0.40% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3584` |
| 26 | 0.40% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 27 | 0.37% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1408` |
| 28 | 0.36% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 29 | 0.33% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 30 | 0.33% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |

GC pauses: 0, total 0 ms

