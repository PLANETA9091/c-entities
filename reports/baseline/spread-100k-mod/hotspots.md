# JVM hotspots (JFR ExecutionSample)

Samples: 26456

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.62% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 4.01% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.00% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4200` |
| 4 | 2.63% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 6509` |
| 5 | 2.59% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 6 | 2.29% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 2.14% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 1.75% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6456` |
| 9 | 1.74% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 10 | 1.53% | `dev.planeta9091.centities.CeHooks.setSharedFlag$(Entity, int, boolean) line: 6345` |
| 11 | 1.46% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4055` |
| 12 | 0.99% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 13 | 0.98% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 14 | 0.98% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 15 | 0.96% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 16 | 0.92% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 17 | 0.90% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 18 | 0.88% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 19 | 0.87% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 20 | 0.84% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 21 | 0.83% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 22 | 0.80% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 23 | 0.80% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 24 | 0.80% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 25 | 0.79% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 26 | 0.78% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 27 | 0.75% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1428` |
| 28 | 0.71% | `java.util.ArrayDeque.pollFirst() line: 378` |
| 29 | 0.70% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 30 | 0.68% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1412` |
| 31 | 0.68% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 32 | 0.65% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 33 | 0.62% | `java.util.EnumMap.get(Object) line: 248` |
| 34 | 0.62% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 35 | 0.60% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 36 | 0.57% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 37 | 0.55% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 38 | 0.54% | `dev.planeta9091.centities.CeHooks.aiPackSel(int[], int, WrappedGoal[], int[], int[], long[], int[], float[], Mob) line: 5885` |
| 39 | 0.53% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 40 | 0.53% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 41 | 0.52% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 42 | 0.50% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 43 | 0.49% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 44 | 0.46% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 45 | 0.43% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2078` |
| 46 | 0.42% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2024` |
| 47 | 0.42% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 48 | 0.40% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 49 | 0.40% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 50 | 0.39% | `dev.planeta9091.centities.CeHooks.gsMirrorTick(GoalSelector) line: 4818` |
| 51 | 0.39% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 52 | 0.37% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 53 | 0.37% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1409` |
| 54 | 0.36% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 55 | 0.35% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 56 | 0.34% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 57 | 0.34% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 58 | 0.34% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 59 | 0.33% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 60 | 0.33% | `net.minecraft.world.entity.Entity.computeSpeed() line: 961` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.31% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.31% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 2.96% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6456` |
| 4 | 2.70% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4005` |
| 5 | 2.63% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 6 | 2.61% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 7 | 2.46% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1062` |
| 8 | 2.14% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 1.77% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk)` |
| 10 | 1.74% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 11 | 1.72% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4202` |
| 12 | 1.67% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 13 | 1.60% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 14 | 1.54% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean)` |
| 15 | 1.32% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 16 | 1.24% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 17 | 1.21% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 18 | 1.19% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 19 | 0.99% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4658` |
| 20 | 0.98% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 21 | 0.97% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 22 | 0.94% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 23 | 0.92% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 24 | 0.90% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1413` |
| 25 | 0.88% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 26 | 0.80% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 27 | 0.79% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |
| 28 | 0.71% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 29 | 0.71% | `java.util.ArrayDeque.removeFirst() line: 358` |
| 30 | 0.70% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |

GC pauses: 0, total 0 ms

