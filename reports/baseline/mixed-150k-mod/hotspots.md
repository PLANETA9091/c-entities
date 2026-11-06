# JVM hotspots (JFR ExecutionSample)

Samples: 43357

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 15.17% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2747` |
| 2 | 10.42% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 3 | 3.25% | `net.minecraft.world.entity.Entity.moonrise$getSectionZ() line: 463` |
| 4 | 3.08% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10508` |
| 5 | 2.36% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 2.35% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 11144` |
| 7 | 1.68% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6883` |
| 8 | 1.60% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 9 | 1.34% | `dev.planeta9091.centities.CeHooks.ordSortVanilla(ArrayList) line: 2450` |
| 10 | 1.20% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 11 | 1.15% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 1.11% | `dev.planeta9091.centities.CeHooks.ordKey(Entity, int) line: 2307` |
| 13 | 1.05% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 14 | 0.96% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 15 | 0.92% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 16 | 0.80% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 17 | 0.78% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1293` |
| 18 | 0.69% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2715` |
| 19 | 0.68% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6120` |
| 20 | 0.66% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 21 | 0.64% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 22 | 0.51% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 11056` |
| 23 | 0.50% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 24 | 0.47% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 25 | 0.45% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 26 | 0.45% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 27 | 0.44% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 28 | 0.43% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 29 | 0.43% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 30 | 0.41% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 31 | 0.41% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 32 | 0.38% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10499` |
| 33 | 0.38% | `java.util.EnumMap.get(Object) line: 248` |
| 34 | 0.38% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 35 | 0.35% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7403` |
| 36 | 0.33% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 37 | 0.33% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 38 | 0.33% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 39 | 0.32% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 40 | 0.32% | `dev.planeta9091.centities.CeHooks.ordSortVanilla(ArrayList) line: 2448` |
| 41 | 0.31% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 42 | 0.28% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 43 | 0.27% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 44 | 0.27% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 45 | 0.27% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 46 | 0.27% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2996` |
| 47 | 0.25% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4568` |
| 48 | 0.25% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4436` |
| 49 | 0.24% | `java.lang.String.equals(Object) line: 1926` |
| 50 | 0.24% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 51 | 0.24% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 7056` |
| 52 | 0.24% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6896` |
| 53 | 0.24% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 54 | 0.23% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 55 | 0.23% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 56 | 0.23% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6191` |
| 57 | 0.23% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 58 | 0.22% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 59 | 0.21% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 60 | 0.21% | `dev.planeta9091.centities.CeHooks.equipUpdates$(LivingEntity) line: 10952` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 16.43% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2520` |
| 2 | 10.42% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4788` |
| 3 | 4.04% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10691` |
| 4 | 3.41% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 3.25% | `dev.planeta9091.centities.CeHooks.ordKey(Entity, int) line: 2304` |
| 6 | 2.41% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 7 | 2.09% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 8 | 1.70% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2800` |
| 9 | 1.55% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2741` |
| 10 | 1.36% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 11 | 1.27% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 12 | 1.21% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2559` |
| 13 | 1.21% | `dev.planeta9091.centities.CeHooks.ordSortVanilla(ArrayList) line: 2451` |
| 14 | 1.11% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 15 | 1.05% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 16 | 0.98% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6055` |
| 17 | 0.91% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 18 | 0.79% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 19 | 0.77% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2817` |
| 20 | 0.72% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 21 | 0.69% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 22 | 0.65% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 23 | 0.61% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 24 | 0.57% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 25 | 0.50% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6893` |
| 26 | 0.49% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 7902` |
| 27 | 0.48% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 28 | 0.46% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7496` |
| 29 | 0.46% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 30 | 0.46% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |

GC pauses: 0, total 0 ms

