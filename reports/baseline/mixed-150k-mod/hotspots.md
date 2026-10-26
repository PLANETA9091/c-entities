# JVM hotspots (JFR ExecutionSample)

Samples: 43834

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.09% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 4.13% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.02% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 2.84% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 6473` |
| 5 | 2.22% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 6 | 2.03% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4197` |
| 7 | 1.81% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 1.78% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1425` |
| 9 | 1.41% | `dev.planeta9091.centities.CeHooks.setSharedFlag$(Entity, int, boolean) line: 6309` |
| 10 | 1.36% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6420` |
| 11 | 1.29% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 12 | 1.22% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 13 | 1.13% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 14 | 1.08% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 15 | 1.05% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1409` |
| 16 | 1.01% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 17 | 0.94% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 18 | 0.90% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 19 | 0.86% | `java.util.EnumMap.get(Object) line: 248` |
| 20 | 0.85% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 21 | 0.78% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1406` |
| 22 | 0.76% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 23 | 0.74% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 24 | 0.74% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 25 | 0.72% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 26 | 0.68% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 27 | 0.67% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4052` |
| 28 | 0.66% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 29 | 0.62% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 30 | 0.59% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 31 | 0.58% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 32 | 0.54% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 33 | 0.53% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 34 | 0.52% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 35 | 0.51% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 36 | 0.49% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 37 | 0.47% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 38 | 0.47% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 39 | 0.47% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 40 | 0.46% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 4375` |
| 41 | 0.43% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 42 | 0.39% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 43 | 0.38% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 44 | 0.37% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 45 | 0.36% | `net.minecraft.world.entity.SteppedInterpolationTracker.updateTracking(Vec3) line: 40` |
| 46 | 0.36% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 47 | 0.36% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 48 | 0.35% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 49 | 0.34% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 50 | 0.33% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 51 | 0.33% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 52 | 0.33% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 53 | 0.32% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2075` |
| 54 | 0.32% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 55 | 0.31% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 56 | 0.31% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 57 | 0.31% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 2559` |
| 58 | 0.31% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 59 | 0.30% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 6522` |
| 60 | 0.29% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.94% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.53% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1059` |
| 3 | 3.02% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 2.84% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 5 | 2.52% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6420` |
| 6 | 2.25% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 7 | 2.06% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4002` |
| 8 | 1.67% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1410` |
| 9 | 1.43% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 10 | 1.41% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean)` |
| 11 | 1.37% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk)` |
| 12 | 1.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 13 | 1.26% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 14 | 1.26% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 15 | 1.25% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 16 | 1.09% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 17 | 1.08% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 18 | 1.07% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 19 | 0.94% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4199` |
| 20 | 0.90% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 21 | 0.87% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 22 | 0.82% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4662` |
| 23 | 0.77% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 24 | 0.73% | `net.minecraft.world.entity.LivingEntity.detectEquipmentUpdates() line: 3558` |
| 25 | 0.69% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 26 | 0.69% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 27 | 0.69% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4730` |
| 28 | 0.64% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 29 | 0.64% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 30 | 0.62% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |

GC pauses: 0, total 0 ms

