# JVM hotspots (JFR ExecutionSample)

Samples: 29778

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.99% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.84% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7195` |
| 3 | 3.62% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7336` |
| 4 | 2.76% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 5 | 2.48% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4255` |
| 6 | 2.43% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 7 | 2.35% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 2.00% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.78% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7142` |
| 10 | 1.78% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 11 | 1.73% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4110` |
| 12 | 1.29% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 13 | 0.96% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 14 | 0.93% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 15 | 0.87% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 16 | 0.83% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 17 | 0.81% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 18 | 0.81% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 19 | 0.77% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 20 | 0.76% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 21 | 0.76% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 22 | 0.71% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 23 | 0.65% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 24 | 0.63% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 25 | 0.63% | `java.util.EnumMap.get(Object) line: 248` |
| 26 | 0.59% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 27 | 0.58% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 28 | 0.57% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 29 | 0.57% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 30 | 0.57% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7156` |
| 31 | 0.57% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 32 | 0.55% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1482` |
| 33 | 0.54% | `java.lang.String.hashCode() line: 2433` |
| 34 | 0.54% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2133` |
| 35 | 0.53% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 4433` |
| 36 | 0.51% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 37 | 0.50% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 38 | 0.50% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7281` |
| 39 | 0.49% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 40 | 0.49% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 41 | 0.48% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 42 | 0.47% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 43 | 0.46% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 44 | 0.44% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 45 | 0.44% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 46 | 0.43% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1463` |
| 47 | 0.42% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 48 | 0.40% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 49 | 0.39% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 50 | 0.39% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 51 | 0.38% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 52 | 0.38% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 53 | 0.36% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 54 | 0.36% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 55 | 0.35% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 56 | 0.35% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 57 | 0.34% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 58 | 0.34% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 59 | 0.33% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1466` |
| 60 | 0.33% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.99% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.99% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 3.63% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 4 | 2.84% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 2.83% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 6 | 2.53% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7191` |
| 7 | 2.43% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 8 | 1.81% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1079` |
| 9 | 1.80% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 10 | 1.78% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 11 | 1.67% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 12 | 1.57% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4257` |
| 13 | 1.29% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 14 | 1.04% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 15 | 1.01% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 16 | 0.88% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7183` |
| 17 | 0.85% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 18 | 0.84% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 19 | 0.82% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 20 | 0.81% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 21 | 0.78% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4720` |
| 22 | 0.76% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1467` |
| 23 | 0.73% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 898` |
| 24 | 0.72% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 25 | 0.69% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 26 | 0.64% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2195` |
| 27 | 0.62% | `net.minecraft.world.entity.LivingEntity.detectEquipmentUpdates() line: 3558` |
| 28 | 0.61% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 125` |
| 29 | 0.58% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |
| 30 | 0.57% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |

GC pauses: 0, total 0 ms

