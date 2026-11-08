# JVM hotspots (JFR ExecutionSample)

Samples: 99866

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 31.18% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 5.41% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 3 | 2.43% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 4 | 2.17% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 5 | 1.68% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 6 | 1.66% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 7 | 1.35% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 1.28% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 9 | 1.20% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 10 | 1.05% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 11 | 1.04% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 12 | 0.95% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 13 | 0.93% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 14 | 0.88% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 15 | 0.86% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 16 | 0.75% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 17 | 0.74% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 18 | 0.72% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 19 | 0.69% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 20 | 0.64% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 21 | 0.59% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 22 | 0.56% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 23 | 0.56% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 24 | 0.56% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 25 | 0.52% | `java.util.HashMap.getNode(Object) line: 586` |
| 26 | 0.50% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 27 | 0.49% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 28 | 0.49% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 29 | 0.45% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 30 | 0.44% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 31 | 0.41% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 32 | 0.40% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 33 | 0.40% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 34 | 0.38% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 35 | 0.37% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 36 | 0.37% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 37 | 0.36% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 323` |
| 38 | 0.36% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 39 | 0.36% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 40 | 0.33% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 41 | 0.33% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 42 | 0.33% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 43 | 0.32% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 44 | 0.31% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 320` |
| 45 | 0.31% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 46 | 0.31% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 47 | 0.30% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 48 | 0.28% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 49 | 0.27% | `net.minecraft.server.level.ServerChunkCache.pollTask() line: 449` |
| 50 | 0.26% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 51 | 0.26% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 52 | 0.26% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 53 | 0.26% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 54 | 0.25% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 55 | 0.24% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 56 | 0.23% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 57 | 0.23% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 58 | 0.23% | `java.util.EnumMap.get(Object) line: 248` |
| 59 | 0.22% | `net.minecraft.world.entity.Entity.baseTick() line: 914` |
| 60 | 0.22% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 235` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 31.18% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 5.86% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 2.64% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 2.43% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3803` |
| 5 | 2.17% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 6 | 2.10% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 7 | 1.67% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 8 | 1.28% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 9 | 1.27% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 10 | 1.24% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 11 | 1.12% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 125` |
| 12 | 1.00% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 13 | 0.98% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 14 | 0.96% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 15 | 0.92% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 16 | 0.92% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 17 | 0.90% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 18 | 0.72% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |
| 19 | 0.70% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 20 | 0.65% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 21 | 0.56% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |
| 22 | 0.56% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 23 | 0.55% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 24 | 0.48% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 25 | 0.48% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 26 | 0.45% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 27 | 0.45% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 28 | 0.44% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 29 | 0.41% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 30 | 0.41% | `net.minecraft.world.level.chunk.PalettedContainer.readPalette(PalettedContainer$Data, int) line: 79` |

GC pauses: 0, total 0 ms

