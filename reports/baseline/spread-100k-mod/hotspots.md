# JVM hotspots (JFR ExecutionSample)

Samples: 26660

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 8.80% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 2 | 7.06% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3490` |
| 3 | 3.60% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3434` |
| 4 | 2.88% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7416` |
| 5 | 2.69% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 6 | 2.69% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.59% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3464` |
| 8 | 2.30% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12048` |
| 9 | 2.15% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3454` |
| 10 | 1.67% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3451` |
| 11 | 1.67% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 12 | 1.65% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 13 | 1.63% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3447` |
| 14 | 1.48% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 15 | 1.43% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 16 | 1.42% | `java.util.ArrayList.<init>(int) line: 157` |
| 17 | 1.36% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3472` |
| 18 | 1.35% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3236` |
| 19 | 1.34% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 20 | 1.32% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.popChunkWait() line: 927` |
| 21 | 1.29% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 22 | 1.26% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3436` |
| 23 | 1.10% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 24 | 1.08% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3584` |
| 25 | 0.98% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 26 | 0.98% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 27 | 0.90% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 28 | 0.82% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3465` |
| 29 | 0.76% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 30 | 0.74% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3442` |
| 31 | 0.73% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3226` |
| 32 | 0.72% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4916` |
| 33 | 0.66% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 34 | 0.65% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4676` |
| 35 | 0.62% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 36 | 0.58% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 37 | 0.52% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3441` |
| 38 | 0.51% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 39 | 0.50% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 40 | 0.46% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 41 | 0.45% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3448` |
| 42 | 0.44% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6319` |
| 43 | 0.44% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 44 | 0.44% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 45 | 0.42% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 46 | 0.41% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3433` |
| 47 | 0.37% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 48 | 0.36% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 49 | 0.34% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 50 | 0.33% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 51 | 0.32% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 52 | 0.32% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 53 | 0.31% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 54 | 0.31% | `java.util.EnumMap.get(Object) line: 248` |
| 55 | 0.30% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1484` |
| 56 | 0.29% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 57 | 0.29% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 58 | 0.27% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 59 | 0.25% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 60 | 0.23% | `net.minecraft.world.level.chunk.ChunkAccess.getSections() line: 241` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 28.82% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2713` |
| 2 | 8.69% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3469` |
| 3 | 4.16% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 3.22% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 2.34% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 6 | 1.64% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 7 | 1.62% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5152` |
| 8 | 1.44% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3382` |
| 9 | 1.43% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2817` |
| 10 | 1.43% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 11 | 1.38% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 12 | 1.34% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 13 | 1.32% | `net.minecraft.server.level.ServerChunkCache.syncLoad(int, int, ChunkStatus) line: 124` |
| 14 | 1.31% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6484` |
| 15 | 1.11% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6566` |
| 16 | 1.10% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |
| 17 | 1.10% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3696` |
| 18 | 1.02% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 19 | 0.93% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 20 | 0.78% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 21 | 0.78% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7426` |
| 22 | 0.76% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4684` |
| 23 | 0.74% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 24 | 0.73% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 25 | 0.69% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1408` |
| 26 | 0.66% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 27 | 0.51% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 8459` |
| 28 | 0.50% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10325` |
| 29 | 0.46% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 30 | 0.45% | `net.minecraft.world.entity.LivingEntity.tick() line: 3492` |

GC pauses: 0, total 0 ms

