# JVM hotspots (JFR ExecutionSample)

Samples: 39733

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.74% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 2 | 3.15% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7914` |
| 3 | 3.13% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7773` |
| 4 | 3.06% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4833` |
| 5 | 2.88% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 2.54% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.39% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 2.16% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 9 | 1.63% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4115` |
| 10 | 1.62% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7720` |
| 11 | 1.33% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.25% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 13 | 1.23% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4688` |
| 14 | 1.22% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 15 | 1.19% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 16 | 1.13% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 17 | 0.99% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 18 | 0.97% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2005` |
| 19 | 0.91% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 20 | 0.89% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 21 | 0.86% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 22 | 0.84% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 23 | 0.81% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 24 | 0.75% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 25 | 0.72% | `java.util.EnumMap.get(Object) line: 248` |
| 26 | 0.70% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 27 | 0.69% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 28 | 0.65% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7859` |
| 29 | 0.63% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 30 | 0.62% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 31 | 0.61% | `java.lang.ref.Reference.get() line: 361` |
| 32 | 0.57% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 33 | 0.55% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 34 | 0.55% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 35 | 0.54% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 36 | 0.53% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 37 | 0.53% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 38 | 0.51% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 39 | 0.51% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2811` |
| 40 | 0.50% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1989` |
| 41 | 0.49% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 42 | 0.48% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 43 | 0.47% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7734` |
| 44 | 0.46% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 45 | 0.46% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 46 | 0.44% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 47 | 0.42% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 48 | 0.40% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 49 | 0.39% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1986` |
| 50 | 0.37% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 51 | 0.36% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 52 | 0.34% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 5000` |
| 53 | 0.34% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 54 | 0.34% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 55 | 0.34% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 56 | 0.32% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 57 | 0.31% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 58 | 0.30% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 59 | 0.29% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 60 | 0.28% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1484` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.21% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.83% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 3 | 3.47% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 3.25% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 3.15% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 6 | 2.44% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1528` |
| 7 | 2.39% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 2.25% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7769` |
| 9 | 2.16% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 10 | 1.97% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 11 | 1.34% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 12 | 1.28% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 13 | 1.25% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 14 | 1.23% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7761` |
| 15 | 1.21% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 16 | 1.19% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1990` |
| 17 | 1.15% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 18 | 1.10% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 19 | 1.07% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4152` |
| 20 | 0.91% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 21 | 0.90% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 22 | 0.88% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 23 | 0.85% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |
| 24 | 0.81% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |
| 25 | 0.79% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 26 | 0.76% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 27 | 0.75% | `com.google.common.collect.MapMakerInternalMap$Segment.getEntry(Object, int) line: 1415` |
| 28 | 0.72% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 29 | 0.69% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 30 | 0.65% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |

GC pauses: 0, total 0 ms

