# JVM hotspots (JFR ExecutionSample)

Samples: 89972

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 30.42% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.28% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 3 | 3.21% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 3.14% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 2.83% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 6 | 2.48% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 1.92% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 8 | 1.43% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 9 | 1.39% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.29% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 11 | 1.23% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 12 | 1.23% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 13 | 1.13% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 14 | 1.02% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 15 | 0.99% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 16 | 0.82% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 17 | 0.82% | `java.util.EnumMap.get(Object) line: 248` |
| 18 | 0.78% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 19 | 0.75% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 20 | 0.66% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 21 | 0.53% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 22 | 0.52% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 23 | 0.49% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 24 | 0.49% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 25 | 0.45% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 26 | 0.44% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 27 | 0.43% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 28 | 0.43% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 29 | 0.43% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 30 | 0.42% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 31 | 0.41% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 92` |
| 32 | 0.36% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 33 | 0.36% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 34 | 0.35% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 35 | 0.32% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 36 | 0.32% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 37 | 0.32% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 38 | 0.31% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 39 | 0.31% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 40 | 0.30% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 41 | 0.27% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 42 | 0.27% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 43 | 0.27% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 44 | 0.27% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 45 | 0.27% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 46 | 0.26% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 47 | 0.25% | `it.unimi.dsi.fastutil.objects.AbstractObject2ObjectMap.putAll(Map) line: 337` |
| 48 | 0.25% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 49 | 0.24% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 50 | 0.24% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 320` |
| 51 | 0.23% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 52 | 0.22% | `java.util.HashMap.getNode(Object) line: 586` |
| 53 | 0.22% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 54 | 0.22% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 55 | 0.22% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 56 | 0.21% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 57 | 0.21% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |
| 58 | 0.21% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 59 | 0.20% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 60 | 0.20% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 30.42% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 7.35% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 4.93% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 3.86% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 3.14% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 6 | 2.83% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 7 | 1.88% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 8 | 1.75% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 1.24% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 10 | 1.13% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 11 | 1.08% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 12 | 1.00% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 13 | 0.87% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 14 | 0.78% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 15 | 0.70% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 16 | 0.64% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 17 | 0.51% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 18 | 0.48% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 19 | 0.45% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 20 | 0.43% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 21 | 0.43% | `java.util.ArrayList.toArray() line: 370` |
| 22 | 0.42% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 23 | 0.41% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 24 | 0.40% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 25 | 0.36% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 26 | 0.36% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 27 | 0.36% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 28 | 0.35% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 29 | 0.35% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 30 | 0.34% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |

GC pauses: 0, total 0 ms

