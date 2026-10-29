# JVM hotspots (JFR ExecutionSample)

Samples: 28057

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.30% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 2.99% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8056` |
| 3 | 2.95% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5092` |
| 4 | 2.89% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 2.76% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.62% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8197` |
| 7 | 2.57% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 2.46% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8003` |
| 9 | 2.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 10 | 1.96% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4947` |
| 11 | 1.96% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 12 | 1.71% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 13 | 1.68% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 14 | 1.48% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 15 | 1.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4374` |
| 16 | 1.13% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 17 | 1.12% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 18 | 1.08% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 19 | 1.07% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 20 | 0.98% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 21 | 0.97% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 22 | 0.90% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 23 | 0.89% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3069` |
| 24 | 0.87% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 25 | 0.86% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 26 | 0.84% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 27 | 0.82% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 28 | 0.77% | `java.util.EnumMap.get(Object) line: 248` |
| 29 | 0.76% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 30 | 0.70% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 31 | 0.69% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 32 | 0.68% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 33 | 0.65% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 34 | 0.62% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 35 | 0.60% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 36 | 0.56% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 37 | 0.56% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 38 | 0.55% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 39 | 0.55% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 40 | 0.54% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 41 | 0.53% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 42 | 0.53% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5097` |
| 43 | 0.51% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 44 | 0.50% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 45 | 0.49% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 46 | 0.47% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 47 | 0.46% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 48 | 0.45% | `net.minecraft.world.entity.Entity.computeSpeed() line: 961` |
| 49 | 0.44% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2961` |
| 50 | 0.43% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8017` |
| 51 | 0.42% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 52 | 0.42% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2190` |
| 53 | 0.40% | `java.util.EnumMap$EnumMapIterator.hasNext() line: 524` |
| 54 | 0.39% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 55 | 0.37% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 56 | 0.35% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8142` |
| 57 | 0.34% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 58 | 0.34% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 59 | 0.34% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 60 | 0.34% | `net.minecraft.world.entity.Entity.getType() line: 633` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.86% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.01% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 3.13% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 3.05% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8052` |
| 5 | 2.89% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 2.62% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 7 | 2.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 8 | 1.98% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 9 | 1.96% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 10 | 1.88% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 11 | 1.68% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 12 | 1.64% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 13 | 1.48% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 14 | 1.46% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 15 | 1.29% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 16 | 1.16% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 17 | 1.14% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4411` |
| 18 | 1.13% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 19 | 1.09% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 20 | 1.08% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 21 | 0.98% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 898` |
| 22 | 0.98% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8044` |
| 23 | 0.98% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1709` |
| 24 | 0.92% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2982` |
| 25 | 0.92% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 26 | 0.85% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 27 | 0.84% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 28 | 0.82% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5094` |
| 29 | 0.81% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4947` |
| 30 | 0.76% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |

GC pauses: 0, total 0 ms

