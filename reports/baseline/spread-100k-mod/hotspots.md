# JVM hotspots (JFR ExecutionSample)

Samples: 30684

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.98% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7216` |
| 2 | 3.73% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7357` |
| 3 | 3.53% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 3.29% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4276` |
| 5 | 2.96% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.75% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 7 | 1.98% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 8 | 1.67% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4131` |
| 9 | 1.66% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7163` |
| 10 | 1.66% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.34% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 12 | 1.21% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 13 | 1.16% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 14 | 0.92% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 15 | 0.88% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 16 | 0.87% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1484` |
| 17 | 0.86% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 18 | 0.80% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 19 | 0.77% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 20 | 0.76% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 21 | 0.71% | `java.util.EnumMap.get(Object) line: 248` |
| 22 | 0.69% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 23 | 0.68% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 24 | 0.67% | `net.minecraft.server.level.ServerChunkCache.syncLoad(int, int, ChunkStatus) line: 123` |
| 25 | 0.67% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 26 | 0.63% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 27 | 0.58% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 28 | 0.58% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 4454` |
| 29 | 0.57% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 30 | 0.56% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 31 | 0.56% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 32 | 0.56% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 33 | 0.55% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4281` |
| 34 | 0.54% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7177` |
| 35 | 0.53% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 36 | 0.52% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 37 | 0.51% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 38 | 0.51% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 39 | 0.50% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2135` |
| 40 | 0.50% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 41 | 0.49% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1465` |
| 42 | 0.45% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 43 | 0.44% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 44 | 0.43% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7302` |
| 45 | 0.42% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1468` |
| 46 | 0.39% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 47 | 0.38% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 48 | 0.38% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 49 | 0.37% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 50 | 0.37% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 51 | 0.36% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 52 | 0.36% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 53 | 0.36% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 54 | 0.35% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 55 | 0.35% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 56 | 0.33% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2081` |
| 57 | 0.33% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 58 | 0.32% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 59 | 0.32% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 60 | 0.31% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.19% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.10% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 4.10% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 3.75% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 5 | 2.90% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 6 | 2.38% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1081` |
| 7 | 2.35% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7212` |
| 8 | 1.99% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 9 | 1.72% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 10 | 1.67% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7204` |
| 11 | 1.60% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 12 | 1.46% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4278` |
| 13 | 1.34% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 14 | 1.20% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 15 | 1.07% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 16 | 1.03% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 17 | 0.93% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 18 | 0.91% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4741` |
| 19 | 0.87% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 20 | 0.87% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1469` |
| 21 | 0.79% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 22 | 0.78% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 23 | 0.76% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 24 | 0.69% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 25 | 0.67% | `net.minecraft.server.level.ServerChunkCache.getChunkFallback(int, int, ChunkStatus, boolean) line: 157` |
| 26 | 0.66% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 27 | 0.63% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 28 | 0.61% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 898` |
| 29 | 0.59% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |
| 30 | 0.56% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2195` |

GC pauses: 0, total 0 ms

