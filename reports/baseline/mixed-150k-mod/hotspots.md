# JVM hotspots (JFR ExecutionSample)

Samples: 40982

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 12.54% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2747` |
| 2 | 7.84% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 3 | 3.28% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10521` |
| 4 | 3.28% | `net.minecraft.world.entity.Entity.moonrise$getSectionZ() line: 463` |
| 5 | 2.69% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 2.65% | `dev.planeta9091.centities.CeHooks.ordSortVanilla(ArrayList) line: 2450` |
| 7 | 2.41% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6883` |
| 8 | 1.94% | `dev.planeta9091.centities.CeHooks.ordKey(Entity, int) line: 2307` |
| 9 | 1.45% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.43% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 11 | 1.34% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 12 | 1.16% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 13 | 1.15% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1293` |
| 14 | 1.12% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 15 | 1.06% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 16 | 0.97% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 17 | 0.84% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 18 | 0.79% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6120` |
| 19 | 0.71% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 20 | 0.70% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2715` |
| 21 | 0.66% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 22 | 0.66% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 23 | 0.63% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 24 | 0.61% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 25 | 0.60% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 26 | 0.59% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 27 | 0.58% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 28 | 0.54% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 29 | 0.53% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 30 | 0.46% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 31 | 0.46% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10512` |
| 32 | 0.46% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 33 | 0.45% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 11069` |
| 34 | 0.44% | `java.util.EnumMap.get(Object) line: 248` |
| 35 | 0.43% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 36 | 0.41% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3584` |
| 37 | 0.40% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 38 | 0.39% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 39 | 0.38% | `dev.planeta9091.centities.CeHooks.ordSortVanilla(ArrayList) line: 2448` |
| 40 | 0.36% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7403` |
| 41 | 0.35% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 42 | 0.34% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 43 | 0.34% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4568` |
| 44 | 0.32% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 45 | 0.32% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10540` |
| 46 | 0.32% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 47 | 0.30% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 48 | 0.29% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 49 | 0.29% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 50 | 0.29% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 51 | 0.28% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 52 | 0.28% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2996` |
| 53 | 0.28% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 7056` |
| 54 | 0.28% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4934` |
| 55 | 0.27% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 56 | 0.27% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 57 | 0.27% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 58 | 0.26% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1290` |
| 59 | 0.24% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 60 | 0.24% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3707` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 13.75% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2520` |
| 2 | 7.84% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4788` |
| 3 | 4.55% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10704` |
| 4 | 4.14% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 3.27% | `dev.planeta9091.centities.CeHooks.ordKey(Entity, int) line: 2304` |
| 6 | 3.08% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2800` |
| 7 | 2.68% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 8 | 2.03% | `dev.planeta9091.centities.CeHooks.ordSortVanilla(ArrayList) line: 2451` |
| 9 | 1.59% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 10 | 1.55% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 11 | 1.52% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 12 | 1.30% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2817` |
| 13 | 1.24% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6055` |
| 14 | 1.06% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 15 | 1.02% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 16 | 0.96% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2559` |
| 17 | 0.95% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 18 | 0.81% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 19 | 0.77% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2741` |
| 20 | 0.74% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 21 | 0.67% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 22 | 0.66% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 23 | 0.64% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 24 | 0.62% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 25 | 0.60% | `net.minecraft.world.entity.LivingEntity.tick() line: 3491` |
| 26 | 0.60% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 27 | 0.60% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 28 | 0.58% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7481` |
| 29 | 0.57% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 7902` |
| 30 | 0.56% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |

GC pauses: 0, total 0 ms

