# JVM hotspots (JFR ExecutionSample)

Samples: 33609

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.03% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8929` |
| 2 | 3.80% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 3 | 2.96% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8735` |
| 4 | 2.94% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 2.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 6 | 2.23% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 2.18% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5824` |
| 8 | 1.97% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 9 | 1.78% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5679` |
| 10 | 1.56% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8788` |
| 11 | 1.43% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 12 | 1.40% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 13 | 1.34% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 14 | 1.30% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 15 | 1.20% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 16 | 1.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5033` |
| 17 | 1.02% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 18 | 1.01% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 19 | 1.00% | `java.util.ArrayDeque.pollFirst() line: 378` |
| 20 | 1.00% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 21 | 0.86% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 22 | 0.76% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 23 | 0.75% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 24 | 0.71% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 25 | 0.69% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2661` |
| 26 | 0.69% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 27 | 0.69% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 28 | 0.68% | `java.util.Optional.ifPresent(Consumer) line: 178` |
| 29 | 0.68% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 30 | 0.65% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 31 | 0.62% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 389` |
| 32 | 0.59% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 33 | 0.58% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 34 | 0.56% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 35 | 0.55% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 36 | 0.55% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 37 | 0.52% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5826` |
| 38 | 0.49% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 39 | 0.49% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 40 | 0.49% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 41 | 0.49% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 42 | 0.49% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 43 | 0.47% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 44 | 0.45% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 45 | 0.45% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 46 | 0.44% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 47 | 0.41% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 48 | 0.41% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 49 | 0.40% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 50 | 0.40% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 51 | 0.40% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 52 | 0.40% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5829` |
| 53 | 0.39% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 54 | 0.39% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 55 | 0.39% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 56 | 0.38% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 57 | 0.37% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 58 | 0.37% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8874` |
| 59 | 0.37% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 60 | 0.36% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 320` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.05% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 2 | 3.94% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 3.23% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 3.22% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8784` |
| 5 | 2.53% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 6 | 2.23% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 1.82% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 8 | 1.74% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 9 | 1.74% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 10 | 1.61% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 11 | 1.51% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5134` |
| 12 | 1.42% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 13 | 1.32% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 14 | 1.20% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 15 | 1.20% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2043` |
| 16 | 1.18% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 17 | 1.15% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 18 | 1.05% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 19 | 1.01% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5679` |
| 20 | 1.00% | `java.util.ArrayDeque.removeFirst() line: 358` |
| 21 | 0.97% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 22 | 0.97% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 23 | 0.89% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 24 | 0.86% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 25 | 0.82% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 26 | 0.79% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 27 | 0.78% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 28 | 0.73% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 29 | 0.70% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 30 | 0.69% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |

GC pauses: 0, total 0 ms

