# JVM hotspots (JFR ExecutionSample)

Samples: 29207

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 7.38% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 2 | 6.42% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3475` |
| 3 | 3.89% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3419` |
| 4 | 2.73% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 5 | 2.69% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 2.49% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3449` |
| 7 | 2.41% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7401` |
| 8 | 2.34% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3436` |
| 9 | 1.81% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12033` |
| 10 | 1.69% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3432` |
| 11 | 1.63% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3439` |
| 12 | 1.58% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3457` |
| 13 | 1.42% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 14 | 1.41% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3433` |
| 15 | 1.22% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3421` |
| 16 | 1.19% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3211` |
| 17 | 1.19% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 18 | 1.15% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 19 | 1.09% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 20 | 1.06% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3427` |
| 21 | 1.02% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 22 | 0.94% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3584` |
| 23 | 0.91% | `net.minecraft.util.thread.BlockableEventLoop.managedBlock(BooleanSupplier) line: 191` |
| 24 | 0.91% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 25 | 0.89% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 26 | 0.88% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3418` |
| 27 | 0.88% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 28 | 0.84% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 29 | 0.81% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3450` |
| 30 | 0.80% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 31 | 0.74% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3426` |
| 32 | 0.72% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 33 | 0.71% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3221` |
| 34 | 0.65% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7411` |
| 35 | 0.64% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 36 | 0.63% | `java.util.ArrayList.<init>(int) line: 157` |
| 37 | 0.55% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 38 | 0.51% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 39 | 0.50% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 40 | 0.49% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4661` |
| 41 | 0.48% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 42 | 0.45% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 43 | 0.44% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 44 | 0.44% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 45 | 0.43% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 46 | 0.43% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 47 | 0.43% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 48 | 0.42% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6478` |
| 49 | 0.41% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4901` |
| 50 | 0.41% | `java.util.EnumMap.get(Object) line: 248` |
| 51 | 0.40% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 52 | 0.38% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 11941` |
| 53 | 0.34% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 12493` |
| 54 | 0.34% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6304` |
| 55 | 0.33% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 56 | 0.33% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 57 | 0.31% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1378` |
| 58 | 0.30% | `net.minecraft.util.SimpleBitStorage.get(int) line: 346` |
| 59 | 0.30% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 60 | 0.29% | `java.util.concurrent.ConcurrentLinkedQueue.poll() line: 387` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 31.07% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2698` |
| 2 | 7.35% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3454` |
| 3 | 3.77% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 3.36% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 1.96% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 6 | 1.84% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 7 | 1.79% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6469` |
| 8 | 1.42% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 9 | 1.41% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 10 | 1.12% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 11 | 0.99% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 12 | 0.98% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5137` |
| 13 | 0.94% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3696` |
| 14 | 0.91% | `net.minecraft.server.level.ServerChunkCache.syncLoad(int, int, ChunkStatus) line: 123` |
| 15 | 0.86% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 16 | 0.80% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |
| 17 | 0.80% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1408` |
| 18 | 0.78% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 19 | 0.71% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 20 | 0.69% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 8444` |
| 21 | 0.64% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2817` |
| 22 | 0.64% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7411` |
| 23 | 0.63% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3367` |
| 24 | 0.60% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 25 | 0.57% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12128` |
| 26 | 0.56% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6551` |
| 27 | 0.54% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 28 | 0.51% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 29 | 0.51% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 30 | 0.50% | `net.minecraft.world.entity.decoration.ArmorStand.pushEntities() line: 210` |

GC pauses: 0, total 0 ms

