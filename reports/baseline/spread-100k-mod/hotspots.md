# JVM hotspots (JFR ExecutionSample)

Samples: 39134

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.67% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 2 | 3.04% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4833` |
| 3 | 3.04% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 2.95% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7773` |
| 5 | 2.87% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 2.50% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 1.66% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7720` |
| 8 | 1.54% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.40% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 10 | 1.39% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4115` |
| 11 | 1.31% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 12 | 1.26% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 13 | 1.26% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7914` |
| 14 | 1.22% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2005` |
| 15 | 1.17% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4688` |
| 16 | 1.16% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 17 | 1.16% | `java.util.ArrayDeque.pollFirst() line: 378` |
| 18 | 1.05% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 19 | 0.99% | `java.lang.ref.Reference.get() line: 361` |
| 20 | 0.94% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 21 | 0.92% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 22 | 0.83% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 23 | 0.80% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 24 | 0.75% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 25 | 0.71% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 26 | 0.70% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 27 | 0.68% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 28 | 0.66% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 29 | 0.65% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 30 | 0.65% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1989` |
| 31 | 0.63% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 32 | 0.62% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 33 | 0.62% | `java.util.EnumMap.get(Object) line: 248` |
| 34 | 0.59% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 35 | 0.58% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 36 | 0.56% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 37 | 0.55% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 38 | 0.52% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 39 | 0.52% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 40 | 0.52% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 41 | 0.50% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7734` |
| 42 | 0.49% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1986` |
| 43 | 0.47% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2811` |
| 44 | 0.47% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 45 | 0.46% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 46 | 0.46% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 47 | 0.45% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 48 | 0.44% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 49 | 0.44% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 50 | 0.43% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7859` |
| 51 | 0.43% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 52 | 0.41% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 5000` |
| 53 | 0.39% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 54 | 0.37% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 55 | 0.36% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 56 | 0.36% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 5356` |
| 57 | 0.35% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 58 | 0.34% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 59 | 0.34% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 320` |
| 60 | 0.34% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.41% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.73% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 3 | 3.39% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 3.05% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 3.04% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 3.03% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1528` |
| 7 | 2.28% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7769` |
| 8 | 1.70% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 9 | 1.57% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 10 | 1.37% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1990` |
| 11 | 1.30% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7786` |
| 12 | 1.28% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7761` |
| 13 | 1.27% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 14 | 1.26% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 15 | 1.26% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 16 | 1.19% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 17 | 1.16% | `java.util.ArrayDeque.removeFirst() line: 358` |
| 18 | 1.11% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 19 | 1.07% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 20 | 1.06% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 21 | 1.05% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 22 | 1.00% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4152` |
| 23 | 0.94% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 24 | 0.83% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |
| 25 | 0.83% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |
| 26 | 0.83% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 5366` |
| 27 | 0.82% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 28 | 0.77% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 29 | 0.72% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 5011` |
| 30 | 0.70% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |

GC pauses: 0, total 0 ms

