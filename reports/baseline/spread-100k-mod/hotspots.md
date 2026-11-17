# JVM hotspots (JFR ExecutionSample)

Samples: 10103

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 10.19% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9503` |
| 2 | 6.07% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 5.91% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14509` |
| 4 | 4.57% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8710` |
| 5 | 2.44% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 6 | 2.20% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 7 | 1.94% | `dev.planeta9091.centities.CeHooks.quietPass() line: 7697` |
| 8 | 1.69% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13784` |
| 9 | 1.60% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5990` |
| 10 | 1.46% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 11 | 1.20% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 12 | 1.17% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 13 | 0.99% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 14 | 0.94% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 15 | 0.88% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 16 | 0.78% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 17 | 0.72% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 18 | 0.71% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 912` |
| 19 | 0.68% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8512` |
| 20 | 0.57% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 21 | 0.52% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 22 | 0.52% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5661` |
| 23 | 0.52% | `sun.misc.Unsafe.getByte(Object, long) line: 279` |
| 24 | 0.52% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5994` |
| 25 | 0.52% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5665` |
| 26 | 0.50% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 11113` |
| 27 | 0.49% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 28 | 0.48% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14546` |
| 29 | 0.47% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 30 | 0.44% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 31 | 0.43% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6268` |
| 32 | 0.42% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14528` |
| 33 | 0.42% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 34 | 0.42% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 35 | 0.34% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 36 | 0.34% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9513` |
| 37 | 0.29% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 38 | 0.28% | `net.minecraft.server.level.WorldGenRegion.lambda$new$2(ChunkAccess, ChunkStep, GenerationChunkHolder, int, int) line: 124` |
| 39 | 0.28% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 40 | 0.27% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5996` |
| 41 | 0.26% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5948` |
| 42 | 0.25% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 43 | 0.25% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 44 | 0.25% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 45 | 0.23% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8442` |
| 46 | 0.22% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.getChunkHolder(int, int) line: 929` |
| 47 | 0.22% | `net.minecraft.world.entity.Entity.setPosRaw(double, double, double, boolean) line: 5186` |
| 48 | 0.22% | `java.util.Map.forEach(BiConsumer) line: 723` |
| 49 | 0.21% | `java.lang.invoke.DirectMethodHandle.allocateInstance(Object) line: 500` |
| 50 | 0.21% | `org.spigotmc.WatchdogThread.monotonicMillis() line: 36` |
| 51 | 0.21% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5957` |
| 52 | 0.21% | `dev.planeta9091.centities.CeHooks.batchUpsert(int, int, Entity, AABB, boolean, int) line: 2556` |
| 53 | 0.20% | `net.minecraft.util.StaticCache2D.get(int, int) line: 49` |
| 54 | 0.20% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 55 | 0.19% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 56 | 0.19% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 57 | 0.19% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14796` |
| 58 | 0.18% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 59 | 0.18% | `net.minecraft.world.level.levelgen.Heightmap.lambda$static$0(BlockState) line: 28` |
| 60 | 0.18% | `java.util.ArrayList$Itr.next() line: 1058` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 11.06% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 2 | 7.19% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 14800` |
| 3 | 6.80% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 4 | 3.37% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9513` |
| 5 | 2.71% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7793` |
| 6 | 2.59% | `dev.planeta9091.centities.CeHooks.standMirror(ArmorStand) line: 5662` |
| 7 | 1.95% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 8 | 1.65% | `dev.planeta9091.centities.CeHooks.stackFast(ItemEntity) line: 982` |
| 9 | 1.41% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 12426` |
| 10 | 1.25% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5838` |
| 11 | 1.20% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13792` |
| 12 | 1.10% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 12359` |
| 13 | 1.06% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 14 | 0.94% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 15 | 0.93% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 13801` |
| 16 | 0.78% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9533` |
| 17 | 0.78% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 18 | 0.75% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 896` |
| 19 | 0.73% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8428` |
| 20 | 0.70% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 21 | 0.67% | `net.minecraft.world.entity.Entity.setBoundingBox(AABB)` |
| 22 | 0.61% | `ca.spottedleaf.moonrise.patches.starlight.light.SkyStarLightEngine.lightChunk(LightChunkGetter, ChunkAccess, boolean) line: 551` |
| 23 | 0.55% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 5896` |
| 24 | 0.54% | `dev.planeta9091.centities.CeHooks$NmMap.get(int) line: 11120` |
| 25 | 0.52% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |
| 26 | 0.48% | `net.minecraft.util.StaticCache2D.lambda$map$0(StaticCache2D$MappingFunction, int, int) line: 36` |
| 27 | 0.48% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 28 | 0.48% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 14531` |
| 29 | 0.47% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 30 | 0.47% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |

GC pauses: 0, total 0 ms

