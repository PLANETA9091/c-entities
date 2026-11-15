# JVM hotspots (JFR ExecutionSample)

Samples: 7812

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 8.90% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 2 | 6.96% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7936` |
| 3 | 6.26% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8698` |
| 4 | 5.04% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13618` |
| 5 | 4.31% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 2.87% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 7 | 2.66% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 1.98% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5374` |
| 9 | 1.66% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 10 | 1.63% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 11 | 1.61% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5378` |
| 12 | 1.29% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 13 | 1.28% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 14 | 1.10% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 15 | 1.06% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7766` |
| 16 | 1.06% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7982` |
| 17 | 1.01% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 12965` |
| 18 | 1.00% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 19 | 0.91% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 20 | 0.86% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 21 | 0.79% | `net.minecraft.world.entity.decoration.ArmorStand.isMarker() line: 631` |
| 22 | 0.78% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 23 | 0.65% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13637` |
| 24 | 0.60% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 25 | 0.59% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5380` |
| 26 | 0.59% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7562` |
| 27 | 0.58% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7696` |
| 28 | 0.58% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 29 | 0.56% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13655` |
| 30 | 0.55% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 1000` |
| 31 | 0.49% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 32 | 0.41% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 33 | 0.41% | `dev.planeta9091.centities.CeHooks.probeBox(Entity, AABB) line: 1982` |
| 34 | 0.40% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5332` |
| 35 | 0.40% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 10294` |
| 36 | 0.38% | `dev.planeta9091.centities.CeHooks.ttlQuietSkip(Entity, int) line: 13470` |
| 37 | 0.38% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8708` |
| 38 | 0.37% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 39 | 0.33% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 40 | 0.33% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5250` |
| 41 | 0.33% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 42 | 0.33% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 43 | 0.32% | `net.minecraft.world.entity.Entity.getPassengers() line: 4812` |
| 44 | 0.32% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 45 | 0.28% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 2513` |
| 46 | 0.27% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 47 | 0.27% | `dev.planeta9091.centities.CeHooks.nativeMobTick(Mob) line: 11422` |
| 48 | 0.26% | `dev.planeta9091.centities.CeHooks.batchUpsert(int, int, Entity, AABB, boolean, int) line: 2109` |
| 49 | 0.26% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 50 | 0.23% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |
| 51 | 0.23% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 52 | 0.23% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 53 | 0.22% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(int, int) line: 929` |
| 54 | 0.22% | `net.minecraft.world.entity.Mob.getLeashData() line: 1487` |
| 55 | 0.22% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 56 | 0.20% | `net.minecraft.world.entity.Entity.onGround() line: 1147` |
| 57 | 0.19% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 58 | 0.19% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 179` |
| 59 | 0.18% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 60 | 0.17% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 13.20% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 11.42% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 3 | 7.10% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 6.78% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13896` |
| 5 | 5.82% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8708` |
| 6 | 2.69% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5923` |
| 7 | 2.29% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5262` |
| 8 | 1.47% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 896` |
| 9 | 1.47% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11540` |
| 10 | 1.28% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 12973` |
| 11 | 1.24% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 12 | 1.11% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7682` |
| 13 | 1.06% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 11607` |
| 14 | 0.99% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 12982` |
| 15 | 0.91% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 16 | 0.88% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8728` |
| 17 | 0.79% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5098` |
| 18 | 0.70% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 19 | 0.67% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 20 | 0.63% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 21 | 0.60% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 22 | 0.60% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 8724` |
| 23 | 0.59% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 24 | 0.58% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 25 | 0.56% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 26 | 0.55% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 27 | 0.51% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 13643` |
| 28 | 0.49% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 1026` |
| 29 | 0.44% | `dev.planeta9091.centities.CeHooks$NmMap.get(int) line: 10301` |
| 30 | 0.41% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 13883` |

GC pauses: 0, total 0 ms

