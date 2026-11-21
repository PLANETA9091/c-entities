# JVM hotspots (JFR ExecutionSample)

Samples: 49416

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 11.97% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.97% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 3.73% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 3.55% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 5 | 2.36% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 2.08% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 7 | 1.78% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 1.78% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 9 | 1.68% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 10 | 1.58% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 11 | 1.29% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 12 | 1.29% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 13 | 1.21% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 14 | 1.19% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 15 | 1.04% | `java.util.EnumMap.get(Object) line: 248` |
| 16 | 0.99% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 17 | 0.98% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 18 | 0.90% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 19 | 0.87% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 20 | 0.81% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 21 | 0.79% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 22 | 0.77% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 23 | 0.77% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 24 | 0.73% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 25 | 0.72% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 26 | 0.72% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 27 | 0.71% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 28 | 0.67% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 29 | 0.67% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 30 | 0.64% | `net.minecraft.world.item.ItemStack.getMaxDamage() line: 582` |
| 31 | 0.62% | `java.util.concurrent.CompletableFuture.isDone() line: 2076` |
| 32 | 0.60% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 33 | 0.54% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 34 | 0.54% | `net.minecraft.server.level.ServerChunkCache.pollTask() line: 449` |
| 35 | 0.54% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 36 | 0.54% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 37 | 0.53% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 38 | 0.51% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 39 | 0.50% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 40 | 0.49% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 41 | 0.49% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 42 | 0.49% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 43 | 0.48% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 44 | 0.48% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 45 | 0.48% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 46 | 0.45% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 47 | 0.44% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 48 | 0.41% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 49 | 0.40% | `java.util.HashMap.getNode(Object) line: 586` |
| 50 | 0.38% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 51 | 0.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 323` |
| 52 | 0.38% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 53 | 0.37% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 54 | 0.35% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 55 | 0.34% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 56 | 0.34% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 57 | 0.33% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 58 | 0.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 59 | 0.32% | `net.minecraft.world.entity.Entity.baseTick() line: 914` |
| 60 | 0.32% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 11.97% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.09% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 3.97% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 894` |
| 4 | 3.55% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3803` |
| 5 | 3.06% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 6 | 2.83% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 7 | 2.78% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 8 | 2.36% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 2.18% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 10 | 1.89% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 11 | 1.72% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 12 | 1.02% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 13 | 0.92% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 14 | 0.86% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 15 | 0.85% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 16 | 0.82% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 17 | 0.81% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 18 | 0.79% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 19 | 0.79% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 20 | 0.78% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 21 | 0.73% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 22 | 0.72% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 23 | 0.67% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 24 | 0.64% | `net.minecraft.world.item.ItemStack.isBroken() line: 586` |
| 25 | 0.64% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 26 | 0.62% | `net.minecraft.util.thread.BlockableEventLoop.managedBlock(BooleanSupplier) line: 190` |
| 27 | 0.59% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 28 | 0.59% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 29 | 0.54% | `net.minecraft.server.MinecraftServer.tickMidTickTasks() line: 448` |
| 30 | 0.54% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |

GC pauses: 0, total 0 ms

