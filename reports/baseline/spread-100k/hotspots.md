# JVM hotspots (JFR ExecutionSample)

Samples: 87385

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 20.80% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.91% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 3 | 2.77% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 2.33% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 5 | 2.16% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 1.87% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 1.69% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 8 | 1.41% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 9 | 1.35% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 10 | 1.31% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 11 | 1.30% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 12 | 1.24% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 1.18% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 14 | 1.17% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 15 | 0.97% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 16 | 0.96% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 17 | 0.96% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 18 | 0.92% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 19 | 0.90% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 20 | 0.86% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 21 | 0.80% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 22 | 0.80% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 23 | 0.79% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 24 | 0.67% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 25 | 0.62% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 26 | 0.58% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 27 | 0.57% | `java.util.EnumMap.get(Object) line: 248` |
| 28 | 0.56% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 29 | 0.55% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 30 | 0.54% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 31 | 0.52% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 32 | 0.50% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 33 | 0.45% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 34 | 0.43% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 323` |
| 35 | 0.41% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 36 | 0.40% | `java.util.HashMap.getNode(Object) line: 586` |
| 37 | 0.40% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 38 | 0.40% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 39 | 0.40% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 40 | 0.39% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 41 | 0.39% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 92` |
| 42 | 0.38% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 43 | 0.37% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 44 | 0.36% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1484` |
| 45 | 0.34% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 46 | 0.32% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |
| 47 | 0.32% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 48 | 0.32% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 49 | 0.31% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 50 | 0.31% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 51 | 0.31% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 52 | 0.30% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 53 | 0.30% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 54 | 0.29% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 55 | 0.29% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 56 | 0.27% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 57 | 0.26% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 58 | 0.26% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 59 | 0.26% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 60 | 0.26% | `net.minecraft.core.Vec3i.getX() line: 63` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 20.80% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 3.44% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 3.39% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 2.94% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 5 | 2.91% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3803` |
| 6 | 2.77% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 7 | 2.36% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 8 | 1.82% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 9 | 1.38% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 10 | 1.35% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 11 | 1.31% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 12 | 1.27% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 13 | 1.23% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 14 | 0.99% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 15 | 0.91% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 16 | 0.91% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 17 | 0.90% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 18 | 0.89% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 19 | 0.88% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 20 | 0.80% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |
| 21 | 0.68% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 22 | 0.62% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 23 | 0.62% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 24 | 0.61% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 25 | 0.59% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 26 | 0.59% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |
| 27 | 0.58% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 28 | 0.53% | `java.util.ArrayList.toArray() line: 370` |
| 29 | 0.49% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |
| 30 | 0.47% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |

GC pauses: 0, total 0 ms

