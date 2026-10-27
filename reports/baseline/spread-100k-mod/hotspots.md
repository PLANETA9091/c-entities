# JVM hotspots (JFR ExecutionSample)

Samples: 29269

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.99% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7317` |
| 2 | 3.65% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.42% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7176` |
| 4 | 3.28% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4236` |
| 5 | 2.46% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 6 | 2.28% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.20% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 1.99% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 9 | 1.87% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.76% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4091` |
| 11 | 1.69% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7123` |
| 12 | 1.02% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 13 | 0.90% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 14 | 0.85% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 15 | 0.85% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 16 | 0.80% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 17 | 0.78% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 18 | 0.78% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 19 | 0.77% | `java.util.EnumMap.get(Object) line: 248` |
| 20 | 0.74% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 21 | 0.73% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 22 | 0.72% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 23 | 0.67% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 24 | 0.64% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1464` |
| 25 | 0.61% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 26 | 0.60% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 27 | 0.59% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 28 | 0.58% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 29 | 0.58% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7137` |
| 30 | 0.56% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 31 | 0.55% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 32 | 0.55% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 33 | 0.54% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 34 | 0.53% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 35 | 0.51% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 36 | 0.50% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7262` |
| 37 | 0.47% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2114` |
| 38 | 0.46% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 39 | 0.45% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1445` |
| 40 | 0.45% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 41 | 0.45% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 4414` |
| 42 | 0.42% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 43 | 0.42% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 44 | 0.42% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 45 | 0.41% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1448` |
| 46 | 0.41% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 47 | 0.41% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 48 | 0.40% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 49 | 0.39% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 50 | 0.37% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 51 | 0.37% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 52 | 0.36% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 53 | 0.36% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 54 | 0.35% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 55 | 0.34% | `java.lang.ref.Reference.refersToImpl(Object) line: 384` |
| 56 | 0.34% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 57 | 0.34% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 58 | 0.33% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 59 | 0.31% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 60 | 0.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.52% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.00% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 3 | 3.66% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 3.58% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 2.57% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 6 | 2.37% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7172` |
| 7 | 2.20% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 2.01% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 9 | 1.99% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1067` |
| 10 | 1.81% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 11 | 1.55% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 12 | 1.43% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4238` |
| 13 | 1.11% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 14 | 0.98% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7164` |
| 15 | 0.96% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 16 | 0.92% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 17 | 0.90% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 18 | 0.80% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 19 | 0.77% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 20 | 0.75% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 21 | 0.75% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 22 | 0.73% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1449` |
| 23 | 0.70% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |
| 24 | 0.70% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4701` |
| 25 | 0.67% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 26 | 0.61% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 898` |
| 27 | 0.59% | `net.minecraft.world.entity.LivingEntity.detectEquipmentUpdates() line: 3558` |
| 28 | 0.58% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 29 | 0.57% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2195` |
| 30 | 0.57% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |

GC pauses: 0, total 0 ms

