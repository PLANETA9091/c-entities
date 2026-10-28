# JVM hotspots (JFR ExecutionSample)

Samples: 32862

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.91% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.68% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7914` |
| 3 | 3.17% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4833` |
| 4 | 2.97% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 2.84% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 6 | 2.67% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7773` |
| 7 | 2.37% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 2.25% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 2.08% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7720` |
| 10 | 1.76% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4688` |
| 11 | 1.75% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 1.49% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 13 | 1.30% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 14 | 1.27% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 15 | 1.10% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 16 | 1.06% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 17 | 1.04% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2005` |
| 18 | 1.03% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 19 | 1.03% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4115` |
| 20 | 0.99% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 21 | 0.91% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 22 | 0.89% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 23 | 0.88% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 24 | 0.88% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1986` |
| 25 | 0.87% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 26 | 0.85% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 27 | 0.80% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2811` |
| 28 | 0.77% | `net.minecraft.server.level.ServerChunkCache.syncLoad(int, int, ChunkStatus) line: 114` |
| 29 | 0.75% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 30 | 0.74% | `java.util.EnumMap.get(Object) line: 248` |
| 31 | 0.69% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 32 | 0.67% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 33 | 0.67% | `jdk.internal.util.ArraysSupport.hashCodeOfUnsigned(byte[], int, int, int) line: 275` |
| 34 | 0.64% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 35 | 0.63% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 36 | 0.62% | `java.lang.ref.Reference.get() line: 361` |
| 37 | 0.62% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 38 | 0.61% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 39 | 0.60% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 40 | 0.58% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 41 | 0.55% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 42 | 0.53% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 43 | 0.52% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 44 | 0.51% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 45 | 0.50% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 46 | 0.47% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 47 | 0.47% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1989` |
| 48 | 0.46% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 49 | 0.43% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4838` |
| 50 | 0.43% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 51 | 0.43% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7734` |
| 52 | 0.43% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 53 | 0.42% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 54 | 0.41% | `net.minecraft.world.entity.Entity.computeSpeed() line: 961` |
| 55 | 0.40% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 56 | 0.40% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 57 | 0.39% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 58 | 0.37% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2713` |
| 59 | 0.37% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 60 | 0.36% | `org.spigotmc.WatchdogThread.run() line: 72` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.16% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.04% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 3.68% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 4 | 2.97% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 2.95% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1528` |
| 6 | 2.84% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 7 | 2.80% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 8 | 2.72% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7769` |
| 9 | 1.97% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 10 | 1.79% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 11 | 1.67% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 12 | 1.57% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 13 | 1.45% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1990` |
| 14 | 1.41% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 15 | 1.30% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 16 | 1.27% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 17 | 1.27% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 18 | 1.03% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7761` |
| 19 | 1.01% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 20 | 0.99% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 21 | 0.98% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4152` |
| 22 | 0.97% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 5366` |
| 23 | 0.95% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 24 | 0.93% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 25 | 0.89% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 26 | 0.88% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 898` |
| 27 | 0.83% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 28 | 0.83% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2734` |
| 29 | 0.77% | `net.minecraft.server.level.ServerChunkCache.getChunkFallback(int, int, ChunkStatus, boolean) line: 157` |
| 30 | 0.72% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |

GC pauses: 0, total 0 ms

