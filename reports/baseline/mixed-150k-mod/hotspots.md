# JVM hotspots (JFR ExecutionSample)

Samples: 28644

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.20% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6222` |
| 2 | 3.93% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.26% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 4 | 3.06% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10099` |
| 5 | 2.86% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 2.30% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 7 | 2.15% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9497` |
| 8 | 2.03% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 2.00% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5454` |
| 10 | 1.84% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 11 | 1.68% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 12 | 1.64% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 13 | 1.35% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 14 | 1.20% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 15 | 1.12% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 16 | 1.10% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 17 | 0.97% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 18 | 0.92% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 19 | 0.86% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 20 | 0.84% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 21 | 0.83% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 22 | 0.75% | `java.util.EnumMap.get(Object) line: 248` |
| 23 | 0.68% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 24 | 0.65% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 25 | 0.65% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 26 | 0.63% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 27 | 0.62% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6729` |
| 28 | 0.61% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 10011` |
| 29 | 0.58% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 30 | 0.53% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 31 | 0.50% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 32 | 0.49% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 33 | 0.48% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6797` |
| 34 | 0.48% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9471` |
| 35 | 0.48% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 36 | 0.45% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 37 | 0.44% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4070` |
| 38 | 0.43% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 39 | 0.43% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 40 | 0.42% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 41 | 0.42% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 6395` |
| 42 | 0.42% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4416` |
| 43 | 0.42% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 44 | 0.42% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 45 | 0.40% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 46 | 0.40% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9468` |
| 47 | 0.39% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 48 | 0.38% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 49 | 0.37% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 50 | 0.36% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 51 | 0.34% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 10019` |
| 52 | 0.34% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 53 | 0.31% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 54 | 0.31% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 55 | 0.31% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 56 | 0.30% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 57 | 0.29% | `java.lang.ref.Reference.refersToImpl(Object) line: 391` |
| 58 | 0.29% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9492` |
| 59 | 0.29% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 60 | 0.29% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7339` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.96% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.71% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 4.16% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9645` |
| 4 | 3.08% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 5 | 2.96% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 6 | 2.86% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 2.74% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 8 | 2.40% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 9 | 1.94% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9644` |
| 10 | 1.79% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 11 | 1.74% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 12 | 1.57% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 13 | 1.48% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 14 | 1.45% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 15 | 1.23% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 16 | 1.15% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 7215` |
| 17 | 1.10% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 18 | 1.00% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 19 | 0.98% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 20 | 0.95% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6812` |
| 21 | 0.93% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 22 | 0.85% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 23 | 0.84% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 24 | 0.79% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 25 | 0.75% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6232` |
| 26 | 0.71% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 27 | 0.67% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 28 | 0.66% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6797` |
| 29 | 0.65% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 30 | 0.63% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6728` |

GC pauses: 0, total 0 ms

