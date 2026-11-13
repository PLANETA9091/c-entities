# JVM hotspots (JFR ExecutionSample)

Samples: 29983

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.71% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7355` |
| 2 | 5.61% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 11977` |
| 3 | 3.66% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 2.94% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 5 | 2.56% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 1.97% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 7 | 1.76% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 1.61% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 9 | 1.55% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 10 | 1.53% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 11 | 1.38% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 12 | 1.35% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 1.16% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 14 | 1.07% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3584` |
| 15 | 1.05% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 898` |
| 16 | 1.00% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 17 | 0.96% | `dev.planeta9091.centities.CeHooks.vtabHash(int) line: 425` |
| 18 | 0.84% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 19 | 0.84% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 20 | 0.84% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 11839` |
| 21 | 0.83% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 22 | 0.82% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 23 | 0.77% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 24 | 0.76% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 25 | 0.67% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 26 | 0.67% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 27 | 0.65% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 28 | 0.64% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 29 | 0.64% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 30 | 0.60% | `java.util.EnumMap.get(Object) line: 248` |
| 31 | 0.55% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 32 | 0.53% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 33 | 0.52% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 237` |
| 34 | 0.52% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 35 | 0.51% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 36 | 0.51% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 37 | 0.49% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 38 | 0.48% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1466` |
| 39 | 0.47% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 40 | 0.47% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 41 | 0.47% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 42 | 0.46% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 43 | 0.46% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 44 | 0.46% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 45 | 0.43% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 73` |
| 46 | 0.42% | `net.minecraft.world.entity.LivingEntity.getItemBySlot(EquipmentSlot) line: 2901` |
| 47 | 0.38% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 48 | 0.38% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 49 | 0.37% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3460` |
| 50 | 0.37% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 51 | 0.37% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 52 | 0.36% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 11843` |
| 53 | 0.35% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 54 | 0.34% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 55 | 0.33% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7346` |
| 56 | 0.33% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 57 | 0.33% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 12437` |
| 58 | 0.32% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 59 | 0.32% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 60 | 0.31% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.28% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 2 | 6.22% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 5.65% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 3.10% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 5 | 2.18% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2683` |
| 6 | 1.87% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 7 | 1.76% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 1.73% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 9 | 1.71% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 10 | 1.60% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12072` |
| 11 | 1.56% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 12 | 1.53% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5122` |
| 13 | 1.53% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2817` |
| 14 | 1.40% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 15 | 1.15% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 16 | 1.12% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 8389` |
| 17 | 1.10% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7376` |
| 18 | 1.09% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3696` |
| 19 | 0.96% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 439` |
| 20 | 0.96% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 21 | 0.95% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 22 | 0.90% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 23 | 0.84% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 24 | 0.82% | `net.minecraft.world.entity.Entity.isEyeInFluid(TagKey) line: 2311` |
| 25 | 0.81% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 26 | 0.69% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 27 | 0.69% | `net.minecraft.world.entity.Entity.tick() line: 873` |
| 28 | 0.69% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity) line: 1408` |
| 29 | 0.66% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10255` |
| 30 | 0.65% | `dev.planeta9091.centities.CeHooks.mirrorTyped(Level, Class, AABB, Predicate) line: 1461` |

GC pauses: 0, total 0 ms

