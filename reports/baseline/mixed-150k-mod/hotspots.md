# JVM hotspots (JFR ExecutionSample)

Samples: 28052

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.88% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.60% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6305` |
| 3 | 2.13% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 4 | 2.07% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10182` |
| 5 | 1.98% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9580` |
| 6 | 1.96% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5537` |
| 7 | 1.90% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 1.74% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 9 | 1.61% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 10 | 1.49% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 11 | 1.46% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 12 | 1.32% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 13 | 1.23% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 14 | 1.09% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 15 | 0.97% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 16 | 0.89% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 17 | 0.83% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 18 | 0.79% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 10094` |
| 19 | 0.74% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 20 | 0.72% | `java.util.EnumMap.get(Object) line: 248` |
| 21 | 0.70% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 22 | 0.69% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 23 | 0.63% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 24 | 0.63% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 25 | 0.61% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 26 | 0.60% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 27 | 0.59% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 28 | 0.59% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 29 | 0.58% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6812` |
| 30 | 0.57% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 31 | 0.55% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 32 | 0.52% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 33 | 0.50% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 34 | 0.50% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6318` |
| 35 | 0.49% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 36 | 0.46% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 37 | 0.46% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 38 | 0.46% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 39 | 0.43% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 40 | 0.42% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4071` |
| 41 | 0.41% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9551` |
| 42 | 0.41% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 43 | 0.40% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 44 | 0.38% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6880` |
| 45 | 0.37% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 46 | 0.36% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 47 | 0.36% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 48 | 0.36% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap$MapIterator.nextEntry() line: 655` |
| 49 | 0.36% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 50 | 0.34% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 51 | 0.34% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 52 | 0.34% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 53 | 0.34% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9554` |
| 54 | 0.33% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 6656` |
| 55 | 0.33% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 56 | 0.32% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 57 | 0.32% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 6478` |
| 58 | 0.32% | `java.lang.ref.Reference.refersToImpl(Object) line: 391` |
| 59 | 0.32% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 60 | 0.32% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7422` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.78% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.44% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 3.89% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9728` |
| 4 | 2.61% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 5 | 2.42% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 6 | 2.13% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4292` |
| 7 | 2.11% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 8 | 1.79% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 9 | 1.58% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 10 | 1.55% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 11 | 1.44% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 12 | 1.32% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 13 | 1.30% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 14 | 1.03% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 15 | 1.02% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 16 | 0.97% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 17 | 0.92% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 18 | 0.91% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 19 | 0.87% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 20 | 0.85% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6895` |
| 21 | 0.84% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 22 | 0.75% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 7298` |
| 23 | 0.75% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |
| 24 | 0.67% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6315` |
| 25 | 0.67% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 26 | 0.62% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6880` |
| 27 | 0.62% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 28 | 0.60% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 29 | 0.60% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 30 | 0.59% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2195` |

GC pauses: 0, total 0 ms

