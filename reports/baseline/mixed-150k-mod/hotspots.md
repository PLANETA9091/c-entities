# JVM hotspots (JFR ExecutionSample)

Samples: 19189

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.89% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 2 | 6.38% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 5.69% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3907` |
| 4 | 4.23% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 3.96% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 6 | 3.37% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 7 | 2.03% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 8 | 1.70% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.62% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2403` |
| 10 | 1.53% | `dev.planeta9091.centities.CeHooks.arenaRef(long) line: 2479` |
| 11 | 1.48% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3868` |
| 12 | 1.10% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 13 | 1.03% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 1948` |
| 14 | 1.02% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 995` |
| 15 | 0.98% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 16 | 0.96% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 17 | 0.96% | `net.minecraft.world.entity.Entity.isRemoved() line: 5328` |
| 18 | 0.95% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2386` |
| 19 | 0.92% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 20 | 0.88% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 21 | 0.70% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 22 | 0.68% | `java.util.EnumMap.get(Object) line: 248` |
| 23 | 0.59% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 24 | 0.57% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1876` |
| 25 | 0.54% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 26 | 0.53% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 27 | 0.52% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 28 | 0.51% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 29 | 0.51% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 30 | 0.51% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 31 | 0.50% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1837` |
| 32 | 0.50% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 33 | 0.48% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 34 | 0.48% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1844` |
| 35 | 0.44% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 36 | 0.43% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 37 | 0.43% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity) line: 599` |
| 38 | 0.42% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 39 | 0.36% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1413` |
| 40 | 0.35% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 41 | 0.34% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |
| 42 | 0.33% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 43 | 0.32% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2195` |
| 44 | 0.31% | `net.minecraft.world.entity.SteppedInterpolationTracker.updateTracking(Vec3) line: 40` |
| 45 | 0.31% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 46 | 0.30% | `net.minecraft.world.entity.Entity.baseTick() line: 913` |
| 47 | 0.29% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 48 | 0.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 49 | 0.29% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1428` |
| 50 | 0.29% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 51 | 0.28% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1523` |
| 52 | 0.28% | `net.minecraft.world.entity.LivingEntity$SwingState.tick() line: 5015` |
| 53 | 0.26% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 54 | 0.25% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2413` |
| 55 | 0.25% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 56 | 0.24% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5075` |
| 57 | 0.23% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1851` |
| 58 | 0.22% | `java.util.ImmutableCollections.listCopy(Collection) line: 188` |
| 59 | 0.22% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 60 | 0.22% | `net.minecraft.network.syncher.SynchedEntityData.packDirty() line: 82` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.93% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 2 | 6.38% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 3 | 5.92% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 5.72% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 5 | 5.26% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 6 | 3.92% | `dev.planeta9091.centities.CeHooks.arenaRef(long) line: 2480` |
| 7 | 2.55% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 8 | 2.33% | `net.minecraft.world.entity.LivingEntity.tick() line: 3432` |
| 9 | 2.26% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity)` |
| 10 | 2.02% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2413` |
| 11 | 1.67% | `net.minecraft.world.entity.Entity.setOldPos() line: 2395` |
| 12 | 1.54% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2379` |
| 13 | 1.43% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 14 | 1.39% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 969` |
| 15 | 1.34% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 16 | 1.19% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 17 | 1.10% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1872` |
| 18 | 1.05% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 19 | 0.98% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 20 | 0.97% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 21 | 0.96% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 22 | 0.93% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 23 | 0.91% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 24 | 0.87% | `net.minecraft.world.entity.decoration.ArmorStand.tick() line: 567` |
| 25 | 0.83% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2212` |
| 26 | 0.80% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 27 | 0.75% | `net.minecraft.world.entity.Entity.tick() line: 872` |
| 28 | 0.65% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 454` |
| 29 | 0.58% | `net.minecraft.world.entity.Entity.setDeltaMovement(double, double, double) line: 5086` |
| 30 | 0.57% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |

GC pauses: 0, total 0 ms

