# JVM hotspots (JFR ExecutionSample)

Samples: 57266

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 12.15% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2831` |
| 2 | 11.86% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 3 | 2.41% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3366` |
| 4 | 2.18% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1312` |
| 5 | 2.07% | `dev.planeta9091.centities.CeHooks.vtabHash(int) line: 393` |
| 6 | 1.91% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3310` |
| 7 | 1.77% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10935` |
| 8 | 1.51% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 9 | 1.50% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7072` |
| 10 | 1.23% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 11 | 1.20% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 12 | 1.02% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3340` |
| 13 | 0.98% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 14 | 0.98% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 15 | 0.90% | `java.util.ArrayDeque.pollFirst() line: 378` |
| 16 | 0.90% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 17 | 0.89% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 18 | 0.75% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 19 | 0.69% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 20 | 0.69% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3112` |
| 21 | 0.68% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3324` |
| 22 | 0.67% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 23 | 0.67% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 24 | 0.66% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 25 | 0.59% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 26 | 0.59% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 27 | 0.55% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6309` |
| 28 | 0.55% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2799` |
| 29 | 0.55% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 30 | 0.53% | `java.util.HashMap.getNode(Object) line: 586` |
| 31 | 0.49% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 32 | 0.49% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 33 | 0.48% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 34 | 0.48% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 35 | 0.45% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 36 | 0.41% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 37 | 0.41% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 38 | 0.39% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 39 | 0.37% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 11395` |
| 40 | 0.36% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3312` |
| 41 | 0.35% | `java.util.EnumMap.get(Object) line: 248` |
| 42 | 0.35% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 43 | 0.34% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3318` |
| 44 | 0.34% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7592` |
| 45 | 0.33% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3707` |
| 46 | 0.32% | `java.util.ArrayList.<init>(int) line: 157` |
| 47 | 0.32% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 48 | 0.31% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 49 | 0.31% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3327` |
| 50 | 0.31% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3102` |
| 51 | 0.30% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3584` |
| 52 | 0.29% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3341` |
| 53 | 0.29% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 54 | 0.28% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3309` |
| 55 | 0.25% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 56 | 0.24% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 57 | 0.24% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 441` |
| 58 | 0.24% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 59 | 0.23% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 60 | 0.23% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 7245` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 13.13% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2580` |
| 2 | 10.05% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2619` |
| 3 | 8.86% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2825` |
| 4 | 2.87% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3345` |
| 5 | 2.39% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 6 | 2.07% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 407` |
| 7 | 1.90% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 8 | 1.78% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 9 | 1.77% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 10 | 1.34% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 11 | 0.98% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 12 | 0.92% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 13 | 0.92% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 14 | 0.90% | `java.util.ArrayDeque.removeFirst() line: 358` |
| 15 | 0.89% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4949` |
| 16 | 0.87% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 17 | 0.86% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 18 | 0.77% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 19 | 0.73% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 20 | 0.71% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2817` |
| 21 | 0.69% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 22 | 0.60% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 23 | 0.60% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 24 | 0.59% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |
| 25 | 0.56% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6227` |
| 26 | 0.54% | `java.util.HashMap.get(Object) line: 565` |
| 27 | 0.51% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 28 | 0.51% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 29 | 0.49% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |
| 30 | 0.48% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |

GC pauses: 0, total 0 ms

