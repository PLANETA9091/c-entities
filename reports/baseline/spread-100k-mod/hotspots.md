# JVM hotspots (JFR ExecutionSample)

Samples: 35516

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.50% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 2 | 3.15% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8788` |
| 3 | 2.72% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 2.65% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 5 | 2.63% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 2.30% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8929` |
| 7 | 2.10% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5824` |
| 8 | 1.89% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 1.61% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8735` |
| 10 | 1.54% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 11 | 1.46% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5033` |
| 12 | 1.30% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5679` |
| 13 | 1.29% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 14 | 1.21% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 15 | 1.02% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 16 | 0.95% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 17 | 0.93% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 18 | 0.93% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 19 | 0.83% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 389` |
| 20 | 0.77% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 21 | 0.77% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkTaskScheduler.popChunkWait() line: 927` |
| 22 | 0.76% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 23 | 0.75% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 24 | 0.72% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 25 | 0.72% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 26 | 0.72% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 27 | 0.68% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 28 | 0.66% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 29 | 0.63% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8874` |
| 30 | 0.63% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 31 | 0.61% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 32 | 0.60% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 33 | 0.60% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 34 | 0.59% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 35 | 0.57% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 36 | 0.57% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 37 | 0.50% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 38 | 0.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 39 | 0.45% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 40 | 0.43% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8749` |
| 41 | 0.43% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 42 | 0.40% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 43 | 0.40% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 44 | 0.39% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 45 | 0.39% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 46 | 0.39% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 47 | 0.38% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 48 | 0.37% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4080` |
| 49 | 0.36% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2661` |
| 50 | 0.36% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 51 | 0.35% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 52 | 0.35% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 53 | 0.34% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 54 | 0.34% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 55 | 0.32% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 56 | 0.32% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5829` |
| 57 | 0.32% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 58 | 0.31% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 59 | 0.30% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 60 | 0.30% | `net.minecraft.world.entity.Entity.level() line: 5416` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 3.83% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.23% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 2.73% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 4 | 2.66% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 2.51% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 6 | 2.31% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 7 | 2.20% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8784` |
| 8 | 1.89% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 1.67% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 10 | 1.58% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 11 | 1.40% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 12 | 1.32% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 13 | 1.20% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8776` |
| 14 | 1.01% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 15 | 0.97% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5134` |
| 16 | 0.94% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 17 | 0.88% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 18 | 0.86% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 19 | 0.84% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 20 | 0.84% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2657` |
| 21 | 0.78% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 22 | 0.77% | `net.minecraft.server.level.ServerChunkCache.syncLoad(int, int, ChunkStatus) line: 124` |
| 23 | 0.77% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |
| 24 | 0.76% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 25 | 0.75% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 26 | 0.75% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 27 | 0.74% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2043` |
| 28 | 0.73% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 29 | 0.72% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 30 | 0.68% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |

GC pauses: 0, total 0 ms

