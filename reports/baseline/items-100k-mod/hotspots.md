# JVM hotspots (JFR ExecutionSample)

Samples: 4333

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 7.87% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 7.22% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9373` |
| 3 | 6.37% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8530` |
| 4 | 5.19% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 2.91% | `dev.planeta9091.centities.CeHooks.quietPass() line: 7567` |
| 6 | 2.49% | `dev.planeta9091.centities.CeHooks.quietPass() line: 7571` |
| 7 | 1.71% | `dev.planeta9091.centities.CeHooks.stackFast(ItemEntity) line: 987` |
| 8 | 1.55% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 9 | 1.32% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13651` |
| 10 | 1.20% | `dev.planeta9091.centities.CeHooks.quietPass() line: 7573` |
| 11 | 1.18% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5860` |
| 12 | 1.11% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 13 | 1.08% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5864` |
| 14 | 0.92% | `dev.planeta9091.centities.CeHooks.quietPass() line: 7585` |
| 15 | 0.90% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5866` |
| 16 | 0.83% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 17 | 0.83% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14355` |
| 18 | 0.81% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 19 | 0.81% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8312` |
| 20 | 0.60% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 21 | 0.60% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14628` |
| 22 | 0.60% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 23 | 0.58% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 161` |
| 24 | 0.55% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8316` |
| 25 | 0.46% | `dev.planeta9091.centities.CeHooks.quietPass() line: 7558` |
| 26 | 0.44% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 27 | 0.42% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2940` |
| 28 | 0.39% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14642` |
| 29 | 0.37% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 30 | 0.37% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 31 | 0.37% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 32 | 0.37% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14846` |
| 33 | 0.32% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6138` |
| 34 | 0.32% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.remove(Object) line: 430` |
| 35 | 0.32% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5818` |
| 36 | 0.30% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8294` |
| 37 | 0.30% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14630` |
| 38 | 0.28% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 39 | 0.28% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 40 | 0.28% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8047` |
| 41 | 0.28% | `java.util.ArrayList$Itr.next() line: 1058` |
| 42 | 0.25% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14638` |
| 43 | 0.25% | `dev.planeta9091.centities.CeHooks.quietPass() line: 7586` |
| 44 | 0.25% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 45 | 0.23% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 46 | 0.23% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 47 | 0.23% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 48 | 0.23% | `java.math.BigInteger.implMultiplyToLen(int[], int, int[], int, int[]) line: 1863` |
| 49 | 0.23% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2936` |
| 50 | 0.23% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2935` |
| 51 | 0.21% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 52 | 0.21% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 53 | 0.21% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 54 | 0.21% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 55 | 0.21% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 179` |
| 56 | 0.18% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 57 | 0.18% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2945` |
| 58 | 0.16% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 59 | 0.16% | `java.util.Collection.stream() line: 748` |
| 60 | 0.16% | `sun.misc.Unsafe.beforeMemoryAccess() line: 1777` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 9.49% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 9.00% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7663` |
| 3 | 7.45% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 5.19% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 4.11% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9383` |
| 6 | 3.88% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5766` |
| 7 | 2.61% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 8 | 1.94% | `dev.planeta9091.centities.CeHooks.stackFast(ItemEntity) line: 982` |
| 9 | 1.89% | `dev.planeta9091.centities.CeHooks.fullStackQuiet(ItemEntity) line: 931` |
| 10 | 1.62% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 11 | 1.38% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 12 | 1.18% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13643` |
| 13 | 1.06% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 14 | 0.97% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14646` |
| 15 | 0.97% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8294` |
| 16 | 0.95% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8298` |
| 17 | 0.83% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 18 | 0.74% | `dev.planeta9091.centities.CeHooks.quietPass() line: 7588` |
| 19 | 0.60% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 20 | 0.60% | `dev.planeta9091.centities.CeHooks.quietPass() line: 7593` |
| 21 | 0.44% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 22 | 0.44% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5875` |
| 23 | 0.42% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 168` |
| 24 | 0.39% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 25 | 0.39% | `dev.planeta9091.centities.CeHooks$RefField.getInt(Object) line: 7202` |
| 26 | 0.37% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 27 | 0.37% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 28 | 0.35% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 29 | 0.35% | `dev.planeta9091.centities.CeHooks.stackFast(ItemEntity) line: 980` |
| 30 | 0.32% | `net.minecraft.server.level.ChunkMap$TrackedEntity.removePlayer(ServerPlayer) line: 1400` |

GC pauses: 0, total 0 ms

