# JVM hotspots (JFR ExecutionSample)

Samples: 24573

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 8.93% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 2 | 6.61% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3654` |
| 3 | 3.49% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7760` |
| 4 | 3.13% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 5 | 2.96% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 2.72% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3598` |
| 7 | 2.65% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3628` |
| 8 | 2.54% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12392` |
| 9 | 1.85% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.78% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 11 | 1.71% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3615` |
| 12 | 1.59% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 13 | 1.49% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 14 | 1.40% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3584` |
| 15 | 1.39% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3627` |
| 16 | 1.36% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3612` |
| 17 | 1.23% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 18 | 1.20% | `net.minecraft.util.thread.BlockableEventLoop.managedBlock(BooleanSupplier) line: 190` |
| 19 | 1.11% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 20 | 1.10% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3702` |
| 21 | 1.09% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 22 | 1.05% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 23 | 0.98% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 24 | 0.97% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 25 | 0.94% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 26 | 0.88% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3390` |
| 27 | 0.87% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 5141` |
| 28 | 0.85% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 29 | 0.79% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 30 | 0.79% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 31 | 0.77% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 32 | 0.77% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3618` |
| 33 | 0.73% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3597` |
| 34 | 0.66% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4891` |
| 35 | 0.66% | `java.util.ArrayList.<init>(int) line: 157` |
| 36 | 0.65% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 37 | 0.59% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 38 | 0.58% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 243` |
| 39 | 0.57% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 40 | 0.55% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3400` |
| 41 | 0.55% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3605` |
| 42 | 0.52% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 43 | 0.49% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 44 | 0.48% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7770` |
| 45 | 0.48% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 46 | 0.48% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 47 | 0.47% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 48 | 0.45% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 49 | 0.42% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6557` |
| 50 | 0.42% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 51 | 0.38% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 52 | 0.35% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 53 | 0.35% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 54 | 0.34% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3600` |
| 55 | 0.33% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3678` |
| 56 | 0.28% | `net.minecraft.util.SimpleBitStorage.get(int) line: 346` |
| 57 | 0.28% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1466` |
| 58 | 0.26% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 59 | 0.25% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12300` |
| 60 | 0.25% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 9356` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 23.52% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2877` |
| 2 | 8.88% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3633` |
| 3 | 4.81% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 4.43% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 2.58% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 6 | 1.74% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5377` |
| 7 | 1.71% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 8 | 1.60% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 9 | 1.59% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6758` |
| 10 | 1.57% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 11 | 1.49% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 12 | 1.42% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 13 | 1.41% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3696` |
| 14 | 1.32% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 15 | 1.20% | `net.minecraft.server.level.ServerChunkCache.syncLoad(int, int, ChunkStatus) line: 123` |
| 16 | 1.12% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6840` |
| 17 | 1.12% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 18 | 1.11% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2817` |
| 19 | 1.00% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1408` |
| 20 | 0.97% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |
| 21 | 0.94% | `net.minecraft.world.entity.Entity.isEyeInFluid(TagKey) line: 2311` |
| 22 | 0.88% | `net.minecraft.world.entity.LivingEntity.tick() line: 3492` |
| 23 | 0.87% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4899` |
| 24 | 0.80% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7770` |
| 25 | 0.78% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 26 | 0.68% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 27 | 0.66% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3546` |
| 28 | 0.65% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 29 | 0.62% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 30 | 0.60% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |

GC pauses: 0, total 0 ms

