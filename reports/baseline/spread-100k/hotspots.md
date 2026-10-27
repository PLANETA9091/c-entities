# JVM hotspots (JFR ExecutionSample)

Samples: 49507

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 8.14% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 5.16% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.26% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 4 | 3.24% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 2.79% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 2.46% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 1.95% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 8 | 1.82% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 9 | 1.62% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 10 | 1.48% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 11 | 1.41% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 12 | 1.40% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 13 | 1.37% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 14 | 1.23% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 15 | 1.22% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 16 | 1.21% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 17 | 1.20% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 18 | 1.19% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 19 | 1.14% | `java.util.EnumMap.get(Object) line: 248` |
| 20 | 1.11% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 21 | 1.04% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 22 | 0.93% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 23 | 0.92% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 24 | 0.91% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 25 | 0.90% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 26 | 0.80% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 27 | 0.79% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 28 | 0.75% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 29 | 0.70% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 30 | 0.69% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 31 | 0.61% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 32 | 0.59% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 33 | 0.58% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 34 | 0.55% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 35 | 0.54% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 36 | 0.49% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 37 | 0.47% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 38 | 0.47% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 39 | 0.47% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 40 | 0.46% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 41 | 0.44% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 42 | 0.43% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 43 | 0.43% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 44 | 0.43% | `java.util.HashMap.getNode(Object) line: 586` |
| 45 | 0.42% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 46 | 0.42% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 47 | 0.42% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 48 | 0.42% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 49 | 0.41% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 50 | 0.40% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 51 | 0.40% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 52 | 0.39% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 53 | 0.39% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 54 | 0.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 323` |
| 55 | 0.37% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 56 | 0.36% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 57 | 0.36% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 58 | 0.36% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 59 | 0.35% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 60 | 0.35% | `net.minecraft.world.entity.Entity.baseTick() line: 914` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 8.14% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 7.93% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 3.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 3.50% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 5 | 3.26% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 6 | 3.24% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 7 | 3.09% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 8 | 2.61% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 2.05% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 10 | 1.81% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 11 | 1.41% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 12 | 1.40% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 13 | 1.22% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 14 | 1.19% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 15 | 1.12% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 16 | 0.99% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 17 | 0.92% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 18 | 0.90% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 19 | 0.89% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 20 | 0.85% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 21 | 0.80% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 22 | 0.73% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 23 | 0.71% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 24 | 0.70% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 25 | 0.70% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 26 | 0.67% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 27 | 0.65% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 28 | 0.60% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2195` |
| 29 | 0.55% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 30 | 0.53% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |

GC pauses: 0, total 0 ms

