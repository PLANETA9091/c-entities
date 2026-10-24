# JVM hotspots (JFR ExecutionSample)

Samples: 33193

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.84% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.get(VarHandle, Object) line: 92` |
| 2 | 4.34% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 4.34% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3097` |
| 4 | 4.02% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 2962` |
| 5 | 3.65% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 3659` |
| 6 | 3.21% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 7 | 2.61% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 2972` |
| 8 | 2.41% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1467` |
| 9 | 1.99% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.66% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 11 | 1.58% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 12 | 1.45% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 3633` |
| 13 | 1.42% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 14 | 1.21% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 15 | 1.17% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 16 | 1.06% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2403` |
| 17 | 0.94% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 18 | 0.92% | `jdk.internal.reflect.MethodHandleObjectFieldAccessorImpl.get(Object) line: 57` |
| 19 | 0.92% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 20 | 0.89% | `java.util.EnumMap.get(Object) line: 248` |
| 21 | 0.87% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3868` |
| 22 | 0.85% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 23 | 0.85% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3099` |
| 24 | 0.83% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 3332` |
| 25 | 0.81% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 26 | 0.75% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 27 | 0.73% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 28 | 0.70% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2062` |
| 29 | 0.67% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 30 | 0.67% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 31 | 0.64% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 32 | 0.62% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 33 | 0.62% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1403` |
| 34 | 0.58% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 35 | 0.55% | `net.minecraft.world.entity.Entity.isRemoved() line: 5328` |
| 36 | 0.49% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1107` |
| 37 | 0.49% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 38 | 0.49% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 39 | 0.47% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 40 | 0.47% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 41 | 0.46% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 42 | 0.45% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 43 | 0.44% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4706` |
| 44 | 0.43% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 45 | 0.43% | `jdk.internal.util.ArraysSupport.hashCodeOfUnsigned(byte[], int, int, int) line: 275` |
| 46 | 0.43% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 47 | 0.42% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 48 | 0.42% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1387` |
| 49 | 0.41% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 50 | 0.41% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 51 | 0.41% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 52 | 0.39% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 53 | 0.37% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 603` |
| 54 | 0.36% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 55 | 0.35% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 56 | 0.34% | `net.minecraft.world.entity.Entity.getX() line: 5094` |
| 57 | 0.34% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1384` |
| 58 | 0.33% | `dev.planeta9091.centities.CeHooks.disabledBacking(GoalSelector) line: 3256` |
| 59 | 0.31% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 60 | 0.30% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.89% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 2 | 6.33% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 5.38% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 4.91% | `dev.planeta9091.centities.CeHooks$RefField.getObj(Object) line: 2218` |
| 5 | 3.65% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 6 | 3.21% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 7 | 2.29% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity) line: 599` |
| 8 | 1.80% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1047` |
| 9 | 1.70% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 10 | 1.66% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 11 | 1.58% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 12 | 1.48% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk)` |
| 13 | 1.47% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 14 | 1.39% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 15 | 1.21% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 16 | 1.09% | `net.minecraft.world.entity.Entity.setOldPos() line: 2395` |
| 17 | 1.06% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 18 | 1.00% | `java.lang.reflect.Field.get(Object) line: 440` |
| 19 | 0.96% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 20 | 0.88% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 21 | 0.85% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 3332` |
| 22 | 0.83% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 969` |
| 23 | 0.81% | `com.google.common.collect.MapMakerInternalMap$Segment.getEntry(Object, int) line: 1415` |
| 24 | 0.75% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 898` |
| 25 | 0.75% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 26 | 0.74% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1388` |
| 27 | 0.73% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1986` |
| 28 | 0.71% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 29 | 0.71% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 30 | 0.64% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2195` |

GC pauses: 0, total 0 ms

