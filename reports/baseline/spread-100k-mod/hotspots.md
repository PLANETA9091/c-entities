# JVM hotspots (JFR ExecutionSample)

Samples: 36448

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.22% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9129` |
| 2 | 3.26% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 2.99% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8935` |
| 4 | 2.81% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 5 | 2.45% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.38% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6011` |
| 7 | 2.30% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8988` |
| 8 | 2.03% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5857` |
| 9 | 1.79% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 10 | 1.73% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 11 | 1.66% | `net.minecraft.world.entity.ai.control.LookControl.getYRotD() line: 102` |
| 12 | 1.55% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 13 | 1.23% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 14 | 1.22% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 15 | 1.17% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 16 | 1.15% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 17 | 1.13% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 18 | 1.11% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 19 | 1.08% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 20 | 1.03% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 21 | 1.03% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5275` |
| 22 | 1.03% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 23 | 0.92% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 24 | 0.88% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 25 | 0.85% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 26 | 0.75% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 27 | 0.75% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2697` |
| 28 | 0.75% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 29 | 0.72% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 30 | 0.69% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 31 | 0.67% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 32 | 0.64% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 33 | 0.63% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 34 | 0.62% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 35 | 0.61% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 36 | 0.59% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 37 | 0.58% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 38 | 0.57% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 39 | 0.56% | `java.util.Optional.ifPresent(Consumer) line: 178` |
| 40 | 0.53% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 41 | 0.52% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 42 | 0.52% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 43 | 0.51% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 44 | 0.51% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 9074` |
| 45 | 0.47% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 46 | 0.46% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 47 | 0.46% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 48 | 0.45% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 49 | 0.44% | `java.util.EnumMap.get(Object) line: 248` |
| 50 | 0.44% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 51 | 0.44% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2738` |
| 52 | 0.43% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 53 | 0.43% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4214` |
| 54 | 0.43% | `java.lang.ref.Reference.get() line: 361` |
| 55 | 0.42% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 56 | 0.41% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3868` |
| 57 | 0.41% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 58 | 0.38% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 59 | 0.38% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 60 | 0.38% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.41% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.23% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 3 | 3.19% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8984` |
| 4 | 2.83% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 5 | 2.74% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 6 | 2.47% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 7 | 2.06% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 8 | 1.90% | `net.minecraft.world.entity.ai.control.LookControl.tick() line: 54` |
| 9 | 1.82% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2079` |
| 10 | 1.50% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9001` |
| 11 | 1.41% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 12 | 1.35% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 13 | 1.32% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5312` |
| 14 | 1.27% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 15 | 1.23% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 16 | 1.23% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 17 | 1.14% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 18 | 1.13% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 19 | 1.08% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 20 | 1.05% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 21 | 1.05% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 22 | 1.03% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 23 | 0.97% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 125` |
| 24 | 0.92% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5857` |
| 25 | 0.88% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 26 | 0.86% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 27 | 0.85% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 28 | 0.81% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 29 | 0.80% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 30 | 0.80% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |

GC pauses: 0, total 0 ms

