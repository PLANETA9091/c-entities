# JVM hotspots (JFR ExecutionSample)

Samples: 6990

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 11.37% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 4.31% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14975` |
| 3 | 3.91% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9870` |
| 4 | 3.79% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8777` |
| 5 | 2.73% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 6 | 1.99% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 982` |
| 7 | 1.67% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 15013` |
| 8 | 1.46% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14210` |
| 9 | 1.44% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 15031` |
| 10 | 1.34% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 11 | 1.32% | `dev.planeta9091.centities.CeHooks.stackFast(ItemEntity) line: 1030` |
| 12 | 1.24% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 13 | 1.22% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6044` |
| 14 | 1.06% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5711` |
| 15 | 1.02% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 16 | 0.97% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |
| 17 | 0.90% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 18 | 0.89% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 19 | 0.84% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5715` |
| 20 | 0.79% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3009` |
| 21 | 0.74% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8513` |
| 22 | 0.72% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 23 | 0.69% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 24 | 0.64% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 25 | 0.60% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 6040` |
| 26 | 0.60% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 984` |
| 27 | 0.59% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 28 | 0.59% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 29 | 0.57% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9871` |
| 30 | 0.57% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 31 | 0.56% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 32 | 0.54% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 33 | 0.53% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 987` |
| 34 | 0.53% | `dev.planeta9091.centities.CeHooks.dispatchQuietSkip(ItemEntity, int) line: 954` |
| 35 | 0.52% | `dev.planeta9091.centities.CeHooks.ubsEmpty(Entity) line: 14400` |
| 36 | 0.49% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 37 | 0.46% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 38 | 0.43% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 39 | 0.43% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 40 | 0.40% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 41 | 0.39% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8509` |
| 42 | 0.36% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5934` |
| 43 | 0.34% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 44 | 0.31% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8491` |
| 45 | 0.31% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 46 | 0.31% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 990` |
| 47 | 0.30% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 48 | 0.29% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(int, int) line: 929` |
| 49 | 0.29% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 50 | 0.29% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 51 | 0.29% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9860` |
| 52 | 0.26% | `net.minecraft.server.level.WorldGenRegion.lambda$new$2(ChunkAccess, ChunkStep, GenerationChunkHolder, int, int) line: 124` |
| 53 | 0.26% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.remove(Object) line: 430` |
| 54 | 0.26% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14966` |
| 55 | 0.24% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 56 | 0.24% | `it.unimi.dsi.fastutil.shorts.Short2ShortOpenHashMap.find(short) line: 243` |
| 57 | 0.23% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 58 | 0.23% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3014` |
| 59 | 0.23% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5804` |
| 60 | 0.23% | `dev.planeta9091.centities.CeHooks.dispatchQuietSkip(ItemEntity, int) line: 968` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 8.05% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 15285` |
| 2 | 6.37% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 3 | 5.11% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 4.25% | `dev.planeta9091.centities.CeHooks.dispatchQuietSkip(ItemEntity, int) line: 968` |
| 5 | 3.13% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5712` |
| 6 | 3.00% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9871` |
| 7 | 2.40% | `dev.planeta9091.centities.CeHooks.stackFast(ItemEntity) line: 1028` |
| 8 | 2.32% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5888` |
| 9 | 2.17% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5946` |
| 10 | 1.73% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 11 | 1.33% | `dev.planeta9091.centities.CeHooks.fullStackQuiet(ItemEntity) line: 931` |
| 12 | 1.24% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 13 | 1.23% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 896` |
| 14 | 1.13% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 12852` |
| 15 | 1.10% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9879` |
| 16 | 0.97% | `net.minecraft.world.entity.Leashable.isLeashed() line: 49` |
| 17 | 0.96% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 18 | 0.90% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 19 | 0.87% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 14227` |
| 20 | 0.80% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5975` |
| 21 | 0.80% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 994` |
| 22 | 0.77% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 23 | 0.77% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8491` |
| 24 | 0.76% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5713` |
| 25 | 0.70% | `dev.planeta9091.centities.CeHooks.mergeMaxStack(Object) line: 879` |
| 26 | 0.64% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 27 | 0.60% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 28 | 0.59% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 29 | 0.59% | `dev.planeta9091.centities.CeHooks.stackFast(ItemEntity) line: 1032` |
| 30 | 0.56% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 1026` |

GC pauses: 0, total 0 ms

