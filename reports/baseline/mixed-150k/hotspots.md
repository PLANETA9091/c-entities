# JVM hotspots (JFR ExecutionSample)

Samples: 70956

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 25.39% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.59% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 3 | 3.49% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 3.14% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 3.11% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 2.76% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 7 | 2.06% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 2.00% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 9 | 1.51% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 10 | 1.44% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 11 | 1.42% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 12 | 1.41% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 13 | 1.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 14 | 1.23% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 15 | 1.23% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 16 | 1.14% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 17 | 1.08% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 18 | 0.84% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 19 | 0.84% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 20 | 0.73% | `java.util.EnumMap.get(Object) line: 248` |
| 21 | 0.67% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 22 | 0.66% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 23 | 0.66% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 24 | 0.63% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 25 | 0.60% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 26 | 0.60% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 27 | 0.59% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 28 | 0.57% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 29 | 0.56% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 30 | 0.55% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 31 | 0.52% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 32 | 0.50% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 33 | 0.49% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 34 | 0.43% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 35 | 0.41% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 36 | 0.39% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 37 | 0.38% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 38 | 0.37% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 92` |
| 39 | 0.36% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 40 | 0.36% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 41 | 0.35% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 42 | 0.34% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 43 | 0.34% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 44 | 0.34% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 45 | 0.33% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 46 | 0.30% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 47 | 0.29% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 48 | 0.29% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 49 | 0.29% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 50 | 0.28% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 51 | 0.28% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 52 | 0.27% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 53 | 0.27% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 54 | 0.27% | `net.minecraft.server.level.ServerChunkCache.pollTask() line: 449` |
| 55 | 0.26% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1424` |
| 56 | 0.25% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 57 | 0.24% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 58 | 0.23% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 59 | 0.23% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 60 | 0.23% | `java.util.ArrayDeque.size() line: 647` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 25.39% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.11% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 5.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 5.17% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 3.59% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 6 | 3.14% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 7 | 2.34% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 8 | 1.85% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 1.83% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 10 | 1.51% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 11 | 1.35% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 12 | 1.30% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 13 | 0.93% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 14 | 0.87% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 15 | 0.83% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 16 | 0.77% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 17 | 0.67% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 18 | 0.66% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 19 | 0.66% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 20 | 0.61% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 21 | 0.59% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 22 | 0.52% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 23 | 0.50% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 24 | 0.49% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 25 | 0.49% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 26 | 0.49% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3584` |
| 27 | 0.47% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 28 | 0.46% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 29 | 0.43% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |
| 30 | 0.40% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |

GC pauses: 0, total 0 ms

