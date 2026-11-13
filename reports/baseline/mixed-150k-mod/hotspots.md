# JVM hotspots (JFR ExecutionSample)

Samples: 26884

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 7.61% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7410` |
| 2 | 5.81% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 4.55% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12033` |
| 4 | 4.55% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 5 | 4.37% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 6 | 2.83% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 7 | 1.89% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 8 | 1.79% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3584` |
| 9 | 1.69% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 10 | 1.63% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 11 | 1.56% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 12 | 1.50% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 13 | 1.41% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 14 | 1.32% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 15 | 1.24% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 16 | 1.11% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 17 | 1.10% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 18 | 1.04% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 19 | 1.04% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 20 | 1.03% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 21 | 1.02% | `java.util.EnumMap.get(Object) line: 248` |
| 22 | 0.92% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 23 | 0.83% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 24 | 0.77% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 25 | 0.72% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 11895` |
| 26 | 0.72% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 27 | 0.70% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 28 | 0.70% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 29 | 0.68% | `net.minecraft.world.entity.Entity.commonTick() line: 882` |
| 30 | 0.68% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 31 | 0.68% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 32 | 0.60% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 33 | 0.56% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 34 | 0.55% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 35 | 0.53% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 36 | 0.48% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 37 | 0.48% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 73` |
| 38 | 0.44% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3475` |
| 39 | 0.44% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 40 | 0.44% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1429` |
| 41 | 0.44% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2817` |
| 42 | 0.43% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 43 | 0.42% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 44 | 0.42% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 45 | 0.42% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 46 | 0.41% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 47 | 0.39% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 48 | 0.38% | `net.minecraft.world.entity.Entity.baseTick() line: 914` |
| 49 | 0.37% | `net.minecraft.world.entity.Entity.getVehicle() line: 4939` |
| 50 | 0.36% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 51 | 0.35% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 52 | 0.31% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 53 | 0.31% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.addAll(int, Collection) line: 762` |
| 54 | 0.30% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 55 | 0.29% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3419` |
| 56 | 0.29% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 11941` |
| 57 | 0.28% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7401` |
| 58 | 0.28% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 8997` |
| 59 | 0.26% | `java.util.ArrayList$Itr.hasNext() line: 1045` |
| 60 | 0.26% | `java.lang.ThreadLocal.get() line: 171` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 10.36% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 8.15% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 4.66% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 4 | 4.60% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 3.23% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 6 | 2.39% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 7 | 2.33% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 8 | 1.93% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2698` |
| 9 | 1.81% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3696` |
| 10 | 1.63% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 11 | 1.63% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 12 | 1.57% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2817` |
| 13 | 1.33% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12128` |
| 14 | 1.32% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 15 | 1.18% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 16 | 1.17% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 17 | 1.15% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 8444` |
| 18 | 1.09% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 19 | 1.04% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 20 | 1.04% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |
| 21 | 1.04% | `net.minecraft.world.entity.Entity.isEyeInFluid(TagKey) line: 2311` |
| 22 | 1.03% | `net.minecraft.world.entity.LivingEntity.tick() line: 3491` |
| 23 | 1.02% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1408` |
| 24 | 1.02% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 25 | 0.93% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 26 | 0.87% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5137` |
| 27 | 0.84% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 28 | 0.82% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 29 | 0.68% | `net.minecraft.world.entity.Entity.tick() line: 873` |
| 30 | 0.66% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |

GC pauses: 0, total 0 ms

