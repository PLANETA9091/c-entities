# JVM hotspots (JFR ExecutionSample)

Samples: 29211

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.97% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7769` |
| 2 | 5.05% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 4.14% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12392` |
| 4 | 4.02% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 5 | 3.18% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 2.53% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 2.27% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 8 | 2.07% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 9 | 1.81% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 10 | 1.66% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 11 | 1.63% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 12 | 1.54% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 1.42% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 14 | 1.22% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3584` |
| 15 | 1.21% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 16 | 1.10% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 17 | 0.88% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 18 | 0.85% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 19 | 0.85% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 20 | 0.82% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 21 | 0.78% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 22 | 0.76% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 23 | 0.76% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 24 | 0.73% | `java.util.EnumMap.get(Object) line: 248` |
| 25 | 0.72% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 26 | 0.70% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 27 | 0.68% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 28 | 0.67% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 29 | 0.62% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12254` |
| 30 | 0.62% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 31 | 0.60% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 32 | 0.60% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 33 | 0.59% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 34 | 0.58% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 73` |
| 35 | 0.56% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 36 | 0.55% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 37 | 0.54% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1466` |
| 38 | 0.53% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7760` |
| 39 | 0.52% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 40 | 0.48% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 41 | 0.45% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 42 | 0.45% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 43 | 0.44% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 44 | 0.44% | `net.minecraft.server.level.ServerChunkCache.pollTask() line: 449` |
| 45 | 0.42% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 46 | 0.41% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 47 | 0.39% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7008` |
| 48 | 0.39% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 49 | 0.38% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 50 | 0.37% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 9356` |
| 51 | 0.35% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 52 | 0.35% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1484` |
| 53 | 0.32% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 54 | 0.32% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 8792` |
| 55 | 0.30% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 56 | 0.30% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12300` |
| 57 | 0.27% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 58 | 0.26% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 59 | 0.25% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 60 | 0.25% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 8.23% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 7.77% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 4.22% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 4 | 4.17% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 2.65% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 6 | 2.53% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 2.27% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2817` |
| 8 | 2.21% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 9 | 1.93% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 10 | 1.62% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 11 | 1.40% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2877` |
| 12 | 1.39% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 13 | 1.35% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 8803` |
| 14 | 1.22% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12487` |
| 15 | 1.22% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3696` |
| 16 | 1.18% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5377` |
| 17 | 1.14% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 18 | 1.10% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |
| 19 | 0.90% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 20 | 0.90% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 21 | 0.85% | `net.minecraft.world.entity.Entity.isEyeInFluid(TagKey) line: 2311` |
| 22 | 0.78% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 23 | 0.76% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 24 | 0.76% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 25 | 0.76% | `net.minecraft.world.entity.LivingEntity.tick() line: 3491` |
| 26 | 0.68% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 27 | 0.65% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 28 | 0.65% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10669` |
| 29 | 0.65% | `net.minecraft.world.phys.AABB.inflate(double) line: 189` |
| 30 | 0.62% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |

GC pauses: 0, total 0 ms

