# JVM hotspots (JFR ExecutionSample)

Samples: 5769

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 12.05% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9942` |
| 2 | 5.53% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 5.32% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8799` |
| 4 | 4.65% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 4.00% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1018` |
| 6 | 1.66% | `dev.planeta9091.centities.CeHooks.stackFast(ItemEntity) line: 1067` |
| 7 | 1.63% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1019` |
| 8 | 1.51% | `dev.planeta9091.centities.CeHooks.ubsEmpty(Entity) line: 14481` |
| 9 | 1.49% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14279` |
| 10 | 1.25% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6116` |
| 11 | 1.25% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 12 | 1.13% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 13 | 1.11% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8585` |
| 14 | 0.99% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 15 | 0.94% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 16 | 0.90% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6070` |
| 17 | 0.87% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 18 | 0.75% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 19 | 0.66% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 20 | 0.55% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3041` |
| 21 | 0.47% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 22 | 0.45% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9934` |
| 23 | 0.45% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6390` |
| 24 | 0.42% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6112` |
| 25 | 0.42% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1016` |
| 26 | 0.38% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8581` |
| 27 | 0.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 161` |
| 28 | 0.36% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 15388` |
| 29 | 0.33% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8316` |
| 30 | 0.31% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3036` |
| 31 | 0.31% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6006` |
| 32 | 0.29% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 33 | 0.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 34 | 0.29% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 35 | 0.28% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 36 | 0.28% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 37 | 0.28% | `dev.planeta9091.centities.CeHooks.dispatchQuietSkip(ItemEntity, int) line: 1002` |
| 38 | 0.24% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 39 | 0.24% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 40 | 0.23% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 41 | 0.23% | `net.minecraft.util.StaticCache2D.get(int, int) line: 49` |
| 42 | 0.23% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9943` |
| 43 | 0.23% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3046` |
| 44 | 0.23% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6079` |
| 45 | 0.21% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 46 | 0.21% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3037` |
| 47 | 0.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8563` |
| 48 | 0.21% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1028` |
| 49 | 0.21% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1017` |
| 50 | 0.21% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1015` |
| 51 | 0.19% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 52 | 0.19% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 53 | 0.19% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(int, int) line: 929` |
| 54 | 0.19% | `java.lang.Class.cast(Object) line: 3491` |
| 55 | 0.19% | `java.lang.Double.isFinite(double) line: 1023` |
| 56 | 0.17% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 57 | 0.17% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 58 | 0.17% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 59 | 0.17% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 60 | 0.17% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 13.16% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 2 | 8.68% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 3 | 7.09% | `dev.planeta9091.centities.CeHooks.dispatchQuietSkip(ItemEntity, int) line: 1002` |
| 4 | 5.53% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 3.76% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9943` |
| 6 | 1.92% | `dev.planeta9091.centities.CeHooks.fullStackQuiet(ItemEntity) line: 965` |
| 7 | 1.82% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 8 | 1.68% | `dev.planeta9091.centities.CeHooks.stackFast(ItemEntity) line: 1062` |
| 9 | 1.66% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6018` |
| 10 | 1.56% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 11 | 1.51% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 15442` |
| 12 | 1.25% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 13 | 1.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8563` |
| 14 | 1.13% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8567` |
| 15 | 0.99% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 16 | 0.85% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14271` |
| 17 | 0.81% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1034` |
| 18 | 0.75% | `dev.planeta9091.centities.CeHooks.syncVelRead(Entity) line: 15048` |
| 19 | 0.71% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 20 | 0.69% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6047` |
| 21 | 0.64% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 22 | 0.49% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6127` |
| 23 | 0.47% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1028` |
| 24 | 0.45% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 25 | 0.43% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8651` |
| 26 | 0.35% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 27 | 0.31% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 28 | 0.31% | `net.minecraft.util.StaticCache2D.lambda$map$0(StaticCache2D$MappingFunction, int, int) line: 36` |
| 29 | 0.29% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 30 | 0.29% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1023` |

GC pauses: 0, total 0 ms

