# JVM hotspots (JFR ExecutionSample)

Samples: 53969

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 11.14% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.12% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 2.79% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 2.35% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3908` |
| 5 | 2.33% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 2.32% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 1.95% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 8 | 1.70% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 9 | 1.65% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 10 | 1.58% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 11 | 1.45% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 1.34% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 13 | 1.25% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 14 | 1.24% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 15 | 1.22% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 16 | 1.22% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 17 | 1.22% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 18 | 1.16% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 19 | 1.14% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 20 | 0.94% | `java.util.EnumMap.get(Object) line: 248` |
| 21 | 0.93% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 22 | 0.92% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 23 | 0.92% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 24 | 0.92% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 25 | 0.84% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 26 | 0.67% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 27 | 0.66% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 28 | 0.62% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 29 | 0.56% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 30 | 0.55% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 31 | 0.54% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 32 | 0.53% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 33 | 0.52% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 34 | 0.50% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 35 | 0.50% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 36 | 0.49% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 37 | 0.48% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 38 | 0.47% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 39 | 0.46% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 40 | 0.45% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 41 | 0.45% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 42 | 0.44% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 43 | 0.42% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 44 | 0.40% | `java.util.HashMap.getNode(Object) line: 586` |
| 45 | 0.38% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 46 | 0.37% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 47 | 0.37% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 48 | 0.37% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 49 | 0.35% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 50 | 0.35% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 51 | 0.35% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 52 | 0.34% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 53 | 0.34% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 54 | 0.34% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 55 | 0.32% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 56 | 0.31% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 57 | 0.31% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 58 | 0.31% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 59 | 0.31% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 92` |
| 60 | 0.31% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 11.14% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.46% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 4.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 4.15% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 5 | 2.79% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 6 | 2.69% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 7 | 2.36% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 8 | 2.35% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 9 | 1.73% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 10 | 1.43% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 11 | 1.25% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 12 | 1.17% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 13 | 1.16% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 14 | 1.14% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 15 | 0.94% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 16 | 0.87% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 17 | 0.86% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 18 | 0.82% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 19 | 0.77% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 20 | 0.71% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 21 | 0.70% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 22 | 0.69% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 23 | 0.57% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 24 | 0.56% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 25 | 0.56% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 26 | 0.55% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 27 | 0.55% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 28 | 0.53% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 29 | 0.51% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2195` |
| 30 | 0.51% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |

GC pauses: 0, total 0 ms

