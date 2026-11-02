# JVM hotspots (JFR ExecutionSample)

Samples: 32602

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.20% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 4.85% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.83% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6203` |
| 4 | 3.04% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 2.91% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9819` |
| 6 | 2.21% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 1.99% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5435` |
| 8 | 1.92% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9344` |
| 9 | 1.61% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 10 | 1.55% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 11 | 1.48% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 1.30% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 13 | 1.27% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 14 | 1.17% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 15 | 1.09% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 16 | 1.06% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 17 | 1.01% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 18 | 0.98% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 19 | 0.95% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 20 | 0.93% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 21 | 0.82% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 22 | 0.68% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 23 | 0.67% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 24 | 0.65% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 25 | 0.57% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 26 | 0.57% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 27 | 0.54% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9335` |
| 28 | 0.52% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 29 | 0.51% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 9764` |
| 30 | 0.50% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 31 | 0.48% | `java.util.EnumMap.get(Object) line: 248` |
| 32 | 0.48% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4028` |
| 33 | 0.48% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 34 | 0.46% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 35 | 0.44% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 36 | 0.44% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 37 | 0.42% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 38 | 0.42% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 39 | 0.42% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 40 | 0.42% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 41 | 0.40% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 42 | 0.40% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 43 | 0.39% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3901` |
| 44 | 0.39% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 45 | 0.38% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 46 | 0.37% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 47 | 0.37% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 48 | 0.36% | `java.lang.ref.Reference.get() line: 361` |
| 49 | 0.36% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 50 | 0.34% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 51 | 0.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 52 | 0.33% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 53 | 0.33% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4374` |
| 54 | 0.33% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 55 | 0.33% | `java.lang.ref.Reference.refersToImpl(Object) line: 391` |
| 56 | 0.32% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 57 | 0.32% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 58 | 0.32% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 59 | 0.32% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9330` |
| 60 | 0.30% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.06% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.30% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 3.51% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9437` |
| 4 | 3.04% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 2.92% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 6 | 2.80% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9334` |
| 7 | 2.64% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 8 | 2.13% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 9 | 1.71% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 10 | 1.60% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 11 | 1.44% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 12 | 1.42% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 13 | 1.35% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9427` |
| 14 | 1.16% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 7122` |
| 15 | 1.13% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 16 | 1.09% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 17 | 1.00% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 18 | 0.95% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 19 | 0.95% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 20 | 0.93% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 21 | 0.90% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 22 | 0.83% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 23 | 0.81% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 24 | 0.76% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6213` |
| 25 | 0.75% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 26 | 0.67% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |
| 27 | 0.65% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 28 | 0.65% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2195` |
| 29 | 0.64% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 30 | 0.62% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |

GC pauses: 0, total 0 ms

