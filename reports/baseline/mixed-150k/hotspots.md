# JVM hotspots (JFR ExecutionSample)

Samples: 90642

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 30.17% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.95% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 3 | 2.61% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 2.33% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 2.23% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 2.18% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 7 | 1.54% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 8 | 1.53% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 9 | 1.45% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.28% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 11 | 1.24% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 12 | 1.17% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 13 | 1.15% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 14 | 1.08% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 15 | 0.89% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 16 | 0.87% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 17 | 0.83% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 18 | 0.81% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 19 | 0.65% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 20 | 0.59% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 21 | 0.59% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 22 | 0.57% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 23 | 0.56% | `java.util.EnumMap.get(Object) line: 248` |
| 24 | 0.54% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 25 | 0.54% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 26 | 0.54% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 27 | 0.48% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 28 | 0.46% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 29 | 0.46% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 30 | 0.46% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 31 | 0.45% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 32 | 0.41% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 33 | 0.40% | `net.minecraft.world.item.ItemStack.getDamageValue() line: 574` |
| 34 | 0.39% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 35 | 0.39% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 36 | 0.38% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 37 | 0.38% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 38 | 0.36% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 39 | 0.36% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 40 | 0.36% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 41 | 0.34% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 42 | 0.32% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 43 | 0.30% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 44 | 0.30% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 45 | 0.30% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 46 | 0.29% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 47 | 0.28% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 48 | 0.27% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 49 | 0.26% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 50 | 0.25% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 51 | 0.25% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 52 | 0.24% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 53 | 0.24% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 54 | 0.24% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 55 | 0.23% | `java.util.HashMap.getNode(Object) line: 586` |
| 56 | 0.22% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 57 | 0.22% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 58 | 0.22% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 59 | 0.22% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 60 | 0.22% | `net.minecraft.core.Vec3i.getX() line: 63` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 30.17% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 7.17% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 4.67% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 3.68% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 2.61% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 894` |
| 6 | 2.18% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3803` |
| 7 | 1.97% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 8 | 1.60% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 9 | 1.59% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 10 | 1.30% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 11 | 1.21% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 12 | 0.86% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 13 | 0.80% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 14 | 0.65% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 15 | 0.59% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 16 | 0.59% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 17 | 0.59% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 18 | 0.56% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 19 | 0.55% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 20 | 0.54% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 21 | 0.46% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2817` |
| 22 | 0.45% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 23 | 0.45% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 24 | 0.41% | `net.minecraft.world.entity.LivingEntity.tick() line: 3491` |
| 25 | 0.40% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 26 | 0.40% | `java.util.ArrayList.toArray() line: 370` |
| 27 | 0.39% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 28 | 0.39% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 29 | 0.39% | `net.minecraft.world.item.ItemStack.isBroken() line: 586` |
| 30 | 0.39% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |

GC pauses: 0, total 0 ms

