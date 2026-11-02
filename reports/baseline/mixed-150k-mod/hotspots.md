# JVM hotspots (JFR ExecutionSample)

Samples: 28977

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.26% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 4.00% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 3.73% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6273` |
| 4 | 2.92% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10150` |
| 5 | 2.00% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 6 | 1.95% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 1.92% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5505` |
| 8 | 1.74% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 9 | 1.73% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 10 | 1.71% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 11 | 1.69% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 12 | 1.61% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 13 | 1.47% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9548` |
| 14 | 1.15% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 15 | 1.13% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 16 | 1.08% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 17 | 0.98% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 18 | 0.92% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 19 | 0.86% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 20 | 0.85% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 21 | 0.81% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 22 | 0.79% | `java.util.EnumMap.get(Object) line: 248` |
| 23 | 0.76% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 24 | 0.70% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 25 | 0.70% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 26 | 0.66% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 10062` |
| 27 | 0.64% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 28 | 0.62% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 29 | 0.62% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4071` |
| 30 | 0.61% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 31 | 0.60% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 32 | 0.58% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 33 | 0.56% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6780` |
| 34 | 0.49% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 35 | 0.46% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6848` |
| 36 | 0.45% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 37 | 0.44% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 320` |
| 38 | 0.43% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 39 | 0.42% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 40 | 0.42% | `dev.planeta9091.centities.CeHooks.equipUpdates$(LivingEntity) line: 9958` |
| 41 | 0.40% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 42 | 0.40% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9519` |
| 43 | 0.39% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 44 | 0.39% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9522` |
| 45 | 0.37% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 46 | 0.36% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 47 | 0.36% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 48 | 0.35% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 49 | 0.34% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 6446` |
| 50 | 0.32% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 51 | 0.32% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7390` |
| 52 | 0.32% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 53 | 0.31% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 54 | 0.31% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 55 | 0.31% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 56 | 0.31% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9566` |
| 57 | 0.31% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4417` |
| 58 | 0.30% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 501` |
| 59 | 0.30% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 60 | 0.29% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.21% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.18% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 4.00% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 3.38% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9696` |
| 5 | 2.93% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 6 | 2.57% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 7 | 2.44% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 8 | 1.89% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 9 | 1.74% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 10 | 1.57% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 11 | 1.46% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 12 | 1.41% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9695` |
| 13 | 1.38% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 14 | 1.34% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 15 | 1.17% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 16 | 1.15% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 17 | 1.05% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 18 | 1.04% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6863` |
| 19 | 0.98% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 20 | 0.87% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 21 | 0.86% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 7266` |
| 22 | 0.85% | `net.minecraft.world.entity.Entity.isEyeInFluid(TagKey) line: 2311` |
| 23 | 0.85% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 24 | 0.81% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 25 | 0.79% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6283` |
| 26 | 0.76% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 27 | 0.74% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 28 | 0.67% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3977` |
| 29 | 0.64% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6848` |
| 30 | 0.63% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |

GC pauses: 0, total 0 ms

