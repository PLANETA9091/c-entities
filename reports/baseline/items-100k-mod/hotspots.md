# JVM hotspots (JFR ExecutionSample)

Samples: 3820

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 7.93% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10137` |
| 2 | 5.10% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 5.05% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 5.00% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8994` |
| 5 | 3.85% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1018` |
| 6 | 2.64% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8755` |
| 7 | 1.88% | `dev.planeta9091.centities.CeHooks.ubsEmpty(Entity) line: 14688` |
| 8 | 1.49% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1016` |
| 9 | 1.39% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 10 | 1.31% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6512` |
| 11 | 1.23% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 12 | 1.18% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 13 | 1.13% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1017` |
| 14 | 1.10% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6183` |
| 15 | 1.07% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 16 | 1.02% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14474` |
| 17 | 1.02% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1026` |
| 18 | 0.92% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 19 | 0.84% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6521` |
| 20 | 0.81% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 21 | 0.65% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6192` |
| 22 | 0.65% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3154` |
| 23 | 0.63% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8729` |
| 24 | 0.58% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9175` |
| 25 | 0.55% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 26 | 0.52% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 27 | 0.52% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6119` |
| 28 | 0.52% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3149` |
| 29 | 0.50% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1035` |
| 30 | 0.47% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 31 | 0.45% | `dev.planeta9091.centities.CeHooks.dispatchQuietSkip(ItemEntity, int) line: 1002` |
| 32 | 0.39% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 33 | 0.39% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1028` |
| 34 | 0.37% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 35 | 0.34% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3150` |
| 36 | 0.31% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 37 | 0.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 161` |
| 38 | 0.31% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10127` |
| 39 | 0.31% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8611` |
| 40 | 0.31% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 41 | 0.31% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10138` |
| 42 | 0.29% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 43 | 0.29% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 44 | 0.29% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.remove(Object) line: 430` |
| 45 | 0.29% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 46 | 0.29% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 179` |
| 47 | 0.26% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 48 | 0.26% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 49 | 0.26% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 50 | 0.26% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 51 | 0.26% | `java.util.ArrayList$Itr.next() line: 1058` |
| 52 | 0.24% | `java.util.zip.ZipFile$Source.getEntryPos(String, boolean, ZipCoder) line: 1879` |
| 53 | 0.24% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3159` |
| 54 | 0.24% | `dev.planeta9091.centities.CeHooks.dispatchQuietSkip(ItemEntity, int) line: 988` |
| 55 | 0.21% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 56 | 0.21% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 874` |
| 57 | 0.21% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 58 | 0.21% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 59 | 0.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8479` |
| 60 | 0.21% | `java.lang.Double.isFinite(double) line: 1023` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 11.15% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 8.90% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 8.90% | `dev.planeta9091.centities.CeHooks.dispatchQuietSkip(ItemEntity, int) line: 1002` |
| 4 | 5.10% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 3.66% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6131` |
| 6 | 2.80% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10138` |
| 7 | 2.38% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6247` |
| 8 | 1.88% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 15661` |
| 9 | 1.49% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 10 | 1.23% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 11 | 1.23% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8729` |
| 12 | 1.15% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 13 | 1.13% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 14 | 1.02% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1041` |
| 15 | 0.94% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 16 | 0.92% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8733` |
| 17 | 0.89% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6160` |
| 18 | 0.79% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14466` |
| 19 | 0.71% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1035` |
| 20 | 0.58% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 168` |
| 21 | 0.55% | `dev.planeta9091.centities.CeHooks.syncVelRead(Entity) line: 15255` |
| 22 | 0.55% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1030` |
| 23 | 0.50% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 24 | 0.45% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 25 | 0.42% | `dev.planeta9091.centities.CeHooks.fullStackQuiet(ItemEntity) line: 965` |
| 26 | 0.39% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 27 | 0.37% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 28 | 0.37% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 29 | 0.37% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 6822` |
| 30 | 0.34% | `net.minecraft.nbt.ListTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 74` |

GC pauses: 0, total 0 ms

