# JVM hotspots (JFR ExecutionSample)

Samples: 28384

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.15% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.23% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5100` |
| 3 | 2.87% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8064` |
| 4 | 2.71% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 2.64% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.60% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 2.50% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 2.42% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8205` |
| 9 | 2.41% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8011` |
| 10 | 1.97% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4955` |
| 11 | 1.94% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 12 | 1.84% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 13 | 1.70% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 14 | 1.66% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 15 | 1.56% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 16 | 1.17% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4382` |
| 17 | 1.12% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 18 | 1.07% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 19 | 1.06% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 20 | 1.06% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 21 | 1.01% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 22 | 0.99% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 23 | 0.90% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 24 | 0.85% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3077` |
| 25 | 0.81% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 26 | 0.79% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 27 | 0.74% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 28 | 0.74% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 29 | 0.66% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 30 | 0.65% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 31 | 0.63% | `java.util.EnumMap.get(Object) line: 248` |
| 32 | 0.63% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 33 | 0.62% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 34 | 0.62% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 35 | 0.61% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 36 | 0.60% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 37 | 0.60% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 38 | 0.59% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 39 | 0.58% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 40 | 0.52% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 41 | 0.52% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2969` |
| 42 | 0.51% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 43 | 0.51% | `net.minecraft.world.entity.Entity.computeSpeed() line: 961` |
| 44 | 0.50% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 45 | 0.50% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 46 | 0.48% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5105` |
| 47 | 0.47% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 48 | 0.47% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 49 | 0.46% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 50 | 0.45% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 51 | 0.43% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 52 | 0.39% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 53 | 0.39% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8025` |
| 54 | 0.39% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 55 | 0.39% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2190` |
| 56 | 0.38% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 57 | 0.38% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 58 | 0.38% | `java.util.EnumMap$EnumMapIterator.hasNext() line: 524` |
| 59 | 0.33% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 60 | 0.32% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.75% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.28% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 3.03% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 3.00% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8060` |
| 5 | 2.71% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 2.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 7 | 2.43% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 8 | 2.17% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 9 | 1.99% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 10 | 1.94% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 11 | 1.68% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 12 | 1.63% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 13 | 1.62% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 14 | 1.56% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 15 | 1.55% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 16 | 1.26% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 17 | 1.13% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 18 | 1.10% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 19 | 1.06% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8052` |
| 20 | 1.06% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4419` |
| 21 | 1.05% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 22 | 0.99% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 23 | 0.91% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 24 | 0.90% | `net.minecraft.world.entity.Mob.inactiveTick() line: 229` |
| 25 | 0.89% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1709` |
| 26 | 0.88% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 898` |
| 27 | 0.87% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2990` |
| 28 | 0.85% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5102` |
| 29 | 0.81% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 30 | 0.80% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |

GC pauses: 0, total 0 ms

