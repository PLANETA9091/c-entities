# JVM hotspots (JFR ExecutionSample)

Samples: 30005

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 7.38% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7482` |
| 2 | 4.83% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 4.49% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12105` |
| 4 | 4.20% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 5 | 2.94% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 2.51% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 2.21% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 8 | 2.06% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 9 | 1.98% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 10 | 1.94% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 11 | 1.53% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 12 | 1.35% | `java.util.EnumMap.get(Object) line: 248` |
| 13 | 1.31% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 14 | 1.22% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 15 | 1.21% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 16 | 1.19% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3584` |
| 17 | 1.05% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 898` |
| 18 | 1.05% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 19 | 0.87% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 20 | 0.85% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 21 | 0.79% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 22 | 0.72% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 23 | 0.67% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 24 | 0.67% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 25 | 0.66% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 26 | 0.65% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 27 | 0.61% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 28 | 0.61% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 29 | 0.59% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 30 | 0.58% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 73` |
| 31 | 0.57% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 32 | 0.57% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 33 | 0.56% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 34 | 0.56% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 11967` |
| 35 | 0.53% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 36 | 0.50% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 37 | 0.48% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 38 | 0.47% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 39 | 0.46% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 40 | 0.45% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 12573` |
| 41 | 0.45% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 42 | 0.44% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 43 | 0.42% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 44 | 0.42% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 45 | 0.41% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 46 | 0.39% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 12565` |
| 47 | 0.39% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7473` |
| 48 | 0.38% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 49 | 0.36% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 50 | 0.36% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 51 | 0.35% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 8505` |
| 52 | 0.34% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 53 | 0.33% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1429` |
| 54 | 0.33% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3490` |
| 55 | 0.32% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 9069` |
| 56 | 0.31% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1484` |
| 57 | 0.29% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 58 | 0.29% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 59 | 0.29% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 60 | 0.28% | `java.util.EnumMap.isValidKey(Object) line: 320` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 8.11% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 2 | 7.77% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 4.55% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 4.37% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 5 | 2.75% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 6 | 2.51% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 2.08% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 8 | 2.06% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2817` |
| 9 | 2.02% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 10 | 1.68% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 11 | 1.49% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2713` |
| 12 | 1.31% | `net.minecraft.world.entity.Entity.isEyeInFluid(TagKey) line: 2311` |
| 13 | 1.28% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 8516` |
| 14 | 1.21% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3696` |
| 15 | 1.19% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5152` |
| 16 | 1.13% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 17 | 1.08% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12200` |
| 18 | 1.07% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7503` |
| 19 | 1.05% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 20 | 0.95% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 21 | 0.95% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 22 | 0.90% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 23 | 0.84% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 24 | 0.79% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 25 | 0.75% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 26 | 0.72% | `net.minecraft.world.phys.AABB.inflate(double) line: 189` |
| 27 | 0.72% | `net.minecraft.world.entity.LivingEntity.tick() line: 3491` |
| 28 | 0.66% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10382` |
| 29 | 0.64% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 30 | 0.62% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |

GC pauses: 0, total 0 ms

