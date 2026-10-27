# JVM hotspots (JFR ExecutionSample)

Samples: 27512

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.67% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.22% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7685` |
| 3 | 3.10% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7544` |
| 4 | 2.98% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 5 | 2.98% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4604` |
| 6 | 2.47% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.40% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 2.34% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7491` |
| 9 | 2.17% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 10 | 2.14% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 11 | 1.88% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4459` |
| 12 | 1.82% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 13 | 1.78% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 14 | 1.16% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 15 | 1.01% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 16 | 0.98% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 17 | 0.89% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 18 | 0.87% | `java.util.EnumMap.get(Object) line: 248` |
| 19 | 0.87% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 20 | 0.85% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 21 | 0.84% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 22 | 0.83% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 23 | 0.79% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 24 | 0.71% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1778` |
| 25 | 0.70% | `java.lang.ref.Reference.get() line: 361` |
| 26 | 0.67% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 27 | 0.64% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 28 | 0.63% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 29 | 0.63% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 30 | 0.63% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2463` |
| 31 | 0.60% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 32 | 0.57% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 33 | 0.54% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 34 | 0.52% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 35 | 0.52% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1797` |
| 36 | 0.51% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 37 | 0.51% | `net.minecraft.world.entity.Entity.computeSpeed() line: 961` |
| 38 | 0.50% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 39 | 0.49% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 40 | 0.49% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 41 | 0.48% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 42 | 0.48% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7505` |
| 43 | 0.48% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 44 | 0.47% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 45 | 0.45% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 46 | 0.44% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 47 | 0.43% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 48 | 0.43% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 49 | 0.43% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 50 | 0.43% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 51 | 0.42% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 52 | 0.42% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 53 | 0.41% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 54 | 0.41% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 5127` |
| 55 | 0.37% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 56 | 0.37% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7638` |
| 57 | 0.36% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 58 | 0.35% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 59 | 0.34% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 60 | 0.34% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.66% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.30% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 3.27% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 3.24% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 5 | 3.05% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7540` |
| 6 | 2.40% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 7 | 2.14% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 8 | 2.13% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 9 | 2.03% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4606` |
| 10 | 1.92% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 11 | 1.91% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 12 | 1.88% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1320` |
| 13 | 1.82% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 14 | 1.78% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 15 | 1.38% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 16 | 1.34% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 17 | 1.18% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 18 | 1.14% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 19 | 1.03% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 20 | 0.93% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 898` |
| 21 | 0.91% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7532` |
| 22 | 0.90% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 23 | 0.81% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1782` |
| 24 | 0.80% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 5137` |
| 25 | 0.79% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4459` |
| 26 | 0.79% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 27 | 0.67% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 28 | 0.66% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 29 | 0.64% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2195` |
| 30 | 0.64% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |

GC pauses: 0, total 0 ms

