# JVM hotspots (JFR ExecutionSample)

Samples: 89646

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 28.56% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.42% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 3 | 3.63% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 2.82% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 2.59% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 2.19% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 7 | 1.58% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 8 | 1.48% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.22% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 10 | 1.20% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 11 | 1.19% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 12 | 1.19% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 13 | 1.16% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 14 | 1.15% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 15 | 0.96% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 16 | 0.81% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 17 | 0.77% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 18 | 0.74% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 19 | 0.70% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 20 | 0.69% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 21 | 0.62% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 22 | 0.60% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 23 | 0.59% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 24 | 0.55% | `java.util.EnumMap.get(Object) line: 248` |
| 25 | 0.55% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 26 | 0.54% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 27 | 0.51% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 28 | 0.50% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 29 | 0.46% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 30 | 0.45% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 31 | 0.43% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 32 | 0.42% | `net.minecraft.world.item.ItemStack.getMaxDamage() line: 582` |
| 33 | 0.41% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 34 | 0.39% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 35 | 0.39% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 36 | 0.39% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 37 | 0.38% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 38 | 0.38% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 39 | 0.37% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 40 | 0.36% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 243` |
| 41 | 0.35% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 42 | 0.33% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 43 | 0.32% | `java.util.HashMap.getNode(Object) line: 586` |
| 44 | 0.31% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 45 | 0.30% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 46 | 0.30% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 47 | 0.30% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 48 | 0.29% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 49 | 0.29% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 50 | 0.28% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 51 | 0.26% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 52 | 0.26% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 53 | 0.25% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 54 | 0.24% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 55 | 0.24% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 56 | 0.23% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 57 | 0.23% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 58 | 0.22% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 59 | 0.22% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1484` |
| 60 | 0.22% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 28.56% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 7.91% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 4.13% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 4.08% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 2.82% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 6 | 2.19% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3803` |
| 7 | 1.97% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 8 | 1.64% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 9 | 1.60% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 10 | 1.24% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 11 | 1.22% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 12 | 0.79% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 13 | 0.74% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 14 | 0.71% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 15 | 0.70% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 16 | 0.69% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 17 | 0.69% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 18 | 0.62% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 19 | 0.59% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 20 | 0.58% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 21 | 0.56% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 22 | 0.54% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 23 | 0.50% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 24 | 0.48% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 25 | 0.45% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2817` |
| 26 | 0.45% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 27 | 0.42% | `net.minecraft.world.item.ItemStack.isBroken() line: 586` |
| 28 | 0.41% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 29 | 0.41% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 30 | 0.40% | `java.util.ArrayList.toArray() line: 370` |

GC pauses: 0, total 0 ms

