# JVM hotspots (JFR ExecutionSample)

Samples: 68097

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 18.17% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.21% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 2.80% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 2.33% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 5 | 1.97% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 1.83% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 7 | 1.81% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 1.79% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 9 | 1.64% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 10 | 1.60% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 11 | 1.58% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 12 | 1.49% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 13 | 1.42% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 14 | 1.35% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 15 | 1.32% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 16 | 1.07% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 17 | 1.02% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 18 | 1.01% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 19 | 0.86% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 20 | 0.85% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 21 | 0.80% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 22 | 0.77% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 23 | 0.72% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 24 | 0.71% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 25 | 0.69% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 26 | 0.67% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 27 | 0.66% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 28 | 0.64% | `java.util.EnumMap.get(Object) line: 248` |
| 29 | 0.64% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 30 | 0.58% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 31 | 0.54% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 32 | 0.51% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 33 | 0.50% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 34 | 0.50% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 35 | 0.45% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 36 | 0.44% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 37 | 0.44% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 38 | 0.44% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 39 | 0.43% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 40 | 0.42% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 41 | 0.42% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 42 | 0.42% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 43 | 0.41% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 44 | 0.40% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 45 | 0.40% | `java.util.HashMap.getNode(Object) line: 586` |
| 46 | 0.40% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 47 | 0.39% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 92` |
| 48 | 0.39% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 49 | 0.38% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 50 | 0.36% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 51 | 0.34% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 52 | 0.33% | `net.minecraft.world.entity.EntitySelector.lambda$static$4(Entity) line: 17` |
| 53 | 0.32% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 54 | 0.32% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 55 | 0.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 56 | 0.30% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 57 | 0.29% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 323` |
| 58 | 0.29% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 59 | 0.29% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 60 | 0.28% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 18.17% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 4.77% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 4.03% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 3.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 5 | 3.21% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 6 | 2.31% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 7 | 2.08% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 8 | 1.83% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 9 | 1.70% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 10 | 1.59% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 11 | 1.35% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 12 | 1.27% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 13 | 1.13% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 14 | 1.07% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 15 | 1.01% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 16 | 0.95% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 17 | 0.75% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 18 | 0.72% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 19 | 0.70% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 20 | 0.69% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 21 | 0.66% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 22 | 0.64% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |
| 23 | 0.61% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 125` |
| 24 | 0.61% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 25 | 0.60% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 26 | 0.57% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 27 | 0.53% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 28 | 0.52% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 29 | 0.51% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 30 | 0.50% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |

GC pauses: 0, total 0 ms

