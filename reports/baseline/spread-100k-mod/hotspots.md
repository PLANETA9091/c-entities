# JVM hotspots (JFR ExecutionSample)

Samples: 33985

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.83% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 2 | 3.51% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7281` |
| 3 | 3.42% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 3.27% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7145` |
| 5 | 3.12% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4205` |
| 6 | 3.00% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 2.09% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 1.84% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.83% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 10 | 1.68% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7092` |
| 11 | 1.37% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4060` |
| 12 | 1.01% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 13 | 1.00% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 14 | 0.97% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 15 | 0.97% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 16 | 0.96% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 17 | 0.90% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 18 | 0.88% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 19 | 0.79% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1433` |
| 20 | 0.77% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 21 | 0.76% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 22 | 0.74% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 23 | 0.72% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 24 | 0.71% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 25 | 0.68% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 26 | 0.67% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 27 | 0.64% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 28 | 0.63% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 29 | 0.60% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 30 | 0.59% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 31 | 0.58% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7106` |
| 32 | 0.56% | `java.util.EnumMap.get(Object) line: 248` |
| 33 | 0.53% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 34 | 0.53% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 4383` |
| 35 | 0.51% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 36 | 0.51% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 37 | 0.51% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 38 | 0.51% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1417` |
| 39 | 0.49% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 40 | 0.48% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 41 | 0.47% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1414` |
| 42 | 0.46% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 43 | 0.46% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 44 | 0.46% | `java.lang.String.hashCode() line: 2433` |
| 45 | 0.45% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 46 | 0.45% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2083` |
| 47 | 0.45% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 48 | 0.45% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 49 | 0.44% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4210` |
| 50 | 0.41% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 51 | 0.41% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 52 | 0.41% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 53 | 0.40% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 54 | 0.39% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 55 | 0.39% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 56 | 0.38% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 57 | 0.36% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 320` |
| 58 | 0.32% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 59 | 0.31% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 60 | 0.29% | `net.minecraft.world.entity.LivingEntity$SwingState.tick() line: 5015` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.27% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.93% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 3 | 3.79% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 3.52% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 5 | 3.37% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 6 | 3.00% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 2.42% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7141` |
| 8 | 2.27% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1067` |
| 9 | 1.83% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 10 | 1.44% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 11 | 1.42% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 12 | 1.35% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4207` |
| 13 | 1.14% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 14 | 1.03% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 15 | 0.97% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 16 | 0.96% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 17 | 0.93% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 125` |
| 18 | 0.92% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 19 | 0.91% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |
| 20 | 0.90% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |
| 21 | 0.82% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4670` |
| 22 | 0.80% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 23 | 0.79% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7133` |
| 24 | 0.79% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 25 | 0.74% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 26 | 0.74% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1418` |
| 27 | 0.73% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 28 | 0.71% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 29 | 0.61% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 30 | 0.61% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 898` |

GC pauses: 0, total 0 ms

