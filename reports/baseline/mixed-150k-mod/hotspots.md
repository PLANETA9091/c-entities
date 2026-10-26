# JVM hotspots (JFR ExecutionSample)

Samples: 36116

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.92% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 4.45% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 2.91% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 4 | 2.70% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 5 | 2.38% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 2.33% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 7 | 1.95% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4198` |
| 8 | 1.86% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6454` |
| 9 | 1.81% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 10 | 1.64% | `java.util.EnumMap$EnumMapIterator.hasNext() line: 524` |
| 11 | 1.63% | `dev.planeta9091.centities.CeHooks.setSharedFlag$(Entity, int, boolean) line: 6343` |
| 12 | 1.41% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 13 | 1.40% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 14 | 1.36% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1426` |
| 15 | 1.36% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1407` |
| 16 | 1.31% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 17 | 1.18% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 18 | 1.11% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4053` |
| 19 | 0.95% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 20 | 0.93% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 21 | 0.91% | `java.util.EnumMap.get(Object) line: 248` |
| 22 | 0.82% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 23 | 0.78% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 24 | 0.75% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 25 | 0.73% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 26 | 0.70% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 27 | 0.67% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 28 | 0.62% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 4376` |
| 29 | 0.61% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 30 | 0.59% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1410` |
| 31 | 0.54% | `net.minecraft.world.entity.Entity.computeSpeed() line: 961` |
| 32 | 0.54% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 33 | 0.53% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 34 | 0.53% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 35 | 0.52% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 36 | 0.52% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 37 | 0.50% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 38 | 0.50% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 39 | 0.48% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 40 | 0.47% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 41 | 0.46% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 42 | 0.45% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4721` |
| 43 | 0.45% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 44 | 0.43% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 45 | 0.43% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 46 | 0.43% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 47 | 0.42% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 48 | 0.42% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2076` |
| 49 | 0.42% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 50 | 0.42% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 51 | 0.41% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3696` |
| 52 | 0.40% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 2560` |
| 53 | 0.40% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 5122` |
| 54 | 0.39% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity) line: 619` |
| 55 | 0.38% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 56 | 0.38% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 2497` |
| 57 | 0.38% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 58 | 0.35% | `dev.planeta9091.centities.CeHooks.aiShadowPeek(Object) line: 4410` |
| 59 | 0.35% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 60 | 0.35% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.15% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.89% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1060` |
| 3 | 2.90% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 4 | 2.39% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 5 | 2.38% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 2.28% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 7 | 2.23% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4003` |
| 8 | 2.22% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6454` |
| 9 | 2.14% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 10 | 1.87% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk)` |
| 11 | 1.69% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 12 | 1.64% | `net.minecraft.world.entity.EntityEquipment.tick(Entity) line: 50` |
| 13 | 1.63% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean)` |
| 14 | 1.48% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4200` |
| 15 | 1.37% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 16 | 1.36% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1411` |
| 17 | 1.32% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 18 | 1.20% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4731` |
| 19 | 1.20% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 20 | 1.18% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 21 | 1.16% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 22 | 1.07% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4663` |
| 23 | 0.98% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 24 | 0.96% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 25 | 0.91% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 26 | 0.87% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 27 | 0.84% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 28 | 0.76% | `dev.planeta9091.centities.CeHooks.entityDataValue(Entity, Object) line: 5001` |
| 29 | 0.76% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 30 | 0.68% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |

GC pauses: 0, total 0 ms

