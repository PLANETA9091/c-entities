# JVM hotspots (JFR ExecutionSample)

Samples: 2656

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.20% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15622` |
| 2 | 4.71% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 3 | 2.56% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 4 | 1.92% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 15902` |
| 5 | 1.77% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 6 | 1.77% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 1.73% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 8 | 1.69% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 9 | 1.54% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 10 | 1.54% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14742` |
| 11 | 1.47% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.39% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 13 | 1.24% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 14 | 1.17% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 15 | 1.09% | `sun.misc.Unsafe.getBoolean(Object, long) line: 252` |
| 16 | 0.94% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 17 | 0.83% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10286` |
| 18 | 0.79% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |
| 19 | 0.72% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 20 | 0.72% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 21 | 0.64% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 22 | 0.56% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 23 | 0.56% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 24 | 0.56% | `java.util.HashMap.getNode(Object) line: 586` |
| 25 | 0.56% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 26 | 0.53% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 27 | 0.49% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 28 | 0.49% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(long) line: 933` |
| 29 | 0.45% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 30 | 0.45% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.idFor(Object, PaletteResize) line: 135` |
| 31 | 0.45% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 179` |
| 32 | 0.45% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 33 | 0.45% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 34 | 0.45% | `sun.misc.Unsafe.getFloat(Object, long) line: 383` |
| 35 | 0.41% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 36 | 0.41% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2627` |
| 37 | 0.41% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15643` |
| 38 | 0.38% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 39 | 0.38% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 40 | 0.34% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 41 | 0.34% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 42 | 0.34% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 43 | 0.34% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 44 | 0.30% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 45 | 0.30% | `java.math.BigInteger.oddModPow(BigInteger, BigInteger) line: 3263` |
| 46 | 0.30% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 47 | 0.30% | `net.minecraft.core.SectionPos.blockToSectionCoord(int) line: 90` |
| 48 | 0.30% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5219` |
| 49 | 0.26% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 50 | 0.26% | `java.util.zip.ZipFile$Source.getEntryPos(String, boolean, ZipCoder) line: 1879` |
| 51 | 0.26% | `java.lang.Class.cast(Object) line: 3491` |
| 52 | 0.26% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2581` |
| 53 | 0.26% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13364` |
| 54 | 0.26% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 55 | 0.26% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$BasicEntityList.indexOf(Entity) line: 447` |
| 56 | 0.23% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 57 | 0.23% | `net.minecraft.world.level.levelgen.material.rule.ConditionRule.lambda$compile$0(ConditionEvaluator, RuleEvaluator, int, int, int) line: 27` |
| 58 | 0.23% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 59 | 0.23% | `dev.planeta9091.centities.CeHooks.batchUpsert(int, int, Entity, AABB, boolean, int) line: 2760` |
| 60 | 0.23% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap$MapIterator.nextEntry() line: 655` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.65% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 15902` |
| 2 | 4.33% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13381` |
| 3 | 3.01% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 2.71% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 5 | 2.56% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 6 | 2.48% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13314` |
| 7 | 2.22% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 8 | 1.84% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 9 | 1.77% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 10 | 1.73% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 11 | 1.62% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 13334` |
| 12 | 1.39% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10353` |
| 13 | 1.24% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 14 | 1.24% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int, boolean) line: 15628` |
| 15 | 1.17% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14750` |
| 16 | 1.13% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8168` |
| 17 | 1.05% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 18 | 0.94% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 19 | 0.83% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 20 | 0.79% | `net.minecraft.world.entity.Leashable.isLeashed() line: 49` |
| 21 | 0.75% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 22 | 0.75% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 23 | 0.72% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 24 | 0.68% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 168` |
| 25 | 0.64% | `net.minecraft.world.entity.Entity.isPassenger() line: 3733` |
| 26 | 0.60% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 27 | 0.60% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 28 | 0.56% | `java.util.HashMap.containsKey(Object) line: 603` |
| 29 | 0.53% | `it.unimi.dsi.fastutil.objects.ReferenceCollections$UnmodifiableCollection.contains(Object) line: 344` |
| 30 | 0.53% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |

GC pauses: 0, total 0 ms

