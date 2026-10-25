# JVM hotspots (JFR ExecutionSample)

Samples: 7263

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 12.61% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3190` |
| 2 | 9.11% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 6.04% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3053` |
| 4 | 5.25% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 175` |
| 5 | 4.97% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 6 | 4.39% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 3818` |
| 7 | 3.80% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 8 | 3.61% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 9 | 2.59% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2079` |
| 10 | 1.23% | `net.minecraft.world.entity.Entity.isRemoved() line: 5328` |
| 11 | 1.01% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 12 | 0.92% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2588` |
| 13 | 0.88% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 14 | 0.73% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 15 | 0.73% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1982` |
| 16 | 0.72% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 17 | 0.66% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |
| 18 | 0.65% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 19 | 0.63% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 20 | 0.56% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 21 | 0.55% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 22 | 0.51% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5075` |
| 23 | 0.48% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 996` |
| 24 | 0.45% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 25 | 0.43% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 26 | 0.41% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1413` |
| 27 | 0.39% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2624` |
| 28 | 0.33% | `net.minecraft.world.entity.Entity.getVehicle() line: 4933` |
| 29 | 0.32% | `dev.planeta9091.centities.CeHooks.arenaRef(long) line: 2691` |
| 30 | 0.30% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 31 | 0.28% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 32 | 0.28% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1975` |
| 33 | 0.28% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3080` |
| 34 | 0.26% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 35 | 0.26% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1523` |
| 36 | 0.25% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2029` |
| 37 | 0.21% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 38 | 0.21% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 39 | 0.21% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1484` |
| 40 | 0.21% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 41 | 0.21% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3086` |
| 42 | 0.21% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1397` |
| 43 | 0.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2561` |
| 44 | 0.19% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 45 | 0.18% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 46 | 0.18% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 47 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2482` |
| 48 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2581` |
| 49 | 0.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2481` |
| 50 | 0.17% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 246` |
| 51 | 0.17% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3088` |
| 52 | 0.15% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$hasPlayers() line: 1347` |
| 53 | 0.15% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2030` |
| 54 | 0.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2587` |
| 55 | 0.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2480` |
| 56 | 0.15% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 2033` |
| 57 | 0.14% | `net.minecraft.server.level.WorldGenRegion.lambda$new$2(ChunkAccess, ChunkStep, GenerationChunkHolder, int, int) line: 124` |
| 58 | 0.14% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 59 | 0.14% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1394` |
| 60 | 0.14% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1994` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 12.74% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 2 | 9.11% | `dev.planeta9091.centities.CeHooks$RefField.getObj(Object) line: 2296` |
| 3 | 6.86% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 4 | 6.39% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 5 | 4.76% | `dev.planeta9091.centities.CeHooks.arenaRef(long) line: 2692` |
| 6 | 4.47% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk)` |
| 7 | 4.35% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2624` |
| 8 | 3.61% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 9 | 3.61% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 10 | 2.64% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2003` |
| 11 | 1.71% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity)` |
| 12 | 1.47% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 13 | 1.42% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 14 | 1.23% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 15 | 1.02% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 16 | 0.96% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1047` |
| 17 | 0.94% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 18 | 0.89% | `net.minecraft.server.level.ChunkMap.tick() line: 1122` |
| 19 | 0.65% | `java.util.Collections$UnmodifiableCollection.iterator() line: 1076` |
| 20 | 0.63% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 21 | 0.61% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2002` |
| 22 | 0.61% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 3819` |
| 23 | 0.59% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3190` |
| 24 | 0.48% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1983` |
| 25 | 0.48% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 26 | 0.33% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 901` |
| 27 | 0.32% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2581` |
| 28 | 0.30% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 29 | 0.30% | `net.minecraft.world.entity.item.ItemEntity.isMergable() line: 274` |
| 30 | 0.26% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3063` |

GC pauses: 0, total 0 ms

