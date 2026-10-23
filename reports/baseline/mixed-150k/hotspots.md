# JVM hotspots (JFR ExecutionSample)

Samples: 72545

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 23.82% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.31% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4706` |
| 3 | 4.05% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 3.77% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 5 | 3.43% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 2.85% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3907` |
| 7 | 2.44% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 8 | 2.09% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.49% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 10 | 1.18% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2403` |
| 11 | 1.18% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 12 | 1.18% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 13 | 1.17% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 14 | 1.12% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 15 | 1.11% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 16 | 0.94% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 17 | 0.93% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 18 | 0.80% | `java.util.EnumMap.get(Object) line: 248` |
| 19 | 0.78% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 20 | 0.77% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3868` |
| 21 | 0.73% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 22 | 0.73% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 23 | 0.60% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 24 | 0.54% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 25 | 0.50% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 26 | 0.49% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 27 | 0.49% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 28 | 0.49% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 29 | 0.49% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 30 | 0.47% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 31 | 0.46% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 32 | 0.46% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 33 | 0.44% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 34 | 0.42% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 35 | 0.41% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 36 | 0.41% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 37 | 0.39% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 38 | 0.35% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 39 | 0.33% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 92` |
| 40 | 0.33% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 41 | 0.33% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1574` |
| 42 | 0.32% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 43 | 0.30% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 44 | 0.29% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 45 | 0.28% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1866` |
| 46 | 0.28% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 47 | 0.25% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 48 | 0.25% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 49 | 0.25% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 50 | 0.25% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |
| 51 | 0.23% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 52 | 0.23% | `net.minecraft.world.entity.Entity.level() line: 5415` |
| 53 | 0.23% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 54 | 0.22% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 55 | 0.21% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 56 | 0.21% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 57 | 0.21% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 58 | 0.21% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.grow(int) line: 353` |
| 59 | 0.21% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 60 | 0.21% | `java.util.HashMap.getNode(Object) line: 586` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 23.82% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 8.26% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 5.52% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 5.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 5 | 2.85% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 6 | 2.44% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 7 | 2.20% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 8 | 1.97% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 1.39% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 10 | 1.24% | `net.minecraft.world.entity.Entity.setOldPos() line: 2395` |
| 11 | 1.18% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 12 | 1.11% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 13 | 0.93% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 14 | 0.85% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 15 | 0.80% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2212` |
| 16 | 0.80% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 17 | 0.73% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 969` |
| 18 | 0.72% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 19 | 0.60% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 20 | 0.60% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 21 | 0.59% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1475` |
| 22 | 0.55% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 23 | 0.49% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 24 | 0.49% | `java.util.ArrayList.toArray() line: 370` |
| 25 | 0.46% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |
| 26 | 0.46% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 848` |
| 27 | 0.45% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 28 | 0.44% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 29 | 0.42% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 30 | 0.41% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |

GC pauses: 0, total 0 ms

