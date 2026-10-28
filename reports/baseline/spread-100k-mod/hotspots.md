# JVM hotspots (JFR ExecutionSample)

Samples: 30723

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.49% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.41% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7914` |
| 3 | 3.30% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4833` |
| 4 | 2.79% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7773` |
| 5 | 2.59% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 2.35% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 7 | 2.28% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 2.19% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7720` |
| 9 | 2.16% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 1.96% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4688` |
| 11 | 1.82% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 12 | 1.57% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 13 | 1.49% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 14 | 1.25% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 15 | 1.24% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 16 | 1.04% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 17 | 1.04% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 18 | 1.00% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4115` |
| 19 | 0.99% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2005` |
| 20 | 0.96% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 21 | 0.95% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 22 | 0.94% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 23 | 0.93% | `java.util.EnumMap.get(Object) line: 248` |
| 24 | 0.89% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1986` |
| 25 | 0.87% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2811` |
| 26 | 0.83% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 27 | 0.81% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 28 | 0.80% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 29 | 0.77% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 30 | 0.74% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 31 | 0.67% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 32 | 0.66% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 33 | 0.64% | `java.lang.ref.Reference.get() line: 361` |
| 34 | 0.64% | `java.util.ArrayDeque.pollFirst() line: 378` |
| 35 | 0.63% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 36 | 0.61% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 37 | 0.60% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 38 | 0.60% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 39 | 0.60% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 40 | 0.57% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 41 | 0.52% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 42 | 0.50% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 43 | 0.50% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 44 | 0.48% | `net.minecraft.world.entity.Entity.computeSpeed() line: 961` |
| 45 | 0.48% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 46 | 0.48% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4838` |
| 47 | 0.45% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 48 | 0.41% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 49 | 0.41% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 50 | 0.40% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7734` |
| 51 | 0.39% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 52 | 0.39% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 53 | 0.38% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 54 | 0.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 55 | 0.37% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 56 | 0.37% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2713` |
| 57 | 0.37% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1989` |
| 58 | 0.35% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 59 | 0.35% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4835` |
| 60 | 0.35% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.07% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.31% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 3.41% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 4 | 2.92% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 2.76% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7769` |
| 6 | 2.74% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1528` |
| 7 | 2.25% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 8 | 2.20% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 9 | 2.16% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 10 | 1.99% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 11 | 1.82% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 12 | 1.61% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 13 | 1.56% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 14 | 1.49% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 15 | 1.31% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 16 | 1.22% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1990` |
| 17 | 1.22% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 18 | 1.06% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 19 | 1.01% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7761` |
| 20 | 0.97% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 21 | 0.95% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4152` |
| 22 | 0.94% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 23 | 0.93% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 24 | 0.91% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 898` |
| 25 | 0.89% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2734` |
| 26 | 0.84% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 27 | 0.83% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 28 | 0.80% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 29 | 0.79% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 5366` |
| 30 | 0.75% | `com.google.common.collect.MapMakerInternalMap$Segment.getEntry(Object, int) line: 1415` |

GC pauses: 0, total 0 ms

