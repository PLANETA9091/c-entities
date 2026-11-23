# JVM hotspots (JFR ExecutionSample)

Samples: 80743

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 19.76% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.71% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3913` |
| 3 | 2.78% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 2.19% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 1.94% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 6 | 1.63% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 7 | 1.59% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 1.55% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 9 | 1.33% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1286` |
| 10 | 1.23% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.18% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 12 | 1.16% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 13 | 1.13% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 14 | 1.06% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 15 | 0.98% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 16 | 0.94% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 17 | 0.93% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 18 | 0.84% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 19 | 0.79% | `java.util.HashMap.getNode(Object) line: 586` |
| 20 | 0.73% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3874` |
| 21 | 0.69% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 22 | 0.68% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 23 | 0.67% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 24 | 0.63% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 25 | 0.63% | `java.util.EnumMap.get(Object) line: 248` |
| 26 | 0.60% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 27 | 0.56% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 28 | 0.55% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 29 | 0.54% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 30 | 0.53% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 31 | 0.52% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 32 | 0.51% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 33 | 0.51% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 34 | 0.48% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 35 | 0.46% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 36 | 0.45% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 37 | 0.45% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 638` |
| 38 | 0.44% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.<init>(Map, float) line: 124` |
| 39 | 0.41% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getHardCollidingEntities(Entity, AABB, List, Predicate) line: 323` |
| 40 | 0.41% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 41 | 0.39% | `net.minecraft.world.item.ItemStack.getMaxDamage() line: 582` |
| 42 | 0.39% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 43 | 0.39% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 44 | 0.39% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 45 | 0.38% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 46 | 0.37% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 47 | 0.36% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 48 | 0.35% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 49 | 0.34% | `net.minecraft.world.entity.LivingEntity.isCollidable(boolean) line: 4094` |
| 50 | 0.32% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 51 | 0.32% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 52 | 0.31% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 53 | 0.30% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 324` |
| 54 | 0.29% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 55 | 0.28% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 56 | 0.27% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 57 | 0.26% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 58 | 0.26% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 59 | 0.26% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 60 | 0.25% | `net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase.getShape(BlockGetter, BlockPos, CollisionContext) line: 867` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 19.76% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 3.71% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3803` |
| 3 | 3.44% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 4 | 3.41% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 2.78% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 894` |
| 6 | 2.78% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 7 | 1.88% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 8 | 1.52% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 9 | 1.36% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 10 | 1.21% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 11 | 1.16% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 12 | 1.15% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 13 | 1.14% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil.getCollisionsForBlocksOrWorldBorder(Level, Entity, AABB, List, List, int, BiPredicate) line: 1998` |
| 14 | 1.08% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 15 | 0.95% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 16 | 0.92% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 17 | 0.89% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 18 | 0.70% | `net.minecraft.world.level.Level.moonrise$getHardCollidingEntities(Entity, AABB, Predicate) line: 234` |
| 19 | 0.68% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |
| 20 | 0.68% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 21 | 0.63% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 22 | 0.63% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 23 | 0.62% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 24 | 0.60% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 25 | 0.59% | `java.util.HashMap.containsKey(Object) line: 603` |
| 26 | 0.57% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 27 | 0.57% | `java.util.ArrayList.toArray() line: 370` |
| 28 | 0.57% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 29 | 0.55% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 30 | 0.53% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |

GC pauses: 0, total 0 ms

