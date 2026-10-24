# JVM hotspots (JFR ExecutionSample)

Samples: 73043

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 22.76% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.76% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 2.58% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4706` |
| 4 | 2.33% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3907` |
| 5 | 1.94% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 6 | 1.80% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 7 | 1.36% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 1.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 9 | 1.12% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 10 | 1.07% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 11 | 1.05% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 12 | 1.05% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 13 | 0.97% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2403` |
| 14 | 0.89% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 15 | 0.87% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 16 | 0.82% | `java.util.EnumMap.get(Object) line: 248` |
| 17 | 0.82% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 18 | 0.80% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 19 | 0.76% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 20 | 0.75% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 21 | 0.74% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 22 | 0.67% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 23 | 0.61% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 24 | 0.61% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 25 | 0.60% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3868` |
| 26 | 0.60% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 27 | 0.59% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 28 | 0.58% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 29 | 0.54% | `java.util.ArrayDeque.pollFirst() line: 378` |
| 30 | 0.52% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 31 | 0.50% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 32 | 0.48% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 33 | 0.45% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 34 | 0.44% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 35 | 0.42% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 36 | 0.42% | `java.util.HashMap.getNode(Object) line: 586` |
| 37 | 0.41% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 38 | 0.39% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 39 | 0.37% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 40 | 0.35% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 41 | 0.34% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1574` |
| 42 | 0.34% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 43 | 0.34% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 44 | 0.33% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 45 | 0.33% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 46 | 0.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 323` |
| 47 | 0.31% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 48 | 0.31% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 49 | 0.30% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 50 | 0.30% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 51 | 0.30% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 92` |
| 52 | 0.29% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 53 | 0.29% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 311` |
| 54 | 0.28% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 55 | 0.28% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 56 | 0.27% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 57 | 0.26% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |
| 58 | 0.26% | `net.minecraft.world.entity.Entity.level() line: 5415` |
| 59 | 0.26% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 60 | 0.25% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 22.76% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 4.11% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 3.64% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 3.27% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 5 | 2.33% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 6 | 2.09% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 7 | 1.94% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 8 | 1.53% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 1.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 10 | 1.16% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 11 | 1.05% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 12 | 1.01% | `net.minecraft.world.entity.Entity.setOldPos() line: 2395` |
| 13 | 0.87% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 14 | 0.67% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2212` |
| 15 | 0.63% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 16 | 0.62% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 17 | 0.61% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 18 | 0.61% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 19 | 0.60% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 20 | 0.59% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 21 | 0.57% | `java.util.ArrayList.toArray() line: 370` |
| 22 | 0.57% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 23 | 0.55% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 969` |
| 24 | 0.55% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 25 | 0.54% | `java.util.ArrayDeque.removeFirst() line: 358` |
| 26 | 0.52% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 27 | 0.52% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1475` |
| 28 | 0.50% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 29 | 0.49% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 30 | 0.48% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |

GC pauses: 0, total 0 ms

