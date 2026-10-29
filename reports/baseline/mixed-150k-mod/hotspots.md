# JVM hotspots (JFR ExecutionSample)

Samples: 42915

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.80% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.47% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8183` |
| 3 | 3.41% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 3.34% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7989` |
| 5 | 2.50% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.33% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5078` |
| 7 | 2.13% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8042` |
| 8 | 2.04% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 9 | 2.04% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 10 | 1.98% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4933` |
| 11 | 1.81% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.64% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 13 | 1.49% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 14 | 1.23% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4360` |
| 15 | 1.15% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 16 | 1.06% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 17 | 1.05% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 18 | 1.01% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 19 | 0.92% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 20 | 0.91% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 21 | 0.88% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 22 | 0.88% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 23 | 0.86% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 24 | 0.84% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 25 | 0.83% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 26 | 0.81% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 27 | 0.76% | `java.util.EnumMap.get(Object) line: 248` |
| 28 | 0.69% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 29 | 0.62% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 30 | 0.61% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 31 | 0.60% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 32 | 0.59% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 33 | 0.59% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 34 | 0.58% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 35 | 0.57% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 36 | 0.57% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 37 | 0.55% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 38 | 0.53% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 39 | 0.52% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7685` |
| 40 | 0.51% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 41 | 0.51% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 42 | 0.47% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8128` |
| 43 | 0.47% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 44 | 0.45% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 45 | 0.44% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 46 | 0.44% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3055` |
| 47 | 0.43% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 48 | 0.42% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 49 | 0.41% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3401` |
| 50 | 0.40% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 51 | 0.39% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 52 | 0.39% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 53 | 0.38% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 54 | 0.38% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 55 | 0.38% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5080` |
| 56 | 0.37% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 57 | 0.36% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5083` |
| 58 | 0.36% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3338` |
| 59 | 0.36% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 60 | 0.35% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.61% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.48% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8038` |
| 3 | 3.47% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 4 | 3.41% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 3.12% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 6 | 2.26% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 7 | 2.04% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 8 | 2.00% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 9 | 1.78% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 10 | 1.67% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 11 | 1.55% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8055` |
| 12 | 1.39% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 13 | 1.37% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 14 | 1.29% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 15 | 1.16% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 16 | 1.14% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 17 | 1.12% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 18 | 1.05% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 19 | 1.03% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 20 | 1.00% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 21 | 0.92% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 22 | 0.92% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 23 | 0.89% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 24 | 0.88% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4933` |
| 25 | 0.86% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4397` |
| 26 | 0.84% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 27 | 0.78% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 28 | 0.77% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 29 | 0.73% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 30 | 0.70% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |

GC pauses: 0, total 0 ms

