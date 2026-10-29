# JVM hotspots (JFR ExecutionSample)

Samples: 25242

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.43% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.51% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4896` |
| 3 | 3.47% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7977` |
| 4 | 3.32% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7836` |
| 5 | 2.77% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.74% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 2.51% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7783` |
| 8 | 2.06% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4751` |
| 9 | 1.79% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 10 | 1.78% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 11 | 1.68% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 12 | 1.48% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 13 | 1.20% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 14 | 1.19% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 15 | 1.18% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 16 | 1.17% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 17 | 1.17% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 18 | 1.13% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 19 | 1.12% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4178` |
| 20 | 1.08% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 21 | 0.96% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 22 | 0.88% | `java.util.EnumMap.get(Object) line: 248` |
| 23 | 0.86% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 24 | 0.86% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 25 | 0.85% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2873` |
| 26 | 0.80% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 27 | 0.74% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 28 | 0.70% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 29 | 0.67% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 30 | 0.66% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 31 | 0.63% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 32 | 0.59% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 33 | 0.59% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1999` |
| 34 | 0.57% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 35 | 0.57% | `net.minecraft.world.entity.Entity.computeSpeed() line: 961` |
| 36 | 0.55% | `jdk.internal.util.ArraysSupport.hashCodeOfUnsigned(byte[], int, int, int) line: 275` |
| 37 | 0.55% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 38 | 0.54% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 39 | 0.53% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 40 | 0.50% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 41 | 0.50% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2018` |
| 42 | 0.47% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 43 | 0.46% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7922` |
| 44 | 0.46% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 45 | 0.44% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 46 | 0.44% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 47 | 0.43% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 48 | 0.43% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 49 | 0.42% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2765` |
| 50 | 0.41% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 51 | 0.39% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 52 | 0.39% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 53 | 0.39% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 54 | 0.38% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 55 | 0.37% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 56 | 0.36% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 57 | 0.35% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 58 | 0.35% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 59 | 0.35% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 60 | 0.34% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.16% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.14% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 3.49% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 3.47% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 5 | 3.11% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7832` |
| 6 | 2.35% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 7 | 2.07% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 8 | 1.78% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 9 | 1.73% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 10 | 1.72% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1541` |
| 11 | 1.51% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 12 | 1.49% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 13 | 1.47% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 14 | 1.41% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 15 | 1.24% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 16 | 1.15% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7849` |
| 17 | 1.14% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 18 | 1.13% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7824` |
| 19 | 1.11% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 20 | 1.06% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 21 | 1.05% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4215` |
| 22 | 1.02% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 23 | 0.91% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2786` |
| 24 | 0.86% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 25 | 0.86% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 26 | 0.83% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 898` |
| 27 | 0.80% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4751` |
| 28 | 0.78% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4898` |
| 29 | 0.77% | `net.minecraft.world.entity.Mob.inactiveTick() line: 229` |
| 30 | 0.70% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |

GC pauses: 0, total 0 ms

