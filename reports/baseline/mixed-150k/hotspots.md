# JVM hotspots (JFR ExecutionSample)

Samples: 89092

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 28.96% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.86% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 3 | 3.40% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 2.81% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 2.52% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 1.55% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 7 | 1.42% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 1.20% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 9 | 1.19% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 10 | 1.19% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 11 | 1.13% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 12 | 1.12% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 13 | 0.99% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 14 | 0.90% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 15 | 0.86% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 16 | 0.86% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 17 | 0.69% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 18 | 0.67% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 19 | 0.60% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3584` |
| 20 | 0.59% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 21 | 0.59% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 22 | 0.58% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 23 | 0.56% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 24 | 0.53% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 25 | 0.50% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 26 | 0.50% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 27 | 0.49% | `java.util.EnumMap.get(Object) line: 248` |
| 28 | 0.49% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3707` |
| 29 | 0.47% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 30 | 0.46% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 31 | 0.45% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 32 | 0.44% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 33 | 0.43% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 34 | 0.43% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 35 | 0.42% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 36 | 0.40% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 37 | 0.38% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 38 | 0.37% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 39 | 0.36% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 40 | 0.36% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 41 | 0.35% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 42 | 0.35% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 43 | 0.34% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 44 | 0.34% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 45 | 0.33% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 46 | 0.33% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 47 | 0.33% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 48 | 0.32% | `net.minecraft.world.item.ItemStack.getMaxDamage() line: 582` |
| 49 | 0.30% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 50 | 0.29% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 51 | 0.27% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 52 | 0.26% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 53 | 0.25% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 54 | 0.25% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 55 | 0.24% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 56 | 0.23% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 57 | 0.23% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 58 | 0.23% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 59 | 0.23% | `java.util.HashMap.getNode(Object) line: 586` |
| 60 | 0.23% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 28.96% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 8.14% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 4.21% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 3.93% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 2.81% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 894` |
| 6 | 1.80% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 7 | 1.62% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 8 | 1.56% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 1.18% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 10 | 0.89% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 11 | 0.86% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 12 | 0.83% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 13 | 0.75% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 14 | 0.68% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 15 | 0.67% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 16 | 0.61% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3696` |
| 17 | 0.59% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2817` |
| 18 | 0.58% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 19 | 0.56% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 20 | 0.54% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 21 | 0.53% | `net.minecraft.world.entity.LivingEntity.tick() line: 3443` |
| 22 | 0.51% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 23 | 0.50% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 24 | 0.46% | `net.minecraft.world.entity.LivingEntity.tick() line: 3491` |
| 25 | 0.45% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 26 | 0.45% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 27 | 0.44% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 28 | 0.37% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |
| 29 | 0.37% | `java.util.ArrayList.toArray() line: 370` |
| 30 | 0.36% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |

GC pauses: 0, total 0 ms

