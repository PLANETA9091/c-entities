# JVM hotspots (JFR ExecutionSample)

Samples: 24547

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.97% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.58% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4833` |
| 3 | 3.17% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7773` |
| 4 | 3.12% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7914` |
| 5 | 2.96% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.83% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 2.75% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 8 | 2.49% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7720` |
| 9 | 1.96% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4688` |
| 10 | 1.75% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 11 | 1.70% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 12 | 1.66% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 1.46% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 14 | 1.17% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 15 | 1.11% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 16 | 1.10% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 17 | 1.10% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 18 | 1.10% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4115` |
| 19 | 1.07% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 20 | 1.01% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 21 | 0.99% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 22 | 0.94% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2811` |
| 23 | 0.90% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 24 | 0.81% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 25 | 0.76% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 26 | 0.62% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 27 | 0.62% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 28 | 0.60% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1986` |
| 29 | 0.60% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 30 | 0.60% | `java.lang.ref.Reference.get() line: 361` |
| 31 | 0.59% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 32 | 0.59% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 33 | 0.55% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 34 | 0.52% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 35 | 0.52% | `java.util.EnumMap.get(Object) line: 248` |
| 36 | 0.51% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 37 | 0.50% | `net.minecraft.world.entity.Entity.computeSpeed() line: 961` |
| 38 | 0.50% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 39 | 0.49% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 40 | 0.47% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 41 | 0.46% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7734` |
| 42 | 0.46% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 43 | 0.46% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 44 | 0.45% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 45 | 0.43% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 46 | 0.42% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 47 | 0.42% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 5356` |
| 48 | 0.41% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 49 | 0.39% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2713` |
| 50 | 0.39% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 51 | 0.38% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 52 | 0.37% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 53 | 0.37% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 54 | 0.35% | `net.minecraft.world.level.chunk.ChunkAccess.getPos() line: 279` |
| 55 | 0.35% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2005` |
| 56 | 0.34% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 603` |
| 57 | 0.33% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 58 | 0.33% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 59 | 0.33% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 60 | 0.33% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.79% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.15% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 3.32% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 3.16% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7769` |
| 5 | 3.12% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 6 | 2.75% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 7 | 2.25% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 8 | 1.98% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 9 | 1.75% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 10 | 1.70% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 11 | 1.62% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 12 | 1.54% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1528` |
| 13 | 1.52% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 14 | 1.50% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 15 | 1.46% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 16 | 1.21% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 17 | 1.19% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7761` |
| 18 | 1.14% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 19 | 1.12% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 20 | 1.07% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 21 | 1.06% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4152` |
| 22 | 1.05% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 898` |
| 23 | 1.04% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 24 | 0.98% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2734` |
| 25 | 0.81% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2195` |
| 26 | 0.77% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 27 | 0.77% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4835` |
| 28 | 0.73% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 29 | 0.73% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 5366` |
| 30 | 0.67% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 5727` |

GC pauses: 0, total 0 ms

