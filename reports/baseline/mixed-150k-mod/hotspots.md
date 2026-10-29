# JVM hotspots (JFR ExecutionSample)

Samples: 36756

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.74% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.29% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 3.21% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7826` |
| 4 | 3.15% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7967` |
| 5 | 3.12% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4886` |
| 6 | 2.73% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.02% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 8 | 2.00% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.89% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4741` |
| 10 | 1.86% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7773` |
| 11 | 1.56% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 1.51% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4168` |
| 13 | 1.37% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 14 | 1.35% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 15 | 1.10% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 16 | 1.07% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 17 | 1.06% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 18 | 1.06% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 19 | 1.04% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 20 | 1.02% | `java.util.EnumMap.get(Object) line: 248` |
| 21 | 0.96% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 22 | 0.95% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 23 | 0.92% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 24 | 0.89% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 25 | 0.77% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 26 | 0.76% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 320` |
| 27 | 0.75% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 28 | 0.72% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 29 | 0.70% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 30 | 0.60% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 31 | 0.60% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 32 | 0.58% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 33 | 0.53% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 34 | 0.53% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 35 | 0.48% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 36 | 0.47% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 37 | 0.47% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 38 | 0.46% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 39 | 0.46% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 40 | 0.45% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 41 | 0.45% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 42 | 0.45% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7787` |
| 43 | 0.44% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 44 | 0.43% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 45 | 0.42% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2863` |
| 46 | 0.42% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 47 | 0.42% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 48 | 0.40% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 49 | 0.40% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 50 | 0.37% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7912` |
| 51 | 0.37% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 52 | 0.36% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3209` |
| 53 | 0.35% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 54 | 0.34% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 55 | 0.34% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 56 | 0.33% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 57 | 0.32% | `java.util.ArrayList.add(Object, Object[], int) line: 483` |
| 58 | 0.32% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 59 | 0.31% | `java.lang.ref.Reference.get() line: 361` |
| 60 | 0.29% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.74% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.46% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 3.29% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 3.27% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 3.15% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 6 | 2.52% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7822` |
| 7 | 2.02% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 8 | 1.93% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 9 | 1.92% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 10 | 1.64% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 11 | 1.55% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 12 | 1.40% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 13 | 1.36% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 14 | 1.25% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 15 | 1.19% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7814` |
| 16 | 1.13% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4205` |
| 17 | 1.07% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 18 | 1.06% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 19 | 0.98% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 20 | 0.96% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 21 | 0.89% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 22 | 0.84% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 23 | 0.81% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 24 | 0.81% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 25 | 0.77% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 26 | 0.72% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 27 | 0.68% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 28 | 0.67% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 29 | 0.67% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 30 | 0.62% | `net.minecraft.world.entity.LivingEntity.detectEquipmentUpdates() line: 3558` |

GC pauses: 0, total 0 ms

