# JVM hotspots (JFR ExecutionSample)

Samples: 61638

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 19.14% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.43% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.31% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 4 | 2.77% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 5 | 2.60% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 6 | 1.89% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 7 | 1.84% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 1.60% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 9 | 1.50% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 10 | 1.50% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 11 | 1.38% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 12 | 1.19% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 13 | 1.11% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 14 | 1.04% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 15 | 0.97% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 16 | 0.92% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 17 | 0.90% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 18 | 0.89% | `java.util.EnumMap.get(Object) line: 248` |
| 19 | 0.86% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 20 | 0.77% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 21 | 0.77% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 22 | 0.77% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 23 | 0.74% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 24 | 0.73% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 25 | 0.70% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 26 | 0.67% | `net.minecraft.server.level.ServerChunkCache.syncLoad(int, int, ChunkStatus) line: 123` |
| 27 | 0.66% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 28 | 0.64% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 29 | 0.62% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 30 | 0.57% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 31 | 0.57% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 32 | 0.56% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 33 | 0.55% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 34 | 0.54% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 35 | 0.52% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 36 | 0.47% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 37 | 0.45% | `net.minecraft.world.entity.LivingEntity.updateGlowingStatus() line: 1025` |
| 38 | 0.45% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 39 | 0.45% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 323` |
| 40 | 0.45% | `java.util.HashMap.getNode(Object) line: 586` |
| 41 | 0.44% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 42 | 0.42% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 43 | 0.42% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 44 | 0.41% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 45 | 0.38% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 46 | 0.37% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 47 | 0.35% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 48 | 0.33% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 49 | 0.32% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 50 | 0.31% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 51 | 0.31% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 52 | 0.31% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 53 | 0.31% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 54 | 0.29% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 55 | 0.29% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 56 | 0.29% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 57 | 0.28% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 58 | 0.28% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 59 | 0.28% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 92` |
| 60 | 0.27% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 19.14% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 5.26% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 3.89% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 3.65% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 5 | 2.77% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 6 | 2.60% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 7 | 2.52% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 8 | 2.10% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 1.65% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 10 | 1.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 11 | 1.14% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 12 | 1.11% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 13 | 0.97% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 14 | 0.89% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 15 | 0.89% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 16 | 0.87% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 17 | 0.86% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 18 | 0.79% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 19 | 0.74% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |
| 20 | 0.70% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 21 | 0.69% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 22 | 0.67% | `net.minecraft.server.level.ServerChunkCache.getChunkFallback(int, int, ChunkStatus, boolean) line: 157` |
| 23 | 0.67% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 24 | 0.64% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 25 | 0.63% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 26 | 0.61% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 27 | 0.61% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 28 | 0.56% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 29 | 0.54% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 30 | 0.51% | `net.minecraft.world.level.chunk.PalettedContainer.readPalette(PalettedContainer$Data, int) line: 79` |

GC pauses: 0, total 0 ms

