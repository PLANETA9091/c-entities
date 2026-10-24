# JVM hotspots (JFR ExecutionSample)

Samples: 5814

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 15.19% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 10.10% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 3 | 6.42% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 4 | 5.16% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 5 | 4.83% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 6 | 3.80% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 7 | 3.08% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 1948` |
| 8 | 2.43% | `net.minecraft.world.entity.Entity.isRemoved() line: 5328` |
| 9 | 1.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2386` |
| 10 | 1.03% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 11 | 1.00% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 12 | 0.74% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 13 | 0.72% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 14 | 0.67% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 15 | 0.65% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 16 | 0.62% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1851` |
| 17 | 0.62% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |
| 18 | 0.60% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 19 | 0.48% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 20 | 0.41% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1523` |
| 21 | 0.40% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5075` |
| 22 | 0.40% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1837` |
| 23 | 0.36% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 24 | 0.34% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 25 | 0.34% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1413` |
| 26 | 0.34% | `dev.planeta9091.centities.CeHooks.cellOfBox(AABB) line: 806` |
| 27 | 0.33% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity) line: 1902` |
| 28 | 0.31% | `dev.planeta9091.centities.CeHooks.arenaRef(long) line: 2479` |
| 29 | 0.29% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 996` |
| 30 | 0.29% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 31 | 0.29% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 32 | 0.28% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2413` |
| 33 | 0.26% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 34 | 0.26% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 35 | 0.24% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 36 | 0.24% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 37 | 0.24% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 38 | 0.24% | `net.minecraft.world.entity.Entity.moonrise$isHardColliding() line: 418` |
| 39 | 0.24% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1844` |
| 40 | 0.24% | `dev.planeta9091.centities.CeHooks.isMergableFast(ItemEntity) line: 240` |
| 41 | 0.21% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 42 | 0.19% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 43 | 0.19% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1339` |
| 44 | 0.19% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 152` |
| 45 | 0.17% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1156` |
| 46 | 0.17% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(int, int) line: 929` |
| 47 | 0.17% | `net.minecraft.world.entity.Entity.getVehicle() line: 4933` |
| 48 | 0.17% | `net.minecraft.network.syncher.SynchedEntityData.packDirty() line: 82` |
| 49 | 0.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2358` |
| 50 | 0.15% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 51 | 0.15% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 52 | 0.15% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 53 | 0.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2416` |
| 54 | 0.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2360` |
| 55 | 0.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2361` |
| 56 | 0.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2357` |
| 57 | 0.14% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 58 | 0.14% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 59 | 0.14% | `dev.planeta9091.centities.CeHooks.stackEmptyDispatch(ItemEntity) line: 2031` |
| 60 | 0.14% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1863` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 15.19% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 2 | 11.18% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 3 | 10.15% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 4 | 6.00% | `dev.planeta9091.centities.CeHooks.arenaRef(long) line: 2480` |
| 5 | 4.27% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2413` |
| 6 | 3.92% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 7 | 3.34% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1872` |
| 8 | 2.43% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 9 | 1.98% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity)` |
| 10 | 1.86% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 11 | 1.14% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 12 | 1.03% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 13 | 0.76% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 14 | 0.74% | `java.util.Collections$UnmodifiableCollection.iterator() line: 1076` |
| 15 | 0.72% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 16 | 0.67% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 17 | 0.65% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 18 | 0.64% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1871` |
| 19 | 0.45% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 20 | 0.41% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 680` |
| 21 | 0.40% | `java.math.BigInteger.modPow(BigInteger, BigInteger) line: 2924` |
| 22 | 0.36% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 23 | 0.36% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1343` |
| 24 | 0.36% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1025` |
| 25 | 0.36% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2379` |
| 26 | 0.34% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 27 | 0.34% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1352` |
| 28 | 0.33% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 29 | 0.29% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1852` |
| 30 | 0.29% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 848` |

GC pauses: 0, total 0 ms

