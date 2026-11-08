# JVM hotspots (JFR ExecutionSample)

Samples: 34901

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.34% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7120` |
| 2 | 3.16% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10983` |
| 3 | 2.82% | `jdk.internal.misc.Unsafe.getLongUnaligned(Object, long, boolean) line: 3571` |
| 4 | 2.75% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 2.54% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2744` |
| 6 | 1.81% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 7 | 1.64% | `jdk.internal.misc.Unsafe.convEndian(boolean, long) line: 3835` |
| 8 | 1.58% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 9 | 1.55% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 10 | 1.46% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 11 | 1.38% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.29% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 13 | 1.21% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 14 | 1.11% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 15 | 1.08% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 16 | 0.91% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6357` |
| 17 | 0.87% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3584` |
| 18 | 0.86% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 11443` |
| 19 | 0.85% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 20 | 0.79% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 21 | 0.77% | `java.util.HashMap.getNode(Object) line: 586` |
| 22 | 0.72% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 23 | 0.68% | `java.util.EnumMap.get(Object) line: 248` |
| 24 | 0.68% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 25 | 0.66% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 26 | 0.64% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 27 | 0.60% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 28 | 0.58% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1313` |
| 29 | 0.56% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 30 | 0.54% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 31 | 0.53% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 32 | 0.53% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 33 | 0.50% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7640` |
| 34 | 0.50% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 35 | 0.49% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2651` |
| 36 | 0.48% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 37 | 0.47% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 38 | 0.46% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 39 | 0.45% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 40 | 0.42% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 41 | 0.41% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 42 | 0.40% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 43 | 0.40% | `java.lang.ThreadLocal.get() line: 171` |
| 44 | 0.39% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 45 | 0.39% | `dev.planeta9091.centities.CeHooks.vtabHash(int) line: 393` |
| 46 | 0.37% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2837` |
| 47 | 0.36% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 48 | 0.36% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3160` |
| 49 | 0.35% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 50 | 0.34% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 51 | 0.34% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4777` |
| 52 | 0.34% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10845` |
| 53 | 0.34% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 54 | 0.33% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 7293` |
| 55 | 0.33% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7133` |
| 56 | 0.30% | `java.util.stream.MatchOps$1MatchSink.accept(Object) line: 90` |
| 57 | 0.29% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 58 | 0.29% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 59 | 0.29% | `java.lang.ThreadLocal$ThreadLocalMap.getEntry(ThreadLocal) line: 486` |
| 60 | 0.28% | `net.minecraft.world.entity.Entity.level() line: 5421` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.17% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2612` |
| 2 | 4.13% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 3.93% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 3.18% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 2.82% | `jdk.internal.misc.ScopedMemoryAccess.getLongUnalignedInternal(MemorySessionImpl, Object, long, boolean) line: 3861` |
| 6 | 1.90% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2651` |
| 7 | 1.64% | `jdk.internal.misc.Unsafe.getLongUnaligned(Object, long, boolean) line: 3571` |
| 8 | 1.63% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 9 | 1.57% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 10 | 1.56% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 11 | 1.50% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 12 | 1.45% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4997` |
| 13 | 1.44% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6275` |
| 14 | 1.25% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2817` |
| 15 | 1.06% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 16 | 1.03% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 17 | 1.00% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 18 | 0.98% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 19 | 0.93% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 20 | 0.91% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 21 | 0.91% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3696` |
| 22 | 0.91% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 23 | 0.79% | `java.util.HashMap.get(Object) line: 565` |
| 24 | 0.79% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 25 | 0.73% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 26 | 0.70% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 27 | 0.68% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 28 | 0.66% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 29 | 0.64% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 30 | 0.64% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 11078` |

GC pauses: 0, total 0 ms

