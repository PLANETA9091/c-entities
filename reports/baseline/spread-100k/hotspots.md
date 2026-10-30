# JVM hotspots (JFR ExecutionSample)

Samples: 86986

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 20.72% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.80% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 2.11% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 4 | 2.07% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 5 | 2.06% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 1.80% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 1.66% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 8 | 1.43% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 9 | 1.33% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 10 | 1.32% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 11 | 1.27% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 12 | 1.16% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 13 | 1.13% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 14 | 1.07% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 15 | 1.00% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 16 | 1.00% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 17 | 0.95% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 18 | 0.94% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 19 | 0.89% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 20 | 0.89% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 21 | 0.84% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 22 | 0.78% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 23 | 0.77% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 24 | 0.76% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 25 | 0.66% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 26 | 0.64% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 27 | 0.64% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 28 | 0.59% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 29 | 0.57% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 30 | 0.54% | `java.util.EnumMap.get(Object) line: 248` |
| 31 | 0.52% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 32 | 0.50% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 33 | 0.48% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 34 | 0.47% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 35 | 0.44% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 36 | 0.43% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 37 | 0.43% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 38 | 0.43% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 39 | 0.42% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 323` |
| 40 | 0.41% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 41 | 0.40% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 92` |
| 42 | 0.39% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 43 | 0.38% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 44 | 0.38% | `java.util.HashMap.getNode(Object) line: 586` |
| 45 | 0.35% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 46 | 0.35% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 47 | 0.35% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 48 | 0.33% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 49 | 0.31% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 50 | 0.31% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 51 | 0.30% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 52 | 0.30% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 53 | 0.30% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 54 | 0.29% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 55 | 0.28% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 56 | 0.27% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 57 | 0.27% | `java.lang.Enum.ordinal() line: 128` |
| 58 | 0.27% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 59 | 0.27% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1484` |
| 60 | 0.27% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 20.72% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 3.36% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 3.10% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 3.05% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 2.80% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 6 | 2.15% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 7 | 2.07% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 8 | 1.84% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 9 | 1.52% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 10 | 1.33% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 11 | 1.32% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 12 | 1.20% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 13 | 1.03% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 14 | 1.02% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 125` |
| 15 | 0.99% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 16 | 0.94% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 17 | 0.87% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 18 | 0.85% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 19 | 0.78% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 20 | 0.76% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |
| 21 | 0.70% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 22 | 0.69% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 23 | 0.67% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 24 | 0.60% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 25 | 0.59% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |
| 26 | 0.57% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 27 | 0.55% | `java.util.ArrayList.toArray() line: 370` |
| 28 | 0.55% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |
| 29 | 0.52% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 30 | 0.50% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |

GC pauses: 0, total 0 ms

