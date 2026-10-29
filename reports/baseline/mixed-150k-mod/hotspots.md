# JVM hotspots (JFR ExecutionSample)

Samples: 35333

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.83% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.27% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8015` |
| 3 | 3.02% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 2.04% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 1.98% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4959` |
| 6 | 1.92% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8209` |
| 7 | 1.81% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 8 | 1.77% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8068` |
| 9 | 1.75% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5104` |
| 10 | 1.41% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.30% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 12 | 1.27% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 13 | 1.27% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 14 | 1.26% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 15 | 1.20% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2194` |
| 16 | 1.16% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 17 | 1.12% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 18 | 1.10% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 19 | 1.07% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 20 | 0.97% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4386` |
| 21 | 0.96% | `java.util.EnumMap.get(Object) line: 248` |
| 22 | 0.84% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 23 | 0.82% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 24 | 0.80% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 25 | 0.80% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 26 | 0.76% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 27 | 0.74% | `java.util.Optional.ifPresent(Consumer) line: 178` |
| 28 | 0.73% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 29 | 0.69% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 30 | 0.67% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 31 | 0.65% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5106` |
| 32 | 0.63% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 33 | 0.59% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 34 | 0.59% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 35 | 0.55% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 36 | 0.54% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 37 | 0.54% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 38 | 0.53% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 39 | 0.53% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 40 | 0.52% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2213` |
| 41 | 0.51% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 42 | 0.50% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 43 | 0.48% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8154` |
| 44 | 0.48% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 45 | 0.47% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 46 | 0.47% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 47 | 0.47% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 48 | 0.46% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 49 | 0.46% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 50 | 0.46% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 51 | 0.45% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 52 | 0.44% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 53 | 0.44% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 54 | 0.41% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 55 | 0.41% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 56 | 0.40% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 57 | 0.40% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 58 | 0.39% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3081` |
| 59 | 0.37% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 60 | 0.36% | `net.minecraft.world.entity.Entity.getId() line: 651` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.25% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.50% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8064` |
| 3 | 3.02% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 2.60% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 2.35% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1713` |
| 6 | 2.04% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 7 | 1.92% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 8 | 1.91% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 9 | 1.81% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 10 | 1.43% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 11 | 1.41% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4423` |
| 12 | 1.37% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 13 | 1.28% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 14 | 1.14% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 15 | 1.13% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 16 | 1.12% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 17 | 1.10% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 18 | 1.10% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 19 | 0.90% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 20 | 0.89% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 21 | 0.87% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 22 | 0.86% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 23 | 0.85% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 24 | 0.84% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4959` |
| 25 | 0.82% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 26 | 0.80% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 27 | 0.75% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 28 | 0.74% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 29 | 0.68% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 30 | 0.68% | `net.minecraft.world.entity.LivingEntity.detectEquipmentUpdates() line: 3558` |

GC pauses: 0, total 0 ms

