# JVM hotspots (JFR ExecutionSample)

Samples: 35454

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.87% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8593` |
| 2 | 3.70% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8734` |
| 3 | 3.32% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 3.09% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 5 | 2.98% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5629` |
| 6 | 2.82% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.31% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4838` |
| 8 | 1.81% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8540` |
| 9 | 1.55% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 10 | 1.48% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.45% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5484` |
| 12 | 1.37% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 13 | 1.22% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 14 | 1.14% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 15 | 1.07% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 16 | 1.05% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 17 | 1.00% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 18 | 0.92% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 19 | 0.86% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 20 | 0.86% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 379` |
| 21 | 0.77% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 22 | 0.74% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 23 | 0.72% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 24 | 0.71% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 25 | 0.69% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8679` |
| 26 | 0.62% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 27 | 0.61% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 28 | 0.60% | `net.minecraft.world.entity.ai.control.LookControl.getYRotD() line: 102` |
| 29 | 0.59% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 30 | 0.58% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8554` |
| 31 | 0.56% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 32 | 0.53% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 33 | 0.52% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 34 | 0.51% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 35 | 0.50% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 36 | 0.47% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 37 | 0.45% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 38 | 0.45% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 39 | 0.41% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 40 | 0.41% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3539` |
| 41 | 0.41% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 42 | 0.39% | `java.util.EnumMap.get(Object) line: 248` |
| 43 | 0.39% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 44 | 0.38% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 45 | 0.38% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 46 | 0.37% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5634` |
| 47 | 0.37% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 48 | 0.37% | `java.util.ArrayList.add(Object, Object[], int) line: 483` |
| 49 | 0.35% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 50 | 0.34% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 51 | 0.34% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 52 | 0.34% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 53 | 0.34% | `net.minecraft.world.entity.SteppedInterpolationTracker.updateTracking(Vec3) line: 40` |
| 54 | 0.33% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 55 | 0.33% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 320` |
| 56 | 0.32% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 57 | 0.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 58 | 0.31% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 59 | 0.31% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 60 | 0.31% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.30% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.01% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 3.70% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 4 | 3.63% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 2.58% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 6 | 2.56% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8589` |
| 7 | 1.94% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 8 | 1.86% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 9 | 1.70% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8606` |
| 10 | 1.52% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8581` |
| 11 | 1.47% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 12 | 1.46% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 13 | 1.31% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 14 | 1.22% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 15 | 1.16% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4939` |
| 16 | 1.14% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 17 | 0.98% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 18 | 0.94% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 19 | 0.92% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 20 | 0.90% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3584` |
| 21 | 0.85% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 22 | 0.79% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 23 | 0.79% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 24 | 0.73% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 25 | 0.72% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 26 | 0.72% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 6523` |
| 27 | 0.71% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 28 | 0.70% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 29 | 0.68% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2160` |
| 30 | 0.67% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |

GC pauses: 0, total 0 ms

