# JVM hotspots (JFR ExecutionSample)

Samples: 7203

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 10.80% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 10.20% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 3 | 7.83% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1289` |
| 4 | 5.98% | `net.minecraft.world.entity.Entity.moonrise$getChunkData() line: 411` |
| 5 | 5.00% | `java.lang.invoke.MethodHandles$Lookup.<init>(Class, Class, int) line: 1644` |
| 6 | 4.23% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 4.07% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 8 | 3.22% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 518` |
| 9 | 3.18% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1302` |
| 10 | 2.65% | `net.minecraft.server.level.ServerLevel.lambda$tick$4(TickRateManager, ProfilerFiller, Entity) line: 795` |
| 11 | 2.60% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1430` |
| 12 | 2.46% | `dev.planeta9091.centities.CeHooks.arenaRef(long) line: 2023` |
| 13 | 1.94% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 214` |
| 14 | 1.29% | `net.minecraft.world.entity.Entity.getType() line: 605` |
| 15 | 1.29% | `org.spigotmc.AsyncCatcher.catchOp(String) line: 8` |
| 16 | 1.26% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 80` |
| 17 | 1.21% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1165` |
| 18 | 1.19% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 131` |
| 19 | 1.15% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1169` |
| 20 | 1.10% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1419` |
| 21 | 0.94% | `net.minecraft.server.level.ServerLevel.lambda$tick$4(TickRateManager, ProfilerFiller, Entity) line: 802` |
| 22 | 0.83% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1000` |
| 23 | 0.64% | `net.minecraft.world.entity.Entity.getX() line: 4899` |
| 24 | 0.60% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 25 | 0.53% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 134` |
| 26 | 0.53% | `org.bukkit.craftbukkit.entity.CraftPlayer.canSee(Entity) line: 2217` |
| 27 | 0.50% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4881` |
| 28 | 0.46% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1215` |
| 29 | 0.44% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1318` |
| 30 | 0.44% | `com.google.common.collect.Iterators$5.computeNext() line: 671` |
| 31 | 0.43% | `it.unimi.dsi.fastutil.objects.Object2DoubleArrayMap.getDouble(Object) line: 431` |
| 32 | 0.42% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1426` |
| 33 | 0.39% | `net.minecraft.world.item.ItemStack.isEmpty() line: 313` |
| 34 | 0.36% | `net.minecraft.server.level.ServerPlayer.moonrise$getChunkLoader() line: 449` |
| 35 | 0.35% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 4883` |
| 36 | 0.32% | `java.util.HashMap.getNode(Object) line: 585` |
| 37 | 0.31% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 38 | 0.31% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 1526` |
| 39 | 0.26% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 40 | 0.26% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 135` |
| 41 | 0.26% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1295` |
| 42 | 0.25% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 43 | 0.24% | `java.util.ArrayList$Itr.next() line: 1057` |
| 44 | 0.24% | `net.minecraft.nbt.CompoundTag.write(DataOutput) line: 180` |
| 45 | 0.22% | `com.google.common.collect.RegularImmutableList.get(int) line: 79` |
| 46 | 0.22% | `io.papermc.paper.entity.activation.ActivationRange.activateEntity(Entity) line: 178` |
| 47 | 0.22% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1425` |
| 48 | 0.21% | `dev.planeta9091.centities.CeHooks.cellOfBox(AABB) line: 720` |
| 49 | 0.19% | `ca.spottedleaf.moonrise.patches.starlight.light.StarLightEngine.performLightIncrease(LightChunkGetter) line: 1125` |
| 50 | 0.18% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.add(Object) line: 367` |
| 51 | 0.17% | `java.lang.AbstractStringBuilder.ensureCapacityInternal(int) line: 243` |
| 52 | 0.17% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1206` |
| 53 | 0.17% | `it.unimi.dsi.fastutil.longs.LongOpenHashSet.contains(long) line: 477` |
| 54 | 0.15% | `net.minecraft.world.level.chunk.ImposterProtoChunk.getNoiseBiome(int, int, int) line: 175` |
| 55 | 0.15% | `net.minecraft.world.item.ItemStack.getMaxStackSize() line: 581` |
| 56 | 0.15% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity) line: 576` |
| 57 | 0.14% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 120` |
| 58 | 0.14% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.remove(Object) line: 427` |
| 59 | 0.12% | `net.minecraft.world.level.levelgen.SurfaceRules$TestRule.tryApply(int, int, int) line: 712` |
| 60 | 0.12% | `net.minecraft.util.Mth.packDegrees(float) line: 178` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 12.15% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 12.04% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1175` |
| 3 | 10.80% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 793` |
| 4 | 8.47% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1013` |
| 5 | 5.72% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1016` |
| 6 | 5.00% | `java.lang.invoke.MethodHandles$Lookup.<init>(Class) line: 1637` |
| 7 | 4.48% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity)` |
| 8 | 4.23% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 3.65% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 10 | 3.61% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1432` |
| 11 | 2.43% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 1924` |
| 12 | 1.32% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller, long) line: 561` |
| 13 | 1.29% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1282` |
| 14 | 1.28% | `net.minecraft.world.level.NaturalSpawner.createState(int, Iterable, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 82` |
| 15 | 0.90% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 212` |
| 16 | 0.87% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 1958` |
| 17 | 0.85% | `dev.planeta9091.centities.CeHooks.getEntities(Level, Entity, AABB, Predicate) line: 460` |
| 18 | 0.68% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 19 | 0.61% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1286` |
| 20 | 0.56% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1161` |
| 21 | 0.53% | `net.minecraft.server.level.ChunkMap$TrackedEntity.updatePlayer(ServerPlayer) line: 1305` |
| 22 | 0.44% | `net.minecraft.world.entity.Entity.isInLava() line: 2081` |
| 23 | 0.44% | `com.google.common.collect.AbstractIterator.tryToComputeNext() line: 145` |
| 24 | 0.42% | `io.papermc.paper.entity.activation.ActivationRange.activateEntities(Level) line: 167` |
| 25 | 0.40% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |
| 26 | 0.36% | `ca.spottedleaf.moonrise.patches.chunk_system.player.RegionizedPlayerChunkLoader.getAPISendViewDistance(ServerPlayer) line: 174` |
| 27 | 0.35% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 937` |
| 28 | 0.35% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1451` |
| 29 | 0.31% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.setupNoRandPartC() line: 948` |
| 30 | 0.31% | `net.minecraft.world.level.Level.tickBlockEntities()` |

GC pauses: 0, total 0 ms

