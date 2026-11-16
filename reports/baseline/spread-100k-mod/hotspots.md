# JVM hotspots (JFR ExecutionSample)

Samples: 8977

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 8.22% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8945` |
| 2 | 6.77% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8152` |
| 3 | 5.53% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13865` |
| 4 | 5.40% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 5 | 4.67% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 3.77% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 7 | 3.64% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5566` |
| 8 | 3.48% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 9 | 1.96% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 10 | 1.39% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13212` |
| 11 | 1.38% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 12 | 1.30% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 13 | 1.11% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 14 | 1.06% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 15 | 0.94% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 16 | 0.79% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 17 | 0.76% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7881` |
| 18 | 0.76% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 19 | 0.71% | `net.minecraft.world.entity.decoration.ArmorStand.isMarker() line: 631` |
| 20 | 0.66% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 21 | 0.65% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 22 | 0.65% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 23 | 0.55% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 24 | 0.53% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 5844` |
| 25 | 0.52% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 26 | 0.50% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 27 | 0.50% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 28 | 0.48% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 29 | 0.46% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.contains(Object) line: 450` |
| 30 | 0.40% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13884` |
| 31 | 0.38% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 32 | 0.36% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 33 | 0.35% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13902` |
| 34 | 0.33% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(int, int) line: 929` |
| 35 | 0.32% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14139` |
| 36 | 0.31% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7899` |
| 37 | 0.31% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 13717` |
| 38 | 0.31% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 39 | 0.31% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 10541` |
| 40 | 0.30% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 41 | 0.30% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7765` |
| 42 | 0.29% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 2083` |
| 43 | 0.29% | `dev.planeta9091.centities.CeHooks.nativeMobTick(Mob) line: 11669` |
| 44 | 0.28% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13856` |
| 45 | 0.26% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 46 | 0.26% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 47 | 0.23% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 48 | 0.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7693` |
| 49 | 0.20% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 50 | 0.20% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8955` |
| 51 | 0.19% | `ca.spottedleaf.moonrise.patches.starlight.light.SWMRNibbleArray.getUpdating(int) line: 393` |
| 52 | 0.19% | `org.spigotmc.WatchdogThread.monotonicMillis() line: 36` |
| 53 | 0.19% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5572` |
| 54 | 0.19% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5570` |
| 55 | 0.18% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 56 | 0.18% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8970` |
| 57 | 0.18% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 58 | 0.17% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 59 | 0.17% | `java.util.zip.ZipFile$Source.getEntryPos(String, boolean, ZipCoder) line: 1879` |
| 60 | 0.17% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2639` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 10.07% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 9.00% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 3 | 8.81% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 7.07% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14143` |
| 5 | 4.93% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8955` |
| 6 | 3.73% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 6115` |
| 7 | 1.57% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 8 | 1.48% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5454` |
| 9 | 1.33% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11854` |
| 10 | 1.21% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11787` |
| 11 | 1.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7885` |
| 12 | 0.95% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 896` |
| 13 | 0.94% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13220` |
| 14 | 0.80% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 15 | 0.79% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 16 | 0.77% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13229` |
| 17 | 0.71% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 18 | 0.71% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 19 | 0.71% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5290` |
| 20 | 0.68% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8975` |
| 21 | 0.65% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 22 | 0.58% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 23 | 0.58% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7881` |
| 24 | 0.58% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8971` |
| 25 | 0.55% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5581` |
| 26 | 0.55% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 27 | 0.52% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 28 | 0.50% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 29 | 0.48% | `it.unimi.dsi.fastutil.objects.ReferenceCollections$UnmodifiableCollection.contains(Object) line: 344` |
| 30 | 0.45% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |

GC pauses: 0, total 0 ms

