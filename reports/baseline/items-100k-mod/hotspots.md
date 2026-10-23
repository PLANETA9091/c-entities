# JVM hotspots (JFR ExecutionSample)

Samples: 8008

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 9.45% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 2 | 8.05% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 7.13% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 4 | 6.69% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 5 | 5.44% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 5.23% | `dev.planeta9091.centities.CeHooks.arenaRef(long) line: 2014` |
| 7 | 4.60% | `java.lang.invoke.MethodHandles$Lookup.<init>(Class, Class, int) line: 1644` |
| 8 | 3.57% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 9 | 2.93% | `java.util.HashMap.getNode(Object) line: 585` |
| 10 | 2.14% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 11 | 2.10% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1302` |
| 12 | 1.85% | `net.minecraft.server.level.ServerLevel.lambda$tick$4(TickRateManager, ProfilerFiller, Entity) line: 795` |
| 13 | 1.69% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 14 | 1.17% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 15 | 1.17% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1421` |
| 16 | 1.15% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1000` |
| 17 | 0.80% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1169` |
| 18 | 0.74% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 131` |
| 19 | 0.74% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 82` |
| 20 | 0.65% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 21 | 0.57% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 22 | 0.55% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 134` |
| 23 | 0.51% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |
| 24 | 0.47% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 25 | 0.42% | `ca.spottedleaf.moonrise.patches.starlight.light.SWMRNibbleArray.set(int, int) line: 422` |
| 26 | 0.40% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 27 | 0.40% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 28 | 0.35% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 29 | 0.34% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 916` |
| 30 | 0.29% | `java.lang.AbstractStringBuilder.ensureCapacityInternal(int) line: 243` |
| 31 | 0.29% | `net.minecraft.server.level.ServerLevel.lambda$tick$4(TickRateManager, ProfilerFiller, Entity) line: 802` |
| 32 | 0.27% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 33 | 0.27% | `java.util.ArrayList$Itr.next() line: 1057` |
| 34 | 0.27% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 1522` |
| 35 | 0.25% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1410` |
| 36 | 0.25% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1215` |
| 37 | 0.24% | `net.minecraft.world.level.chunk.LinearPalette.idFor(Object) line: 52` |
| 38 | 0.24% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 135` |
| 39 | 0.22% | `net.minecraft.world.level.chunk.ImposterProtoChunk.getNoiseBiome(int, int, int) line: 175` |
| 40 | 0.21% | `dev.planeta9091.centities.CeHooks.cellOfBox(AABB) line: 720` |
| 41 | 0.20% | `net.minecraft.server.level.ServerPlayer.moonrise$getChunkLoader() line: 449` |
| 42 | 0.19% | `java.lang.String.hashCode() line: 2358` |
| 43 | 0.19% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity)` |
| 44 | 0.17% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupBlock() line: 824` |
| 45 | 0.17% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 46 | 0.16% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 767` |
| 47 | 0.16% | `org.objectweb.asm.ClassReader.readCode(MethodVisitor, Context, int) line: 2425` |
| 48 | 0.16% | `net.minecraft.network.protocol.game.VecDeltaCodec.encodeY(Vec3) line: 36` |
| 49 | 0.16% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 1520` |
| 50 | 0.15% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 282` |
| 51 | 0.15% | `java.lang.StringLatin1.hashCode(byte[]) line: 194` |
| 52 | 0.15% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.schedule(int, int, ChunkStatus, NewChunkHolder, List, Priority) line: 768` |
| 53 | 0.15% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 54 | 0.15% | `net.minecraft.util.Mth.packDegrees(float) line: 178` |
| 55 | 0.15% | `net.minecraft.world.item.ItemStack.getMaxStackSize() line: 581` |
| 56 | 0.15% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1206` |
| 57 | 0.15% | `net.minecraft.world.item.ItemStack.isEmpty() line: 313` |
| 58 | 0.14% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 59 | 0.14% | `net.minecraft.world.level.levelgen.material.MaterialRuleList.calculate(DensityFunction$FunctionContext) line: 12` |
| 60 | 0.14% | `org.bukkit.craftbukkit.entity.CraftPlayer.canSee(Entity) line: 2217` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 10.10% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 9.80% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 3 | 9.37% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 4 | 8.05% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 5 | 5.44% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 5.27% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 1915` |
| 7 | 4.72% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 8 | 4.60% | `java.lang.invoke.MethodHandles$Lookup.<init>(Class) line: 1637` |
| 9 | 2.91% | `java.util.HashMap.containsKey(Object) line: 602` |
| 10 | 2.30% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 11 | 2.05% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 12 | 1.82% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1423` |
| 13 | 1.75% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity)` |
| 14 | 1.20% | `dev.planeta9091.centities.CeHooks.getEntities(Level, Entity, AABB, Predicate) line: 460` |
| 15 | 0.90% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 16 | 0.84% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 212` |
| 17 | 0.70% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1442` |
| 18 | 0.64% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 19 | 0.54% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 1949` |
| 20 | 0.54% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 21 | 0.51% | `com.google.common.collect.AbstractIterator.tryToComputeNext() line: 145` |
| 22 | 0.47% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartC() line: 948` |
| 23 | 0.44% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 24 | 0.42% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1168` |
| 25 | 0.40% | `org.objectweb.asm.ClassReader.readMethod(ClassVisitor, Context, int) line: 1512` |
| 26 | 0.40% | `net.minecraft.world.level.chunk.status.ChunkDependencies.get(int) line: 51` |
| 27 | 0.35% | `net.minecraft.server.level.ServerChunkCache.iterateTickingChunksFaster() line: 176` |
| 28 | 0.34% | `net.minecraft.world.level.levelgen.NoiseBasedChunkGenerator.lambda$fillFromNoise$11(ChunkAccess, int, NoiseSettings, int, Blender, StructureManager, RandomState, int) line: 312` |
| 29 | 0.32% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.schedule(int, int, ChunkStatus, NewChunkHolder, List) line: 681` |
| 30 | 0.27% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 937` |

GC pauses: 0, total 0 ms

