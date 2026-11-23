# JVM hotspots (JFR ExecutionSample)

Samples: 105757

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 31.97% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.03% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 2.98% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 4 | 2.13% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 2.03% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 6 | 1.91% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 1.40% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 8 | 1.37% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 9 | 1.29% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 10 | 1.17% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.12% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 12 | 0.92% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 13 | 0.91% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 14 | 0.77% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 15 | 0.70% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 16 | 0.67% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 17 | 0.65% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 18 | 0.62% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 19 | 0.57% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 20 | 0.55% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 21 | 0.55% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 22 | 0.52% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 23 | 0.52% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 24 | 0.52% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 25 | 0.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 26 | 0.47% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 27 | 0.46% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 28 | 0.44% | `java.util.EnumMap.get(Object) line: 248` |
| 29 | 0.42% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 30 | 0.40% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 31 | 0.39% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 32 | 0.39% | `net.minecraft.world.item.ItemStack.getMaxDamage() line: 582` |
| 33 | 0.39% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 34 | 0.38% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 35 | 0.37% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 36 | 0.37% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 37 | 0.34% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 38 | 0.34% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 39 | 0.31% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 40 | 0.29% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 41 | 0.29% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 42 | 0.29% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 43 | 0.28% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 44 | 0.27% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 45 | 0.27% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 46 | 0.26% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 47 | 0.26% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 48 | 0.26% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 49 | 0.25% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 50 | 0.25% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 51 | 0.25% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 52 | 0.24% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 53 | 0.24% | `java.util.HashMap.getNode(Object) line: 586` |
| 54 | 0.23% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 55 | 0.23% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 56 | 0.23% | `net.minecraft.world.item.ItemStack.isEmpty() line: 335` |
| 57 | 0.22% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 58 | 0.22% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 59 | 0.21% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 60 | 0.21% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 31.97% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 5.91% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 4.62% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 3.07% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 2.13% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 894` |
| 6 | 2.03% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3803` |
| 7 | 1.71% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 8 | 1.33% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 9 | 1.29% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 10 | 1.03% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 11 | 0.81% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 12 | 0.68% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 13 | 0.67% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 14 | 0.63% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 15 | 0.55% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 16 | 0.55% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 17 | 0.53% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 18 | 0.52% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 19 | 0.51% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 20 | 0.50% | `java.util.ArrayList.toArray() line: 370` |
| 21 | 0.48% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 22 | 0.46% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 23 | 0.43% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 24 | 0.42% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 848` |
| 25 | 0.40% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 26 | 0.39% | `net.minecraft.world.item.ItemStack.isBroken() line: 586` |
| 27 | 0.39% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 28 | 0.39% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 29 | 0.38% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 30 | 0.38% | `net.minecraft.world.entity.LivingEntity.tick() line: 3491` |

GC pauses: 0, total 0 ms

