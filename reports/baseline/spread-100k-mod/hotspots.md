# JVM hotspots (JFR ExecutionSample)

Samples: 47678

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 9.70% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.79% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 3 | 3.59% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3907` |
| 4 | 3.08% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 2.51% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 6 | 2.00% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 7 | 1.71% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 1.48% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 9 | 1.43% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 551` |
| 10 | 1.31% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 11 | 1.28% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 12 | 1.07% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4706` |
| 13 | 1.02% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 14 | 0.91% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 15 | 0.89% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 16 | 0.87% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 17 | 0.79% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3868` |
| 18 | 0.75% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 19 | 0.73% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 20 | 0.72% | `java.util.EnumMap.get(Object) line: 248` |
| 21 | 0.68% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 22 | 0.67% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 23 | 0.66% | `java.util.HashMap.getNode(Object) line: 586` |
| 24 | 0.65% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 25 | 0.65% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 26 | 0.62% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 27 | 0.61% | `java.lang.String.hashCode() line: 2433` |
| 28 | 0.59% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 29 | 0.59% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 30 | 0.58% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 31 | 0.57% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 32 | 0.55% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 33 | 0.52% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 34 | 0.51% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 35 | 0.50% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 36 | 0.50% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 37 | 0.49% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 38 | 0.48% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 39 | 0.47% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 40 | 0.46% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2403` |
| 41 | 0.46% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 543` |
| 42 | 0.45% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 43 | 0.43% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 44 | 0.42% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 45 | 0.41% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 46 | 0.37% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 47 | 0.37% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 48 | 0.36% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 49 | 0.32% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 1661` |
| 50 | 0.32% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 51 | 0.32% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 52 | 0.31% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 53 | 0.31% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 54 | 0.30% | `net.minecraft.world.entity.LivingEntity.canCollideWithBukkit(Entity) line: 4089` |
| 55 | 0.30% | `net.minecraft.world.entity.LivingEntity.updateGlowingStatus() line: 1025` |
| 56 | 0.30% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 57 | 0.30% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 58 | 0.29% | `net.minecraft.world.entity.LivingEntity.pushEntities() line: 3926` |
| 59 | 0.29% | `java.lang.String.hashCode() line: 2430` |
| 60 | 0.29% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 9.70% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 4.79% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 3.79% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 4 | 3.59% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 5 | 3.28% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 6 | 2.51% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 7 | 2.44% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 8 | 2.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 9 | 2.05% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 10 | 0.98% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 11 | 0.93% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 12 | 0.89% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 0.87% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 14 | 0.81% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 15 | 0.80% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 16 | 0.79% | `dev.planeta9091.centities.CeHooks.arenaRef(long) line: 2193` |
| 17 | 0.75% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 969` |
| 18 | 0.74% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2195` |
| 19 | 0.70% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 937` |
| 20 | 0.70% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 21 | 0.66% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2126` |
| 22 | 0.65% | `java.util.HashMap.containsKey(Object) line: 603` |
| 23 | 0.65% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 24 | 0.61% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 25 | 0.60% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 26 | 0.59% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 27 | 0.57% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 28 | 0.55% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 29 | 0.55% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 30 | 0.53% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |

GC pauses: 0, total 0 ms

