# JVM hotspots (JFR ExecutionSample)

Samples: 51406

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.85% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 2 | 5.25% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.75% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 4 | 3.15% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3907` |
| 5 | 2.64% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 6 | 2.42% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 1.79% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 1.76% | `java.util.HashMap.getNode(Object) line: 586` |
| 9 | 1.59% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1048` |
| 10 | 1.57% | `ca.spottedleaf.moonrise.common.util.CoordinateUtils.getChunkKey(BlockPos) line: 15` |
| 11 | 1.52% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 12 | 1.48% | `net.minecraft.world.entity.Entity.level() line: 5415` |
| 13 | 1.37% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 14 | 1.32% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 15 | 1.30% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 16 | 1.15% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 17 | 1.15% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 18 | 1.03% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3868` |
| 19 | 0.98% | `java.util.EnumMap.get(Object) line: 248` |
| 20 | 0.96% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 21 | 0.89% | `net.minecraft.world.entity.LivingEntity.isPushable() line: 4073` |
| 22 | 0.86% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 23 | 0.86% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 24 | 0.85% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 25 | 0.85% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2403` |
| 26 | 0.81% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 27 | 0.74% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3926` |
| 28 | 0.73% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 29 | 0.70% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 30 | 0.70% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 31 | 0.68% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 32 | 0.66% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 33 | 0.58% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 34 | 0.56% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 35 | 0.55% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 36 | 0.53% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 37 | 0.52% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 38 | 0.49% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 39 | 0.49% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 40 | 0.48% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 41 | 0.46% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 42 | 0.45% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 43 | 0.40% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 44 | 0.40% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 45 | 0.39% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 46 | 0.38% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 47 | 0.38% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 48 | 0.38% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 49 | 0.37% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 50 | 0.37% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 237` |
| 51 | 0.36% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1325` |
| 52 | 0.36% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 53 | 0.35% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 54 | 0.34% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 55 | 0.32% | `net.minecraft.world.entity.Entity.isRemoved() line: 5328` |
| 56 | 0.32% | `net.minecraft.world.entity.SteppedInterpolationTracker.updateTracking(Vec3) line: 40` |
| 57 | 0.32% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity) line: 576` |
| 58 | 0.31% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 4089` |
| 59 | 0.30% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 60 | 0.30% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.67% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 6.31% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1049` |
| 3 | 3.75% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 4 | 3.29% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 5 | 3.15% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 6 | 2.66% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 7 | 1.94% | `dev.planeta9091.centities.CeHooks.getEntities(Level, Entity, AABB, Predicate) line: 460` |
| 8 | 1.80% | `java.util.HashMap.containsKey(Object) line: 603` |
| 9 | 1.79% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 10 | 1.64% | `net.minecraft.server.level.ServerLevel.isPositionEntityTicking(BlockPos) line: 2956` |
| 11 | 1.60% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 12 | 1.43% | `net.minecraft.world.entity.LivingEntity.isPushable() line: 4073` |
| 13 | 1.39% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 14 | 1.34% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 15 | 1.34% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2195` |
| 16 | 1.28% | `net.minecraft.world.entity.EntitySelector.lambda$pushableBy$0(Entity, Team, Team$CollisionRule, Entity) line: 52` |
| 17 | 1.25% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 18 | 1.22% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 19 | 1.08% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 20 | 1.00% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 969` |
| 21 | 0.87% | `net.minecraft.world.entity.Entity.setOldPos() line: 2395` |
| 22 | 0.83% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 23 | 0.80% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3810` |
| 24 | 0.75% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 25 | 0.72% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 26 | 0.72% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 27 | 0.71% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2212` |
| 28 | 0.63% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 29 | 0.62% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 30 | 0.58% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |

GC pauses: 0, total 0 ms

