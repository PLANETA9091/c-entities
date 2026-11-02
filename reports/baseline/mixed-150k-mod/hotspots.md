# JVM hotspots (JFR ExecutionSample)

Samples: 26485

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.81% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 4.67% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 4.09% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6328` |
| 4 | 2.41% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10216` |
| 5 | 2.24% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 2.21% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 7 | 2.09% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 8 | 1.96% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 9 | 1.94% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 10 | 1.92% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5546` |
| 11 | 1.88% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9614` |
| 12 | 1.57% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 13 | 1.31% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 14 | 1.31% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 15 | 1.27% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 16 | 1.17% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 17 | 1.11% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 18 | 1.07% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 19 | 0.87% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 20 | 0.82% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 21 | 0.77% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 22 | 0.74% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 23 | 0.70% | `java.util.EnumMap.get(Object) line: 248` |
| 24 | 0.69% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 25 | 0.69% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 26 | 0.68% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 27 | 0.63% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 28 | 0.62% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 29 | 0.61% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4077` |
| 30 | 0.57% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6835` |
| 31 | 0.54% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 32 | 0.51% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 33 | 0.51% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 10128` |
| 34 | 0.50% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 35 | 0.48% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 36 | 0.45% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 37 | 0.44% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6903` |
| 38 | 0.42% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 39 | 0.41% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 40 | 0.41% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 41 | 0.41% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 6501` |
| 42 | 0.40% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4444` |
| 43 | 0.40% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9585` |
| 44 | 0.39% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 45 | 0.39% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 46 | 0.38% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 47 | 0.38% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9588` |
| 48 | 0.36% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 49 | 0.36% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 50 | 0.35% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 51 | 0.35% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 52 | 0.35% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 53 | 0.34% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 54 | 0.34% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 55 | 0.33% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 56 | 0.32% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 57 | 0.32% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7445` |
| 58 | 0.32% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 59 | 0.31% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 60 | 0.30% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.05% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.67% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 4.56% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 3.58% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9762` |
| 5 | 2.81% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 6 | 2.43% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 7 | 2.42% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 8 | 2.17% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 9 | 2.08% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4298` |
| 10 | 1.67% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 11 | 1.57% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 12 | 1.57% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 13 | 1.54% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 14 | 1.35% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 15 | 1.26% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 7321` |
| 16 | 1.19% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 17 | 1.18% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 18 | 1.16% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6918` |
| 19 | 1.12% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 20 | 1.03% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 21 | 0.82% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 22 | 0.82% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 23 | 0.79% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 24 | 0.78% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 25 | 0.74% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6338` |
| 26 | 0.73% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 27 | 0.69% | `net.minecraft.world.entity.Entity.isEyeInFluid(TagKey) line: 2311` |
| 28 | 0.68% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 848` |
| 29 | 0.66% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3983` |
| 30 | 0.65% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6903` |

GC pauses: 0, total 0 ms

