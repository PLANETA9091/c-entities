# JVM hotspots (JFR ExecutionSample)

Samples: 4923

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 9.81% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 4.55% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 4.55% | `dev.planeta9091.centities.CeHooks.quietPass() line: 7612` |
| 4 | 4.12% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9414` |
| 5 | 2.88% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8571` |
| 6 | 2.46% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8357` |
| 7 | 2.17% | `dev.planeta9091.centities.CeHooks.stackFast(ItemEntity) line: 987` |
| 8 | 1.73% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 9 | 1.69% | `dev.planeta9091.centities.CeHooks.quietPass() line: 7614` |
| 10 | 1.50% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5905` |
| 11 | 1.40% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5901` |
| 12 | 1.38% | `dev.planeta9091.centities.CeHooks.quietPass() line: 7608` |
| 13 | 1.28% | `dev.planeta9091.centities.CeHooks.quietPass() line: 7599` |
| 14 | 1.08% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13692` |
| 15 | 1.04% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 16 | 0.93% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 17 | 0.81% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 18 | 0.81% | `dev.planeta9091.centities.CeHooks.quietPass() line: 7626` |
| 19 | 0.79% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 20 | 0.67% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 161` |
| 21 | 0.65% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14693` |
| 22 | 0.65% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5859` |
| 23 | 0.65% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 24 | 0.57% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9424` |
| 25 | 0.55% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14420` |
| 26 | 0.53% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8353` |
| 27 | 0.53% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8335` |
| 28 | 0.49% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2946` |
| 29 | 0.45% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 30 | 0.43% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14703` |
| 31 | 0.43% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2941` |
| 32 | 0.41% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 33 | 0.39% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5868` |
| 34 | 0.37% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8088` |
| 35 | 0.37% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5907` |
| 36 | 0.35% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 37 | 0.35% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2951` |
| 38 | 0.35% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 179` |
| 39 | 0.33% | `java.util.ArrayList$Itr.next() line: 1058` |
| 40 | 0.33% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5795` |
| 41 | 0.30% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 42 | 0.30% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2942` |
| 43 | 0.28% | `java.util.zip.ZipFile$Source.getEntryPos(String, boolean, ZipCoder) line: 1879` |
| 44 | 0.28% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 45 | 0.26% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 46 | 0.26% | `java.lang.Double.isFinite(double) line: 1023` |
| 47 | 0.26% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5711` |
| 48 | 0.24% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 49 | 0.24% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 50 | 0.24% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14707` |
| 51 | 0.22% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 52 | 0.22% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 53 | 0.22% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.remove(Object) line: 430` |
| 54 | 0.22% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8260` |
| 55 | 0.22% | `dev.planeta9091.centities.CeHooks.stackFast(ItemEntity) line: 984` |
| 56 | 0.20% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 57 | 0.20% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14911` |
| 58 | 0.20% | `dev.planeta9091.centities.CeHooks.quietPass() line: 7630` |
| 59 | 0.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8755` |
| 60 | 0.18% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 10.83% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7704` |
| 2 | 8.84% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 3 | 5.63% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5807` |
| 4 | 5.50% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9424` |
| 5 | 5.02% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 6 | 4.55% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 2.50% | `dev.planeta9091.centities.CeHooks.fullStackQuiet(ItemEntity) line: 931` |
| 8 | 1.91% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 9 | 1.87% | `dev.planeta9091.centities.CeHooks.stackFast(ItemEntity) line: 982` |
| 10 | 1.56% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 11 | 1.34% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8335` |
| 12 | 1.18% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 13 | 1.12% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13684` |
| 14 | 0.98% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 15 | 0.89% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8339` |
| 16 | 0.83% | `dev.planeta9091.centities.CeHooks.quietPass() line: 7634` |
| 17 | 0.81% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 18 | 0.79% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14711` |
| 19 | 0.79% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 20 | 0.71% | `dev.planeta9091.centities.CeHooks.quietPass() line: 7629` |
| 21 | 0.65% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8423` |
| 22 | 0.65% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 168` |
| 23 | 0.61% | `dev.planeta9091.centities.CeHooks$RefField.getInt(Object) line: 7243` |
| 24 | 0.43% | `dev.planeta9091.centities.CeHooks.quietPass() line: 7635` |
| 25 | 0.41% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5836` |
| 26 | 0.41% | `dev.planeta9091.centities.CeHooks.stackFast(ItemEntity) line: 980` |
| 27 | 0.39% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 28 | 0.37% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 29 | 0.37% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 30 | 0.35% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |

GC pauses: 0, total 0 ms

