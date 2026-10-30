# JVM hotspots (JFR ExecutionSample)

Samples: 74090

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 16.76% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.53% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 3.00% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 4 | 2.56% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 5 | 2.35% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 2.02% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 7 | 1.59% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 8 | 1.57% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 9 | 1.53% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 10 | 1.52% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.48% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 12 | 1.43% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 13 | 1.39% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 14 | 1.28% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 15 | 1.27% | `java.util.concurrent.CompletableFuture.isDone() line: 2076` |
| 16 | 1.19% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 17 | 1.17% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 18 | 1.12% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 19 | 1.10% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 20 | 0.95% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 21 | 0.91% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 22 | 0.87% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 23 | 0.87% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 24 | 0.85% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 25 | 0.85% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 26 | 0.73% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 27 | 0.71% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 28 | 0.71% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 29 | 0.69% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 30 | 0.68% | `java.util.EnumMap.get(Object) line: 248` |
| 31 | 0.67% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 32 | 0.62% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 33 | 0.53% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 34 | 0.52% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 35 | 0.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 36 | 0.51% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 37 | 0.50% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 38 | 0.50% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 39 | 0.50% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 40 | 0.47% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 41 | 0.46% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 323` |
| 42 | 0.46% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 43 | 0.44% | `java.util.HashMap.getNode(Object) line: 586` |
| 44 | 0.41% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 45 | 0.40% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 92` |
| 46 | 0.39% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 47 | 0.39% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 48 | 0.39% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 49 | 0.39% | `net.minecraft.world.entity.LivingEntity.updateGlowingStatus() line: 1025` |
| 50 | 0.38% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 51 | 0.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 52 | 0.34% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 53 | 0.34% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 320` |
| 54 | 0.33% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 55 | 0.31% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 56 | 0.30% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 57 | 0.28% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 311` |
| 58 | 0.28% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 59 | 0.27% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 60 | 0.27% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 16.76% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 4.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 3.87% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 3.66% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 5 | 3.53% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 6 | 2.03% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 7 | 2.02% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 8 | 2.02% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 9 | 1.63% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 10 | 1.59% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 11 | 1.39% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 12 | 1.38% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 13 | 1.28% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 14 | 1.27% | `net.minecraft.util.thread.BlockableEventLoop.managedBlock(BooleanSupplier) line: 190` |
| 15 | 1.25% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 16 | 1.11% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 17 | 1.10% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 18 | 1.00% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 19 | 0.97% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 20 | 0.95% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |
| 21 | 0.88% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 22 | 0.75% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 23 | 0.71% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 24 | 0.70% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 25 | 0.68% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 26 | 0.68% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 27 | 0.67% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 28 | 0.64% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 29 | 0.62% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 30 | 0.58% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |

GC pauses: 0, total 0 ms

