# JVM hotspots (JFR ExecutionSample)

Samples: 15440

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 9.44% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer) line: 39` |
| 2 | 6.73% | `net.minecraft.world.entity.Entity.setTicksFrozen(int) line: 3907` |
| 3 | 6.06% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$clearPlayers() line: 1336` |
| 4 | 4.74% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 3.61% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 118` |
| 6 | 2.95% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 2.50% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5073` |
| 8 | 2.36% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 157` |
| 9 | 2.11% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2403` |
| 10 | 1.60% | `net.minecraft.world.entity.LivingEntity.updateDirtyEffects() line: 995` |
| 11 | 1.59% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 12 | 1.30% | `net.minecraft.world.entity.Entity.isRemoved() line: 5328` |
| 13 | 1.19% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 14 | 1.16% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3868` |
| 15 | 1.02% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 16 | 1.02% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 1948` |
| 17 | 0.97% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 18 | 0.95% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 19 | 0.90% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 20 | 0.78% | `net.minecraft.world.entity.Entity.computeSpeed() line: 960` |
| 21 | 0.76% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 22 | 0.76% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1837` |
| 23 | 0.74% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 24 | 0.74% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 25 | 0.70% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 26 | 0.66% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 27 | 0.65% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 28 | 0.65% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1851` |
| 29 | 0.65% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 30 | 0.62% | `java.util.EnumMap.get(Object) line: 248` |
| 31 | 0.62% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 32 | 0.58% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 33 | 0.56% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 34 | 0.54% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 35 | 0.54% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 36 | 0.51% | `net.minecraft.world.entity.Entity.getVehicle() line: 4933` |
| 37 | 0.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 38 | 0.45% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 39 | 0.45% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |
| 40 | 0.44% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2386` |
| 41 | 0.44% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 42 | 0.39% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 43 | 0.39% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 44 | 0.38% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 45 | 0.36% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 114` |
| 46 | 0.34% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 501` |
| 47 | 0.33% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 48 | 0.32% | `ca.spottedleaf.moonrise.common.list.IteratorSafeOrderedReferenceSet$BaseIterator.hasNext() line: 283` |
| 49 | 0.31% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 50 | 0.30% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1428` |
| 51 | 0.30% | `it.unimi.dsi.fastutil.objects.ReferenceOpenHashSet.isEmpty() line: 477` |
| 52 | 0.29% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 320` |
| 53 | 0.29% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1844` |
| 54 | 0.28% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 55 | 0.27% | `net.minecraft.world.phys.AABB.inflate(double, double, double) line: 185` |
| 56 | 0.25% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 57 | 0.25% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2195` |
| 58 | 0.25% | `it.unimi.dsi.fastutil.objects.ObjectArrayList.addAll(int, Collection) line: 762` |
| 59 | 0.25% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1523` |
| 60 | 0.25% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 9.44% | `net.minecraft.server.level.ServerLevel.tick(BooleanSupplier) line: 893` |
| 2 | 7.69% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 7.33% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 4 | 6.74% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 5 | 6.08% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk) line: 1275` |
| 6 | 2.46% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 2413` |
| 7 | 2.20% | `net.minecraft.world.entity.Entity.setOldPos() line: 2395` |
| 8 | 2.16% | `net.minecraft.server.level.ServerLevel.tickNonPassenger(Entity)` |
| 9 | 1.60% | `net.minecraft.world.entity.LivingEntity.updateDataBeforeSync() line: 939` |
| 10 | 1.55% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 11 | 1.48% | `dev.planeta9091.centities.CeHooks.arenaRef(long) line: 2480` |
| 12 | 1.39% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 13 | 1.33% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 14 | 1.26% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 895` |
| 15 | 1.15% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 16 | 1.08% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 969` |
| 17 | 1.07% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 1872` |
| 18 | 1.06% | `net.minecraft.world.entity.LivingEntity.tick() line: 3432` |
| 19 | 1.01% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 20 | 0.91% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 21 | 0.88% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 22 | 0.85% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 23 | 0.78% | `net.minecraft.world.entity.Entity.baseTick() line: 902` |
| 24 | 0.76% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 25 | 0.76% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 26 | 0.75% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2212` |
| 27 | 0.74% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 28 | 0.71% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 89` |
| 29 | 0.71% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 226` |
| 30 | 0.63% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |

GC pauses: 0, total 0 ms

