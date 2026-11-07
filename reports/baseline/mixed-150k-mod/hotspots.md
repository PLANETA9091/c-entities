# JVM hotspots (JFR ExecutionSample)

Samples: 36485

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 2.98% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10830` |
| 2 | 2.95% | `jdk.internal.misc.Unsafe.getLongUnaligned(Object, long, boolean) line: 3571` |
| 3 | 2.86% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6967` |
| 4 | 2.69% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 2.60% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2690` |
| 6 | 2.11% | `jdk.internal.misc.Unsafe.convEndian(boolean, long) line: 3835` |
| 7 | 1.89% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 8 | 1.69% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 9 | 1.51% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 10 | 1.35% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 11 | 1.31% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 1.30% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 13 | 1.27% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 14 | 1.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6204` |
| 15 | 1.13% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 16 | 1.13% | `java.lang.ThreadLocal.get() line: 171` |
| 17 | 1.07% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 18 | 1.00% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 19 | 0.86% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 11290` |
| 20 | 0.84% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3584` |
| 21 | 0.82% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 22 | 0.77% | `java.util.HashMap.getNode(Object) line: 586` |
| 23 | 0.74% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 24 | 0.72% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3250` |
| 25 | 0.70% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2797` |
| 26 | 0.64% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 27 | 0.63% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 28 | 0.62% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 29 | 0.56% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 30 | 0.53% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 31 | 0.52% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 32 | 0.50% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 33 | 0.49% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 34 | 0.49% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 35 | 0.48% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 36 | 0.47% | `java.util.EnumMap.get(Object) line: 248` |
| 37 | 0.45% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 38 | 0.45% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2597` |
| 39 | 0.44% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 40 | 0.44% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 41 | 0.43% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 42 | 0.43% | `dev.planeta9091.centities.CeHooks.vtabHash(int) line: 393` |
| 43 | 0.42% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3289` |
| 44 | 0.42% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7487` |
| 45 | 0.41% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 46 | 0.38% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 47 | 0.37% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 48 | 0.37% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 7140` |
| 49 | 0.37% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 50 | 0.37% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 51 | 0.36% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3046` |
| 52 | 0.35% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2765` |
| 53 | 0.34% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10692` |
| 54 | 0.33% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 55 | 0.31% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 56 | 0.31% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 57 | 0.30% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 58 | 0.30% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4624` |
| 59 | 0.30% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 60 | 0.29% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.80% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2558` |
| 2 | 3.96% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 3.11% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 3.01% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 2.95% | `jdk.internal.misc.ScopedMemoryAccess.getLongUnalignedInternal(MemorySessionImpl, Object, long, boolean) line: 3861` |
| 6 | 2.54% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2597` |
| 7 | 2.11% | `jdk.internal.misc.Unsafe.getLongUnaligned(Object, long, boolean) line: 3571` |
| 8 | 1.80% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 9 | 1.66% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 10 | 1.65% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 11 | 1.55% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6122` |
| 12 | 1.51% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4844` |
| 13 | 1.30% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2817` |
| 14 | 1.20% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 15 | 1.15% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 16 | 1.12% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 17 | 1.12% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 18 | 1.04% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 19 | 0.89% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3696` |
| 20 | 0.87% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2577` |
| 21 | 0.86% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 22 | 0.86% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 23 | 0.79% | `java.util.HashMap.get(Object) line: 565` |
| 24 | 0.77% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 25 | 0.69% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10925` |
| 26 | 0.67% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 27 | 0.66% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 28 | 0.65% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6977` |
| 29 | 0.63% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 30 | 0.62% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2791` |

GC pauses: 0, total 0 ms

