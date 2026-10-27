# JVM hotspots (JFR ExecutionSample)

Samples: 30726

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.84% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7685` |
| 2 | 3.33% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.31% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7544` |
| 4 | 2.65% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 5 | 2.33% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4604` |
| 6 | 2.32% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.11% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 1.98% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.57% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7491` |
| 10 | 1.50% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4459` |
| 11 | 0.92% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 12 | 0.90% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 13 | 0.81% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 14 | 0.80% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 15 | 0.80% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 16 | 0.77% | `java.util.EnumMap.get(Object) line: 248` |
| 17 | 0.73% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 18 | 0.73% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 19 | 0.70% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 20 | 0.69% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 21 | 0.69% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 22 | 0.68% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 23 | 0.68% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 24 | 0.67% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 25 | 0.62% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7630` |
| 26 | 0.59% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 27 | 0.59% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7505` |
| 28 | 0.57% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 29 | 0.55% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 30 | 0.54% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 31 | 0.52% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 32 | 0.52% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 33 | 0.52% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 34 | 0.52% | `jdk.internal.util.ArraysSupport.hashCodeOfUnsigned(byte[], int, int, int) line: 275` |
| 35 | 0.51% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 36 | 0.51% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 37 | 0.51% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 38 | 0.51% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2463` |
| 39 | 0.50% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 40 | 0.48% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1797` |
| 41 | 0.47% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 4782` |
| 42 | 0.46% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 43 | 0.44% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 44 | 0.44% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 45 | 0.43% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 46 | 0.42% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 47 | 0.39% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 48 | 0.38% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1778` |
| 49 | 0.38% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 50 | 0.37% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 51 | 0.37% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 52 | 0.36% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 53 | 0.36% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 54 | 0.35% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 55 | 0.35% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 56 | 0.34% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 57 | 0.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 58 | 0.31% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 59 | 0.31% | `dev.planeta9091.centities.CeHooks.aiPackSel(int[], int, WrappedGoal[], int[], int[], long[], int[], int[], int[], float[], Mob) line: 6578` |
| 60 | 0.31% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 5127` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.31% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.85% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 3 | 3.41% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 2.65% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 5 | 2.61% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 6 | 2.29% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7540` |
| 7 | 2.11% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 1.58% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4606` |
| 9 | 1.53% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 10 | 1.53% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 11 | 1.50% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1320` |
| 12 | 1.14% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 13 | 1.12% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7557` |
| 14 | 1.07% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 15 | 1.03% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 16 | 0.92% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7532` |
| 17 | 0.88% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 18 | 0.82% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 19 | 0.81% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 5069` |
| 20 | 0.76% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 21 | 0.75% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 22 | 0.73% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 23 | 0.71% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 24 | 0.70% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 25 | 0.70% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 26 | 0.67% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |
| 27 | 0.65% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 898` |
| 28 | 0.63% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 29 | 0.62% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1782` |
| 30 | 0.58% | `net.minecraft.world.entity.LivingEntity.detectEquipmentUpdates() line: 3558` |

GC pauses: 0, total 0 ms

