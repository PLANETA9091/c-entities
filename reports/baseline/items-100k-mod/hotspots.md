# JVM hotspots (JFR ExecutionSample)

Samples: 4324

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.94% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10701` |
| 2 | 4.69% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 4.67% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9400` |
| 4 | 3.12% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1029` |
| 5 | 3.08% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean, int) line: 6256` |
| 6 | 2.84% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9161` |
| 8 | 2.08% | `dev.planeta9091.centities.CeHooks.ubsEmpty(Entity) line: 15335` |
| 9 | 1.71% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 10 | 1.39% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 11 | 1.34% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 12 | 1.23% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1030` |
| 13 | 0.97% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15109` |
| 14 | 0.90% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 15 | 0.86% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9133` |
| 16 | 0.79% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 17 | 0.79% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean) line: 6211` |
| 18 | 0.79% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 19 | 0.76% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6706` |
| 20 | 0.74% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1048` |
| 21 | 0.69% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1039` |
| 22 | 0.67% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 164` |
| 23 | 0.60% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 24 | 0.60% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1027` |
| 25 | 0.51% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9581` |
| 26 | 0.46% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup$ArrayIterable$ArrayIterator.next() line: 161` |
| 27 | 0.42% | `io.papermc.paperclip.Util.readFully(InputStream) line: 46` |
| 28 | 0.42% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean, int) line: 6715` |
| 29 | 0.39% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.remove(Object) line: 430` |
| 30 | 0.37% | `dev.planeta9091.centities.CeHooks.stackFast(ItemEntity) line: 1179` |
| 31 | 0.35% | `sun.security.provider.DigestBase.implCompressMultiBlock0(byte[], int, int) line: 148` |
| 32 | 0.35% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10714` |
| 33 | 0.35% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3177` |
| 34 | 0.32% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1041` |
| 35 | 0.30% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 36 | 0.30% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 37 | 0.30% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 38 | 0.28% | `com.sun.crypto.provider.CounterMode.implCrypt(byte[], int, int, byte[], int) line: 209` |
| 39 | 0.28% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 8850` |
| 40 | 0.28% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 41 | 0.28% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3173` |
| 42 | 0.28% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 43 | 0.25% | `java.util.Arrays.copyOf(byte[], int) line: 3537` |
| 44 | 0.25% | `io.papermc.paperclip.Util.readFully(InputStream) line: 49` |
| 45 | 0.25% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupBlock() line: 824` |
| 46 | 0.25% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10696` |
| 47 | 0.25% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 179` |
| 48 | 0.23% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 281` |
| 49 | 0.23% | `ca.spottedleaf.moonrise.patches.collisions.CollisionUtil$MergedVoxelCoordinateList.merge(double[], double, double[], double, boolean, boolean) line: 1496` |
| 50 | 0.23% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 51 | 0.23% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean, int) line: 6236` |
| 52 | 0.23% | `dev.planeta9091.centities.CeHooks.vec3Cached(int, double, double, double) line: 3172` |
| 53 | 0.21% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 284` |
| 54 | 0.21% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.getEntry(Object) line: 88` |
| 55 | 0.21% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 56 | 0.21% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10616` |
| 57 | 0.21% | `org.bukkit.craftbukkit.entity.CraftEntity.isValid() line: 510` |
| 58 | 0.19% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 764` |
| 59 | 0.19% | `com.mojang.serialization.DataResult$Instance.ap2(App, App, App) line: 434` |
| 60 | 0.19% | `net.minecraft.nbt.CompoundTag.moonrise$writeWithoutId(AbstractBufferOutputStream, boolean) line: 57` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 10.85% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 2 | 7.33% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 10701` |
| 3 | 7.12% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 4.69% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 3.91% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean) line: 6211` |
| 6 | 2.27% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 7 | 2.27% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1043` |
| 8 | 2.08% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 16332` |
| 9 | 1.71% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartA() line: 880` |
| 10 | 1.53% | `dev.planeta9091.centities.CeHooks.itemDispatchItem(ItemEntity, int, boolean, int) line: 6441` |
| 11 | 1.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 12 | 1.13% | `dev.planeta9091.centities.CeHooks.spawnerCreateState$(int, Object, Object, Object, boolean) line: 15101` |
| 13 | 1.04% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean)` |
| 14 | 0.97% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9133` |
| 15 | 0.90% | `dev.planeta9091.centities.CeHooks.syncVelRead(Entity) line: 15904` |
| 16 | 0.79% | `dev.planeta9091.centities.CeHooks.itemDispatchWalk(ItemEntity, int) line: 6197` |
| 17 | 0.76% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1413` |
| 18 | 0.60% | `io.papermc.paperclip.Util.readBytes(Path) line: 30` |
| 19 | 0.60% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 9137` |
| 20 | 0.58% | `dev.planeta9091.centities.CeHooks.fullStackQuiet(ItemEntity) line: 965` |
| 21 | 0.51% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 22 | 0.51% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 168` |
| 23 | 0.49% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 7138` |
| 24 | 0.44% | `com.sun.crypto.provider.GHASH.update(byte[], int, int) line: 180` |
| 25 | 0.42% | `net.minecraft.world.entity.Entity.setPos(double, double, double) line: 832` |
| 26 | 0.42% | `net.minecraft.server.level.ChunkMap$TrackedEntity.removePlayer(ServerPlayer) line: 1400` |
| 27 | 0.39% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 28 | 0.37% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 29 | 0.35% | `sun.security.provider.DigestBase.implCompressMultiBlock(byte[], int, int) line: 143` |
| 30 | 0.32% | `dev.planeta9091.centities.CeHooks.quietFuseBump(ItemEntity, int, int) line: 1048` |

GC pauses: 0, total 0 ms

