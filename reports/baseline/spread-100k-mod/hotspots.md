# JVM hotspots (JFR ExecutionSample)

Samples: 31080

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.83% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4886` |
| 2 | 3.82% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.50% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7826` |
| 4 | 2.82% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.68% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 6 | 2.44% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7967` |
| 7 | 2.29% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 2.11% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7773` |
| 9 | 1.88% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 10 | 1.56% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4741` |
| 11 | 1.53% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 12 | 1.48% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 13 | 1.34% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4168` |
| 14 | 1.29% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 15 | 1.14% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 16 | 1.12% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 17 | 1.09% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 18 | 1.00% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 19 | 0.98% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 20 | 0.96% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 21 | 0.93% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 22 | 0.92% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2018` |
| 23 | 0.89% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 24 | 0.88% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 25 | 0.86% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 26 | 0.85% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 27 | 0.83% | `java.util.EnumMap.get(Object) line: 248` |
| 28 | 0.81% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 29 | 0.77% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 30 | 0.77% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 31 | 0.75% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 32 | 0.75% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 33 | 0.72% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 34 | 0.71% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 35 | 0.64% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 36 | 0.62% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 37 | 0.61% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 993` |
| 38 | 0.60% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 39 | 0.59% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 40 | 0.57% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 41 | 0.54% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2863` |
| 42 | 0.54% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 43 | 0.53% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2002` |
| 44 | 0.52% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 45 | 0.52% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7787` |
| 46 | 0.48% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 47 | 0.46% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 48 | 0.46% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 49 | 0.45% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 50 | 0.41% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 51 | 0.41% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 52 | 0.40% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 53 | 0.40% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 54 | 0.39% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 55 | 0.39% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 56 | 0.37% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 57 | 0.36% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 58 | 0.34% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 59 | 0.32% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 60 | 0.32% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.11% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.31% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 3.57% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 2.80% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7822` |
| 5 | 2.69% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 6 | 2.44% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 7 | 2.22% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1541` |
| 8 | 2.14% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 9 | 1.88% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 10 | 1.67% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 11 | 1.66% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 12 | 1.59% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 13 | 1.48% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 14 | 1.22% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 15 | 1.17% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7839` |
| 16 | 1.15% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 17 | 1.15% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7814` |
| 18 | 1.14% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 19 | 1.09% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2003` |
| 20 | 1.06% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4205` |
| 21 | 1.02% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 22 | 0.90% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 23 | 0.88% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 24 | 0.88% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 25 | 0.88% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 26 | 0.83% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 27 | 0.83% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 28 | 0.76% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 29 | 0.75% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |
| 30 | 0.74% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |

GC pauses: 0, total 0 ms

