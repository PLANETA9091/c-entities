# JVM hotspots (JFR ExecutionSample)

Samples: 17070

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.92% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8219` |
| 2 | 5.13% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7466` |
| 3 | 4.37% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13025` |
| 4 | 3.35% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 5 | 3.08% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 2.81% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 7 | 2.72% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 8 | 1.65% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 4986` |
| 9 | 1.38% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 1.37% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 11 | 1.15% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 12 | 1.12% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12924` |
| 13 | 1.05% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5191` |
| 14 | 1.03% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 15 | 0.93% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 16 | 0.83% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 17 | 0.80% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 18 | 0.79% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 19 | 0.76% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 20 | 0.76% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 21 | 0.74% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 22 | 0.70% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5165` |
| 23 | 0.69% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 24 | 0.61% | `dev.planeta9091.centities.CeHooks.itemSkip(ItemEntity) line: 6664` |
| 25 | 0.60% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 26 | 0.59% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 27 | 0.57% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 9815` |
| 28 | 0.55% | `dev.planeta9091.centities.CeHooks.getTicksFrozen$(Entity) line: 13603` |
| 29 | 0.54% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5354` |
| 30 | 0.53% | `net.minecraft.world.entity.Entity.computeSpeed() line: 966` |
| 31 | 0.52% | `net.minecraft.world.entity.EntityFluidInteraction.getFluidHeight(TagKey) line: 176` |
| 32 | 0.50% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 12739` |
| 33 | 0.50% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7406` |
| 34 | 0.49% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1916` |
| 35 | 0.49% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 36 | 0.47% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12878` |
| 37 | 0.47% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 38 | 0.46% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 243` |
| 39 | 0.46% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 40 | 0.45% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 41 | 0.44% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 42 | 0.43% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 43 | 0.42% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 44 | 0.42% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5116` |
| 45 | 0.41% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6942` |
| 46 | 0.40% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 47 | 0.39% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 48 | 0.38% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 49 | 0.37% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7200` |
| 50 | 0.35% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 51 | 0.33% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 52 | 0.33% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 53 | 0.32% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 54 | 0.32% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5213` |
| 55 | 0.30% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 56 | 0.30% | `dev.planeta9091.centities.CeHooks.probeDeltaCommon(Entity, double, double, double) line: 1707` |
| 57 | 0.29% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 5817` |
| 58 | 0.28% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 59 | 0.26% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5219` |
| 60 | 0.25% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5318` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 8.51% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 7.64% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 6.42% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 4.63% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8229` |
| 5 | 4.48% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 6 | 3.13% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 7 | 2.71% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5735` |
| 8 | 1.97% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13129` |
| 9 | 1.72% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5099` |
| 10 | 1.38% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 11 | 1.36% | `net.minecraft.world.entity.Entity.baseTick() line: 921` |
| 12 | 1.06% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 13 | 0.97% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 14 | 0.93% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 15 | 0.90% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8249` |
| 16 | 0.88% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 17 | 0.87% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11061` |
| 18 | 0.84% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1408` |
| 19 | 0.83% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5128` |
| 20 | 0.75% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 21 | 0.75% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11128` |
| 22 | 0.66% | `net.minecraft.world.entity.item.ItemEntity.tick()` |
| 23 | 0.63% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 24 | 0.62% | `net.minecraft.world.entity.Entity.getTicksFrozen()` |
| 25 | 0.62% | `dev.planeta9091.centities.CeHooks$NmMap.get(int) line: 9822` |
| 26 | 0.59% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 27 | 0.58% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 28 | 0.57% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 29 | 0.57% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 30 | 0.56% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13116` |

GC pauses: 0, total 0 ms

