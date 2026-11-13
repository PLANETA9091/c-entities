# JVM hotspots (JFR ExecutionSample)

Samples: 33063

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.97% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7355` |
| 2 | 4.33% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 11977` |
| 3 | 3.83% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 3.52% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 5 | 2.67% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 1.81% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 1.78% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 8 | 1.59% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 9 | 1.52% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 10 | 1.48% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 11 | 1.41% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 12 | 1.33% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 1.28% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 14 | 1.26% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 15 | 1.23% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 16 | 1.13% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3584` |
| 17 | 1.12% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 898` |
| 18 | 1.03% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 11839` |
| 19 | 0.91% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 20 | 0.89% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 21 | 0.87% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 22 | 0.86% | `java.util.EnumMap.get(Object) line: 248` |
| 23 | 0.85% | `dev.planeta9091.centities.CeHooks.vtabHash(int) line: 425` |
| 24 | 0.77% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 25 | 0.75% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3460` |
| 26 | 0.75% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 27 | 0.73% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 28 | 0.65% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 29 | 0.62% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 30 | 0.62% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 31 | 0.59% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 32 | 0.58% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 33 | 0.57% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 34 | 0.55% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 35 | 0.54% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 36 | 0.52% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 37 | 0.52% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 38 | 0.51% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 39 | 0.51% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 40 | 0.49% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 41 | 0.47% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 42 | 0.46% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 43 | 0.46% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 73` |
| 44 | 0.46% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 45 | 0.44% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1466` |
| 46 | 0.44% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 47 | 0.43% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 48 | 0.42% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 49 | 0.41% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 50 | 0.40% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 51 | 0.38% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7346` |
| 52 | 0.34% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 53 | 0.34% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 54 | 0.32% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 11843` |
| 55 | 0.31% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 56 | 0.30% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 57 | 0.30% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 58 | 0.28% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 12437` |
| 59 | 0.28% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 60 | 0.28% | `net.minecraft.world.entity.Entity.baseTick() line: 914` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.70% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 2 | 6.50% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 4.37% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 3.67% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 5 | 2.80% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2683` |
| 6 | 2.19% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 7 | 1.97% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 8 | 1.88% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 9 | 1.81% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 10 | 1.70% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12072` |
| 11 | 1.59% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5122` |
| 12 | 1.56% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 13 | 1.23% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2817` |
| 14 | 1.15% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7376` |
| 15 | 1.14% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3696` |
| 16 | 1.02% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 17 | 1.01% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3439` |
| 18 | 1.00% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 19 | 0.97% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 8389` |
| 20 | 0.91% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 21 | 0.90% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 22 | 0.87% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 23 | 0.87% | `net.minecraft.world.entity.Entity.isEyeInFluid(TagKey) line: 2311` |
| 24 | 0.85% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 439` |
| 25 | 0.73% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 26 | 0.70% | `java.util.ArrayList.toArray() line: 370` |
| 27 | 0.65% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 28 | 0.63% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 29 | 0.61% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10255` |
| 30 | 0.60% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |

GC pauses: 0, total 0 ms

