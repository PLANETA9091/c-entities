# JVM hotspots (JFR ExecutionSample)

Samples: 39536

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.73% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 3.05% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 2.69% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 2.53% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 6660` |
| 5 | 1.95% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4202` |
| 6 | 1.78% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 7 | 1.75% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1430` |
| 8 | 1.75% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 9 | 1.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 10 | 1.38% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.31% | `dev.planeta9091.centities.CeHooks.setSharedFlag$(Entity, int, boolean) line: 6496` |
| 12 | 1.23% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 13 | 1.21% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6607` |
| 14 | 1.12% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 15 | 1.01% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 16 | 1.00% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4057` |
| 17 | 0.93% | `java.util.EnumMap.get(Object) line: 248` |
| 18 | 0.92% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 19 | 0.81% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 20 | 0.81% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1411` |
| 21 | 0.79% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1414` |
| 22 | 0.72% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 23 | 0.69% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 24 | 0.68% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 25 | 0.68% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 26 | 0.67% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 27 | 0.67% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 28 | 0.65% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 29 | 0.65% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 30 | 0.65% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 31 | 0.53% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 32 | 0.47% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 33 | 0.47% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 4380` |
| 34 | 0.46% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 35 | 0.45% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 36 | 0.43% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 37 | 0.43% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 38 | 0.42% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 39 | 0.40% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 40 | 0.40% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 41 | 0.39% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 42 | 0.37% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 43 | 0.37% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 44 | 0.37% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 45 | 0.37% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 46 | 0.35% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 47 | 0.34% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2080` |
| 48 | 0.33% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 49 | 0.33% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 50 | 0.32% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 51 | 0.32% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 52 | 0.32% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 53 | 0.32% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 54 | 0.32% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 55 | 0.32% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 4369` |
| 56 | 0.31% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 57 | 0.31% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 58 | 0.31% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 59 | 0.30% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 60 | 0.29% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.42% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.19% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1064` |
| 3 | 2.69% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 2.53% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 5 | 2.38% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6607` |
| 6 | 2.24% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 7 | 2.16% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4007` |
| 8 | 1.58% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 9 | 1.56% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 10 | 1.45% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 11 | 1.45% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1415` |
| 12 | 1.32% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean)` |
| 13 | 1.26% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 14 | 1.22% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk)` |
| 15 | 1.17% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 16 | 1.16% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 17 | 1.15% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 18 | 1.13% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4204` |
| 19 | 1.11% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 20 | 1.11% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 21 | 1.01% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 22 | 0.87% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4667` |
| 23 | 0.75% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 24 | 0.72% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 25 | 0.70% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 26 | 0.70% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 27 | 0.67% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 28 | 0.66% | `net.minecraft.world.entity.LivingEntity.detectEquipmentUpdates() line: 3558` |
| 29 | 0.64% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 30 | 0.63% | `dev.planeta9091.centities.CeHooks.entityDataValue(Entity, Object) line: 5005` |

GC pauses: 0, total 0 ms

