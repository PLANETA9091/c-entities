# JVM hotspots (JFR ExecutionSample)

Samples: 64804

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 11.11% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 2 | 10.01% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2797` |
| 3 | 5.33% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3250` |
| 4 | 3.30% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3289` |
| 5 | 2.39% | `dev.planeta9091.centities.CeHooks.ordSortVanilla(ArrayList) line: 2488` |
| 6 | 2.26% | `dev.planeta9091.centities.CeHooks.ordKey(Entity, int) line: 2343` |
| 7 | 1.62% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10830` |
| 8 | 1.36% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6967` |
| 9 | 1.31% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 10 | 1.22% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 11 | 1.04% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3263` |
| 12 | 0.91% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3233` |
| 13 | 0.87% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 14 | 0.85% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3247` |
| 15 | 0.84% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 16 | 0.80% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 17 | 0.77% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.executeMainThreadTask() line: 329` |
| 18 | 0.76% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 19 | 0.69% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 20 | 0.67% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 21 | 0.65% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3046` |
| 22 | 0.64% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 23 | 0.59% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 24 | 0.59% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 25 | 0.58% | `net.minecraft.world.entity.Entity.moonrise$getSectionY() line: 453` |
| 26 | 0.53% | `java.util.ArrayList.<init>(int) line: 157` |
| 27 | 0.53% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3330` |
| 28 | 0.53% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 29 | 0.52% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 30 | 0.50% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 31 | 0.49% | `java.util.HashMap.getNode(Object) line: 586` |
| 32 | 0.48% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 441` |
| 33 | 0.46% | `dev.planeta9091.centities.CeHooks.vtabHash(int) line: 393` |
| 34 | 0.46% | `java.util.EnumMap.get(Object) line: 248` |
| 35 | 0.41% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 36 | 0.41% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 37 | 0.40% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 38 | 0.39% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 39 | 0.38% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3240` |
| 40 | 0.38% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 41 | 0.38% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 42 | 0.38% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6204` |
| 43 | 0.37% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 44 | 0.37% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 45 | 0.36% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 46 | 0.36% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 47 | 0.36% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2765` |
| 48 | 0.35% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3584` |
| 49 | 0.34% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 50 | 0.34% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3264` |
| 51 | 0.33% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7487` |
| 52 | 0.32% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 53 | 0.32% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 54 | 0.31% | `dev.planeta9091.centities.CeHooks.ordSortVanilla(ArrayList) line: 2486` |
| 55 | 0.30% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3246` |
| 56 | 0.30% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 11290` |
| 57 | 0.24% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 58 | 0.24% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 59 | 0.24% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3707` |
| 60 | 0.23% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 15.37% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2597` |
| 2 | 10.78% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2558` |
| 3 | 6.82% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2791` |
| 4 | 4.21% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3268` |
| 5 | 3.05% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2850` |
| 6 | 2.26% | `dev.planeta9091.centities.CeHooks.ordSortVanilla(ArrayList) line: 2489` |
| 7 | 1.96% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 8 | 1.63% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 9 | 1.52% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 10 | 1.05% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 11 | 0.88% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 12 | 0.80% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4844` |
| 13 | 0.77% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 853` |
| 14 | 0.76% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 15 | 0.71% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 16 | 0.67% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 17 | 0.66% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 18 | 0.65% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3246` |
| 19 | 0.63% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2817` |
| 20 | 0.62% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 21 | 0.60% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 22 | 0.59% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 23 | 0.58% | `dev.planeta9091.centities.CeHooks.ordKey(Entity, int) line: 2342` |
| 24 | 0.54% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 25 | 0.52% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3192` |
| 26 | 0.50% | `java.util.HashMap.get(Object) line: 565` |
| 27 | 0.50% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |
| 28 | 0.50% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6122` |
| 29 | 0.46% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 407` |
| 30 | 0.45% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3739` |

GC pauses: 0, total 0 ms

