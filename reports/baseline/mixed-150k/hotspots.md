# JVM hotspots (JFR ExecutionSample)

Samples: 89724

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 28.63% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.70% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 3 | 3.56% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 4 | 3.54% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 2.71% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 6 | 2.66% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 1.54% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 8 | 1.34% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.21% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 10 | 1.21% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 11 | 1.19% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 12 | 1.16% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 13 | 1.11% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 14 | 1.09% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 15 | 0.80% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 16 | 0.74% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 17 | 0.70% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 18 | 0.68% | `java.util.EnumMap.get(Object) line: 248` |
| 19 | 0.68% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 20 | 0.63% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 21 | 0.61% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 22 | 0.59% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 23 | 0.59% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 24 | 0.54% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 25 | 0.51% | `net.minecraft.world.item.ItemStack.getMaxDamage() line: 582` |
| 26 | 0.50% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 27 | 0.48% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 28 | 0.45% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 29 | 0.44% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 30 | 0.43% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 31 | 0.42% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 32 | 0.42% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 33 | 0.41% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 34 | 0.40% | `net.minecraft.world.entity.LivingEntity.updateGlowingStatus() line: 1025` |
| 35 | 0.38% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 36 | 0.37% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 37 | 0.35% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 38 | 0.34% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 39 | 0.33% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 40 | 0.33% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 41 | 0.32% | `java.util.HashMap.getNode(Object) line: 586` |
| 42 | 0.32% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 43 | 0.31% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 44 | 0.30% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 45 | 0.29% | `java.util.EnumMap$EntryIterator$Entry.getKey() line: 589` |
| 46 | 0.29% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 47 | 0.29% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 48 | 0.28% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 49 | 0.27% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 853` |
| 50 | 0.27% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 51 | 0.25% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 52 | 0.24% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 53 | 0.24% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 54 | 0.23% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 55 | 0.23% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 56 | 0.23% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 57 | 0.23% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 58 | 0.22% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 59 | 0.22% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |
| 60 | 0.21% | `net.minecraft.world.entity.Entity.getX() line: 5100` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 28.63% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 8.12% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 4.09% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 4.00% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 3.56% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3803` |
| 6 | 2.71% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 894` |
| 7 | 2.01% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 8 | 1.68% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 1.59% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 10 | 1.15% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 11 | 1.09% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 12 | 0.77% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 13 | 0.75% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 14 | 0.75% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 15 | 0.72% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 16 | 0.63% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 17 | 0.59% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 18 | 0.54% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 19 | 0.53% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 20 | 0.51% | `net.minecraft.world.item.ItemStack.isBroken() line: 586` |
| 21 | 0.49% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 22 | 0.44% | `net.minecraft.world.entity.LivingEntity.tick() line: 3491` |
| 23 | 0.42% | `java.util.ArrayList.toArray() line: 370` |
| 24 | 0.41% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 25 | 0.41% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 26 | 0.40% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 27 | 0.40% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 997` |
| 28 | 0.38% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |
| 29 | 0.36% | `java.util.Map.getOrDefault(Object, Object) line: 691` |
| 30 | 0.35% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2817` |

GC pauses: 0, total 0 ms

