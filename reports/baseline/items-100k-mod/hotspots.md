# JVM hotspots (JFR ExecutionSample)

Samples: 6017

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 15.09% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5033` |
| 2 | 11.87% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8788` |
| 3 | 5.85% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 4 | 4.20% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 3.84% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8735` |
| 6 | 2.96% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3734` |
| 7 | 2.78% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 389` |
| 8 | 2.19% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 9 | 1.36% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3607` |
| 10 | 1.20% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 11 | 1.03% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5097` |
| 12 | 0.96% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2661` |
| 13 | 0.93% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 14 | 0.88% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 15 | 0.85% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 16 | 0.81% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 17 | 0.75% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 18 | 0.61% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 19 | 0.58% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8749` |
| 20 | 0.53% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5826` |
| 21 | 0.53% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5134` |
| 22 | 0.45% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5824` |
| 23 | 0.45% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3687` |
| 24 | 0.38% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 8429` |
| 25 | 0.38% | `java.util.HashMap.getNode(Object) line: 577` |
| 26 | 0.32% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 27 | 0.30% | `java.lang.Integer.valueOf(int) line: 1006` |
| 28 | 0.30% | `net.minecraft.world.entity.Entity.isOnPortalCooldown() line: 997` |
| 29 | 0.30% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5081` |
| 30 | 0.28% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 31 | 0.28% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 32 | 0.28% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2673` |
| 33 | 0.28% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4916` |
| 34 | 0.27% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 35 | 0.27% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 36 | 0.23% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 37 | 0.22% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 38 | 0.22% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 388` |
| 39 | 0.22% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5142` |
| 40 | 0.22% | `dev.planeta9091.centities.CeHooks.probeDeltaCommon(Entity, double, double, double) line: 1176` |
| 41 | 0.22% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8739` |
| 42 | 0.22% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 43 | 0.22% | `dev.planeta9091.centities.CeHooks.itemCheckActive(ItemEntity, boolean) line: 3689` |
| 44 | 0.20% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 45 | 0.20% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 46 | 0.18% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 47 | 0.18% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 48 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5096` |
| 49 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4915` |
| 50 | 0.17% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 51 | 0.17% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 52 | 0.17% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 53 | 0.17% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1475` |
| 54 | 0.17% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3600` |
| 55 | 0.17% | `net.minecraft.world.item.ItemInstance.getMaxStackSize() line: 15` |
| 56 | 0.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5003` |
| 57 | 0.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5102` |
| 58 | 0.15% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 59 | 0.15% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3509` |
| 60 | 0.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5044` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 19.63% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 12.18% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 5.87% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 4 | 4.94% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8784` |
| 5 | 3.66% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8776` |
| 6 | 3.26% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3640` |
| 7 | 2.78% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5826` |
| 8 | 2.33% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3509` |
| 9 | 2.11% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2657` |
| 10 | 1.91% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2043` |
| 11 | 1.86% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 12 | 1.21% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 13 | 1.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5134` |
| 14 | 1.08% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 15 | 0.95% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3638` |
| 16 | 0.91% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3607` |
| 17 | 0.75% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 18 | 0.73% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2674` |
| 19 | 0.68% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2228` |
| 20 | 0.66% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 21 | 0.61% | `java.util.HashMap.get(Object) line: 565` |
| 22 | 0.47% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 23 | 0.37% | `net.minecraft.util.StaticCache2D.map(StaticCache2D$MappingFunction) line: 36` |
| 24 | 0.33% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 25 | 0.33% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 26 | 0.32% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8742` |
| 27 | 0.30% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 28 | 0.30% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3608` |
| 29 | 0.28% | `com.sun.crypto.provider.CounterMode.crypt(byte[], int, int, byte[], int) line: 195` |
| 30 | 0.28% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 389` |

GC pauses: 0, total 0 ms

