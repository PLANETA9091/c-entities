# JVM hotspots (JFR ExecutionSample)

Samples: 29070

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 7.24% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7769` |
| 2 | 5.09% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 4.31% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12392` |
| 4 | 4.31% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 5 | 2.73% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 2.46% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 2.18% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 8 | 2.05% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 9 | 1.83% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 10 | 1.62% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 11 | 1.56% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 12 | 1.36% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3584` |
| 13 | 1.29% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 14 | 1.19% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 15 | 1.10% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 16 | 1.04% | `java.util.EnumMap.get(Object) line: 248` |
| 17 | 1.02% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 18 | 0.86% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 19 | 0.85% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 20 | 0.84% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 21 | 0.84% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 22 | 0.82% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 23 | 0.78% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 24 | 0.73% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 25 | 0.71% | `net.minecraft.server.level.ServerChunkCache.pollTask() line: 449` |
| 26 | 0.69% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 27 | 0.69% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 28 | 0.68% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 29 | 0.62% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 30 | 0.59% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 31 | 0.58% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 32 | 0.57% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 33 | 0.57% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 12254` |
| 34 | 0.54% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 35 | 0.53% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 36 | 0.52% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 37 | 0.52% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 38 | 0.50% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 39 | 0.49% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7760` |
| 40 | 0.48% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 41 | 0.48% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 42 | 0.44% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 73` |
| 43 | 0.43% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 44 | 0.43% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 7008` |
| 45 | 0.43% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 46 | 0.42% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 12852` |
| 47 | 0.37% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 48 | 0.37% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 49 | 0.35% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1484` |
| 50 | 0.34% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 51 | 0.34% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 8792` |
| 52 | 0.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 53 | 0.31% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 54 | 0.30% | `dev.planeta9091.centities.CeHooks.getTicksFrozen$(Entity) line: 12951` |
| 55 | 0.30% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1429` |
| 56 | 0.30% | `net.minecraft.world.entity.LivingEntity$SwingState.tick() line: 5026` |
| 57 | 0.29% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 58 | 0.29% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 59 | 0.28% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 9356` |
| 60 | 0.27% | `java.util.ArrayDeque.size() line: 647` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 8.07% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 2 | 7.83% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 4.46% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 4 | 4.34% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 2.46% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 2.40% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 7 | 2.05% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2817` |
| 8 | 1.90% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 9 | 1.74% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 10 | 1.63% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 11 | 1.46% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2877` |
| 12 | 1.37% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3696` |
| 13 | 1.28% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 8803` |
| 14 | 1.27% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5377` |
| 15 | 1.21% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 16 | 1.19% | `net.minecraft.world.entity.Entity.isEyeInFluid(TagKey) line: 2311` |
| 17 | 1.11% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12487` |
| 18 | 1.10% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |
| 19 | 0.99% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 20 | 0.96% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 21 | 0.89% | `net.minecraft.world.phys.AABB.inflate(double) line: 189` |
| 22 | 0.87% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 23 | 0.79% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 24 | 0.78% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 25 | 0.72% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 26 | 0.71% | `net.minecraft.server.MinecraftServer.tickMidTickTasks() line: 448` |
| 27 | 0.69% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 28 | 0.69% | `net.minecraft.world.entity.LivingEntity.tick() line: 3491` |
| 29 | 0.66% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10669` |
| 30 | 0.60% | `dev.planeta9091.centities.CeHooks.getSharedFlag$(Entity, int) line: 9202` |

GC pauses: 0, total 0 ms

