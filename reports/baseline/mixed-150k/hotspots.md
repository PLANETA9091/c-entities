# JVM hotspots (JFR ExecutionSample)

Samples: 89935

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 28.01% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.04% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 2.94% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 4 | 2.64% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 2.31% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 6 | 1.89% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 7 | 1.83% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 8 | 1.82% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 9 | 1.39% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.38% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 11 | 1.02% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 12 | 1.00% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 13 | 0.90% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 14 | 0.89% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 15 | 0.89% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 16 | 0.82% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 17 | 0.81% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 18 | 0.78% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 19 | 0.73% | `java.util.EnumMap.get(Object) line: 248` |
| 20 | 0.73% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 21 | 0.68% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 22 | 0.64% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 23 | 0.61% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 24 | 0.61% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 25 | 0.60% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 26 | 0.48% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 27 | 0.42% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 28 | 0.42% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 29 | 0.41% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 30 | 0.39% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 31 | 0.38% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 32 | 0.38% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 33 | 0.37% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 92` |
| 34 | 0.35% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 35 | 0.34% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 36 | 0.33% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 37 | 0.32% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 38 | 0.32% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 39 | 0.31% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 40 | 0.30% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 41 | 0.30% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 42 | 0.30% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 43 | 0.27% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 44 | 0.27% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 45 | 0.27% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 46 | 0.27% | `java.util.HashMap.getNode(Object) line: 586` |
| 47 | 0.25% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 48 | 0.25% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 49 | 0.24% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 50 | 0.24% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 51 | 0.24% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 52 | 0.24% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 53 | 0.23% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |
| 54 | 0.23% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 55 | 0.23% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 56 | 0.22% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 57 | 0.22% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 58 | 0.21% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 59 | 0.21% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 60 | 0.20% | `java.util.EnumMap.isValidKey(Object) line: 320` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 28.01% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.86% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 5.79% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 4.03% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 2.31% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 6 | 1.94% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 7 | 1.47% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 8 | 1.38% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 9 | 1.13% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 10 | 1.00% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 11 | 0.95% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 12 | 0.83% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 13 | 0.80% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 14 | 0.77% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 15 | 0.76% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 16 | 0.65% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 17 | 0.64% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 18 | 0.64% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 19 | 0.60% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 20 | 0.53% | `java.util.ArrayList.toArray() line: 370` |
| 21 | 0.52% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 22 | 0.44% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 23 | 0.44% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 24 | 0.41% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 25 | 0.39% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 26 | 0.39% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 27 | 0.38% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 28 | 0.36% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 29 | 0.36% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 30 | 0.35% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |

GC pauses: 0, total 0 ms

