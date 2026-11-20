# JVM hotspots (JFR ExecutionSample)

Samples: 26116

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 36.82% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3838` |
| 2 | 4.96% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3910` |
| 3 | 1.73% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3820` |
| 4 | 1.65% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3852` |
| 5 | 1.61% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 1.37% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3837` |
| 7 | 1.26% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 8 | 1.06% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 9 | 0.96% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 10 | 0.92% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 11 | 0.86% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 12 | 0.76% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 13 | 0.65% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 62` |
| 14 | 0.63% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 10604` |
| 15 | 0.62% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 16 | 0.59% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 17 | 0.57% | `java.util.HashMap.getNode(Object) line: 586` |
| 18 | 0.54% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 19 | 0.52% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 10682` |
| 20 | 0.49% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 21 | 0.49% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 22 | 0.48% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 6849` |
| 23 | 0.46% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 24 | 0.43% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3836` |
| 25 | 0.43% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 9529` |
| 26 | 0.43% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3707` |
| 27 | 0.42% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 10257` |
| 28 | 0.41% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 11227` |
| 29 | 0.40% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 30 | 0.39% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 14153` |
| 31 | 0.38% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 6786` |
| 32 | 0.38% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 10435` |
| 33 | 0.37% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 34 | 0.34% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 35 | 0.34% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 36 | 0.32% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 37 | 0.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 38 | 0.31% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 39 | 0.30% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 40 | 0.29% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 41 | 0.29% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 42 | 0.28% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3596` |
| 43 | 0.27% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 44 | 0.27% | `java.util.EnumMap.get(Object) line: 248` |
| 45 | 0.26% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean, Class) line: 3828` |
| 46 | 0.25% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 47 | 0.23% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getAttributesToSync() line: 34` |
| 48 | 0.23% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 9977` |
| 49 | 0.22% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 7095` |
| 50 | 0.21% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 51 | 0.21% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 52 | 0.20% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 53 | 0.19% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 54 | 0.19% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 15973` |
| 55 | 0.19% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 56 | 0.18% | `java.util.ImmutableCollections.listCopy(Collection) line: 191` |
| 57 | 0.18% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$hasPlayers() line: 1347` |
| 58 | 0.18% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 59 | 0.18% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 60 | 0.17% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 47.67% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1967` |
| 2 | 1.46% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 3 | 1.39% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 10697` |
| 4 | 1.29% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 1.28% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 6 | 1.19% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 7 | 1.08% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 8 | 0.95% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 9 | 0.92% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 10 | 0.91% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 11 | 0.90% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3739` |
| 12 | 0.76% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 14153` |
| 13 | 0.69% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 10682` |
| 14 | 0.59% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 15 | 0.57% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 10603` |
| 16 | 0.52% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 17 | 0.51% | `dev.planeta9091.centities.CeHooks.goalSelectorTickRunning(GoalSelector, boolean) line: 10773` |
| 18 | 0.49% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 19 | 0.49% | `net.minecraft.world.entity.ai.sensing.Sensing.tick()` |
| 20 | 0.47% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 7096` |
| 21 | 0.46% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 22 | 0.44% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 23 | 0.43% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 216` |
| 24 | 0.40% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk)` |
| 25 | 0.38% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 26 | 0.38% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 27 | 0.37% | `net.minecraft.world.phys.Vec3.subtract(double, double, double) line: 111` |
| 28 | 0.35% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 29 | 0.34% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThreadFor(Entity) line: 170` |
| 30 | 0.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |

GC pauses: 0, total 0 ms

