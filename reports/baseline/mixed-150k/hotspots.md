# JVM hotspots (JFR ExecutionSample)

Samples: 70163

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 25.03% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.49% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 3.49% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 3.43% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 2.66% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 6 | 2.48% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 7 | 2.13% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 8 | 2.00% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.43% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 10 | 1.42% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 11 | 1.38% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 12 | 1.35% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 13 | 1.34% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 14 | 1.33% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 15 | 1.25% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 16 | 1.21% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 17 | 1.14% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 18 | 1.09% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 19 | 0.84% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 20 | 0.80% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 21 | 0.73% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 22 | 0.69% | `java.util.EnumMap.get(Object) line: 248` |
| 23 | 0.65% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 24 | 0.64% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 25 | 0.62% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 26 | 0.62% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 27 | 0.61% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 28 | 0.61% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 29 | 0.58% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 30 | 0.58% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 31 | 0.54% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 32 | 0.52% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 33 | 0.42% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 34 | 0.41% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 35 | 0.39% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 36 | 0.37% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 92` |
| 37 | 0.37% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 38 | 0.37% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 39 | 0.36% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 40 | 0.35% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 41 | 0.33% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 42 | 0.33% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 43 | 0.32% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 44 | 0.32% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 45 | 0.31% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 46 | 0.30% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 47 | 0.30% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 48 | 0.29% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 49 | 0.29% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 50 | 0.27% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 51 | 0.27% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 52 | 0.26% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 53 | 0.26% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 54 | 0.26% | `net.minecraft.world.entity.Entity.baseTick() line: 914` |
| 55 | 0.25% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 56 | 0.25% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 57 | 0.25% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 58 | 0.24% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 320` |
| 59 | 0.24% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 60 | 0.23% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 25.03% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.00% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 5.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 5.49% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 3.43% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 6 | 2.49% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 7 | 2.34% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 8 | 1.89% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 1.84% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 10 | 1.33% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 11 | 1.32% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 12 | 1.10% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 13 | 1.06% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 14 | 0.97% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 15 | 0.95% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 16 | 0.83% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 17 | 0.83% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 18 | 0.80% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 19 | 0.65% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 20 | 0.61% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 21 | 0.61% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 22 | 0.61% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 23 | 0.52% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |
| 24 | 0.50% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 25 | 0.49% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 26 | 0.47% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 27 | 0.44% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3584` |
| 28 | 0.43% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 29 | 0.40% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 30 | 0.39% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |

GC pauses: 0, total 0 ms

