# JVM hotspots (JFR ExecutionSample)

Samples: 26386

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.08% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.49% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8125` |
| 3 | 3.23% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5020` |
| 4 | 3.19% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7984` |
| 5 | 2.85% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.68% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 2.47% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7931` |
| 8 | 2.06% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4875` |
| 9 | 1.76% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 10 | 1.75% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 11 | 1.70% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 1.66% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 13 | 1.57% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 14 | 1.24% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 15 | 1.13% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 16 | 1.13% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 17 | 1.11% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 18 | 1.08% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4302` |
| 19 | 1.07% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 20 | 1.00% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 21 | 0.99% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 22 | 0.98% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 23 | 0.88% | `java.util.EnumMap.get(Object) line: 248` |
| 24 | 0.83% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2997` |
| 25 | 0.83% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 26 | 0.81% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 27 | 0.81% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 28 | 0.80% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 29 | 0.71% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 30 | 0.65% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5025` |
| 31 | 0.64% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 32 | 0.62% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 33 | 0.61% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 34 | 0.60% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 35 | 0.60% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 36 | 0.59% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 37 | 0.54% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 38 | 0.52% | `net.minecraft.world.entity.Entity.computeSpeed() line: 961` |
| 39 | 0.52% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 40 | 0.50% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 41 | 0.47% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 42 | 0.47% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 43 | 0.47% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 44 | 0.47% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 45 | 0.45% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2142` |
| 46 | 0.44% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 47 | 0.42% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 48 | 0.42% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2123` |
| 49 | 0.42% | `java.util.ArrayDeque.pollFirst() line: 378` |
| 50 | 0.42% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 243` |
| 51 | 0.41% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2889` |
| 52 | 0.41% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 53 | 0.41% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 54 | 0.40% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 55 | 0.38% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 56 | 0.37% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 57 | 0.36% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 58 | 0.35% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 59 | 0.34% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 60 | 0.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.76% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.38% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 3.49% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 4 | 3.40% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 2.96% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7980` |
| 6 | 2.38% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 7 | 2.24% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 8 | 2.07% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 9 | 1.66% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 10 | 1.61% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 11 | 1.58% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 12 | 1.57% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 13 | 1.56% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 14 | 1.47% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7997` |
| 15 | 1.39% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1665` |
| 16 | 1.37% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 17 | 1.13% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 18 | 1.12% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 19 | 1.08% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7972` |
| 20 | 1.06% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 21 | 1.02% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 22 | 1.00% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4339` |
| 23 | 1.00% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 24 | 0.93% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 898` |
| 25 | 0.89% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2910` |
| 26 | 0.88% | `net.minecraft.world.entity.Mob.inactiveTick() line: 229` |
| 27 | 0.79% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4875` |
| 28 | 0.74% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5022` |
| 29 | 0.73% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 30 | 0.72% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2195` |

GC pauses: 0, total 0 ms

