# JVM hotspots (JFR ExecutionSample)

Samples: 89855

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 28.17% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.95% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 3 | 2.16% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 1.89% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 5 | 1.81% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 1.63% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 7 | 1.33% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 8 | 1.18% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 1.13% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 10 | 1.10% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.07% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 12 | 1.06% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 13 | 1.01% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 14 | 1.00% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 15 | 0.98% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 16 | 0.84% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 17 | 0.80% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 18 | 0.67% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 19 | 0.64% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 20 | 0.64% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 21 | 0.64% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 22 | 0.52% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 23 | 0.51% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 24 | 0.49% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 25 | 0.49% | `java.util.EnumMap.get(Object) line: 248` |
| 26 | 0.48% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 27 | 0.48% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 28 | 0.47% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 29 | 0.45% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 30 | 0.42% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 31 | 0.42% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 32 | 0.41% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 33 | 0.39% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 34 | 0.39% | `java.util.HashMap.getNode(Object) line: 586` |
| 35 | 0.38% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 36 | 0.38% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 37 | 0.35% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 38 | 0.35% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 39 | 0.35% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 40 | 0.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 41 | 0.34% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 42 | 0.33% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 43 | 0.32% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 44 | 0.31% | `net.minecraft.world.item.ItemStack.getMaxDamage() line: 582` |
| 45 | 0.30% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 46 | 0.30% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 47 | 0.29% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 48 | 0.28% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 49 | 0.27% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 50 | 0.26% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 51 | 0.26% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 52 | 0.25% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1484` |
| 53 | 0.25% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 323` |
| 54 | 0.24% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 320` |
| 55 | 0.24% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 56 | 0.23% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 57 | 0.23% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 58 | 0.23% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 59 | 0.23% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 60 | 0.22% | `net.minecraft.world.entity.Entity.level() line: 5421` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 28.17% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 5.02% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 2.91% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 2.18% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 5 | 2.16% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 894` |
| 6 | 1.89% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3803` |
| 7 | 1.60% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 8 | 1.55% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 9 | 1.49% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 10 | 1.07% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 11 | 1.05% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 12 | 1.01% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 13 | 0.97% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 14 | 0.91% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 15 | 0.89% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 125` |
| 16 | 0.86% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 17 | 0.79% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 18 | 0.64% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |
| 19 | 0.62% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 20 | 0.61% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 21 | 0.58% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 22 | 0.54% | `java.util.ArrayList.toArray() line: 370` |
| 23 | 0.51% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 24 | 0.51% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 25 | 0.49% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 26 | 0.47% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 27 | 0.46% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 28 | 0.45% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |
| 29 | 0.39% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 30 | 0.39% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |

GC pauses: 0, total 0 ms

