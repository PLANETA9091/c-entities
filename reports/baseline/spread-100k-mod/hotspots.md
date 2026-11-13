# JVM hotspots (JFR ExecutionSample)

Samples: 25596

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 7.31% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 2 | 5.57% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3490` |
| 3 | 3.82% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3434` |
| 4 | 3.22% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 5 | 2.96% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7473` |
| 6 | 2.45% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3463` |
| 7 | 2.34% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12105` |
| 8 | 2.16% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 9 | 2.05% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3448` |
| 10 | 1.88% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 11 | 1.67% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3451` |
| 12 | 1.56% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 13 | 1.55% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3464` |
| 14 | 1.50% | `java.util.ArrayList.<init>(int) line: 157` |
| 15 | 1.47% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 16 | 1.36% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3236` |
| 17 | 1.23% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 18 | 1.14% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 19 | 1.05% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 20 | 1.04% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3454` |
| 21 | 0.99% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3584` |
| 22 | 0.96% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 23 | 0.95% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 24 | 0.92% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 25 | 0.91% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 26 | 0.89% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 27 | 0.88% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4916` |
| 28 | 0.86% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3226` |
| 29 | 0.83% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3538` |
| 30 | 0.82% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 31 | 0.74% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 32 | 0.66% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 33 | 0.61% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4676` |
| 34 | 0.57% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 35 | 0.55% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 36 | 0.54% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 37 | 0.54% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3436` |
| 38 | 0.52% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 39 | 0.51% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3433` |
| 40 | 0.49% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 41 | 0.47% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3441` |
| 42 | 0.47% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 43 | 0.45% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 44 | 0.44% | `java.util.EnumMap.get(Object) line: 248` |
| 45 | 0.36% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 46 | 0.36% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 47 | 0.35% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7503` |
| 48 | 0.34% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6319` |
| 49 | 0.32% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 50 | 0.30% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 51 | 0.30% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 12565` |
| 52 | 0.29% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 53 | 0.27% | `net.minecraft.world.entity.LivingEntity.getItemBySlot(EquipmentSlot) line: 2901` |
| 54 | 0.26% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 55 | 0.26% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 56 | 0.25% | `net.minecraft.world.entity.LivingEntity$SwingState.tick() line: 5026` |
| 57 | 0.24% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 58 | 0.22% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 59 | 0.22% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 60 | 0.21% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 25.24% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2713` |
| 2 | 7.27% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3469` |
| 3 | 3.75% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 3.40% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 2.40% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 6 | 1.97% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6504` |
| 7 | 1.88% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5152` |
| 8 | 1.67% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 9 | 1.64% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 10 | 1.52% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3382` |
| 11 | 1.47% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2817` |
| 12 | 1.08% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6586` |
| 13 | 1.03% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 14 | 1.00% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3696` |
| 15 | 0.99% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 16 | 0.96% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4684` |
| 17 | 0.92% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 18 | 0.91% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 19 | 0.85% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 20 | 0.80% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1408` |
| 21 | 0.77% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 22 | 0.75% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7483` |
| 23 | 0.66% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 848` |
| 24 | 0.59% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 25 | 0.57% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 26 | 0.57% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |
| 27 | 0.54% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10382` |
| 28 | 0.50% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 29 | 0.46% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |
| 30 | 0.46% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 8516` |

GC pauses: 0, total 0 ms

