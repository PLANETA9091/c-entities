# JVM hotspots (JFR ExecutionSample)

Samples: 35498

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.71% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4838` |
| 2 | 3.36% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.26% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7778` |
| 4 | 2.80% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.46% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 6 | 2.12% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7725` |
| 7 | 2.09% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7919` |
| 8 | 1.88% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.65% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 10 | 1.61% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4693` |
| 11 | 1.52% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 230` |
| 12 | 1.52% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4120` |
| 13 | 1.48% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 14 | 1.36% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 15 | 1.17% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 16 | 1.07% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 17 | 1.05% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 18 | 1.04% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 19 | 1.00% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 20 | 0.98% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 21 | 0.91% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 22 | 0.88% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 23 | 0.81% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 24 | 0.78% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 25 | 0.78% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 26 | 0.76% | `java.util.EnumMap.get(Object) line: 248` |
| 27 | 0.75% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 28 | 0.73% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 29 | 0.73% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 30 | 0.66% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 31 | 0.65% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 993` |
| 32 | 0.63% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 33 | 0.63% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2005` |
| 34 | 0.59% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 35 | 0.58% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 36 | 0.57% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 37 | 0.57% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 38 | 0.55% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 39 | 0.55% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 40 | 0.54% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 41 | 0.53% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 42 | 0.52% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 43 | 0.52% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 44 | 0.51% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4843` |
| 45 | 0.50% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 46 | 0.50% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 47 | 0.49% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 48 | 0.48% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7739` |
| 49 | 0.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 50 | 0.45% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2816` |
| 51 | 0.44% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 52 | 0.41% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 53 | 0.41% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 54 | 0.39% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1989` |
| 55 | 0.36% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 56 | 0.35% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7421` |
| 57 | 0.34% | `java.lang.String.hashCode() line: 2433` |
| 58 | 0.34% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7864` |
| 59 | 0.34% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 60 | 0.33% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.25% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.62% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 3.39% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 2.81% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7774` |
| 5 | 2.47% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 6 | 2.10% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 7 | 1.95% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 8 | 1.73% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1528` |
| 9 | 1.65% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 10 | 1.65% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 11 | 1.63% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 12 | 1.61% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 13 | 1.52% | `java.util.stream.MatchOps$MatchOp.evaluateSequential(PipelineHelper, Spliterator) line: 196` |
| 14 | 1.48% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 15 | 1.21% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7766` |
| 16 | 1.16% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 17 | 1.08% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 18 | 1.06% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 19 | 1.04% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 20 | 1.04% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 21 | 1.00% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 22 | 0.98% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 23 | 0.96% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 24 | 0.83% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4157` |
| 25 | 0.83% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1990` |
| 26 | 0.80% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 27 | 0.78% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 28 | 0.77% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 29 | 0.75% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |
| 30 | 0.75% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |

GC pauses: 0, total 0 ms

