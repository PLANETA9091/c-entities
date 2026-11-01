# JVM hotspots (JFR ExecutionSample)

Samples: 30246

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.87% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 2 | 4.05% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8783` |
| 3 | 3.17% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 2.98% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8977` |
| 5 | 2.74% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5872` |
| 6 | 2.63% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 2.52% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 2.22% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8836` |
| 9 | 2.21% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5727` |
| 10 | 1.70% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 11 | 1.41% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 12 | 1.33% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 1.32% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5145` |
| 14 | 1.19% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 15 | 1.15% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 16 | 1.08% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 17 | 1.02% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 18 | 0.99% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 19 | 0.95% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 20 | 0.93% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 21 | 0.91% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 22 | 0.86% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 23 | 0.82% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 24 | 0.73% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 25 | 0.69% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5874` |
| 26 | 0.63% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 27 | 0.61% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 28 | 0.57% | `java.util.EnumMap.get(Object) line: 248` |
| 29 | 0.56% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 30 | 0.50% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 31 | 0.50% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 32 | 0.50% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8922` |
| 33 | 0.49% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 34 | 0.49% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 8479` |
| 35 | 0.49% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 36 | 0.48% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 37 | 0.47% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 38 | 0.47% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 39 | 0.45% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 40 | 0.45% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 41 | 0.45% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 42 | 0.44% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 43 | 0.44% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3770` |
| 44 | 0.43% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 45 | 0.43% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 46 | 0.43% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 47 | 0.42% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 48 | 0.42% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 49 | 0.42% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 50 | 0.42% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 51 | 0.39% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 52 | 0.39% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 53 | 0.38% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getAttributesToSync() line: 34` |
| 54 | 0.38% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 55 | 0.37% | `java.util.EnumMap$EntrySet.iterator() line: 475` |
| 56 | 0.37% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 57 | 0.36% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 58 | 0.35% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 59 | 0.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 60 | 0.34% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3643` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.50% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.34% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8832` |
| 3 | 3.69% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 2.99% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 5 | 2.63% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 6 | 2.40% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 7 | 2.28% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 8 | 2.02% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 9 | 1.96% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 10 | 1.78% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 11 | 1.43% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5182` |
| 12 | 1.34% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 13 | 1.32% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 14 | 1.32% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 15 | 1.19% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5727` |
| 16 | 1.13% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 17 | 1.06% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3584` |
| 18 | 1.00% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 19 | 1.00% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 20 | 0.98% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 21 | 0.94% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 22 | 0.93% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 23 | 0.84% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 24 | 0.82% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 25 | 0.72% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 26 | 0.70% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 27 | 0.70% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 6766` |
| 28 | 0.70% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 29 | 0.68% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 30 | 0.68% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8824` |

GC pauses: 0, total 0 ms

