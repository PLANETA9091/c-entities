# JVM hotspots (JFR ExecutionSample)

Samples: 32833

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.46% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.47% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 3 | 3.45% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8271` |
| 4 | 2.84% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5166` |
| 5 | 2.80% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8077` |
| 6 | 2.53% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.35% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 2.01% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8130` |
| 9 | 1.66% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 10 | 1.46% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 11 | 1.37% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 12 | 1.37% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 13 | 1.29% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 14 | 1.29% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4448` |
| 15 | 1.08% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 16 | 1.08% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5021` |
| 17 | 1.02% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 18 | 0.91% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2256` |
| 19 | 0.88% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 20 | 0.87% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 21 | 0.87% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 22 | 0.83% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 23 | 0.77% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 24 | 0.76% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 25 | 0.75% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5168` |
| 26 | 0.72% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 27 | 0.70% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 28 | 0.70% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 29 | 0.69% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 30 | 0.64% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 31 | 0.58% | `java.util.EnumMap.get(Object) line: 248` |
| 32 | 0.57% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 33 | 0.55% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 34 | 0.55% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 35 | 0.55% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 36 | 0.54% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 37 | 0.52% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3143` |
| 38 | 0.52% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 320` |
| 39 | 0.50% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 40 | 0.50% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 41 | 0.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 42 | 0.48% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 43 | 0.48% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 44 | 0.48% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 45 | 0.45% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 46 | 0.44% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 47 | 0.42% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 48 | 0.42% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 49 | 0.41% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 50 | 0.40% | `it.unimi.dsi.fastutil.objects.ObjectOpenHashSet.add(Object) line: 367` |
| 51 | 0.39% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 52 | 0.36% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8216` |
| 53 | 0.36% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 54 | 0.36% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 55 | 0.35% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 56 | 0.34% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2275` |
| 57 | 0.32% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 58 | 0.31% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 59 | 0.30% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 60 | 0.30% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.46% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 3.83% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 3.62% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 3.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 5 | 3.45% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 6 | 3.01% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8126` |
| 7 | 2.25% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 8 | 1.85% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1748` |
| 9 | 1.72% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 10 | 1.70% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 11 | 1.58% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4485` |
| 12 | 1.48% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 13 | 1.46% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 14 | 1.37% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 15 | 1.35% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 16 | 1.29% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8118` |
| 17 | 1.14% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 18 | 1.03% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 19 | 1.03% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 20 | 1.02% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 21 | 0.83% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |
| 22 | 0.82% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 23 | 0.78% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 24 | 0.76% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 25 | 0.72% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 26 | 0.69% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 27 | 0.67% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |
| 28 | 0.64% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 29 | 0.63% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 30 | 0.60% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3056` |

GC pauses: 0, total 0 ms

