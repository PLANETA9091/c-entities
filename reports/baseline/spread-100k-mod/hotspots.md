# JVM hotspots (JFR ExecutionSample)

Samples: 31789

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.69% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 2 | 4.12% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3025` |
| 3 | 3.30% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 2.50% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 2898` |
| 5 | 2.22% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 2927` |
| 6 | 2.18% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3907` |
| 7 | 2.09% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 8 | 2.05% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 2.00% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 10 | 1.77% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 11 | 1.65% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 12 | 1.32% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3027` |
| 13 | 1.25% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 14 | 1.14% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 15 | 1.05% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 16 | 1.00% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 17 | 0.94% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 18 | 0.91% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3868` |
| 19 | 0.84% | `java.util.EnumMap.get(Object) line: 248` |
| 20 | 0.82% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 21 | 0.79% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 22 | 0.77% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 23 | 0.77% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 24 | 0.75% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2403` |
| 25 | 0.75% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 26 | 0.75% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 27 | 0.72% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 28 | 0.70% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 29 | 0.68% | `jdk.internal.reflect.MethodHandleObjectFieldAccessorImpl.get(Object) line: 57` |
| 30 | 0.63% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 31 | 0.61% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 32 | 0.60% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 996` |
| 33 | 0.58% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 34 | 0.57% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 35 | 0.54% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2010` |
| 36 | 0.53% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 37 | 0.52% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 38 | 0.51% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 39 | 0.50% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 40 | 0.48% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 41 | 0.48% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 42 | 0.47% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 43 | 0.46% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1443` |
| 44 | 0.46% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 45 | 0.46% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 46 | 0.44% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 47 | 0.43% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 48 | 0.43% | `net.minecraft.world.entity.Entity.getX() line: 5094` |
| 49 | 0.43% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 50 | 0.41% | `java.util.ArrayDeque.pollFirst() line: 378` |
| 51 | 0.39% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 52 | 0.38% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 53 | 0.35% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 54 | 0.34% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 55 | 0.33% | `net.minecraft.world.entity.Entity.isRemoved() line: 5328` |
| 56 | 0.32% | `dev.planeta9091.centities.CeHooks.cellOfBox(AABB) line: 817` |
| 57 | 0.32% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 58 | 0.30% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 59 | 0.30% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4706` |
| 60 | 0.29% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.72% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 2 | 5.50% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 5.33% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 5.16% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 5 | 2.18% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 6 | 2.11% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 7 | 2.07% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 8 | 1.77% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 1.65% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 10 | 1.25% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 11 | 1.25% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 12 | 1.23% | `net.minecraft.world.entity.Entity.setDeltaMovement(double, double, double) line: 5086` |
| 13 | 1.21% | `dev.planeta9091.centities.CeHooks.arenaRef(long) line: 2542` |
| 14 | 1.13% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2475` |
| 15 | 0.90% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 16 | 0.89% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 17 | 0.86% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 969` |
| 18 | 0.82% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 19 | 0.79% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 20 | 0.78% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1036` |
| 21 | 0.77% | `net.minecraft.world.entity.Entity.setOldPos() line: 2395` |
| 22 | 0.76% | `java.lang.reflect.Field.get(Object) line: 440` |
| 23 | 0.72% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity)` |
| 24 | 0.70% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 25 | 0.66% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 26 | 0.64% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 27 | 0.63% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 28 | 0.61% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 29 | 0.60% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 30 | 0.59% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |

GC pauses: 0, total 0 ms

