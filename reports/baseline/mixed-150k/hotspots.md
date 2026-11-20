# JVM hotspots (JFR ExecutionSample)

Samples: 88885

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 27.51% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.28% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 3 | 3.26% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 4 | 3.04% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 2.91% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 6 | 2.57% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 1.65% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 8 | 1.38% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.30% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 10 | 1.27% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 11 | 1.20% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 12 | 1.12% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 13 | 1.10% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 14 | 1.05% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 15 | 0.93% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 16 | 0.91% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 17 | 0.90% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 18 | 0.74% | `java.util.EnumMap.get(Object) line: 248` |
| 19 | 0.72% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 20 | 0.69% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 21 | 0.61% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 22 | 0.58% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 23 | 0.56% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 24 | 0.55% | `net.minecraft.world.item.ItemStack.getDamageValue() line: 574` |
| 25 | 0.55% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 26 | 0.54% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 27 | 0.53% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 28 | 0.52% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 29 | 0.52% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 30 | 0.45% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 31 | 0.45% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 32 | 0.44% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 33 | 0.41% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 34 | 0.40% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 35 | 0.40% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 36 | 0.39% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 37 | 0.38% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 38 | 0.36% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 39 | 0.35% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 40 | 0.34% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 41 | 0.34% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 42 | 0.34% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 43 | 0.32% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 44 | 0.32% | `net.minecraft.server.level.ServerChunkCache.pollTask() line: 449` |
| 45 | 0.31% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 46 | 0.28% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 47 | 0.28% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 48 | 0.26% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 49 | 0.26% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 50 | 0.25% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 51 | 0.25% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 52 | 0.24% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 53 | 0.24% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 54 | 0.24% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 55 | 0.23% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 56 | 0.23% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.<init>(Map, float) line: 124` |
| 57 | 0.23% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 58 | 0.23% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 59 | 0.23% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 60 | 0.22% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 27.51% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 7.18% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 3.95% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 3.90% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 5 | 3.26% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3803` |
| 6 | 2.91% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 894` |
| 7 | 2.18% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 8 | 1.84% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 1.68% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 10 | 1.44% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 11 | 1.25% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 12 | 0.96% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 13 | 0.91% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 14 | 0.71% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 15 | 0.69% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 16 | 0.62% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 17 | 0.58% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 18 | 0.57% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 19 | 0.56% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 20 | 0.55% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 21 | 0.54% | `net.minecraft.world.item.ItemStack.isBroken() line: 586` |
| 22 | 0.45% | `net.minecraft.world.entity.LivingEntity.tick() line: 3491` |
| 23 | 0.44% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 24 | 0.42% | `java.util.ArrayList.toArray() line: 370` |
| 25 | 0.41% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 26 | 0.39% | `net.minecraft.world.entity.Entity.setDeltaMovement(double, double, double) line: 5092` |
| 27 | 0.39% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 28 | 0.38% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 29 | 0.38% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 30 | 0.38% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |

GC pauses: 0, total 0 ms

