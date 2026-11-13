# JVM hotspots (JFR ExecutionSample)

Samples: 28125

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 7.05% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7356` |
| 2 | 4.96% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 4.34% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 11978` |
| 4 | 4.24% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 5 | 3.62% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 2.32% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 7 | 2.24% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 8 | 1.67% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 9 | 1.66% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 10 | 1.42% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 11 | 1.38% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 12 | 1.35% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3584` |
| 13 | 1.33% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 14 | 1.14% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 15 | 1.06% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7377` |
| 16 | 1.06% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 17 | 1.00% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 18 | 0.95% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 19 | 0.93% | `java.util.EnumMap.get(Object) line: 248` |
| 20 | 0.91% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 21 | 0.87% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 22 | 0.85% | `dev.planeta9091.centities.CeHooks.vtabHash(int) line: 425` |
| 23 | 0.80% | `sun.misc.Unsafe.getDouble(Object, long) line: 409` |
| 24 | 0.78% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 25 | 0.78% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 26 | 0.73% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 27 | 0.72% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 28 | 0.70% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 11840` |
| 29 | 0.70% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 30 | 0.69% | `net.minecraft.server.level.ServerChunkCache.pollTask() line: 449` |
| 31 | 0.69% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 32 | 0.66% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 33 | 0.58% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 34 | 0.56% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 35 | 0.52% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 36 | 0.52% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 37 | 0.50% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 38 | 0.50% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 39 | 0.49% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1466` |
| 40 | 0.48% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3461` |
| 41 | 0.47% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 42 | 0.46% | `net.minecraft.world.entity.Mob.isPersistenceRequired() line: 1308` |
| 43 | 0.44% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 73` |
| 44 | 0.42% | `sun.misc.Unsafe.getBoolean(Object, long) line: 253` |
| 45 | 0.42% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 46 | 0.40% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 12438` |
| 47 | 0.39% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 48 | 0.38% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 49 | 0.37% | `dev.planeta9091.centities.CeHooks$NmMap.probe(int) line: 8943` |
| 50 | 0.36% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 51 | 0.34% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 52 | 0.33% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7347` |
| 53 | 0.33% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1484` |
| 54 | 0.33% | `dev.planeta9091.centities.CeHooks.equipUpdates$(LivingEntity) line: 12334` |
| 55 | 0.32% | `net.minecraft.world.entity.LivingEntity.getItemBySlot(EquipmentSlot) line: 2901` |
| 56 | 0.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 57 | 0.31% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 58 | 0.31% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 8379` |
| 59 | 0.28% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 60 | 0.27% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 11844` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 8.67% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 2 | 8.58% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 4.50% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 4 | 4.38% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 2.39% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 6 | 2.32% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2817` |
| 7 | 2.27% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2684` |
| 8 | 2.25% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 9 | 1.76% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 10 | 1.50% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 11 | 1.38% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 12 | 1.35% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3696` |
| 13 | 1.32% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 14 | 1.22% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12073` |
| 15 | 1.14% | `net.minecraft.world.entity.Entity.isEyeInFluid(TagKey) line: 2311` |
| 16 | 1.10% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 17 | 1.09% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 8390` |
| 18 | 1.00% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 19 | 0.93% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 5123` |
| 20 | 0.91% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 21 | 0.87% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 22 | 0.85% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 439` |
| 23 | 0.79% | `dev.planeta9091.centities.CeHooks.nativeMobBoundary(Level) line: 10256` |
| 24 | 0.78% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 25 | 0.78% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 146` |
| 26 | 0.78% | `net.minecraft.world.entity.LivingEntity.tick() line: 3491` |
| 27 | 0.75% | `net.minecraft.world.level.LevelHeightAccessor.isInsideBuildHeight(BlockPos) line: 28` |
| 28 | 0.70% | `net.minecraft.server.MinecraftServer.tickMidTickTasks() line: 448` |
| 29 | 0.63% | `net.minecraft.world.phys.AABB.inflate(double) line: 189` |
| 30 | 0.62% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |

GC pauses: 0, total 0 ms

