# JVM hotspots (JFR ExecutionSample)

Samples: 35626

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.52% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 4.94% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 2.88% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 2.70% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 5 | 1.95% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 6 | 1.94% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6444` |
| 7 | 1.87% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 8 | 1.79% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4198` |
| 9 | 1.77% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 10 | 1.70% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 11 | 1.65% | `java.util.EnumMap$EnumMapIterator.hasNext() line: 524` |
| 12 | 1.43% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1426` |
| 13 | 1.31% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 14 | 1.30% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 15 | 1.29% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1407` |
| 16 | 1.22% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 17 | 1.19% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 18 | 1.13% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 19 | 1.11% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4053` |
| 20 | 1.04% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 21 | 1.03% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 22 | 0.96% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getAcquire(VarHandle, Object) line: 113` |
| 23 | 0.93% | `java.util.EnumMap.get(Object) line: 248` |
| 24 | 0.86% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 25 | 0.81% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 26 | 0.80% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 27 | 0.78% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 28 | 0.69% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 29 | 0.68% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 30 | 0.68% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 31 | 0.63% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 32 | 0.62% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 33 | 0.59% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 34 | 0.59% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 4376` |
| 35 | 0.57% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 36 | 0.54% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 37 | 0.52% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 38 | 0.51% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 39 | 0.51% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1410` |
| 40 | 0.50% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4721` |
| 41 | 0.50% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 42 | 0.49% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 43 | 0.49% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 44 | 0.48% | `net.minecraft.world.entity.Entity.computeSpeed() line: 961` |
| 45 | 0.48% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 5112` |
| 46 | 0.47% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 47 | 0.47% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 48 | 0.46% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 2560` |
| 49 | 0.46% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 50 | 0.44% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 51 | 0.43% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 52 | 0.43% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 53 | 0.42% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 54 | 0.42% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 55 | 0.40% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3696` |
| 56 | 0.40% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 57 | 0.39% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 58 | 0.39% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getAttributesToSync() line: 34` |
| 59 | 0.38% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2076` |
| 60 | 0.38% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4664` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 8.22% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.78% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1060` |
| 3 | 2.88% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 2.62% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 5 | 2.36% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 6 | 2.15% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4003` |
| 7 | 2.14% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 8 | 2.10% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6444` |
| 9 | 1.97% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk)` |
| 10 | 1.87% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 11 | 1.66% | `net.minecraft.world.entity.EntityEquipment.tick(Entity) line: 50` |
| 12 | 1.56% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 13 | 1.44% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4200` |
| 14 | 1.39% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 15 | 1.34% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1411` |
| 16 | 1.28% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 17 | 1.22% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 18 | 1.20% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 19 | 1.16% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 20 | 1.15% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 21 | 1.14% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 22 | 1.12% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4731` |
| 23 | 1.09% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 24 | 1.05% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 25 | 1.04% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 26 | 1.00% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4663` |
| 27 | 0.93% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getNextAcquire() line: 1837` |
| 28 | 0.92% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 29 | 0.83% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 30 | 0.82% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |

GC pauses: 0, total 0 ms

