# JVM hotspots (JFR ExecutionSample)

Samples: 26991

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.90% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.86% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4276` |
| 3 | 3.51% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7357` |
| 4 | 3.35% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7216` |
| 5 | 2.45% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.19% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 2.14% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 2.13% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4131` |
| 9 | 1.96% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7163` |
| 10 | 1.86% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 11 | 1.65% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 12 | 1.62% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 13 | 1.42% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 14 | 1.13% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 15 | 1.00% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 16 | 0.97% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 17 | 0.90% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 18 | 0.87% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 19 | 0.86% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 20 | 0.83% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 21 | 0.76% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 22 | 0.72% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 23 | 0.71% | `java.util.EnumMap.get(Object) line: 248` |
| 24 | 0.69% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 25 | 0.68% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1484` |
| 26 | 0.67% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1468` |
| 27 | 0.66% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 28 | 0.66% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 29 | 0.65% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 30 | 0.64% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 31 | 0.64% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 32 | 0.58% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 33 | 0.58% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 4454` |
| 34 | 0.56% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7177` |
| 35 | 0.55% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 36 | 0.55% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 37 | 0.54% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 38 | 0.54% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 39 | 0.51% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 40 | 0.51% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 41 | 0.50% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 42 | 0.49% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 43 | 0.46% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 44 | 0.44% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 45 | 0.44% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 46 | 0.42% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 47 | 0.42% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2135` |
| 48 | 0.42% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1465` |
| 49 | 0.41% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 50 | 0.41% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7302` |
| 51 | 0.41% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 52 | 0.41% | `net.minecraft.world.entity.Entity.computeSpeed() line: 961` |
| 53 | 0.37% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 54 | 0.36% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 55 | 0.36% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 56 | 0.35% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 57 | 0.35% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 58 | 0.35% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 59 | 0.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 60 | 0.34% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.09% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.08% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 3.52% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 3.51% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 5 | 2.76% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7212` |
| 6 | 2.33% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1081` |
| 7 | 2.15% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 8 | 2.14% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 9 | 1.86% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 10 | 1.75% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 11 | 1.64% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 12 | 1.62% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 13 | 1.58% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4278` |
| 14 | 1.35% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 15 | 1.33% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 16 | 1.23% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 17 | 1.06% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 18 | 0.98% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 19 | 0.98% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 20 | 0.96% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1469` |
| 21 | 0.96% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 22 | 0.90% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7204` |
| 23 | 0.87% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 24 | 0.79% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4741` |
| 25 | 0.72% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 898` |
| 26 | 0.69% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4131` |
| 27 | 0.68% | `net.minecraft.world.level.EntityGetter.findNearbyPlayer(Entity, double, Predicate) line: 117` |
| 28 | 0.65% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 29 | 0.64% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 30 | 0.63% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |

GC pauses: 0, total 0 ms

