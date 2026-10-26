# JVM hotspots (JFR ExecutionSample)

Samples: 42266

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.59% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 3.28% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 3.23% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 2.49% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 6320` |
| 5 | 1.69% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 6 | 1.59% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1425` |
| 7 | 1.59% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 1.56% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4066` |
| 9 | 1.47% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 10 | 1.34% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 11 | 1.20% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 12 | 1.13% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 13 | 1.10% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 14 | 1.10% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6277` |
| 15 | 1.10% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1409` |
| 16 | 1.08% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1496` |
| 17 | 0.97% | `dev.planeta9091.centities.CeHooks.setSharedFlag$(Entity, int, boolean) line: 6166` |
| 18 | 0.85% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 19 | 0.81% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 20 | 0.78% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 21 | 0.77% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 22 | 0.76% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3921` |
| 23 | 0.76% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 24 | 0.74% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 25 | 0.74% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1406` |
| 26 | 0.72% | `java.util.EnumMap.get(Object) line: 248` |
| 27 | 0.67% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 28 | 0.67% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1526` |
| 29 | 0.66% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 30 | 0.62% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 31 | 0.60% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 32 | 0.57% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 33 | 0.57% | `java.lang.String.hashCode() line: 2435` |
| 34 | 0.53% | `dev.planeta9091.centities.CeHooks.gsMirrorTickRunning(GoalSelector, boolean) line: 4745` |
| 35 | 0.50% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 36 | 0.49% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 37 | 0.49% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 38 | 0.48% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 39 | 0.47% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 40 | 0.46% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 41 | 0.44% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 42 | 0.43% | `java.util.Arrays.copyOf(Object[], int) line: 3478` |
| 43 | 0.42% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 44 | 0.41% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 45 | 0.41% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 46 | 0.40% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 47 | 0.39% | `dev.planeta9091.centities.CeHooks.gsMirrorTick(GoalSelector) line: 4679` |
| 48 | 0.37% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 49 | 0.36% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 50 | 0.35% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 51 | 0.35% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 52 | 0.34% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 53 | 0.32% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 54 | 0.31% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 55 | 0.31% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 56 | 0.31% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 57 | 0.30% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 58 | 0.30% | `net.minecraft.world.entity.Entity.move(MoverType, Vec3) line: 1170` |
| 59 | 0.29% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 60 | 0.29% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2068` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.81% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.36% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1059` |
| 3 | 3.28% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 2.52% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 5 | 2.45% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 6277` |
| 6 | 2.09% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3871` |
| 7 | 1.78% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 8 | 1.57% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 9 | 1.36% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 10 | 1.34% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1410` |
| 11 | 1.28% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 12 | 1.20% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 13 | 1.19% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 14 | 1.19% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 15 | 1.18% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 16 | 1.13% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk)` |
| 17 | 1.10% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 18 | 1.05% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4068` |
| 19 | 1.03% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4519` |
| 20 | 1.00% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean)` |
| 21 | 0.97% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 22 | 0.91% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 23 | 0.83% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 24 | 0.81% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 25 | 0.79% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 26 | 0.79% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity) line: 603` |
| 27 | 0.70% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 28 | 0.69% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 29 | 0.69% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 30 | 0.68% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |

GC pauses: 0, total 0 ms

