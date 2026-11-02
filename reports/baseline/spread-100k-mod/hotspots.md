# JVM hotspots (JFR ExecutionSample)

Samples: 28281

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.82% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9898` |
| 2 | 3.72% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.43% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 4 | 2.93% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9395` |
| 5 | 2.88% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 6 | 2.80% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6241` |
| 7 | 2.27% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 2.23% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 9 | 2.15% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 10 | 1.98% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 11 | 1.66% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 12 | 1.58% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 13 | 1.40% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 14 | 1.40% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 15 | 1.22% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 16 | 1.22% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 17 | 1.18% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 18 | 1.18% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 19 | 1.10% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5472` |
| 20 | 1.03% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 21 | 1.00% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 22 | 0.92% | `java.util.concurrent.CompletableFuture.isDone() line: 2076` |
| 23 | 0.91% | `java.util.EnumMap.get(Object) line: 248` |
| 24 | 0.85% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 25 | 0.81% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 26 | 0.80% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 27 | 0.75% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 28 | 0.71% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 29 | 0.69% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 30 | 0.65% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 31 | 0.65% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 32 | 0.63% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 33 | 0.63% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4052` |
| 34 | 0.63% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 35 | 0.62% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 36 | 0.62% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 37 | 0.59% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 38 | 0.59% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 39 | 0.56% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4335` |
| 40 | 0.55% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 41 | 0.53% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 9843` |
| 42 | 0.52% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 43 | 0.52% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 44 | 0.52% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4398` |
| 45 | 0.50% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2720` |
| 46 | 0.49% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 47 | 0.48% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 48 | 0.46% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7284` |
| 49 | 0.46% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 50 | 0.41% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 51 | 0.40% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 52 | 0.39% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 53 | 0.38% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6254` |
| 54 | 0.38% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 55 | 0.38% | `java.lang.ref.Reference.get() line: 361` |
| 56 | 0.38% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 57 | 0.37% | `dev.planeta9091.centities.CeHooks.equipUpdates$(LivingEntity) line: 9739` |
| 58 | 0.37% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 59 | 0.36% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 501` |
| 60 | 0.36% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.99% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.82% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 3 | 3.78% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9504` |
| 4 | 3.62% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 3.43% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 6 | 2.22% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 7 | 2.22% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 8 | 1.97% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 9 | 1.61% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 10 | 1.57% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 11 | 1.51% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 12 | 1.40% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 13 | 1.27% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 14 | 1.27% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 15 | 1.25% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 16 | 1.24% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 17 | 1.22% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 18 | 1.21% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 19 | 1.20% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 20 | 1.20% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 21 | 1.17% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2102` |
| 22 | 0.98% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 23 | 0.92% | `net.minecraft.util.thread.BlockableEventLoop.managedBlock(BooleanSupplier) line: 190` |
| 24 | 0.90% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9494` |
| 25 | 0.86% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 26 | 0.85% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |
| 27 | 0.79% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 28 | 0.78% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 29 | 0.76% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |
| 30 | 0.74% | `net.minecraft.world.entity.Mob.serverAiStep() line: 836` |

GC pauses: 0, total 0 ms

