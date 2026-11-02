# JVM hotspots (JFR ExecutionSample)

Samples: 25833

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.22% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.90% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9949` |
| 3 | 3.12% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 4 | 2.61% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6241` |
| 5 | 2.46% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 6 | 2.46% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 2.45% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 8 | 2.37% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 9 | 2.30% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 10 | 2.22% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 11 | 1.65% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 12 | 1.63% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 13 | 1.45% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 14 | 1.36% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 15 | 1.30% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 16 | 1.29% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 17 | 1.22% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 18 | 1.19% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 19 | 1.19% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9433` |
| 20 | 0.94% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5473` |
| 21 | 0.90% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 22 | 0.84% | `java.util.EnumMap.get(Object) line: 248` |
| 23 | 0.82% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 24 | 0.79% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 25 | 0.76% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 26 | 0.69% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 27 | 0.68% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 28 | 0.67% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 29 | 0.66% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 30 | 0.63% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 31 | 0.62% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4054` |
| 32 | 0.56% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 33 | 0.55% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7301` |
| 34 | 0.54% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 35 | 0.54% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4400` |
| 36 | 0.51% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 37 | 0.51% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 38 | 0.50% | `dev.planeta9091.centities.CeHooks.equipUpdates$(LivingEntity) line: 9757` |
| 39 | 0.50% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 40 | 0.48% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 41 | 0.48% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 9861` |
| 42 | 0.48% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 43 | 0.47% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 44 | 0.46% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 45 | 0.45% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 46 | 0.45% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 47 | 0.45% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 48 | 0.45% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 49 | 0.45% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 50 | 0.43% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 51 | 0.41% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4337` |
| 52 | 0.41% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 53 | 0.40% | `java.lang.ref.Reference.get() line: 361` |
| 54 | 0.38% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2722` |
| 55 | 0.37% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 56 | 0.37% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 57 | 0.36% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 58 | 0.35% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9451` |
| 59 | 0.34% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 60 | 0.34% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6254` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.68% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.91% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 3 | 3.38% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 2.72% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 5 | 2.68% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 6 | 2.22% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 7 | 2.14% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 8 | 2.04% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 9 | 1.94% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9521` |
| 10 | 1.83% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 11 | 1.61% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9452` |
| 12 | 1.61% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 13 | 1.57% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 14 | 1.54% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 15 | 1.31% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 16 | 1.22% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 17 | 1.17% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 18 | 1.05% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 19 | 1.04% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 20 | 0.90% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 21 | 0.90% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2104` |
| 22 | 0.89% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 23 | 0.85% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 24 | 0.85% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 25 | 0.82% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 26 | 0.82% | `net.minecraft.world.entity.Mob.serverAiStep() line: 843` |
| 27 | 0.78% | `net.minecraft.world.entity.Mob.serverAiStep() line: 836` |
| 28 | 0.76% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 29 | 0.74% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 898` |
| 30 | 0.73% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |

GC pauses: 0, total 0 ms

