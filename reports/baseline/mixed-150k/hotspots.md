# JVM hotspots (JFR ExecutionSample)

Samples: 75754

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 27.14% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.60% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 3.34% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 4 | 3.14% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 2.97% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 1.90% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 7 | 1.72% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 1.57% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 9 | 1.48% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 10 | 1.33% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 11 | 1.31% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 12 | 1.29% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 13 | 1.20% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 14 | 1.20% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 15 | 1.16% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 16 | 1.10% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 17 | 1.07% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 18 | 1.06% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 19 | 1.04% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 20 | 1.00% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 21 | 0.95% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 22 | 0.81% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 23 | 0.74% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 24 | 0.66% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 25 | 0.64% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 26 | 0.63% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 27 | 0.62% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 28 | 0.62% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 29 | 0.57% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 30 | 0.54% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 31 | 0.54% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 32 | 0.53% | `java.util.EnumMap.get(Object) line: 248` |
| 33 | 0.49% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 34 | 0.46% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 35 | 0.45% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 36 | 0.41% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 37 | 0.39% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 38 | 0.37% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 39 | 0.37% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 92` |
| 40 | 0.36% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 41 | 0.36% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 42 | 0.35% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 43 | 0.32% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 44 | 0.32% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 45 | 0.32% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 46 | 0.30% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 47 | 0.29% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 48 | 0.28% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 49 | 0.28% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 50 | 0.28% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 51 | 0.26% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 52 | 0.26% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 53 | 0.26% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 54 | 0.26% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 55 | 0.26% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 56 | 0.25% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 57 | 0.25% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 58 | 0.24% | `net.minecraft.world.entity.Entity.baseTick() line: 914` |
| 59 | 0.23% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 60 | 0.23% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 27.14% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.76% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 4.88% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 4.69% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 3.14% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 6 | 2.20% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 7 | 2.00% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 8 | 1.91% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 1.57% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 10 | 1.36% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 11 | 1.20% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 12 | 1.07% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 13 | 1.03% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 14 | 1.03% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 15 | 1.00% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 16 | 0.96% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 17 | 0.80% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 18 | 0.76% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 19 | 0.74% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 20 | 0.67% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 21 | 0.63% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 22 | 0.62% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 23 | 0.62% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 24 | 0.61% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 25 | 0.57% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 26 | 0.48% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 27 | 0.46% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 28 | 0.44% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 29 | 0.43% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3584` |
| 30 | 0.42% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |

GC pauses: 0, total 0 ms

