# JVM hotspots (JFR ExecutionSample)

Samples: 27075

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.88% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.17% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 3 | 2.73% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7722` |
| 4 | 2.59% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 5 | 2.49% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.47% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4782` |
| 7 | 2.38% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7863` |
| 8 | 2.34% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4637` |
| 9 | 2.28% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7669` |
| 10 | 2.06% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 11 | 1.73% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 12 | 1.38% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 13 | 1.35% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 14 | 1.31% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 15 | 1.29% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 16 | 1.20% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 17 | 1.03% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 18 | 1.02% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 19 | 1.02% | `java.util.EnumMap.get(Object) line: 248` |
| 20 | 1.01% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 21 | 1.00% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2769` |
| 22 | 0.95% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 23 | 0.93% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 24 | 0.91% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 25 | 0.90% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 26 | 0.82% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4073` |
| 27 | 0.77% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1944` |
| 28 | 0.67% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 29 | 0.63% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 30 | 0.63% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 31 | 0.63% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 32 | 0.61% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 33 | 0.59% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 34 | 0.56% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1963` |
| 35 | 0.54% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 36 | 0.54% | `net.minecraft.world.entity.Entity.computeSpeed() line: 961` |
| 37 | 0.51% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 38 | 0.50% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 39 | 0.49% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 40 | 0.48% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 41 | 0.46% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 42 | 0.45% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2671` |
| 43 | 0.42% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 44 | 0.42% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 45 | 0.42% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 46 | 0.41% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 47 | 0.41% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 48 | 0.41% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7683` |
| 49 | 0.41% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 4960` |
| 50 | 0.40% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 51 | 0.40% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 52 | 0.39% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 53 | 0.39% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7808` |
| 54 | 0.39% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4784` |
| 55 | 0.38% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 56 | 0.37% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 57 | 0.37% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 58 | 0.36% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 59 | 0.35% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 60 | 0.34% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 8.05% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.06% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 2.88% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7718` |
| 4 | 2.88% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 2.58% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 6 | 2.39% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 7 | 2.37% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 8 | 2.01% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 9 | 1.95% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1486` |
| 10 | 1.94% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 11 | 1.73% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 12 | 1.61% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 13 | 1.54% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 14 | 1.35% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 15 | 1.29% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 16 | 1.29% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 17 | 1.23% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4110` |
| 18 | 1.17% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 19 | 1.07% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 20 | 1.05% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 21 | 1.05% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2692` |
| 22 | 1.04% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 898` |
| 23 | 1.02% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 24 | 1.01% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4637` |
| 25 | 0.93% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 26 | 0.90% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 27 | 0.79% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 28 | 0.77% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7710` |
| 29 | 0.76% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4784` |
| 30 | 0.72% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 5315` |

GC pauses: 0, total 0 ms

