# JVM hotspots (JFR ExecutionSample)

Samples: 36943

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.84% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 3.55% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 2.96% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 6509` |
| 4 | 1.91% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4200` |
| 5 | 1.78% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 6 | 1.55% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 7 | 1.46% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 8 | 1.45% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 1.41% | `dev.planeta9091.centities.CeHooks.setSharedFlag$(Entity, int, boolean) line: 6345` |
| 10 | 1.39% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.36% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1428` |
| 12 | 1.27% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6456` |
| 13 | 1.25% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 14 | 1.14% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 15 | 1.12% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 16 | 1.09% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 17 | 0.89% | `dev.planeta9091.centities.CeHooks.aiPackSel(int[], int, WrappedGoal[], int[], int[], long[], int[], float[], Mob) line: 5885` |
| 18 | 0.85% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4055` |
| 19 | 0.80% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 20 | 0.78% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 21 | 0.73% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 22 | 0.73% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 23 | 0.71% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1409` |
| 24 | 0.70% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1412` |
| 25 | 0.70% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 26 | 0.65% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 27 | 0.64% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 28 | 0.63% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 29 | 0.62% | `java.util.EnumMap.get(Object) line: 248` |
| 30 | 0.60% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 31 | 0.58% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 32 | 0.57% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 33 | 0.57% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 34 | 0.56% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 35 | 0.55% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.<init>(Map, float) line: 124` |
| 36 | 0.49% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 37 | 0.49% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 38 | 0.47% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 39 | 0.46% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 40 | 0.42% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 41 | 0.42% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 42 | 0.40% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 43 | 0.40% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 44 | 0.40% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 45 | 0.40% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 46 | 0.38% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 47 | 0.37% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 603` |
| 48 | 0.36% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 49 | 0.35% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 50 | 0.34% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 51 | 0.33% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 52 | 0.32% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 53 | 0.32% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2078` |
| 54 | 0.32% | `dev.planeta9091.centities.CeHooks.disabledBacking(GoalSelector) line: 4474` |
| 55 | 0.32% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 56 | 0.32% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 57 | 0.31% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 58 | 0.31% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 59 | 0.31% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 60 | 0.30% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.94% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.56% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1062` |
| 3 | 2.96% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 4 | 2.57% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6456` |
| 5 | 2.13% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 6 | 2.12% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4005` |
| 7 | 1.69% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 8 | 1.53% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 9 | 1.50% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 10 | 1.45% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 11 | 1.43% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 12 | 1.42% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean)` |
| 13 | 1.28% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk)` |
| 14 | 1.26% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1413` |
| 15 | 1.26% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 16 | 1.25% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 17 | 1.24% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 18 | 1.19% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 19 | 1.09% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 20 | 1.03% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 21 | 0.99% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 22 | 0.86% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4202` |
| 23 | 0.78% | `dev.planeta9091.centities.CeHooks.aiPack(Mob, CeHooks$AiCtx, int, int, int, double, double, double) line: 5872` |
| 24 | 0.76% | `dev.planeta9091.centities.CeHooks.entityDataValue(Entity, Object) line: 5003` |
| 25 | 0.76% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 26 | 0.75% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 27 | 0.73% | `dev.planeta9091.centities.CeHooks.aiPack(Mob, CeHooks$AiCtx, int, int, int, double, double, double) line: 5870` |
| 28 | 0.70% | `net.minecraft.world.entity.LivingEntity.detectEquipmentUpdates() line: 3558` |
| 29 | 0.66% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 30 | 0.65% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |

GC pauses: 0, total 0 ms

