# JVM hotspots (JFR ExecutionSample)

Samples: 31431

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 7.06% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7817` |
| 2 | 5.53% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 4.01% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12440` |
| 4 | 3.95% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 5 | 3.15% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 2.72% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 2.05% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 8 | 2.00% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 9 | 1.97% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 10 | 1.80% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 11 | 1.63% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 12 | 1.44% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 13 | 1.43% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 14 | 1.37% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3584` |
| 15 | 1.07% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 16 | 1.03% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 898` |
| 17 | 1.03% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 18 | 0.98% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 19 | 0.95% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 20 | 0.91% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 21 | 0.87% | `java.util.EnumMap.get(Object) line: 248` |
| 22 | 0.80% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 23 | 0.77% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 24 | 0.74% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 25 | 0.73% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 26 | 0.69% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 27 | 0.67% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 28 | 0.66% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 29 | 0.65% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 30 | 0.65% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 31 | 0.64% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12302` |
| 32 | 0.63% | `net.minecraft.server.level.ServerChunkCache.pollTask() line: 449` |
| 33 | 0.63% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 34 | 0.61% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 35 | 0.59% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 36 | 0.57% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 37 | 0.55% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 38 | 0.54% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 39 | 0.50% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 40 | 0.49% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 73` |
| 41 | 0.48% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 42 | 0.47% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1466` |
| 43 | 0.47% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7808` |
| 44 | 0.44% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 45 | 0.40% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 46 | 0.39% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3698` |
| 47 | 0.38% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 48 | 0.35% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 49 | 0.35% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 50 | 0.33% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1484` |
| 51 | 0.32% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 52 | 0.30% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12306` |
| 53 | 0.30% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 54 | 0.29% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12348` |
| 55 | 0.29% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 9404` |
| 56 | 0.29% | `net.minecraft.world.entity.item.ItemEntity.inactiveTick() line: 125` |
| 57 | 0.28% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 12900` |
| 58 | 0.28% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 59 | 0.27% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 243` |
| 60 | 0.24% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 8840` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 8.68% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 7.79% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 4.12% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 4 | 4.06% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 2.72% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 2.52% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 7 | 2.37% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 8 | 2.10% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 9 | 2.01% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2817` |
| 10 | 1.44% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 11 | 1.43% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2921` |
| 12 | 1.40% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 13 | 1.38% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5421` |
| 14 | 1.37% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 8851` |
| 15 | 1.37% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3696` |
| 16 | 1.32% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12535` |
| 17 | 1.14% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 18 | 1.12% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 19 | 1.10% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7838` |
| 20 | 1.07% | `net.minecraft.world.entity.Entity.isEyeInFluid(TagKey) line: 2311` |
| 21 | 0.95% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 22 | 0.77% | `net.minecraft.world.entity.LivingEntity.tick() line: 3491` |
| 23 | 0.77% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 24 | 0.73% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 25 | 0.71% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 26 | 0.69% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 27 | 0.67% | `net.minecraft.world.phys.AABB.inflate(double) line: 189` |
| 28 | 0.65% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2206` |
| 29 | 0.64% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10717` |
| 30 | 0.63% | `net.minecraft.server.MinecraftServer.tickMidTickTasks() line: 448` |

GC pauses: 0, total 0 ms

