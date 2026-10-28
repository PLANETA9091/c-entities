# JVM hotspots (JFR ExecutionSample)

Samples: 30250

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.43% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.36% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7656` |
| 3 | 2.72% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 4 | 2.42% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4716` |
| 5 | 2.24% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.10% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 1.92% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 8 | 1.78% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.63% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7797` |
| 10 | 1.45% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4571` |
| 11 | 1.40% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7603` |
| 12 | 1.23% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4007` |
| 13 | 1.21% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 14 | 1.15% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 15 | 1.00% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 16 | 0.92% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 17 | 0.87% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 18 | 0.83% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 19 | 0.80% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 20 | 0.77% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 21 | 0.70% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 22 | 0.68% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 23 | 0.67% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 24 | 0.66% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 25 | 0.65% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 26 | 0.63% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 27 | 0.62% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 28 | 0.62% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 29 | 0.62% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 30 | 0.61% | `java.util.ArrayDeque.pollFirst() line: 378` |
| 31 | 0.60% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 32 | 0.59% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 33 | 0.59% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7617` |
| 34 | 0.52% | `java.util.EnumMap.get(Object) line: 248` |
| 35 | 0.52% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 36 | 0.51% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2703` |
| 37 | 0.51% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 4894` |
| 38 | 0.50% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 39 | 0.49% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1897` |
| 40 | 0.49% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 41 | 0.48% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 42 | 0.47% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 43 | 0.47% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7742` |
| 44 | 0.46% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 45 | 0.45% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 46 | 0.42% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 47 | 0.41% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 48 | 0.41% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 49 | 0.40% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1878` |
| 50 | 0.40% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 51 | 0.36% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 52 | 0.35% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 53 | 0.35% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 54 | 0.35% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 55 | 0.35% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 56 | 0.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 57 | 0.33% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 58 | 0.32% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 59 | 0.30% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 60 | 0.29% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1881` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.21% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.45% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 2.84% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 2.80% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 5 | 2.13% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7652` |
| 6 | 2.10% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 1.92% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 8 | 1.67% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1420` |
| 9 | 1.64% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 10 | 1.54% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 11 | 1.51% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 12 | 1.49% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 13 | 1.20% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 14 | 1.08% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 15 | 0.99% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4044` |
| 16 | 0.97% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 17 | 0.94% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 18 | 0.92% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 19 | 0.84% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 20 | 0.80% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7644` |
| 21 | 0.79% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 5181` |
| 22 | 0.74% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 23 | 0.70% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 24 | 0.69% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |
| 25 | 0.69% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 26 | 0.68% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 5610` |
| 27 | 0.66% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 28 | 0.66% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 29 | 0.65% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 898` |
| 30 | 0.63% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4718` |

GC pauses: 0, total 0 ms

