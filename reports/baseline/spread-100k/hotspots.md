# JVM hotspots (JFR ExecutionSample)

Samples: 86798

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 20.80% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.44% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 2.10% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 4 | 1.97% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 5 | 1.91% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 1.84% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 1.63% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 8 | 1.52% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 9 | 1.32% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 10 | 1.25% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 11 | 1.21% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.11% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 13 | 1.11% | `java.util.concurrent.CompletableFuture.isDone() line: 2076` |
| 14 | 1.10% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 15 | 1.05% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 16 | 0.99% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 17 | 0.96% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 18 | 0.94% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 19 | 0.93% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 20 | 0.86% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 21 | 0.78% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 22 | 0.77% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 23 | 0.74% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 24 | 0.63% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 25 | 0.60% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 26 | 0.59% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 27 | 0.57% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 28 | 0.57% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 29 | 0.57% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 30 | 0.52% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 31 | 0.52% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 32 | 0.50% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 33 | 0.46% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 34 | 0.46% | `java.util.HashMap.getNode(Object) line: 586` |
| 35 | 0.45% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 36 | 0.45% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 37 | 0.41% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 38 | 0.41% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 39 | 0.41% | `java.util.EnumMap.get(Object) line: 248` |
| 40 | 0.40% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 92` |
| 41 | 0.39% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 42 | 0.38% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 43 | 0.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 44 | 0.37% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 45 | 0.37% | `java.lang.Enum.ordinal() line: 128` |
| 46 | 0.33% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 47 | 0.33% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 48 | 0.33% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 49 | 0.33% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 50 | 0.33% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 51 | 0.31% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 848` |
| 52 | 0.31% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 53 | 0.30% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 54 | 0.29% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 55 | 0.29% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 323` |
| 56 | 0.27% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |
| 57 | 0.26% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 58 | 0.26% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 59 | 0.25% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 60 | 0.25% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 20.80% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 3.37% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 3.12% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 3.11% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 2.44% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 6 | 2.15% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 7 | 1.79% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 8 | 1.53% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 9 | 1.36% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 10 | 1.32% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 11 | 1.15% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 12 | 1.11% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 13 | 1.11% | `net.minecraft.util.thread.BlockableEventLoop.managedBlock(BooleanSupplier) line: 190` |
| 14 | 0.99% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 15 | 0.98% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 16 | 0.97% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 17 | 0.96% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 18 | 0.93% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 19 | 0.89% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 20 | 0.86% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 21 | 0.74% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |
| 22 | 0.69% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 23 | 0.65% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 24 | 0.64% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 25 | 0.59% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |
| 26 | 0.57% | `java.util.ArrayList.toArray() line: 370` |
| 27 | 0.56% | `net.minecraft.world.level.chunk.PalettedContainer.readPalette(PalettedContainer$Data, int) line: 79` |
| 28 | 0.56% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 29 | 0.56% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 30 | 0.54% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |

GC pauses: 0, total 0 ms

