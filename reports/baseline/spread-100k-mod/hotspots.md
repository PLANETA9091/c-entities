# JVM hotspots (JFR ExecutionSample)

Samples: 28641

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.05% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.08% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7721` |
| 3 | 3.07% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7862` |
| 4 | 2.83% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4781` |
| 5 | 2.57% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 6 | 2.34% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 2.29% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 2.15% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 9 | 2.12% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4636` |
| 10 | 1.86% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7668` |
| 11 | 1.82% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 12 | 1.76% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 13 | 1.05% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 14 | 1.05% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 15 | 1.03% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 16 | 1.01% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 17 | 1.00% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 18 | 0.97% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 19 | 0.94% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 20 | 0.93% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 21 | 0.92% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 22 | 0.92% | `net.minecraft.world.item.ItemStack.getDamageValue() line: 573` |
| 23 | 0.90% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 24 | 0.82% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 25 | 0.81% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 26 | 0.76% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 27 | 0.73% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 28 | 0.71% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2768` |
| 29 | 0.71% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 30 | 0.71% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1962` |
| 31 | 0.70% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4072` |
| 32 | 0.70% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 33 | 0.69% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 34 | 0.68% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 35 | 0.61% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 36 | 0.59% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 37 | 0.58% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1946` |
| 38 | 0.56% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 39 | 0.53% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7682` |
| 40 | 0.53% | `java.util.concurrent.CompletableFuture.isDone() line: 2076` |
| 41 | 0.51% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 42 | 0.49% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 43 | 0.49% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 44 | 0.47% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 45 | 0.45% | `java.util.EnumMap.get(Object) line: 248` |
| 46 | 0.45% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 47 | 0.43% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 48 | 0.43% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 49 | 0.43% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 4959` |
| 50 | 0.39% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 51 | 0.39% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 52 | 0.38% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 603` |
| 53 | 0.35% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 54 | 0.35% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 55 | 0.35% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 56 | 0.35% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 57 | 0.34% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 58 | 0.34% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 59 | 0.33% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7807` |
| 60 | 0.32% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1943` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.33% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.32% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 3.19% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 3.07% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 5 | 2.60% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7717` |
| 6 | 2.57% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 7 | 2.34% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 2.14% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 9 | 2.10% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1485` |
| 10 | 1.98% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 11 | 1.82% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 12 | 1.76% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 13 | 1.24% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 14 | 1.19% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 15 | 1.10% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 16 | 1.08% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 17 | 1.03% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4109` |
| 18 | 1.02% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 19 | 0.98% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 20 | 0.97% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 21 | 0.96% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 22 | 0.94% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 23 | 0.90% | `net.minecraft.world.item.ItemStack.isBroken() line: 585` |
| 24 | 0.90% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1947` |
| 25 | 0.77% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 26 | 0.77% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 27 | 0.76% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2691` |
| 28 | 0.76% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 29 | 0.75% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 898` |
| 30 | 0.73% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |

GC pauses: 0, total 0 ms

