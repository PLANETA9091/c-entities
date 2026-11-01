# JVM hotspots (JFR ExecutionSample)

Samples: 35638

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.63% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5943` |
| 2 | 3.57% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8920` |
| 3 | 3.50% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 3.27% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9061` |
| 5 | 3.17% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 3.10% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 7 | 3.02% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 1.93% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5789` |
| 9 | 1.89% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8867` |
| 10 | 1.64% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 11 | 1.60% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5207` |
| 12 | 1.59% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 1.45% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 14 | 1.27% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 15 | 1.11% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 16 | 1.00% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 17 | 0.97% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 18 | 0.95% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 19 | 0.94% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 20 | 0.85% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 21 | 0.81% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 22 | 0.79% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 23 | 0.78% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 24 | 0.77% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 25 | 0.76% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 26 | 0.74% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 27 | 0.71% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 28 | 0.70% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 29 | 0.68% | `java.util.EnumMap.get(Object) line: 248` |
| 30 | 0.59% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 9006` |
| 31 | 0.59% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 32 | 0.59% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 33 | 0.55% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 34 | 0.54% | `net.minecraft.world.entity.ai.control.LookControl.getYRotD() line: 102` |
| 35 | 0.49% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8881` |
| 36 | 0.48% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 37 | 0.46% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 38 | 0.45% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3800` |
| 39 | 0.45% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 40 | 0.43% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 41 | 0.42% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 42 | 0.40% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 43 | 0.40% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 44 | 0.38% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 45 | 0.38% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 46 | 0.37% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 47 | 0.36% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 48 | 0.35% | `java.util.ArrayList.add(Object, Object[], int) line: 483` |
| 49 | 0.35% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 50 | 0.35% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 51 | 0.35% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 52 | 0.34% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 53 | 0.33% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 54 | 0.33% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 55 | 0.33% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 56 | 0.32% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 57 | 0.32% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 58 | 0.32% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4146` |
| 59 | 0.31% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 60 | 0.31% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.60% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.09% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 3.70% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 3.50% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 3.28% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 6 | 2.61% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8916` |
| 7 | 2.24% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 8 | 2.16% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 9 | 1.95% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 10 | 1.76% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 11 | 1.69% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 12 | 1.50% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8933` |
| 13 | 1.45% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 14 | 1.38% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8908` |
| 15 | 1.38% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5244` |
| 16 | 1.19% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 17 | 1.15% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 18 | 0.99% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 19 | 0.84% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 20 | 0.83% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 21 | 0.82% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 22 | 0.80% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 23 | 0.79% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 24 | 0.79% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3584` |
| 25 | 0.78% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 26 | 0.77% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 27 | 0.76% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 28 | 0.72% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 29 | 0.70% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5789` |
| 30 | 0.69% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5953` |

GC pauses: 0, total 0 ms

