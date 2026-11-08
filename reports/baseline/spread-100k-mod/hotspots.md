# JVM hotspots (JFR ExecutionSample)

Samples: 40177

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.10% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1326` |
| 2 | 3.25% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 3 | 2.59% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3396` |
| 4 | 2.47% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3340` |
| 5 | 1.92% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7102` |
| 6 | 1.82% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2816` |
| 7 | 1.81% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10965` |
| 8 | 1.70% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 9 | 1.51% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 1.47% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 11 | 1.37% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3370` |
| 12 | 1.16% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 13 | 1.16% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 14 | 1.14% | `java.util.ArrayDeque.addFirst(Object) line: 286` |
| 15 | 1.13% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 16 | 1.13% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 17 | 1.04% | `dev.planeta9091.centities.CeHooks.vtabHash(int) line: 406` |
| 18 | 0.97% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2817` |
| 19 | 0.96% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3142` |
| 20 | 0.88% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 21 | 0.87% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3354` |
| 22 | 0.84% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 23 | 0.76% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 24 | 0.74% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 25 | 0.69% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 26 | 0.69% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3357` |
| 27 | 0.67% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 28 | 0.62% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3353` |
| 29 | 0.61% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 30 | 0.60% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 31 | 0.59% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 32 | 0.58% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 33 | 0.52% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 34 | 0.51% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 35 | 0.50% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 36 | 0.46% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 37 | 0.43% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 38 | 0.43% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6339` |
| 39 | 0.42% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 40 | 0.42% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10873` |
| 41 | 0.40% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7622` |
| 42 | 0.40% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 43 | 0.40% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3584` |
| 44 | 0.40% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 45 | 0.39% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3360` |
| 46 | 0.39% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 47 | 0.38% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7112` |
| 48 | 0.38% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 49 | 0.37% | `java.util.EnumMap.get(Object) line: 248` |
| 50 | 0.37% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7700` |
| 51 | 0.37% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 52 | 0.36% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3342` |
| 53 | 0.35% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 54 | 0.34% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 55 | 0.34% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 56 | 0.34% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 57 | 0.34% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 58 | 0.33% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 59 | 0.33% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3371` |
| 60 | 0.33% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4759` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 12.91% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2633` |
| 2 | 5.36% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 3 | 3.27% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2594` |
| 4 | 2.95% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3375` |
| 5 | 2.52% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 6 | 2.17% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 7 | 1.84% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 8 | 1.51% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 1.48% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 10 | 1.33% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 11 | 1.29% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 12 | 1.16% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4979` |
| 13 | 1.13% | `java.util.ArrayDeque.push(Object) line: 577` |
| 14 | 1.13% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 15 | 1.12% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 16 | 1.04% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 420` |
| 17 | 1.01% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 18 | 0.95% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6257` |
| 19 | 0.75% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 20 | 0.75% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 21 | 0.73% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 22 | 0.70% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7715` |
| 23 | 0.68% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 24 | 0.67% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |
| 25 | 0.62% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 26 | 0.59% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |
| 27 | 0.58% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 11060` |
| 28 | 0.53% | `net.minecraft.world.level.chunk.PalettedContainer.readPalette(PalettedContainer$Data, int) line: 79` |
| 29 | 0.52% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 30 | 0.51% | `dev.planeta9091.centities.CeHooks.mirrorTyped(Level, Class, AABB, Predicate) line: 1411` |

GC pauses: 0, total 0 ms

