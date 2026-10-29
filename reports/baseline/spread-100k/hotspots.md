# JVM hotspots (JFR ExecutionSample)

Samples: 56860

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 15.23% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.85% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.45% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 2.17% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 5 | 2.02% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 2.00% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 1.82% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 8 | 1.73% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 9 | 1.68% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 10 | 1.52% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 11 | 1.45% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 12 | 1.36% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 13 | 1.29% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 14 | 1.25% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 15 | 1.19% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 16 | 1.18% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 17 | 1.18% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 18 | 1.15% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 19 | 1.11% | `java.util.EnumMap.get(Object) line: 248` |
| 20 | 1.04% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 21 | 1.02% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 22 | 0.89% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 23 | 0.77% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 24 | 0.75% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 25 | 0.75% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 26 | 0.73% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 27 | 0.72% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 28 | 0.71% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 29 | 0.68% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 30 | 0.67% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 31 | 0.64% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 32 | 0.53% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 33 | 0.51% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 34 | 0.50% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 35 | 0.48% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 36 | 0.46% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 37 | 0.46% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 38 | 0.45% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 39 | 0.45% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 40 | 0.45% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 41 | 0.44% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 42 | 0.44% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 43 | 0.43% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 44 | 0.42% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 45 | 0.42% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 46 | 0.41% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 47 | 0.41% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 92` |
| 48 | 0.40% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 49 | 0.39% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 50 | 0.38% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 51 | 0.37% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 52 | 0.36% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 53 | 0.36% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 54 | 0.35% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 55 | 0.34% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 56 | 0.34% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 57 | 0.33% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 58 | 0.33% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 59 | 0.30% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 60 | 0.30% | `java.util.ArrayDeque.size() line: 647` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 15.23% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 5.87% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 4.74% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 3.65% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 5 | 3.45% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 6 | 2.50% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 7 | 2.25% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 8 | 1.82% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 9 | 1.76% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 10 | 1.52% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 11 | 1.36% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 12 | 1.25% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 13 | 1.16% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 14 | 0.97% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 15 | 0.93% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 16 | 0.88% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 17 | 0.80% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 18 | 0.79% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 19 | 0.76% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 20 | 0.75% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 21 | 0.73% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 22 | 0.71% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 23 | 0.69% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 24 | 0.68% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 25 | 0.64% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 26 | 0.59% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 27 | 0.57% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 28 | 0.53% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 29 | 0.50% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 30 | 0.50% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |

GC pauses: 0, total 0 ms

