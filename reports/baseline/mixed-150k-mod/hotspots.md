# JVM hotspots (JFR ExecutionSample)

Samples: 35459

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.85% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2731` |
| 2 | 3.29% | `jdk.internal.misc.Unsafe.getLongUnaligned(Object, long, boolean) line: 3571` |
| 3 | 3.27% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7107` |
| 4 | 3.03% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10970` |
| 5 | 2.66% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 1.85% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 7 | 1.77% | `jdk.internal.misc.Unsafe.convEndian(boolean, long) line: 3835` |
| 8 | 1.51% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 9 | 1.44% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 10 | 1.38% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 11 | 1.30% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.28% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 13 | 1.17% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 14 | 1.14% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 15 | 1.05% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 16 | 0.99% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6344` |
| 17 | 0.85% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 18 | 0.77% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3584` |
| 19 | 0.75% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 20 | 0.71% | `dev.planeta9091.centities.CeHooks.vtabHash(int) line: 411` |
| 21 | 0.66% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 22 | 0.65% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 23 | 0.64% | `java.util.HashMap.getNode(Object) line: 586` |
| 24 | 0.64% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 25 | 0.56% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 26 | 0.55% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 27 | 0.55% | `java.util.EnumMap.get(Object) line: 248` |
| 28 | 0.54% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 29 | 0.53% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3401` |
| 30 | 0.52% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 31 | 0.51% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 32 | 0.50% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 33 | 0.49% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 11430` |
| 34 | 0.49% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 35 | 0.48% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2889` |
| 36 | 0.47% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 37 | 0.47% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2638` |
| 38 | 0.46% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 39 | 0.44% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 40 | 0.44% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 41 | 0.44% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 42 | 0.42% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 43 | 0.40% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 44 | 0.40% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7627` |
| 45 | 0.40% | `java.lang.ThreadLocal.get() line: 171` |
| 46 | 0.38% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3147` |
| 47 | 0.37% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4764` |
| 48 | 0.37% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 49 | 0.36% | `dev.planeta9091.centities.CeHooks.getTicksFrozen$(Entity) line: 11529` |
| 50 | 0.35% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 51 | 0.35% | `java.util.EnumMap$EnumMapIterator.hasNext() line: 524` |
| 52 | 0.33% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 7280` |
| 53 | 0.33% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 54 | 0.32% | `java.lang.ref.Reference.refersToImpl(Object) line: 391` |
| 55 | 0.31% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 56 | 0.30% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 57 | 0.30% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10832` |
| 58 | 0.30% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 59 | 0.29% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7120` |
| 60 | 0.29% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3707` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.78% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2599` |
| 2 | 3.96% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 3.84% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 3.29% | `jdk.internal.misc.ScopedMemoryAccess.getLongUnalignedInternal(MemorySessionImpl, Object, long, boolean) line: 3861` |
| 5 | 3.06% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 6 | 2.91% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2638` |
| 7 | 1.77% | `jdk.internal.misc.Unsafe.getLongUnaligned(Object, long, boolean) line: 3571` |
| 8 | 1.69% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 9 | 1.66% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 10 | 1.62% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 11 | 1.44% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4984` |
| 12 | 1.27% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6262` |
| 13 | 1.25% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2817` |
| 14 | 1.15% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 15 | 1.10% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 16 | 0.97% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 17 | 0.88% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 18 | 0.84% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 19 | 0.80% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3696` |
| 20 | 0.78% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 21 | 0.74% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 22 | 0.71% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 425` |
| 23 | 0.69% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 8126` |
| 24 | 0.67% | `java.util.HashMap.get(Object) line: 565` |
| 25 | 0.67% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 26 | 0.67% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 27 | 0.67% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 28 | 0.66% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 29 | 0.63% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7117` |
| 30 | 0.61% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7720` |

GC pauses: 0, total 0 ms

