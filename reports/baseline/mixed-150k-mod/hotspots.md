# JVM hotspots (JFR ExecutionSample)

Samples: 35581

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.85% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 3.63% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 2.04% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4203` |
| 4 | 1.98% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 1.77% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 6923` |
| 6 | 1.71% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 1.51% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 8 | 1.43% | `dev.planeta9091.centities.CeHooks.setSharedFlag$(Entity, int, boolean) line: 6759` |
| 9 | 1.42% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 10 | 1.31% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6870` |
| 11 | 1.29% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1431` |
| 12 | 1.16% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 1.14% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 14 | 1.06% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 15 | 1.05% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 16 | 1.01% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 17 | 0.95% | `java.util.EnumMap.get(Object) line: 248` |
| 18 | 0.90% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 19 | 0.87% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4058` |
| 20 | 0.79% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1412` |
| 21 | 0.78% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 22 | 0.77% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 23 | 0.76% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 24 | 0.74% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 25 | 0.74% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 26 | 0.67% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 27 | 0.67% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 28 | 0.65% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1415` |
| 29 | 0.64% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 30 | 0.62% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 31 | 0.59% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 32 | 0.59% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 33 | 0.51% | `dev.planeta9091.centities.CeHooks.aiPackSel(int[], int, WrappedGoal[], int[], int[], long[], int[], int[], int[], float[], Mob) line: 6079` |
| 34 | 0.51% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 35 | 0.50% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 36 | 0.48% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 37 | 0.47% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 38 | 0.46% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 39 | 0.42% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 237` |
| 40 | 0.40% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 41 | 0.38% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 42 | 0.38% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 43 | 0.38% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 44 | 0.36% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 45 | 0.36% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 46 | 0.35% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 47 | 0.35% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 48 | 0.33% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 49 | 0.32% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 50 | 0.32% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2081` |
| 51 | 0.31% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 52 | 0.30% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 53 | 0.30% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 54 | 0.30% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 55 | 0.30% | `java.util.ArrayList.add(Object, Object[], int) line: 483` |
| 56 | 0.29% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 2565` |
| 57 | 0.29% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 58 | 0.29% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 59 | 0.29% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 60 | 0.28% | `net.minecraft.world.entity.LivingEntity$SwingState.tick() line: 5015` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.34% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.47% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1065` |
| 3 | 2.63% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6870` |
| 4 | 2.29% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 2.17% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4008` |
| 6 | 1.98% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 1.80% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 8 | 1.58% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 9 | 1.45% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 10 | 1.44% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean)` |
| 11 | 1.36% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 12 | 1.34% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk)` |
| 13 | 1.30% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 14 | 1.19% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1416` |
| 15 | 1.14% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 16 | 1.11% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 17 | 1.08% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4205` |
| 18 | 1.06% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 19 | 1.05% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 20 | 1.01% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 21 | 0.97% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 22 | 0.80% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 23 | 0.77% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 24 | 0.77% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 25 | 0.75% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 26 | 0.74% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 27 | 0.73% | `net.minecraft.world.entity.LivingEntity.detectEquipmentUpdates() line: 3558` |
| 28 | 0.71% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 29 | 0.70% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 30 | 0.68% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |

GC pauses: 0, total 0 ms

