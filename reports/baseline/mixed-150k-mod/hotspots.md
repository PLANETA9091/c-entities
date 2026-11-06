# JVM hotspots (JFR ExecutionSample)

Samples: 35447

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 12.33% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2747` |
| 2 | 7.95% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 3 | 4.28% | `net.minecraft.world.entity.Entity.moonrise$getSectionZ() line: 463` |
| 4 | 3.55% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 3.01% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10491` |
| 6 | 2.58% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6883` |
| 7 | 2.21% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 11127` |
| 8 | 1.60% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.47% | `dev.planeta9091.centities.CeHooks.ordKey(Entity, int) line: 2307` |
| 10 | 1.37% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 11 | 1.35% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 12 | 1.26% | `dev.planeta9091.centities.CeHooks.ordSortVanilla(ArrayList) line: 2450` |
| 13 | 1.24% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 14 | 1.21% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 15 | 1.06% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 16 | 0.94% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 17 | 0.93% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 18 | 0.88% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2715` |
| 19 | 0.85% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1293` |
| 20 | 0.82% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 21 | 0.78% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6120` |
| 22 | 0.75% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 23 | 0.74% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 320` |
| 24 | 0.71% | `java.util.EnumMap.get(Object) line: 248` |
| 25 | 0.71% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 26 | 0.69% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 27 | 0.62% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 28 | 0.46% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 29 | 0.46% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 30 | 0.46% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 31 | 0.46% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 11039` |
| 32 | 0.44% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 33 | 0.43% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10482` |
| 34 | 0.43% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 35 | 0.42% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 36 | 0.42% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4568` |
| 37 | 0.41% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 38 | 0.39% | `dev.planeta9091.centities.CeHooks.ordSortVanilla(ArrayList) line: 2448` |
| 39 | 0.39% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7403` |
| 40 | 0.36% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 41 | 0.36% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 42 | 0.35% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 43 | 0.35% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 44 | 0.33% | `dev.planeta9091.centities.CeHooks.equipUpdates$(LivingEntity) line: 10935` |
| 45 | 0.33% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6191` |
| 46 | 0.33% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4933` |
| 47 | 0.32% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6896` |
| 48 | 0.32% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 49 | 0.32% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 50 | 0.30% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10510` |
| 51 | 0.30% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 52 | 0.28% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 53 | 0.27% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 54 | 0.27% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 55 | 0.26% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 7056` |
| 56 | 0.26% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7481` |
| 57 | 0.26% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 58 | 0.26% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 59 | 0.26% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 60 | 0.26% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2559` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 13.95% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2520` |
| 2 | 7.95% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4788` |
| 3 | 5.15% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 4.28% | `dev.planeta9091.centities.CeHooks.ordKey(Entity, int) line: 2304` |
| 5 | 4.09% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10674` |
| 6 | 3.20% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 7 | 2.23% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 8 | 2.01% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 9 | 1.70% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2800` |
| 10 | 1.55% | `dev.planeta9091.centities.CeHooks.ordSortVanilla(ArrayList) line: 2451` |
| 11 | 1.48% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 12 | 1.44% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 13 | 1.27% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2741` |
| 14 | 1.09% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 15 | 1.04% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 16 | 1.00% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 17 | 1.00% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 18 | 0.95% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 19 | 0.94% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2817` |
| 20 | 0.82% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2559` |
| 21 | 0.80% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6055` |
| 22 | 0.71% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 23 | 0.71% | `net.minecraft.world.entity.LivingEntity.tick() line: 3491` |
| 24 | 0.69% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 7902` |
| 25 | 0.61% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 26 | 0.60% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7496` |
| 27 | 0.60% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 28 | 0.56% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 29 | 0.53% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6893` |
| 30 | 0.52% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |

GC pauses: 0, total 0 ms

