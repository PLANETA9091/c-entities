# JVM hotspots (JFR ExecutionSample)

Samples: 66912

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 23.02% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.63% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.20% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 3.20% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 2.82% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 6 | 2.65% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 7 | 2.38% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 8 | 2.04% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 9 | 1.90% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.54% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 11 | 1.37% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 12 | 1.23% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 13 | 1.20% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 14 | 1.18% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 15 | 0.98% | `java.util.EnumMap.get(Object) line: 248` |
| 16 | 0.82% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 17 | 0.81% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 18 | 0.80% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 19 | 0.76% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 20 | 0.70% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 21 | 0.65% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 22 | 0.65% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 23 | 0.64% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 24 | 0.62% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 25 | 0.60% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 26 | 0.56% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 27 | 0.56% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 28 | 0.54% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 29 | 0.50% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 30 | 0.47% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 31 | 0.47% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 32 | 0.47% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 33 | 0.45% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 34 | 0.45% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 35 | 0.43% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 36 | 0.41% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 37 | 0.40% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 38 | 0.39% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 39 | 0.38% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 40 | 0.37% | `net.minecraft.world.item.ItemStack.getMaxDamage() line: 581` |
| 41 | 0.36% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 42 | 0.35% | `java.util.EnumMap$EntryIterator$Entry.getKey() line: 589` |
| 43 | 0.34% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 44 | 0.33% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 45 | 0.33% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 46 | 0.32% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 47 | 0.31% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 48 | 0.30% | `net.minecraft.world.entity.Entity.baseTick() line: 914` |
| 49 | 0.29% | `java.util.HashMap.getNode(Object) line: 586` |
| 50 | 0.29% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 51 | 0.29% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 52 | 0.27% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 53 | 0.27% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 54 | 0.26% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 55 | 0.25% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 56 | 0.25% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 57 | 0.24% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 58 | 0.24% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 59 | 0.24% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 60 | 0.23% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 23.02% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 5.82% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 5.52% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 4.74% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 5 | 3.20% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 6 | 2.65% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3803` |
| 7 | 2.37% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 8 | 2.28% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 9 | 2.02% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 10 | 1.48% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 11 | 1.27% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 12 | 1.01% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 13 | 0.83% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 14 | 0.75% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 15 | 0.74% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 16 | 0.70% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 17 | 0.65% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 18 | 0.64% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 19 | 0.60% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 20 | 0.56% | `net.minecraft.world.entity.LivingEntity.tick() line: 3491` |
| 21 | 0.52% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 22 | 0.47% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 23 | 0.46% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 24 | 0.45% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 25 | 0.45% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 26 | 0.45% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |
| 27 | 0.43% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 28 | 0.42% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 29 | 0.41% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 30 | 0.40% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |

GC pauses: 0, total 0 ms

