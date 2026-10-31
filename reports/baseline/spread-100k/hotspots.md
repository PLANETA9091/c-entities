# JVM hotspots (JFR ExecutionSample)

Samples: 60760

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 14.18% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.57% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 3.39% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 2.10% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 5 | 1.99% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 6 | 1.91% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 1.79% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 1.72% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 9 | 1.71% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 10 | 1.68% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 11 | 1.66% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 12 | 1.59% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 13 | 1.44% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 14 | 1.41% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 15 | 1.38% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 16 | 1.30% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 17 | 1.21% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 18 | 1.19% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 19 | 1.07% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 20 | 1.02% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 21 | 0.95% | `java.util.EnumMap.get(Object) line: 248` |
| 22 | 0.95% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 23 | 0.92% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 24 | 0.90% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 25 | 0.84% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 26 | 0.81% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 27 | 0.79% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 28 | 0.77% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 29 | 0.66% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 30 | 0.66% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 31 | 0.66% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 32 | 0.63% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 33 | 0.60% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 34 | 0.58% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 35 | 0.58% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 36 | 0.57% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 37 | 0.56% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 38 | 0.56% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 39 | 0.50% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 40 | 0.47% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 41 | 0.46% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 42 | 0.45% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 43 | 0.45% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 44 | 0.44% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 45 | 0.43% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 46 | 0.42% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 47 | 0.41% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 48 | 0.41% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 49 | 0.39% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 50 | 0.39% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 323` |
| 51 | 0.39% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 52 | 0.39% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 53 | 0.38% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 54 | 0.37% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 55 | 0.36% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 56 | 0.36% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 92` |
| 57 | 0.35% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 58 | 0.35% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 59 | 0.35% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 60 | 0.32% | `net.minecraft.core.Vec3i.getX() line: 63` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 14.18% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 5.47% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 3.57% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 4 | 3.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 5 | 3.26% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 6 | 2.49% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 7 | 2.14% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 8 | 1.99% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 9 | 1.81% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 10 | 1.80% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 11 | 1.79% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 12 | 1.41% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 13 | 1.29% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 14 | 1.21% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 15 | 1.05% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 16 | 0.98% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 17 | 0.97% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 18 | 0.95% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 19 | 0.92% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 20 | 0.81% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 21 | 0.80% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 22 | 0.80% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 23 | 0.79% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 24 | 0.72% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 25 | 0.72% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 26 | 0.66% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |
| 27 | 0.66% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 28 | 0.66% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 29 | 0.63% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 30 | 0.63% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |

GC pauses: 0, total 0 ms

