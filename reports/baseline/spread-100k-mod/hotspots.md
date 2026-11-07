# JVM hotspots (JFR ExecutionSample)

Samples: 56327

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 11.18% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2831` |
| 2 | 10.64% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 3 | 2.56% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3366` |
| 4 | 2.36% | `dev.planeta9091.centities.CeHooks.vtabHash(int) line: 393` |
| 5 | 2.29% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1312` |
| 6 | 1.91% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10935` |
| 7 | 1.59% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7072` |
| 8 | 1.57% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 9 | 1.32% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3310` |
| 10 | 1.17% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 11 | 0.95% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 12 | 0.91% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3340` |
| 13 | 0.91% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 14 | 0.83% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 15 | 0.82% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 16 | 0.80% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 17 | 0.80% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 18 | 0.78% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1512` |
| 19 | 0.78% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 20 | 0.75% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 21 | 0.74% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 22 | 0.64% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3323` |
| 23 | 0.62% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 24 | 0.62% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3324` |
| 25 | 0.61% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 26 | 0.59% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3318` |
| 27 | 0.57% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3112` |
| 28 | 0.54% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 29 | 0.54% | `java.util.HashMap.getNode(Object) line: 586` |
| 30 | 0.53% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 31 | 0.50% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 32 | 0.48% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 33 | 0.47% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3327` |
| 34 | 0.47% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6309` |
| 35 | 0.46% | `java.util.EnumMap.get(Object) line: 248` |
| 36 | 0.45% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2799` |
| 37 | 0.45% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 38 | 0.44% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 39 | 0.43% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 11395` |
| 40 | 0.43% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 41 | 0.43% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3330` |
| 42 | 0.41% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 43 | 0.41% | `java.util.ArrayList.<init>(int) line: 157` |
| 44 | 0.41% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 45 | 0.40% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 46 | 0.39% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 47 | 0.38% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7592` |
| 48 | 0.36% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 49 | 0.35% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3584` |
| 50 | 0.34% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3312` |
| 51 | 0.32% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 52 | 0.29% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 53 | 0.29% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 54 | 0.28% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3707` |
| 55 | 0.28% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 56 | 0.27% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 57 | 0.26% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3309` |
| 58 | 0.26% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 59 | 0.26% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 60 | 0.25% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 7245` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 12.15% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2580` |
| 2 | 10.43% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2619` |
| 3 | 7.43% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2825` |
| 4 | 3.12% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3345` |
| 5 | 2.56% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 6 | 2.40% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 7 | 2.36% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 407` |
| 8 | 1.93% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 9 | 1.92% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 10 | 1.27% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 11 | 1.03% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 12 | 1.02% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 13 | 0.85% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6227` |
| 14 | 0.84% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 15 | 0.82% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 16 | 0.80% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4949` |
| 17 | 0.77% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 18 | 0.76% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 19 | 0.73% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 20 | 0.68% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 21 | 0.64% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 22 | 0.58% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2817` |
| 23 | 0.56% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 24 | 0.55% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 25 | 0.55% | `java.util.HashMap.get(Object) line: 565` |
| 26 | 0.55% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 27 | 0.53% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |
| 28 | 0.50% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 29 | 0.50% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 30 | 0.48% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |

GC pauses: 0, total 0 ms

