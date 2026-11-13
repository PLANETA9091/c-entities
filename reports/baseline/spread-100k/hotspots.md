# JVM hotspots (JFR ExecutionSample)

Samples: 77744

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 22.94% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.09% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 2.46% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 4 | 2.44% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 5 | 2.11% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 1.70% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 7 | 1.56% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 8 | 1.37% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 9 | 1.36% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 10 | 1.30% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 11 | 1.27% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 12 | 1.10% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 1.10% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 14 | 1.08% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 15 | 0.95% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 16 | 0.83% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 17 | 0.79% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 18 | 0.77% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 19 | 0.71% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 20 | 0.68% | `java.util.EnumMap.get(Object) line: 248` |
| 21 | 0.65% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 22 | 0.58% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 23 | 0.57% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 24 | 0.54% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 25 | 0.53% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 26 | 0.53% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 27 | 0.53% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 28 | 0.52% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 29 | 0.51% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 30 | 0.50% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 31 | 0.49% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 32 | 0.49% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 33 | 0.48% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 34 | 0.47% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 35 | 0.46% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 36 | 0.45% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 37 | 0.44% | `net.minecraft.util.SimpleBitStorage.get(int) line: 346` |
| 38 | 0.44% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3739` |
| 39 | 0.42% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 40 | 0.42% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 41 | 0.41% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 42 | 0.39% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 43 | 0.37% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 44 | 0.37% | `java.util.HashMap.getNode(Object) line: 586` |
| 45 | 0.36% | `java.util.Optional.ifPresent(Consumer) line: 178` |
| 46 | 0.34% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 47 | 0.33% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 48 | 0.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 323` |
| 49 | 0.32% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 50 | 0.32% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 51 | 0.32% | `net.minecraft.world.entity.EntitySelector.lambda$pushableBy$0(Entity, Team, Team$CollisionRule, Entity) line: 52` |
| 52 | 0.32% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 53 | 0.30% | `net.minecraft.world.entity.Entity.baseTick() line: 914` |
| 54 | 0.29% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 55 | 0.29% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 56 | 0.28% | `java.util.concurrent.atomic.AtomicInteger.set(int) line: 101` |
| 57 | 0.28% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 58 | 0.28% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 59 | 0.28% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 60 | 0.26% | `org.spigotmc.WatchdogThread.run() line: 72` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 22.94% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 3.70% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 3.27% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 4 | 3.21% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 3.09% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 6 | 1.77% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 7 | 1.59% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 8 | 1.57% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 9 | 1.38% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 10 | 1.27% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 11 | 1.08% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3803` |
| 12 | 1.05% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 125` |
| 13 | 1.00% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 14 | 0.99% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 15 | 0.97% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 16 | 0.95% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 17 | 0.89% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 18 | 0.80% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 19 | 0.77% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |
| 20 | 0.75% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 21 | 0.65% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 22 | 0.57% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1282` |
| 23 | 0.53% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 24 | 0.53% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 25 | 0.51% | `net.minecraft.world.entity.Entity.applyGravity() line: 2091` |
| 26 | 0.50% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 27 | 0.50% | `net.minecraft.world.level.chunk.PalettedContainer.readPalette(PalettedContainer$Data, int) line: 79` |
| 28 | 0.49% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 29 | 0.48% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 30 | 0.48% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |

GC pauses: 0, total 0 ms

