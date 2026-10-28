# JVM hotspots (JFR ExecutionSample)

Samples: 38049

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.47% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7773` |
| 2 | 3.41% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7914` |
| 3 | 3.19% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 3.13% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4833` |
| 5 | 3.08% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 6 | 2.85% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 1.94% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7720` |
| 8 | 1.91% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 1.80% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4115` |
| 10 | 1.54% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.35% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4688` |
| 12 | 1.21% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 13 | 1.08% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 14 | 1.02% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 15 | 0.97% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 16 | 0.95% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 17 | 0.92% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2005` |
| 18 | 0.85% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 19 | 0.83% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 20 | 0.81% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 21 | 0.81% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 22 | 0.78% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 23 | 0.71% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 24 | 0.70% | `java.util.EnumMap.get(Object) line: 248` |
| 25 | 0.63% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 26 | 0.61% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 27 | 0.61% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 28 | 0.60% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 29 | 0.57% | `java.lang.ref.Reference.get() line: 361` |
| 30 | 0.57% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 31 | 0.56% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7859` |
| 32 | 0.55% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 33 | 0.54% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1989` |
| 34 | 0.54% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 35 | 0.53% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 36 | 0.53% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 37 | 0.53% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 38 | 0.53% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7734` |
| 39 | 0.52% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 40 | 0.52% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 41 | 0.50% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 42 | 0.50% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2811` |
| 43 | 0.45% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 44 | 0.44% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 45 | 0.44% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 46 | 0.44% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 47 | 0.43% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 48 | 0.42% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1986` |
| 49 | 0.40% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 50 | 0.40% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 51 | 0.40% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 52 | 0.39% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 53 | 0.39% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 54 | 0.38% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 55 | 0.34% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 56 | 0.33% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 57 | 0.32% | `dev.planeta9091.centities.CeHooks.aiPackSel(int[], int, WrappedGoal[], int[], int[], long[], int[], int[], int[], float[], Mob) line: 6807` |
| 58 | 0.32% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 59 | 0.30% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 60 | 0.29% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.72% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.58% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 3.58% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 3.42% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 5 | 3.14% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 6 | 2.60% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7769` |
| 7 | 2.44% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1528` |
| 8 | 2.13% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 9 | 1.91% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 10 | 1.43% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 11 | 1.37% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 12 | 1.26% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7761` |
| 13 | 1.15% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7786` |
| 14 | 1.15% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1990` |
| 15 | 1.14% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4152` |
| 16 | 1.08% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 17 | 1.03% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 18 | 1.02% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 19 | 0.90% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 20 | 0.89% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 21 | 0.85% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 22 | 0.82% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |
| 23 | 0.81% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 24 | 0.80% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 25 | 0.76% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 5727` |
| 26 | 0.74% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 27 | 0.72% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 28 | 0.70% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 29 | 0.65% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 30 | 0.63% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |

GC pauses: 0, total 0 ms

