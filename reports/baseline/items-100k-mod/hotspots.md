# JVM hotspots (JFR ExecutionSample)

Samples: 8891

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 10.62% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 2 | 10.27% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 7.42% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 4 | 6.33% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 5 | 5.11% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 4.25% | `java.util.HashMap.getNode(Object) line: 585` |
| 7 | 3.09% | `java.lang.invoke.MethodHandles$Lookup.<init>(Class, Class, int) line: 1644` |
| 8 | 2.61% | `dev.planeta9091.centities.CeHooks.arenaRef(long) line: 2023` |
| 9 | 2.38% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1302` |
| 10 | 2.33% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 11 | 2.29% | `net.minecraft.server.level.ServerLevel.lambda$tick$4(TickRateManager, ProfilerFiller, Entity) line: 795` |
| 12 | 1.80% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 13 | 1.79% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1430` |
| 14 | 1.74% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 15 | 1.30% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 131` |
| 16 | 1.24% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1169` |
| 17 | 1.24% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 18 | 1.15% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 82` |
| 19 | 1.06% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 20 | 1.05% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1000` |
| 21 | 0.74% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 134` |
| 22 | 0.67% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 1936` |
| 23 | 0.66% | `net.minecraft.server.level.ServerLevel.lambda$tick$4(TickRateManager, ProfilerFiller, Entity) line: 802` |
| 24 | 0.60% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 25 | 0.53% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 26 | 0.46% | `java.util.Iterator.forEachRemaining(Consumer) line: 133` |
| 27 | 0.38% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4881` |
| 28 | 0.37% | `net.minecraft.world.level.chunk.ImposterProtoChunk.getNoiseBiome(int, int, int) line: 175` |
| 29 | 0.35% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1425` |
| 30 | 0.34% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 31 | 0.33% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1215` |
| 32 | 0.33% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 33 | 0.31% | `it.unimi.dsi.fastutil.objects.ReferenceLinkedOpenHashSet.add(Object) line: 416` |
| 34 | 0.31% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.resizeIncreaseQueue() line: 1060` |
| 35 | 0.30% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 36 | 0.29% | `net.minecraft.server.level.ServerPlayer.moonrise$getChunkLoader() line: 449` |
| 37 | 0.28% | `java.lang.String.hashCode() line: 2358` |
| 38 | 0.26% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 758` |
| 39 | 0.26% | `net.minecraft.server.level.ServerLevel.tickChunk(LevelChunk, int) line: 916` |
| 40 | 0.25% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1419` |
| 41 | 0.24% | `dev.planeta9091.centities.CeHooks.cellOfBox(AABB) line: 720` |
| 42 | 0.24% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 1526` |
| 43 | 0.22% | `net.minecraft.util.Mth.lerp2(double, double, double, double, double, double) line: 545` |
| 44 | 0.22% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.getNode(long) line: 207` |
| 45 | 0.22% | `java.util.ArrayList$Itr.next() line: 1057` |
| 46 | 0.21% | `net.minecraft.world.item.ItemStack.isEmpty() line: 313` |
| 47 | 0.21% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 48 | 0.19% | `net.minecraft.world.level.levelgen.material.MaterialRuleList.calculate(DensityFunction$FunctionContext) line: 12` |
| 49 | 0.19% | `org.bukkit.craftbukkit.entity.CraftPlayer.canSee(Entity) line: 2217` |
| 50 | 0.18% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 135` |
| 51 | 0.16% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 115` |
| 52 | 0.16% | `net.minecraft.nbt.CompoundTag.write(DataOutput) line: 180` |
| 53 | 0.15% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 714` |
| 54 | 0.15% | `net.minecraft.world.level.chunk.PalettedContainer.getAndSet(int, Object) line: 226` |
| 55 | 0.15% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 173` |
| 56 | 0.15% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 178` |
| 57 | 0.13% | `java.lang.AbstractStringBuilder.ensureCapacityInternal(int) line: 243` |
| 58 | 0.13% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1206` |
| 59 | 0.13% | `net.minecraft.util.Mth.packDegrees(float) line: 178` |
| 60 | 0.12% | `com.sun.crypto.provider.GHASH.processBlocks(byte[], int, int, long[], long[]) line: 282` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 11.85% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 10.27% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 3 | 9.92% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 4 | 8.78% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 5 | 5.11% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 4.25% | `java.util.HashMap.containsKey(Object) line: 602` |
| 7 | 3.82% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 8 | 3.09% | `java.lang.invoke.MethodHandles$Lookup.<init>(Class) line: 1637` |
| 9 | 3.01% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 10 | 2.98% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 11 | 2.65% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity)` |
| 12 | 2.60% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 1924` |
| 13 | 2.49% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1432` |
| 14 | 1.15% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 15 | 1.07% | `dev.planeta9091.centities.CeHooks.getEntities(Level, Entity, AABB, Predicate) line: 460` |
| 16 | 1.00% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 17 | 0.97% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 212` |
| 18 | 0.70% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 1958` |
| 19 | 0.67% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 20 | 0.53% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartC() line: 948` |
| 21 | 0.46% | `net.minecraft.world.level.chunk.HashMapPalette.getEntries() line: 117` |
| 22 | 0.43% | `net.minecraft.world.level.levelgen.NoiseBasedChunkGenerator.lambda$fillFromNoise$11(ChunkAccess, int, NoiseSettings, int, Blender, StructureManager, RandomState, int) line: 312` |
| 23 | 0.37% | `net.minecraft.world.level.LevelReader.getNoiseBiome(int, int, int) line: 78` |
| 24 | 0.36% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1161` |
| 25 | 0.35% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1451` |
| 26 | 0.31% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder.addGenerationBlockingNeighbour(NewChunkHolder) line: 514` |
| 27 | 0.31% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1173` |
| 28 | 0.31% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 167` |
| 29 | 0.30% | `ca.spottedleaf.concurrentutil.map.ConcurrentLong2ReferenceChainedHashTable.get(long) line: 224` |
| 30 | 0.30% | `net.minecraft.world.level.chunk.status.ChunkDependencies.get(int) line: 51` |

GC pauses: 0, total 0 ms

