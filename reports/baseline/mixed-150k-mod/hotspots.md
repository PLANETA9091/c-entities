# JVM hotspots (JFR ExecutionSample)

Samples: 43464

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 14.38% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2669` |
| 2 | 12.44% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 3 | 4.06% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2400` |
| 4 | 2.83% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10100` |
| 5 | 2.28% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 1.64% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6742` |
| 7 | 1.55% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 8 | 1.15% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2637` |
| 9 | 1.15% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 10 | 1.06% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.05% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 12 | 0.91% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 13 | 0.87% | `dev.planeta9091.centities.CeHooks.ordSortVanilla(ArrayList) line: 2398` |
| 14 | 0.80% | `java.util.TimSort.countRunAndMakeAscending(Object[], int, int, Comparator) line: 360` |
| 15 | 0.78% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 16 | 0.78% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5979` |
| 17 | 0.77% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2409` |
| 18 | 0.75% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 19 | 0.70% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1291` |
| 20 | 0.67% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 21 | 0.59% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 22 | 0.53% | `java.util.EnumMap.get(Object) line: 248` |
| 23 | 0.52% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 10635` |
| 24 | 0.50% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 25 | 0.48% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 26 | 0.48% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 27 | 0.48% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 28 | 0.47% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 29 | 0.47% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 30 | 0.46% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 31 | 0.45% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 32 | 0.44% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 33 | 0.41% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3099` |
| 34 | 0.40% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10091` |
| 35 | 0.40% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 36 | 0.40% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 37 | 0.37% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 38 | 0.36% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 39 | 0.35% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7249` |
| 40 | 0.35% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2696` |
| 41 | 0.35% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.<init>(Map, float) line: 124` |
| 42 | 0.32% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 43 | 0.31% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 44 | 0.29% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 45 | 0.28% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 46 | 0.27% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 47 | 0.27% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2896` |
| 48 | 0.27% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 49 | 0.26% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 50 | 0.26% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3137` |
| 51 | 0.26% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6755` |
| 52 | 0.25% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4468` |
| 53 | 0.25% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 54 | 0.24% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2481` |
| 55 | 0.24% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7317` |
| 56 | 0.23% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 57 | 0.23% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 58 | 0.23% | `dev.planeta9091.centities.CeHooks.vtabHash(int) line: 376` |
| 59 | 0.23% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 60 | 0.23% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 16.60% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2442` |
| 2 | 12.44% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4688` |
| 3 | 5.17% | `java.util.TimSort.countRunAndMakeAscending(Object[], int, int, Comparator) line: 360` |
| 4 | 3.70% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10270` |
| 5 | 3.34% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 6 | 2.09% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 7 | 1.65% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 8 | 1.59% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2481` |
| 9 | 1.48% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2663` |
| 10 | 1.24% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 11 | 1.05% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 12 | 1.05% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 266` |
| 13 | 1.01% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 14 | 0.96% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2722` |
| 15 | 0.87% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2817` |
| 16 | 0.84% | `java.util.TimSort.sort(Object[], int, int, Comparator, Object[], int, int) line: 234` |
| 17 | 0.80% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 18 | 0.79% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 19 | 0.69% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 20 | 0.67% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 21 | 0.67% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 22 | 0.57% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7332` |
| 23 | 0.51% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 24 | 0.50% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 25 | 0.49% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 26 | 0.47% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 27 | 0.47% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 28 | 0.45% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 29 | 0.44% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 30 | 0.43% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |

GC pauses: 0, total 0 ms

