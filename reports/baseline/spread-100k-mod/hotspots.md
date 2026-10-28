# JVM hotspots (JFR ExecutionSample)

Samples: 33882

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.58% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7914` |
| 2 | 3.22% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7773` |
| 3 | 3.00% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 2.94% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4833` |
| 5 | 2.64% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 6 | 2.46% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.35% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7720` |
| 8 | 1.92% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 1.92% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 10 | 1.62% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4115` |
| 11 | 1.52% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.23% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 13 | 1.21% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4688` |
| 14 | 1.14% | `java.lang.ref.Reference.get() line: 361` |
| 15 | 1.07% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 16 | 0.93% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 17 | 0.85% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 18 | 0.80% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 19 | 0.73% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 20 | 0.73% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 21 | 0.73% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 22 | 0.71% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 23 | 0.71% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 24 | 0.68% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 25 | 0.64% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 26 | 0.64% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 27 | 0.63% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 28 | 0.63% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 29 | 0.62% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2005` |
| 30 | 0.62% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 31 | 0.60% | `java.util.EnumMap.get(Object) line: 248` |
| 32 | 0.55% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 33 | 0.54% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4838` |
| 34 | 0.53% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 35 | 0.52% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 36 | 0.51% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 37 | 0.51% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 38 | 0.50% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 39 | 0.50% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2811` |
| 40 | 0.48% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 41 | 0.48% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 42 | 0.47% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 43 | 0.46% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7859` |
| 44 | 0.45% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 45 | 0.43% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 46 | 0.43% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 47 | 0.43% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 48 | 0.42% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 49 | 0.41% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 50 | 0.41% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 5356` |
| 51 | 0.40% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 52 | 0.38% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 53 | 0.37% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 54 | 0.35% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 55 | 0.35% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1986` |
| 56 | 0.35% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 57 | 0.35% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 58 | 0.35% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 5000` |
| 59 | 0.34% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 60 | 0.34% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 5299` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.52% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.86% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 3.59% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 4 | 3.33% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 2.74% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7769` |
| 6 | 2.64% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 7 | 1.97% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 8 | 1.92% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 1.92% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 10 | 1.82% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1528` |
| 11 | 1.72% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 12 | 1.38% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 13 | 1.25% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 14 | 1.22% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 15 | 1.13% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7761` |
| 16 | 1.00% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4152` |
| 17 | 0.89% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 18 | 0.86% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 5366` |
| 19 | 0.86% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 5011` |
| 20 | 0.85% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 21 | 0.77% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 22 | 0.77% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 23 | 0.76% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 24 | 0.75% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 25 | 0.75% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |
| 26 | 0.72% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 27 | 0.66% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 28 | 0.66% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1990` |
| 29 | 0.66% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 30 | 0.62% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |

GC pauses: 0, total 0 ms

