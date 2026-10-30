# JVM hotspots (JFR ExecutionSample)

Samples: 34764

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.25% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8717` |
| 2 | 3.16% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.03% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8858` |
| 4 | 3.01% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 5 | 2.82% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.61% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5753` |
| 7 | 2.11% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 1.87% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8664` |
| 9 | 1.71% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 1.58% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.53% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5608` |
| 12 | 1.48% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 1.43% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4962` |
| 14 | 1.40% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 15 | 1.13% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 16 | 1.07% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 389` |
| 17 | 1.01% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 18 | 1.00% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 19 | 0.93% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 20 | 0.89% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 21 | 0.85% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 22 | 0.81% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 23 | 0.78% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 24 | 0.78% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 25 | 0.68% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 26 | 0.67% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 27 | 0.67% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 28 | 0.64% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 993` |
| 29 | 0.64% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 30 | 0.61% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 31 | 0.59% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 32 | 0.59% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 33 | 0.58% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 34 | 0.58% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 35 | 0.56% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 36 | 0.56% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 37 | 0.55% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 38 | 0.54% | `java.util.EnumMap.get(Object) line: 248` |
| 39 | 0.52% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 40 | 0.51% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 41 | 0.49% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 42 | 0.48% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 43 | 0.46% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 44 | 0.45% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 45 | 0.44% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 46 | 0.44% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4009` |
| 47 | 0.43% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 48 | 0.43% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 49 | 0.42% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 50 | 0.42% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 51 | 0.40% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 52 | 0.37% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8803` |
| 53 | 0.37% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 54 | 0.37% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8678` |
| 55 | 0.36% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 56 | 0.35% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 57 | 0.35% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 58 | 0.35% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3663` |
| 59 | 0.35% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 6770` |
| 60 | 0.34% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.75% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.38% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 3.03% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 4 | 2.92% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 2.41% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8713` |
| 6 | 2.12% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 7 | 2.11% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 8 | 1.71% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 1.65% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 10 | 1.60% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8730` |
| 11 | 1.56% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 12 | 1.48% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 13 | 1.40% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 14 | 1.28% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8705` |
| 15 | 1.12% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 16 | 1.11% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 17 | 1.10% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5063` |
| 18 | 1.07% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2597` |
| 19 | 0.96% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 20 | 0.89% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 21 | 0.88% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 22 | 0.84% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 23 | 0.82% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 24 | 0.81% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 25 | 0.80% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3584` |
| 26 | 0.79% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 27 | 0.79% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 28 | 0.76% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 29 | 0.75% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 1988` |
| 30 | 0.72% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |

GC pauses: 0, total 0 ms

