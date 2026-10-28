# JVM hotspots (JFR ExecutionSample)

Samples: 36121

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.19% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 2 | 3.16% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4833` |
| 3 | 3.13% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7773` |
| 4 | 3.11% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 2.64% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.45% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7914` |
| 7 | 1.89% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 1.64% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.54% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7720` |
| 10 | 1.53% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4115` |
| 11 | 1.27% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4688` |
| 12 | 1.16% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 13 | 1.15% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 14 | 0.98% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 15 | 0.94% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 16 | 0.93% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 17 | 0.93% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 18 | 0.90% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 19 | 0.86% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 20 | 0.78% | `java.util.EnumMap.get(Object) line: 248` |
| 21 | 0.75% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 22 | 0.72% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 23 | 0.71% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 24 | 0.70% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 25 | 0.70% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 26 | 0.68% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 27 | 0.68% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 28 | 0.67% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 29 | 0.64% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2005` |
| 30 | 0.63% | `java.lang.ref.Reference.get() line: 361` |
| 31 | 0.63% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 32 | 0.61% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 33 | 0.50% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 34 | 0.49% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 35 | 0.49% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 36 | 0.49% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7734` |
| 37 | 0.48% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 38 | 0.48% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 39 | 0.46% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7859` |
| 40 | 0.46% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 41 | 0.45% | `net.minecraft.world.entity.SteppedInterpolationTracker.updateTracking(Vec3) line: 40` |
| 42 | 0.45% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 43 | 0.45% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 44 | 0.42% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1989` |
| 45 | 0.41% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 46 | 0.40% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2811` |
| 47 | 0.40% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 48 | 0.40% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 49 | 0.40% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 50 | 0.38% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1986` |
| 51 | 0.38% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 52 | 0.38% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 53 | 0.37% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 54 | 0.37% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 55 | 0.37% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 56 | 0.34% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 57 | 0.34% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 58 | 0.32% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 59 | 0.32% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 60 | 0.30% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.74% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.56% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 3.26% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 3.19% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 5 | 2.46% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 6 | 2.16% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7769` |
| 7 | 1.94% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1528` |
| 8 | 1.89% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 1.87% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 10 | 1.61% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 11 | 1.41% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7786` |
| 12 | 1.30% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 13 | 1.23% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7761` |
| 14 | 1.15% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 15 | 1.14% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4152` |
| 16 | 1.12% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Level, int, int) line: 146` |
| 17 | 0.98% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 18 | 0.92% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 19 | 0.89% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 20 | 0.89% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 21 | 0.87% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 22 | 0.79% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1990` |
| 23 | 0.77% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 24 | 0.76% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 25 | 0.76% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |
| 26 | 0.75% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 27 | 0.71% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 28 | 0.71% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 29 | 0.67% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |
| 30 | 0.63% | `com.google.common.collect.MapMakerInternalMap$Segment.getEntry(Object, int) line: 1415` |

GC pauses: 0, total 0 ms

