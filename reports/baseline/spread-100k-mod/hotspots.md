# JVM hotspots (JFR ExecutionSample)

Samples: 29629

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.55% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.10% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4932` |
| 3 | 2.87% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 4 | 2.74% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 5 | 2.72% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7872` |
| 6 | 2.50% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.21% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7819` |
| 8 | 2.00% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4787` |
| 9 | 1.80% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8013` |
| 10 | 1.70% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 11 | 1.67% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 12 | 1.62% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 13 | 1.61% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 14 | 1.51% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 15 | 1.25% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 16 | 1.21% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 17 | 1.17% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 18 | 1.17% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 19 | 1.14% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 20 | 1.10% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 21 | 1.06% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 22 | 1.06% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 23 | 0.99% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4214` |
| 24 | 0.99% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 25 | 0.96% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 26 | 0.88% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2054` |
| 27 | 0.87% | `java.util.EnumMap.get(Object) line: 248` |
| 28 | 0.87% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 29 | 0.80% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2909` |
| 30 | 0.78% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 31 | 0.78% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 32 | 0.78% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 33 | 0.78% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 34 | 0.77% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 35 | 0.74% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 36 | 0.71% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 37 | 0.69% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 38 | 0.65% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 39 | 0.61% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 40 | 0.61% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 41 | 0.59% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 42 | 0.58% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2035` |
| 43 | 0.57% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 44 | 0.57% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 45 | 0.56% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 46 | 0.55% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 47 | 0.54% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 48 | 0.53% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 49 | 0.50% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 50 | 0.49% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 51 | 0.44% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 52 | 0.43% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 53 | 0.43% | `net.minecraft.world.entity.Entity.computeSpeed() line: 961` |
| 54 | 0.43% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7833` |
| 55 | 0.41% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4934` |
| 56 | 0.39% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 57 | 0.38% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 58 | 0.38% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 59 | 0.37% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2801` |
| 60 | 0.36% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.41% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.73% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 2.87% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 2.82% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7868` |
| 5 | 2.74% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 6 | 2.33% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1577` |
| 7 | 2.28% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 8 | 2.03% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 9 | 1.80% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 10 | 1.70% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 11 | 1.67% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 12 | 1.61% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 13 | 1.55% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 14 | 1.40% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 15 | 1.33% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 16 | 1.30% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 17 | 1.29% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 18 | 1.21% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 19 | 1.17% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 20 | 1.12% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 21 | 1.10% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 22 | 1.02% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2039` |
| 23 | 1.00% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 24 | 1.00% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 25 | 0.94% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 898` |
| 26 | 0.92% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7860` |
| 27 | 0.92% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 28 | 0.84% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2822` |
| 29 | 0.78% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |
| 30 | 0.78% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |

GC pauses: 0, total 0 ms

