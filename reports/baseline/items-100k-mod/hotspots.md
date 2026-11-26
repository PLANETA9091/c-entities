# JVM hotspots (JFR ExecutionSample)

Samples: 3374

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.39% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.80% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9530` |
| 3 | 4.39% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1029` |
| 4 | 4.33% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10831` |
| 5 | 3.50% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9276` |
| 6 | 3.29% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 1.96% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1030` |
| 8 | 1.81% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6772` |
| 9 | 1.30% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 10 | 1.10% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 11 | 1.04% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 12 | 1.04% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 13 | 1.04% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1039` |
| 14 | 1.01% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9248` |
| 15 | 1.01% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15285` |
| 16 | 0.92% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6781` |
| 17 | 0.89% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 18 | 0.89% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 16818` |
| 19 | 0.65% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 20 | 0.65% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean, int) line: 6320` |
| 21 | 0.65% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1041` |
| 22 | 0.62% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 161` |
| 23 | 0.56% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1027` |
| 24 | 0.53% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3242` |
| 25 | 0.53% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean) line: 6275` |
| 26 | 0.53% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 27 | 0.50% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean, int) line: 6394` |
| 28 | 0.50% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3238` |
| 29 | 0.44% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9711` |
| 30 | 0.44% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3237` |
| 31 | 0.44% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1048` |
| 32 | 0.41% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 33 | 0.41% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9129` |
| 34 | 0.41% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 16828` |
| 35 | 0.39% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.remove(Object) line: 430` |
| 36 | 0.39% | `java.util.ArrayList$Itr.next() line: 1058` |
| 37 | 0.36% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 38 | 0.36% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 39 | 0.36% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 40 | 0.36% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8965` |
| 41 | 0.36% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10844` |
| 42 | 0.36% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean, int) line: 6300` |
| 43 | 0.33% | `dev.planeta9091.centities.CeHooks.stackFast(ItemEntity) line: 1223` |
| 44 | 0.30% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 45 | 0.30% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 16832` |
| 46 | 0.27% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 47 | 0.27% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 48 | 0.27% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean, int) line: 6403` |
| 49 | 0.27% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9098` |
| 50 | 0.27% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 16820` |
| 51 | 0.24% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 52 | 0.24% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 53 | 0.24% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3247` |
| 54 | 0.24% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3241` |
| 55 | 0.24% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 179` |
| 56 | 0.24% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1044` |
| 57 | 0.24% | `dev.planeta9091.centities.CeHooks.stackFast(ItemEntity) line: 1224` |
| 58 | 0.21% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 59 | 0.21% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 60 | 0.21% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 12.27% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 9.63% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10831` |
| 3 | 5.39% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 5.13% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 2.99% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean, int) line: 6507` |
| 6 | 2.61% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 7 | 2.61% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1043` |
| 8 | 2.22% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean) line: 6275` |
| 9 | 1.63% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9248` |
| 10 | 1.51% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 11 | 1.36% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 12 | 1.16% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15277` |
| 13 | 1.07% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 14 | 1.04% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 15 | 1.04% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9252` |
| 16 | 0.89% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 17 | 0.59% | `net.minecraft.world.level.levelgen.NoiseBasedChunkGenerator.buildSurface(ChunkAccess, NoiseChunk, RandomState, BiomeManager, Set, MaterialRule, ServerLevel) line: 227` |
| 18 | 0.56% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 19 | 0.53% | `dev.planeta9091.centities.CeHooks.itemDispatchWalk(ItemEntity, int) line: 6261` |
| 20 | 0.53% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 168` |
| 21 | 0.53% | `dev.planeta9091.centities.CeHooks.fullStackQuiet(ItemEntity) line: 965` |
| 22 | 0.47% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 23 | 0.41% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 24 | 0.41% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 25 | 0.41% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 26 | 0.39% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 27 | 0.39% | `net.minecraft.server.level.ChunkMap$TrackedEntity.removePlayer(ServerPlayer) line: 1400` |
| 28 | 0.39% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 29 | 0.39% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 163` |
| 30 | 0.39% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1048` |

GC pauses: 0, total 0 ms

