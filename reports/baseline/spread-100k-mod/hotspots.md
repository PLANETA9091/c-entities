# JVM hotspots (JFR ExecutionSample)

Samples: 37095

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.67% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8183` |
| 2 | 3.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 3 | 3.26% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8042` |
| 4 | 3.21% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5078` |
| 5 | 2.85% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 2.54% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.09% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 8 | 2.08% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 1.93% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7989` |
| 10 | 1.56% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4360` |
| 11 | 1.42% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.32% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 13 | 1.23% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4933` |
| 14 | 1.21% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 15 | 1.18% | `java.util.ArrayDeque.pollFirst() line: 378` |
| 16 | 1.07% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 17 | 1.06% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 18 | 1.06% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 19 | 0.99% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 20 | 0.95% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 21 | 0.90% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 22 | 0.87% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 23 | 0.85% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 24 | 0.82% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 25 | 0.82% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2195` |
| 26 | 0.71% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 27 | 0.66% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 28 | 0.66% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 29 | 0.65% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 30 | 0.63% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 31 | 0.62% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 32 | 0.59% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 33 | 0.58% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8128` |
| 34 | 0.58% | `java.util.EnumMap.get(Object) line: 248` |
| 35 | 0.57% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 36 | 0.54% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3055` |
| 37 | 0.52% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 38 | 0.51% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8003` |
| 39 | 0.50% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 40 | 0.49% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 41 | 0.47% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 42 | 0.46% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2179` |
| 43 | 0.45% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 44 | 0.43% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 45 | 0.43% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 46 | 0.41% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 47 | 0.41% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 48 | 0.40% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 49 | 0.37% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 50 | 0.36% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 51 | 0.35% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 52 | 0.35% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 53 | 0.34% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 54 | 0.34% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 55 | 0.33% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 56 | 0.33% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 57 | 0.28% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 320` |
| 58 | 0.28% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 59 | 0.28% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1484` |
| 60 | 0.26% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.26% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.68% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 3 | 3.54% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 3.43% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 5 | 3.40% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 6 | 2.60% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8038` |
| 7 | 2.09% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 8 | 2.08% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 2.03% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1709` |
| 10 | 1.90% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 11 | 1.57% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 12 | 1.41% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 13 | 1.34% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8030` |
| 14 | 1.32% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 15 | 1.27% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 16 | 1.21% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 17 | 1.21% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4397` |
| 18 | 1.18% | `java.util.ArrayDeque.removeFirst() line: 358` |
| 19 | 0.99% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 20 | 0.89% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 21 | 0.86% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 22 | 0.85% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2180` |
| 23 | 0.82% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |
| 24 | 0.82% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |
| 25 | 0.72% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 26 | 0.69% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 27 | 0.69% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 28 | 0.67% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 29 | 0.67% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 30 | 0.60% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |

GC pauses: 0, total 0 ms

