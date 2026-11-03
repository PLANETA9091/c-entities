# JVM hotspots (JFR ExecutionSample)

Samples: 36698

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.79% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 2 | 3.49% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2668` |
| 3 | 3.00% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10643` |
| 4 | 2.90% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3136` |
| 5 | 2.84% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 2.51% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3098` |
| 7 | 2.33% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6736` |
| 8 | 2.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 9 | 1.60% | `dev.planeta9091.centities.CeHooks.vtabHash(int) line: 376` |
| 10 | 1.45% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3081` |
| 11 | 1.39% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.35% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 1.33% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10041` |
| 14 | 1.32% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3110` |
| 15 | 1.05% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 16 | 1.03% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 17 | 1.02% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2399` |
| 18 | 1.02% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 19 | 0.96% | `java.util.concurrent.CompletableFuture.isDone() line: 2076` |
| 20 | 0.90% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 21 | 0.87% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 22 | 0.87% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 23 | 0.77% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 24 | 0.75% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 25 | 0.75% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 26 | 0.71% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 27 | 0.71% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 28 | 0.63% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 29 | 0.62% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 30 | 0.61% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 31 | 0.61% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 32 | 0.56% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 33 | 0.56% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 34 | 0.56% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2895` |
| 35 | 0.55% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 36 | 0.54% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2408` |
| 37 | 0.53% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7243` |
| 38 | 0.51% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 39 | 0.51% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 40 | 0.50% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 41 | 0.48% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 42 | 0.47% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 424` |
| 43 | 0.44% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 44 | 0.44% | `java.util.EnumMap.get(Object) line: 248` |
| 45 | 0.44% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 46 | 0.43% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 47 | 0.41% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5973` |
| 48 | 0.40% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 10555` |
| 49 | 0.40% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3095` |
| 50 | 0.39% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 51 | 0.39% | `java.util.ArrayList.<init>(int) line: 157` |
| 52 | 0.39% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 53 | 0.39% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 54 | 0.36% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 6909` |
| 55 | 0.36% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7311` |
| 56 | 0.36% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 57 | 0.35% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4833` |
| 58 | 0.34% | `dev.planeta9091.centities.CeHooks.ordSortVanilla(ArrayList) line: 2397` |
| 59 | 0.34% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 60 | 0.33% | `java.util.ImmutableCollections$ListItr.next() line: 397` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 10.68% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2480` |
| 2 | 6.79% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4687` |
| 3 | 4.22% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 3.90% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2441` |
| 5 | 3.02% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 6 | 2.52% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 7 | 2.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 8 | 1.89% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10189` |
| 9 | 1.78% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 10 | 1.63% | `java.util.TimSort.countRunAndMakeAscending(Object[], int, int, Comparator) line: 360` |
| 11 | 1.60% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 390` |
| 12 | 1.33% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 13 | 1.03% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 14 | 0.96% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 15 | 0.96% | `net.minecraft.util.thread.BlockableEventLoop.managedBlock(BooleanSupplier) line: 190` |
| 16 | 0.90% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7326` |
| 17 | 0.87% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 18 | 0.87% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 19 | 0.87% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 20 | 0.78% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 21 | 0.76% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 22 | 0.72% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 23 | 0.70% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 24 | 0.69% | `dev.planeta9091.centities.CeHooks.mirrorTyped(Level, Class, AABB, Predicate) line: 1351` |
| 25 | 0.69% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 26 | 0.68% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 27 | 0.66% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 28 | 0.63% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 848` |
| 29 | 0.62% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |
| 30 | 0.62% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7311` |

GC pauses: 0, total 0 ms

