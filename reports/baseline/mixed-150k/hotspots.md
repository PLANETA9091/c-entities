# JVM hotspots (JFR ExecutionSample)

Samples: 109158

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 40.41% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.13% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 3 | 2.78% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 1.92% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 5 | 1.88% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 1.84% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 1.25% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 8 | 1.03% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 0.99% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 10 | 0.82% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 11 | 0.76% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 12 | 0.65% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 13 | 0.64% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 14 | 0.62% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 15 | 0.61% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 16 | 0.61% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 17 | 0.59% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 18 | 0.58% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 19 | 0.53% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 20 | 0.53% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 21 | 0.52% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 22 | 0.51% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 23 | 0.51% | `java.util.EnumMap.get(Object) line: 248` |
| 24 | 0.47% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 25 | 0.47% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 26 | 0.42% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 27 | 0.41% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 28 | 0.41% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3579` |
| 29 | 0.40% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 30 | 0.40% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 31 | 0.39% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 32 | 0.39% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 33 | 0.37% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3696` |
| 34 | 0.36% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 35 | 0.35% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 36 | 0.35% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 37 | 0.35% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 38 | 0.33% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 39 | 0.31% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 40 | 0.31% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 41 | 0.31% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 42 | 0.30% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 43 | 0.28% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 44 | 0.26% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 45 | 0.26% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 46 | 0.26% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 47 | 0.25% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 48 | 0.25% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 49 | 0.24% | `net.minecraft.world.entity.Entity.baseTick() line: 914` |
| 50 | 0.24% | `java.util.Optional.ifPresent(Consumer) line: 178` |
| 51 | 0.24% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 52 | 0.23% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 53 | 0.21% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 54 | 0.21% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 55 | 0.20% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 56 | 0.20% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 57 | 0.19% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 58 | 0.19% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 59 | 0.19% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 60 | 0.18% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 92` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 40.41% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 5.89% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 4.89% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 2.86% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 2.78% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 6 | 1.30% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 7 | 1.22% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 8 | 0.87% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 9 | 0.81% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 10 | 0.65% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 11 | 0.62% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 12 | 0.61% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 13 | 0.59% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 14 | 0.51% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 15 | 0.50% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 16 | 0.48% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 17 | 0.47% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 18 | 0.45% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 19 | 0.41% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 20 | 0.41% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3685` |
| 21 | 0.41% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 22 | 0.40% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 23 | 0.40% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 24 | 0.40% | `net.minecraft.world.entity.LivingEntity.tick() line: 3432` |
| 25 | 0.40% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 26 | 0.40% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 27 | 0.39% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 28 | 0.37% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 29 | 0.37% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 30 | 0.33% | `java.util.Collections$UnmodifiableCollection.iterator() line: 1076` |

GC pauses: 0, total 0 ms

