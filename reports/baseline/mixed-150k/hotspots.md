# JVM hotspots (JFR ExecutionSample)

Samples: 83996

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 27.33% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.22% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 3 | 3.35% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 4 | 2.85% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 2.77% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 2.67% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 1.68% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 8 | 1.57% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.35% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 10 | 1.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 11 | 1.32% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 12 | 1.21% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 13 | 1.11% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 14 | 1.09% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 15 | 0.84% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 16 | 0.71% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 17 | 0.64% | `java.util.EnumMap.get(Object) line: 248` |
| 18 | 0.64% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 19 | 0.61% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 20 | 0.60% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 21 | 0.60% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 22 | 0.60% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 23 | 0.54% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 24 | 0.52% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 25 | 0.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 26 | 0.51% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 27 | 0.48% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 28 | 0.48% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 29 | 0.47% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 30 | 0.46% | `net.minecraft.world.item.ItemStack.getDamageValue() line: 574` |
| 31 | 0.45% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 32 | 0.45% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 33 | 0.44% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 34 | 0.41% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 35 | 0.39% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 36 | 0.38% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 37 | 0.38% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 38 | 0.37% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 39 | 0.37% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 40 | 0.36% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 41 | 0.35% | `net.minecraft.server.level.ServerChunkCache.pollTask() line: 449` |
| 42 | 0.34% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 43 | 0.34% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 44 | 0.33% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 45 | 0.32% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 46 | 0.32% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 47 | 0.31% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 48 | 0.30% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 49 | 0.29% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 50 | 0.27% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 51 | 0.25% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 52 | 0.24% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 53 | 0.24% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 54 | 0.24% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 55 | 0.23% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 56 | 0.23% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 57 | 0.23% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1484` |
| 58 | 0.23% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 59 | 0.23% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 60 | 0.22% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 27.33% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.76% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 4.56% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 4.34% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 3.36% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3803` |
| 6 | 2.85% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 894` |
| 7 | 2.15% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 8 | 1.73% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 9 | 1.60% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 10 | 1.16% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 11 | 1.03% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 12 | 0.81% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 13 | 0.76% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 14 | 0.65% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 15 | 0.61% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 16 | 0.61% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 17 | 0.60% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 18 | 0.54% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 19 | 0.54% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 20 | 0.48% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 21 | 0.47% | `java.util.ArrayList.toArray() line: 370` |
| 22 | 0.47% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2817` |
| 23 | 0.45% | `net.minecraft.world.item.ItemStack.isBroken() line: 586` |
| 24 | 0.44% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 25 | 0.44% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 26 | 0.41% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 27 | 0.39% | `net.minecraft.world.entity.LivingEntity.tick() line: 3491` |
| 28 | 0.39% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 29 | 0.38% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 30 | 0.37% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |

GC pauses: 0, total 0 ms

