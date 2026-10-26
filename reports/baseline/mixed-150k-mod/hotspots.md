# JVM hotspots (JFR ExecutionSample)

Samples: 36141

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.01% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 4.11% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 2.64% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 6461` |
| 4 | 2.02% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4197` |
| 5 | 1.67% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 6 | 1.56% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 1.54% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1425` |
| 8 | 1.41% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6418` |
| 9 | 1.34% | `dev.planeta9091.centities.CeHooks.setSharedFlag$(Entity, int, boolean) line: 6307` |
| 10 | 1.33% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 11 | 1.33% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 12 | 1.25% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 13 | 1.13% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 14 | 1.11% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 15 | 1.08% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 16 | 0.96% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4052` |
| 17 | 0.94% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 18 | 0.90% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 19 | 0.82% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1406` |
| 20 | 0.78% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 21 | 0.71% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 22 | 0.71% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 23 | 0.70% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1409` |
| 24 | 0.69% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 25 | 0.67% | `java.util.EnumMap.get(Object) line: 248` |
| 26 | 0.65% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 27 | 0.64% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 28 | 0.63% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 29 | 0.59% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 30 | 0.56% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 31 | 0.55% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 32 | 0.54% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 33 | 0.52% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 34 | 0.49% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 35 | 0.49% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 36 | 0.48% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 37 | 0.45% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 38 | 0.44% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 4375` |
| 39 | 0.44% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 40 | 0.44% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 41 | 0.43% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 42 | 0.42% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 43 | 0.42% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 44 | 0.40% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 45 | 0.39% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 46 | 0.39% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 47 | 0.38% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 48 | 0.37% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 49 | 0.37% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 50 | 0.36% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 51 | 0.36% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 52 | 0.34% | `dev.planeta9091.centities.CeHooks.aiShadowPeek(Object) line: 4409` |
| 53 | 0.34% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 54 | 0.33% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 55 | 0.32% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2075` |
| 56 | 0.32% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 57 | 0.31% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 58 | 0.31% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 2559` |
| 59 | 0.30% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 60 | 0.29% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.66% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.93% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1059` |
| 3 | 2.73% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6418` |
| 4 | 2.65% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 5 | 2.27% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 6 | 2.18% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4002` |
| 7 | 1.56% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 8 | 1.42% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk)` |
| 9 | 1.37% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1410` |
| 10 | 1.35% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean)` |
| 11 | 1.33% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 12 | 1.32% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 13 | 1.31% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4199` |
| 14 | 1.31% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 15 | 1.26% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 16 | 1.22% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 17 | 1.16% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 18 | 1.13% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 19 | 1.13% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 20 | 1.08% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 21 | 1.02% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 22 | 0.78% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4662` |
| 23 | 0.78% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 24 | 0.77% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 25 | 0.74% | `net.minecraft.world.entity.LivingEntity.detectEquipmentUpdates() line: 3558` |
| 26 | 0.74% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 27 | 0.72% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 28 | 0.70% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 29 | 0.68% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 30 | 0.67% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |

GC pauses: 0, total 0 ms

