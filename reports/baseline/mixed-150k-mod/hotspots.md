# JVM hotspots (JFR ExecutionSample)

Samples: 35089

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.50% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.22% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7783` |
| 3 | 2.65% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 2.15% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 1.82% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4751` |
| 6 | 1.76% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7836` |
| 7 | 1.56% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4896` |
| 8 | 1.40% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 9 | 1.37% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.36% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 11 | 1.32% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 1.25% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 13 | 1.18% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 14 | 1.15% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 15 | 1.03% | `java.util.EnumMap$EnumMapIterator.hasNext() line: 524` |
| 16 | 1.01% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 17 | 0.95% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 18 | 0.91% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4178` |
| 19 | 0.89% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 20 | 0.84% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 21 | 0.80% | `java.util.EnumMap.get(Object) line: 248` |
| 22 | 0.77% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 23 | 0.76% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 24 | 0.73% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 25 | 0.68% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 26 | 0.67% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 27 | 0.64% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 28 | 0.64% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 29 | 0.63% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4898` |
| 30 | 0.62% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 31 | 0.62% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 32 | 0.60% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 33 | 0.59% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 34 | 0.58% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 35 | 0.58% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 36 | 0.56% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 37 | 0.55% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 38 | 0.54% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 39 | 0.52% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 40 | 0.52% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 41 | 0.52% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 42 | 0.51% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 43 | 0.50% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 44 | 0.49% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 45 | 0.49% | `net.minecraft.world.entity.Mob.serverAiStep() line: 860` |
| 46 | 0.48% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7922` |
| 47 | 0.48% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 48 | 0.47% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 49 | 0.43% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 50 | 0.42% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 51 | 0.41% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 52 | 0.41% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 53 | 0.40% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 54 | 0.39% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 55 | 0.38% | `java.util.Optional.ifPresent(Consumer) line: 178` |
| 56 | 0.38% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 57 | 0.38% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 58 | 0.37% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 59 | 0.36% | `net.minecraft.world.entity.Entity.baseTick() line: 914` |
| 60 | 0.35% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.87% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.47% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7832` |
| 3 | 2.65% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 2.43% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 1.96% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 6 | 1.85% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 7 | 1.64% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4215` |
| 8 | 1.40% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 9 | 1.39% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7849` |
| 10 | 1.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 11 | 1.30% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 12 | 1.29% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 13 | 1.24% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 14 | 1.15% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 15 | 1.11% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 16 | 1.05% | `net.minecraft.world.entity.EntityEquipment.tick(Entity) line: 50` |
| 17 | 1.02% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 18 | 1.01% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 19 | 0.95% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 20 | 0.89% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4751` |
| 21 | 0.88% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 22 | 0.83% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 23 | 0.80% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 24 | 0.78% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 25 | 0.78% | `net.minecraft.world.entity.LivingEntity.detectEquipmentUpdates() line: 3558` |
| 26 | 0.77% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 27 | 0.76% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 28 | 0.76% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1541` |
| 29 | 0.68% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 30 | 0.66% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |

GC pauses: 0, total 0 ms

