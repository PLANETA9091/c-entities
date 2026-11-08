# JVM hotspots (JFR ExecutionSample)

Samples: 43598

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.47% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 2 | 2.90% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3414` |
| 3 | 2.61% | `dev.planeta9091.centities.CeHooks.vtabHash(int) line: 393` |
| 4 | 2.24% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10983` |
| 5 | 2.15% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1313` |
| 6 | 1.73% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 1.72% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7120` |
| 8 | 1.70% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 9 | 1.63% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2837` |
| 10 | 1.53% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3358` |
| 11 | 1.11% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 12 | 1.04% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 1.02% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3388` |
| 14 | 1.00% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 311` |
| 15 | 0.98% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 16 | 0.96% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 17 | 0.95% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3378` |
| 18 | 0.92% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 19 | 0.88% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 20 | 0.86% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3372` |
| 21 | 0.85% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 22 | 0.78% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 23 | 0.74% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 24 | 0.73% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 25 | 0.72% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3160` |
| 26 | 0.71% | `java.util.ArrayList.elementData(int) line: 412` |
| 27 | 0.69% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 28 | 0.67% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2838` |
| 29 | 0.66% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 30 | 0.64% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 31 | 0.64% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 32 | 0.64% | `java.util.HashMap.getNode(Object) line: 586` |
| 33 | 0.61% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 34 | 0.61% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6357` |
| 35 | 0.57% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 11443` |
| 36 | 0.56% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 37 | 0.56% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3360` |
| 38 | 0.54% | `java.util.EnumMap.get(Object) line: 248` |
| 39 | 0.52% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 40 | 0.52% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 41 | 0.52% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 42 | 0.48% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 43 | 0.48% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7640` |
| 44 | 0.47% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 45 | 0.47% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3375` |
| 46 | 0.46% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 47 | 0.46% | `java.util.ArrayList.<init>(int) line: 157` |
| 48 | 0.45% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3150` |
| 49 | 0.43% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 50 | 0.43% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3584` |
| 51 | 0.40% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3366` |
| 52 | 0.38% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 53 | 0.36% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 54 | 0.35% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 55 | 0.33% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 56 | 0.32% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 57 | 0.32% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 58 | 0.31% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 59 | 0.31% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3707` |
| 60 | 0.31% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 11.63% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2651` |
| 2 | 3.31% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3393` |
| 3 | 2.78% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 2.76% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2612` |
| 5 | 2.61% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 407` |
| 6 | 2.37% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 7 | 2.26% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 8 | 1.99% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 9 | 1.47% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 10 | 1.29% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 11 | 1.07% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 12 | 1.03% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 13 | 1.00% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 14 | 0.98% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 15 | 0.96% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4997` |
| 16 | 0.87% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 17 | 0.86% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 18 | 0.79% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 19 | 0.76% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 20 | 0.71% | `java.util.ArrayList.get(int) line: 429` |
| 21 | 0.70% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 22 | 0.68% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 23 | 0.67% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 24 | 0.67% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 25 | 0.66% | `java.util.HashMap.get(Object) line: 565` |
| 26 | 0.66% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |
| 27 | 0.65% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 28 | 0.64% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 29 | 0.63% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2817` |
| 30 | 0.61% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6275` |

GC pauses: 0, total 0 ms

