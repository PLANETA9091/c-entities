# JVM hotspots (JFR ExecutionSample)

Samples: 72861

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 26.01% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.74% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 3.61% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 3.27% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 2.62% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 6 | 2.11% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 2.03% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 8 | 1.52% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 544` |
| 9 | 1.48% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 10 | 1.37% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 11 | 1.32% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 892` |
| 12 | 1.27% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 13 | 1.26% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 14 | 1.20% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 15 | 1.14% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 16 | 1.14% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 17 | 0.96% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 18 | 0.90% | `java.util.EnumMap.get(Object) line: 248` |
| 19 | 0.88% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3579` |
| 20 | 0.78% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 21 | 0.73% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3696` |
| 22 | 0.73% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 23 | 0.72% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3869` |
| 24 | 0.71% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 25 | 0.70% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 26 | 0.65% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 27 | 0.62% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 28 | 0.62% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 29 | 0.61% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 30 | 0.60% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 31 | 0.58% | `java.util.function.Predicate.lambda$and$0(Predicate, Object) line: 69` |
| 32 | 0.56% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 33 | 0.52% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 34 | 0.42% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 35 | 0.42% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 36 | 0.41% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 37 | 0.41% | `net.minecraft.world.entity.Entity.getOnPos(float) line: 1575` |
| 38 | 0.40% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 39 | 0.36% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 40 | 0.36% | `net.minecraft.core.Vec3i.getY() line: 67` |
| 41 | 0.36% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 92` |
| 42 | 0.35% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 43 | 0.33% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 77` |
| 44 | 0.32% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 45 | 0.32% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 46 | 0.31% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 47 | 0.31% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 48 | 0.30% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 49 | 0.28% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 50 | 0.28% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 51 | 0.27% | `net.minecraft.world.entity.item.ItemEntity.getItem() line: 513` |
| 52 | 0.26% | `net.minecraft.world.entity.Entity.checkInsideBlocks(List, InsideBlockEffectApplier$StepBasedCollector) line: 1867` |
| 53 | 0.26% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 54 | 0.25% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 55 | 0.24% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 56 | 0.24% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 57 | 0.23% | `net.minecraft.world.entity.Entity.baseTick() line: 914` |
| 58 | 0.23% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 59 | 0.23% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 60 | 0.23% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 26.01% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 6.07% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 3 | 5.38% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 5.16% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 5 | 3.74% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 6 | 2.15% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 7 | 1.88% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 8 | 1.58% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 9 | 1.33% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 10 | 1.32% | `net.minecraft.server.MinecraftServer.tickChildren(BooleanSupplier) line: 1807` |
| 11 | 1.20% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 12 | 1.16% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 13 | 1.06% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 14 | 1.01% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 15 | 0.89% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3685` |
| 16 | 0.81% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 17 | 0.73% | `net.minecraft.world.entity.LivingEntity.tick() line: 3432` |
| 18 | 0.71% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 970` |
| 19 | 0.64% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks(List) line: 1476` |
| 20 | 0.62% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 21 | 0.62% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 22 | 0.61% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 23 | 0.61% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 24 | 0.58% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 550` |
| 25 | 0.57% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 26 | 0.48% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 27 | 0.45% | `net.minecraft.world.entity.item.ItemEntity.tick() line: 179` |
| 28 | 0.45% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3584` |
| 29 | 0.43% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.clear() line: 431` |
| 30 | 0.41% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |

GC pauses: 0, total 0 ms

