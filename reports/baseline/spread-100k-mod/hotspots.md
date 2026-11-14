# JVM hotspots (JFR ExecutionSample)

Samples: 22394

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.41% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3698` |
| 2 | 5.98% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 3 | 3.76% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3642` |
| 4 | 3.74% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 2.57% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12449` |
| 6 | 2.46% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3659` |
| 7 | 2.29% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7064` |
| 8 | 2.05% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3672` |
| 9 | 2.02% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7817` |
| 10 | 1.83% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3656` |
| 11 | 1.72% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 12 | 1.60% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3671` |
| 13 | 1.45% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 14 | 1.34% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 15 | 1.24% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 16 | 1.23% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3649` |
| 17 | 1.22% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 18 | 1.21% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 19 | 1.18% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 20 | 1.13% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 21 | 1.07% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 22 | 1.00% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3434` |
| 23 | 0.97% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3444` |
| 24 | 0.92% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3641` |
| 25 | 0.90% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3584` |
| 26 | 0.88% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 27 | 0.87% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 28 | 0.82% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 29 | 0.82% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 5185` |
| 30 | 0.81% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 31 | 0.75% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3746` |
| 32 | 0.74% | `java.util.ArrayList.<init>(int) line: 157` |
| 33 | 0.71% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 34 | 0.68% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3662` |
| 35 | 0.67% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 36 | 0.66% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4935` |
| 37 | 0.60% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 38 | 0.57% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 39 | 0.56% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 40 | 0.56% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 41 | 0.55% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 12917` |
| 42 | 0.55% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 43 | 0.54% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 44 | 0.53% | `java.util.EnumMap.get(Object) line: 248` |
| 45 | 0.52% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3722` |
| 46 | 0.50% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6967` |
| 47 | 0.48% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3644` |
| 48 | 0.46% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 49 | 0.46% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 50 | 0.40% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 51 | 0.39% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 52 | 0.38% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 53 | 0.36% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12357` |
| 54 | 0.36% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7827` |
| 55 | 0.36% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 56 | 0.36% | `java.util.concurrent.ConcurrentLinkedQueue.poll() line: 387` |
| 57 | 0.35% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3704` |
| 58 | 0.34% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 59 | 0.34% | `dev.planeta9091.centities.CeHooks.getTicksFrozen$(Entity) line: 13008` |
| 60 | 0.33% | `sun.misc.Unsafe.putObjectVolatile(Object, long, Object) line: 1277` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 27.47% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2921` |
| 2 | 5.94% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3677` |
| 3 | 5.08% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 3.92% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 5 | 2.61% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 6 | 2.60% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 7 | 1.85% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 8 | 1.24% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 9 | 1.22% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2817` |
| 10 | 1.21% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 11 | 1.07% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 12 | 1.05% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5421` |
| 13 | 1.00% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 14 | 1.00% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1408` |
| 15 | 0.99% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 16 | 0.92% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3696` |
| 17 | 0.88% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 18 | 0.86% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 19 | 0.86% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4943` |
| 20 | 0.85% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 21 | 0.82% | `net.minecraft.world.entity.LivingEntity.tick() line: 3491` |
| 22 | 0.82% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4914` |
| 23 | 0.80% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7827` |
| 24 | 0.75% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 8860` |
| 25 | 0.74% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3590` |
| 26 | 0.71% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 125` |
| 27 | 0.67% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |
| 28 | 0.61% | `net.minecraft.world.entity.LivingEntity.tick() line: 3492` |
| 29 | 0.58% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 30 | 0.52% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |

GC pauses: 0, total 0 ms

