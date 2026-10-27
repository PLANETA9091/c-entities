# JVM hotspots (JFR ExecutionSample)

Samples: 27936

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.53% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.58% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7357` |
| 3 | 3.15% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7216` |
| 4 | 2.97% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4276` |
| 5 | 2.66% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.55% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 2.42% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7163` |
| 8 | 2.11% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 2.09% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 10 | 2.09% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4131` |
| 11 | 2.03% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 12 | 1.73% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 13 | 1.47% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 14 | 1.38% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 15 | 1.22% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 16 | 1.06% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 17 | 0.98% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 18 | 0.98% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 19 | 0.88% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 20 | 0.86% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 21 | 0.85% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 22 | 0.85% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 23 | 0.79% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 24 | 0.79% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 25 | 0.75% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1465` |
| 26 | 0.73% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2135` |
| 27 | 0.72% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 28 | 0.69% | `java.lang.ref.Reference.get() line: 361` |
| 29 | 0.69% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 30 | 0.67% | `java.util.EnumMap.get(Object) line: 248` |
| 31 | 0.66% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 32 | 0.65% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1484` |
| 33 | 0.58% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 34 | 0.56% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 35 | 0.56% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 36 | 0.56% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 37 | 0.53% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 38 | 0.52% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 39 | 0.51% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 40 | 0.50% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 41 | 0.49% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 42 | 0.49% | `java.util.ArrayDeque.pollFirst() line: 378` |
| 43 | 0.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 44 | 0.48% | `net.minecraft.world.entity.Entity.computeSpeed() line: 961` |
| 45 | 0.47% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 46 | 0.46% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 47 | 0.45% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 48 | 0.45% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 49 | 0.44% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 50 | 0.44% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 51 | 0.44% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 52 | 0.42% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7302` |
| 53 | 0.42% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 54 | 0.42% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 55 | 0.41% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 56 | 0.40% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 57 | 0.39% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7177` |
| 58 | 0.38% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 59 | 0.36% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 60 | 0.36% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4742` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.07% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.58% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 3 | 3.35% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 3.21% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 3.06% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7212` |
| 6 | 2.23% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1081` |
| 7 | 2.11% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 8 | 2.11% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 2.09% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 10 | 2.02% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 11 | 1.99% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4278` |
| 12 | 1.95% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 13 | 1.81% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 14 | 1.73% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 15 | 1.49% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 16 | 1.39% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 17 | 1.21% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7204` |
| 18 | 1.20% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 19 | 1.20% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 20 | 1.08% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 21 | 0.96% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1469` |
| 22 | 0.95% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 898` |
| 23 | 0.89% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 24 | 0.88% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 25 | 0.85% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 26 | 0.84% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4131` |
| 27 | 0.79% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 28 | 0.72% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 29 | 0.72% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4809` |
| 30 | 0.65% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |

GC pauses: 0, total 0 ms

