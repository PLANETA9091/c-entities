# JVM hotspots (JFR ExecutionSample)

Samples: 34346

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.57% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 2 | 2.94% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3199` |
| 3 | 2.87% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10508` |
| 4 | 2.76% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3237` |
| 5 | 2.54% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2747` |
| 6 | 2.34% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.11% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6883` |
| 8 | 1.86% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 9 | 1.79% | `dev.planeta9091.centities.CeHooks.vtabHash(int) line: 376` |
| 10 | 1.37% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 5287` |
| 11 | 1.31% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 1.26% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 1.20% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 14 | 1.07% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 15 | 1.07% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3182` |
| 16 | 1.05% | `net.minecraft.world.entity.Entity.moonrise$getSectionZ() line: 463` |
| 17 | 1.02% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 18 | 0.98% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 19 | 0.95% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 20 | 0.86% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 21 | 0.86% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 22 | 0.83% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 23 | 0.78% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3211` |
| 24 | 0.76% | `java.util.EnumMap.get(Object) line: 248` |
| 25 | 0.74% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 26 | 0.71% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 27 | 0.71% | `java.util.concurrent.CompletableFuture.isDone() line: 2076` |
| 28 | 0.68% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 29 | 0.61% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 30 | 0.59% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 31 | 0.57% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 32 | 0.56% | `dev.planeta9091.centities.CeHooks.ordKey(Entity, int) line: 2307` |
| 33 | 0.56% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 34 | 0.55% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 35 | 0.54% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 36 | 0.54% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 37 | 0.53% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 38 | 0.52% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 424` |
| 39 | 0.50% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 40 | 0.50% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 41 | 0.50% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3196` |
| 42 | 0.49% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7403` |
| 43 | 0.49% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 44 | 0.47% | `java.util.ArrayList.<init>(int) line: 157` |
| 45 | 0.45% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 46 | 0.44% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 47 | 0.44% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 11056` |
| 48 | 0.44% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2996` |
| 49 | 0.44% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3584` |
| 50 | 0.44% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 51 | 0.43% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 52 | 0.43% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6120` |
| 53 | 0.41% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3707` |
| 54 | 0.40% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 55 | 0.39% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4934` |
| 56 | 0.37% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3278` |
| 57 | 0.36% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 58 | 0.36% | `dev.planeta9091.centities.CeHooks.ordSortVanilla(ArrayList) line: 2450` |
| 59 | 0.35% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 60 | 0.35% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 8026` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 10.28% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2559` |
| 2 | 5.57% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4788` |
| 3 | 3.60% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 3.29% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10691` |
| 5 | 2.76% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2520` |
| 6 | 2.42% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 7 | 1.87% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 8 | 1.86% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 9 | 1.81% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 10 | 1.79% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 390` |
| 11 | 1.32% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 12 | 1.10% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2817` |
| 13 | 1.07% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 14 | 1.05% | `dev.planeta9091.centities.CeHooks.ordKey(Entity, int) line: 2304` |
| 15 | 1.01% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 16 | 0.96% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 17 | 0.90% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7481` |
| 18 | 0.80% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 19 | 0.77% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 20 | 0.76% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7496` |
| 21 | 0.75% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 22 | 0.74% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 23 | 0.71% | `net.minecraft.util.thread.BlockableEventLoop.managedBlock(BooleanSupplier) line: 190` |
| 24 | 0.68% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 25 | 0.64% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 26 | 0.61% | `dev.planeta9091.centities.CeHooks.ordSortVanilla(ArrayList) line: 2451` |
| 27 | 0.59% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 28 | 0.58% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 29 | 0.58% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 30 | 0.58% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |

GC pauses: 0, total 0 ms

