# JVM hotspots (JFR ExecutionSample)

Samples: 4433

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.74% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 4.71% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10331` |
| 3 | 4.08% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9082` |
| 4 | 3.36% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean) line: 6200` |
| 5 | 3.23% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1024` |
| 6 | 2.91% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.23% | `dev.planeta9091.centities.CeHooks.ubsEmpty(Entity) line: 14965` |
| 8 | 2.19% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8843` |
| 9 | 1.62% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 10 | 1.49% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 11 | 1.24% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 12 | 1.22% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1025` |
| 13 | 1.11% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14739` |
| 14 | 1.06% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1043` |
| 15 | 0.97% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6600` |
| 16 | 0.97% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8817` |
| 17 | 0.88% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 18 | 0.77% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 19 | 0.72% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 20 | 0.65% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1034` |
| 21 | 0.63% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 22 | 0.63% | `dev.planeta9091.centities.CeHooks.itemDispatchWalk(ItemEntity, int) line: 6154` |
| 23 | 0.61% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 24 | 0.56% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9263` |
| 25 | 0.54% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 161` |
| 26 | 0.54% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1022` |
| 27 | 0.45% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 28 | 0.41% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10344` |
| 29 | 0.41% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3159` |
| 30 | 0.38% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 31 | 0.36% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 32 | 0.36% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1036` |
| 33 | 0.32% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 34 | 0.32% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3163` |
| 35 | 0.29% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 36 | 0.29% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6609` |
| 37 | 0.27% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 38 | 0.27% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 39 | 0.27% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9266` |
| 40 | 0.27% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8739` |
| 41 | 0.27% | `dev.planeta9091.centities.CeHooks.stackFast(ItemEntity) line: 1167` |
| 42 | 0.25% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 43 | 0.25% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 44 | 0.25% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 45 | 0.25% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3168` |
| 46 | 0.25% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8567` |
| 47 | 0.25% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 15884` |
| 48 | 0.23% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 49 | 0.23% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 50 | 0.23% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupBlock() line: 824` |
| 51 | 0.23% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 52 | 0.23% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.remove(Object) line: 430` |
| 53 | 0.23% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6613` |
| 54 | 0.20% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 55 | 0.20% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 56 | 0.20% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5173` |
| 57 | 0.20% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean) line: 6280` |
| 58 | 0.20% | `org.bukkit.craftbukkit.entity.CraftEntity.isValid() line: 510` |
| 59 | 0.18% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1410` |
| 60 | 0.18% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 10.53% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 7.69% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10331` |
| 3 | 6.41% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 4.74% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 4.24% | `dev.planeta9091.centities.CeHooks.itemDispatchWalk(ItemEntity, int) line: 6154` |
| 6 | 2.41% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1038` |
| 7 | 2.23% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 15940` |
| 8 | 2.08% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 9 | 1.67% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean) line: 6335` |
| 10 | 1.62% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 11 | 1.44% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 12 | 1.17% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14731` |
| 13 | 1.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8817` |
| 14 | 1.13% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 15 | 0.86% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 16 | 0.72% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 17 | 0.72% | `dev.planeta9091.centities.CeHooks.syncVelRead(Entity) line: 15534` |
| 18 | 0.68% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10344` |
| 19 | 0.61% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 20 | 0.61% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8821` |
| 21 | 0.50% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 22 | 0.50% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 23 | 0.47% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8934` |
| 24 | 0.45% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 25 | 0.45% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 26 | 0.43% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1043` |
| 27 | 0.43% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 6910` |
| 28 | 0.38% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 29 | 0.38% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 168` |
| 30 | 0.38% | `dev.planeta9091.centities.CeHooks.fullStackQuiet(ItemEntity) line: 965` |

GC pauses: 0, total 0 ms

