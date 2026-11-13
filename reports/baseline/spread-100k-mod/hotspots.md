# JVM hotspots (JFR ExecutionSample)

Samples: 27008

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.97% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 2 | 5.72% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3460` |
| 3 | 3.46% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3404` |
| 4 | 3.14% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7346` |
| 5 | 3.00% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 11977` |
| 6 | 2.47% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 7 | 2.10% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3417` |
| 8 | 2.03% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 9 | 1.94% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3434` |
| 10 | 1.72% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3421` |
| 11 | 1.68% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3442` |
| 12 | 1.66% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 13 | 1.54% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3406` |
| 14 | 1.49% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3424` |
| 15 | 1.41% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 16 | 1.36% | `java.util.ArrayList.<init>(int) line: 157` |
| 17 | 1.33% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 18 | 1.31% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 19 | 1.25% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3206` |
| 20 | 1.23% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 21 | 1.08% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 22 | 0.99% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 23 | 0.97% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 24 | 0.90% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 25 | 0.85% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 26 | 0.85% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3584` |
| 27 | 0.84% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 28 | 0.83% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4886` |
| 29 | 0.83% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3412` |
| 30 | 0.82% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 31 | 0.81% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3196` |
| 32 | 0.78% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 33 | 0.75% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 34 | 0.68% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 35 | 0.60% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3418` |
| 36 | 0.58% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2948` |
| 37 | 0.56% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 38 | 0.53% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3435` |
| 39 | 0.50% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4646` |
| 40 | 0.49% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 41 | 0.46% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 42 | 0.46% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 43 | 0.46% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3403` |
| 44 | 0.44% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3411` |
| 45 | 0.44% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 46 | 0.43% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 47 | 0.41% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 48 | 0.40% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7376` |
| 49 | 0.38% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 50 | 0.37% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6289` |
| 51 | 0.33% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 52 | 0.32% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 53 | 0.31% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 54 | 0.27% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3433` |
| 55 | 0.27% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 56 | 0.27% | `net.minecraft.util.StaticCache2D.<init>(int, int, int, int, StaticCache2D$Initializer) line: 26` |
| 57 | 0.26% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 12437` |
| 58 | 0.26% | `dev.planeta9091.centities.CeHooks.getTicksFrozen$(Entity) line: 12536` |
| 59 | 0.25% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1484` |
| 60 | 0.23% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 27.32% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2683` |
| 2 | 6.93% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3439` |
| 3 | 4.01% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 3.37% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 3.03% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 6 | 1.65% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5122` |
| 7 | 1.40% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 8 | 1.38% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3352` |
| 9 | 1.37% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 10 | 1.27% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 11 | 1.23% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 12 | 1.20% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6454` |
| 13 | 1.19% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 14 | 1.14% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6536` |
| 15 | 1.08% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2817` |
| 16 | 0.93% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 17 | 0.90% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 18 | 0.87% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4654` |
| 19 | 0.87% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3696` |
| 20 | 0.85% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2644` |
| 21 | 0.84% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 22 | 0.78% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |
| 23 | 0.74% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 24 | 0.74% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 25 | 0.70% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1408` |
| 26 | 0.59% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7356` |
| 27 | 0.59% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 28 | 0.49% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 29 | 0.47% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10255` |
| 30 | 0.46% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |

GC pauses: 0, total 0 ms

