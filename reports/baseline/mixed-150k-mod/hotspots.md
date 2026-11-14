# JVM hotspots (JFR ExecutionSample)

Samples: 25921

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 7.79% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7951` |
| 2 | 5.59% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 4.62% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 4 | 4.52% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12574` |
| 5 | 4.15% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 2.63% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 7 | 2.49% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 2.09% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 9 | 2.03% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 10 | 1.80% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 11 | 1.76% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 12 | 1.69% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 13 | 1.58% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3584` |
| 14 | 1.49% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 15 | 1.43% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 16 | 1.12% | `java.util.EnumMap.get(Object) line: 248` |
| 17 | 1.08% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 18 | 1.02% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 19 | 1.00% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 20 | 0.90% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 21 | 0.86% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 22 | 0.81% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 23 | 0.78% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 24 | 0.76% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 25 | 0.74% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 26 | 0.74% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 27 | 0.62% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 28 | 0.60% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12436` |
| 29 | 0.58% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 30 | 0.57% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 31 | 0.54% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 32 | 0.54% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 33 | 0.54% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 34 | 0.53% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 73` |
| 35 | 0.52% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 36 | 0.49% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 37 | 0.48% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1466` |
| 38 | 0.43% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 39 | 0.41% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 40 | 0.41% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 41 | 0.39% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7942` |
| 42 | 0.37% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 43 | 0.37% | `io.papermc.paper.entity.activation.ActivationRange.checkIfActive(Entity) line: 298` |
| 44 | 0.37% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 45 | 0.35% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 46 | 0.35% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12482` |
| 47 | 0.33% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 48 | 0.32% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 49 | 0.30% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 50 | 0.30% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7096` |
| 51 | 0.30% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 52 | 0.30% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 13034` |
| 53 | 0.29% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 9538` |
| 54 | 0.29% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 55 | 0.28% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7189` |
| 56 | 0.28% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 57 | 0.27% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 58 | 0.26% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.addAll(int, Collection) line: 761` |
| 59 | 0.25% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 60 | 0.24% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 9.74% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 8.58% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 4.85% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 4 | 4.56% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 2.92% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 6 | 2.75% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 7 | 2.49% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 2.14% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 9 | 1.82% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 10 | 1.80% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2817` |
| 11 | 1.69% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 12 | 1.60% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3696` |
| 13 | 1.43% | `net.minecraft.world.entity.LivingEntity.tick() line: 3491` |
| 14 | 1.31% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12669` |
| 15 | 1.17% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 8985` |
| 16 | 1.15% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 17 | 1.06% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7972` |
| 18 | 1.05% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 19 | 1.03% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5533` |
| 20 | 1.00% | `net.minecraft.world.entity.Entity.isEyeInFluid(TagKey) line: 2311` |
| 21 | 0.99% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 22 | 0.90% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 23 | 0.89% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 24 | 0.81% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 25 | 0.78% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 848` |
| 26 | 0.76% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 27 | 0.72% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 28 | 0.57% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 29 | 0.57% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 30 | 0.56% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |

GC pauses: 0, total 0 ms

