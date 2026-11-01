# JVM hotspots (JFR ExecutionSample)

Samples: 37042

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.86% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9129` |
| 2 | 3.42% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8988` |
| 3 | 2.89% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 4 | 2.64% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.52% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 6 | 2.50% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.40% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6011` |
| 8 | 1.70% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 1.61% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8935` |
| 10 | 1.45% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5275` |
| 11 | 1.26% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 12 | 1.25% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 1.22% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5857` |
| 14 | 1.14% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 15 | 0.96% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 16 | 0.93% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 17 | 0.89% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 18 | 0.87% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 19 | 0.85% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 20 | 0.81% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 21 | 0.81% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 22 | 0.77% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 23 | 0.73% | `java.util.ArrayDeque.pollFirst() line: 378` |
| 24 | 0.71% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 25 | 0.71% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 26 | 0.69% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 27 | 0.66% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 28 | 0.64% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 29 | 0.63% | `java.util.EnumMap.get(Object) line: 248` |
| 30 | 0.60% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 31 | 0.59% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 9074` |
| 32 | 0.58% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 33 | 0.58% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 34 | 0.58% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 35 | 0.57% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 36 | 0.56% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 37 | 0.52% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 38 | 0.48% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 39 | 0.47% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 40 | 0.47% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2735` |
| 41 | 0.46% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 42 | 0.46% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 43 | 0.46% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 44 | 0.44% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8949` |
| 45 | 0.42% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2697` |
| 46 | 0.41% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 47 | 0.39% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 48 | 0.39% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 49 | 0.39% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 50 | 0.38% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4214` |
| 51 | 0.37% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 52 | 0.36% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 53 | 0.36% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 54 | 0.35% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 55 | 0.35% | `java.lang.ref.Reference.get() line: 361` |
| 56 | 0.34% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 57 | 0.34% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 58 | 0.34% | `net.minecraft.world.entity.SteppedInterpolationTracker.updateTracking(Vec3) line: 40` |
| 59 | 0.33% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6024` |
| 60 | 0.30% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3868` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 3.87% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 2 | 3.75% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 3.54% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 2.99% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 5 | 2.92% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 6 | 2.16% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8984` |
| 7 | 1.83% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 8 | 1.81% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 9 | 1.70% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 10 | 1.57% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2079` |
| 11 | 1.56% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9001` |
| 12 | 1.29% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 13 | 1.27% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 14 | 1.26% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8976` |
| 15 | 1.25% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 16 | 0.97% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5312` |
| 17 | 0.93% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 18 | 0.90% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 19 | 0.87% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 20 | 0.87% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 21 | 0.84% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 22 | 0.81% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 23 | 0.79% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 24 | 0.79% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 25 | 0.73% | `java.util.ArrayDeque.removeFirst() line: 358` |
| 26 | 0.71% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 27 | 0.71% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 28 | 0.69% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 29 | 0.67% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |
| 30 | 0.65% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |

GC pauses: 0, total 0 ms

