# JVM hotspots (JFR ExecutionSample)

Samples: 80788

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 27.87% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.70% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 3 | 3.55% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 3.13% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 3.04% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 6 | 2.73% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 7 | 2.60% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 8 | 1.68% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.55% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 10 | 1.48% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 11 | 1.22% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 12 | 1.21% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 13 | 1.17% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 14 | 1.17% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 15 | 1.14% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 16 | 1.14% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 17 | 1.00% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 18 | 1.00% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 19 | 0.94% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 20 | 0.82% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 21 | 0.78% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 22 | 0.77% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 23 | 0.72% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 24 | 0.71% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 25 | 0.60% | `java.util.EnumMap.get(Object) line: 248` |
| 26 | 0.60% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 27 | 0.59% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 28 | 0.55% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 29 | 0.47% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 30 | 0.46% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 31 | 0.44% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 32 | 0.44% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 33 | 0.43% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 34 | 0.43% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 35 | 0.42% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 36 | 0.42% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 37 | 0.39% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 38 | 0.37% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 39 | 0.36% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 92` |
| 40 | 0.34% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 41 | 0.34% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 42 | 0.34% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 43 | 0.33% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 44 | 0.32% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 45 | 0.32% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 46 | 0.32% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 47 | 0.31% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 48 | 0.29% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 49 | 0.29% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 50 | 0.26% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 51 | 0.26% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 52 | 0.25% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 53 | 0.24% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 54 | 0.24% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 55 | 0.24% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 56 | 0.24% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 320` |
| 57 | 0.23% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 58 | 0.23% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 59 | 0.23% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 60 | 0.23% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 27.87% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 7.09% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 6.09% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 4.80% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 3.04% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 6 | 2.73% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 7 | 2.16% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 8 | 1.96% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 1.63% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 10 | 1.28% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 11 | 1.28% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 12 | 1.14% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 13 | 1.14% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 14 | 0.97% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 15 | 0.95% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 16 | 0.81% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 17 | 0.79% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 18 | 0.71% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 19 | 0.60% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 20 | 0.60% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 21 | 0.59% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 22 | 0.55% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 23 | 0.48% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 24 | 0.45% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 25 | 0.44% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 26 | 0.43% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |
| 27 | 0.42% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 28 | 0.42% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 29 | 0.41% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 30 | 0.40% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |

GC pauses: 0, total 0 ms

