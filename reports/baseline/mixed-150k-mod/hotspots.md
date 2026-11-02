# JVM hotspots (JFR ExecutionSample)

Samples: 29286

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.48% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6262` |
| 2 | 3.36% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 2.91% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 4 | 2.28% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5494` |
| 5 | 2.18% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9532` |
| 6 | 2.13% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 7 | 1.85% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10134` |
| 8 | 1.78% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 9 | 1.77% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 10 | 1.66% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.61% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 12 | 1.51% | `java.lang.ref.Reference.get() line: 361` |
| 13 | 1.46% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 14 | 1.29% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 15 | 1.13% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 16 | 1.11% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 17 | 1.10% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 18 | 0.84% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 19 | 0.81% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 20 | 0.78% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 21 | 0.74% | `java.util.EnumMap.get(Object) line: 248` |
| 22 | 0.72% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 10046` |
| 23 | 0.72% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 24 | 0.70% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 25 | 0.65% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 26 | 0.62% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 27 | 0.61% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 28 | 0.60% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 29 | 0.60% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 30 | 0.59% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 31 | 0.58% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 32 | 0.57% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 33 | 0.53% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9503` |
| 34 | 0.51% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 35 | 0.47% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 36 | 0.46% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 37 | 0.45% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 38 | 0.45% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6275` |
| 39 | 0.45% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 40 | 0.45% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 41 | 0.44% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9506` |
| 42 | 0.44% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 43 | 0.43% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 44 | 0.41% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 45 | 0.40% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 6442` |
| 46 | 0.39% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4056` |
| 47 | 0.39% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 48 | 0.37% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 49 | 0.37% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6764` |
| 50 | 0.36% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 51 | 0.36% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 52 | 0.34% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 53 | 0.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 54 | 0.34% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 55 | 0.31% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 56 | 0.30% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 57 | 0.29% | `java.lang.ref.Reference.refersToImpl(Object) line: 391` |
| 58 | 0.29% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 59 | 0.29% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6832` |
| 60 | 0.29% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.02% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.28% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 4.13% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9680` |
| 4 | 2.93% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 5 | 2.57% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9679` |
| 6 | 2.14% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 7 | 1.87% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 8 | 1.76% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 9 | 1.75% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 10 | 1.54% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 11 | 1.49% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 12 | 1.27% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 13 | 1.25% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 14 | 1.21% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 15 | 1.13% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 16 | 1.11% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 17 | 0.96% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 18 | 0.95% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 19 | 0.89% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 20 | 0.87% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 21 | 0.86% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6847` |
| 22 | 0.82% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 23 | 0.79% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 24 | 0.76% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6272` |
| 25 | 0.75% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6832` |
| 26 | 0.74% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 27 | 0.73% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 6453` |
| 28 | 0.67% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 29 | 0.66% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |
| 30 | 0.64% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |

GC pauses: 0, total 0 ms

