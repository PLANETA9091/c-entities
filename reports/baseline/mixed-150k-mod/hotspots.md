# JVM hotspots (JFR ExecutionSample)

Samples: 31311

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.98% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.71% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5987` |
| 3 | 3.71% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 3.57% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8964` |
| 5 | 3.19% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 3.09% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9105` |
| 7 | 2.89% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 8 | 2.20% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8911` |
| 9 | 2.05% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 2.03% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5833` |
| 11 | 1.64% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 12 | 1.55% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5251` |
| 13 | 1.40% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 14 | 1.36% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 15 | 1.16% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 16 | 1.13% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 17 | 1.09% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 18 | 1.07% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 19 | 1.01% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 20 | 0.97% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 21 | 0.93% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 22 | 0.91% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 23 | 0.87% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 24 | 0.84% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 25 | 0.83% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 26 | 0.70% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 27 | 0.67% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 28 | 0.67% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 29 | 0.62% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 30 | 0.60% | `net.minecraft.world.entity.ai.control.LookControl.getYRotD() line: 102` |
| 31 | 0.56% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 32 | 0.52% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 9050` |
| 33 | 0.50% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 34 | 0.49% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 35 | 0.48% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 36 | 0.48% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 37 | 0.47% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8925` |
| 38 | 0.45% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 39 | 0.44% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 40 | 0.43% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3844` |
| 41 | 0.41% | `java.util.EnumMap.get(Object) line: 248` |
| 42 | 0.40% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 43 | 0.40% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6000` |
| 44 | 0.40% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 45 | 0.39% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 46 | 0.39% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 47 | 0.39% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 48 | 0.38% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 49 | 0.38% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 50 | 0.37% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 51 | 0.37% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4190` |
| 52 | 0.36% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 53 | 0.36% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 54 | 0.36% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 55 | 0.35% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 56 | 0.35% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 57 | 0.34% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 58 | 0.32% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 59 | 0.32% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 60 | 0.32% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getAttributesToSync() line: 34` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.03% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.46% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 3.71% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 3.63% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 3.10% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 6 | 2.89% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8960` |
| 7 | 2.09% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 8 | 2.04% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 9 | 2.01% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 10 | 1.85% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 11 | 1.76% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 12 | 1.64% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 13 | 1.42% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8977` |
| 14 | 1.36% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 15 | 1.34% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 16 | 1.29% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 17 | 1.22% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8952` |
| 18 | 1.14% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5288` |
| 19 | 1.02% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 20 | 0.99% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3584` |
| 21 | 0.97% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 22 | 0.92% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 23 | 0.91% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 24 | 0.83% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 25 | 0.82% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 6894` |
| 26 | 0.79% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 27 | 0.71% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 28 | 0.70% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 29 | 0.69% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 30 | 0.65% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5997` |

GC pauses: 0, total 0 ms

