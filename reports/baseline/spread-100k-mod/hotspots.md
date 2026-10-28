# JVM hotspots (JFR ExecutionSample)

Samples: 34115

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.57% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7722` |
| 2 | 3.35% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7863` |
| 3 | 3.28% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 3.13% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4782` |
| 5 | 2.45% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.39% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 7 | 2.27% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 1.92% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7669` |
| 9 | 1.82% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4073` |
| 10 | 1.64% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 11 | 1.49% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.35% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 1.28% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 14 | 1.24% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4637` |
| 15 | 1.16% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 16 | 1.00% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 17 | 0.91% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 18 | 0.86% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 19 | 0.85% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 20 | 0.85% | `java.util.EnumMap.get(Object) line: 248` |
| 21 | 0.82% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 22 | 0.76% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 23 | 0.74% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 24 | 0.73% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1963` |
| 25 | 0.67% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 26 | 0.60% | `java.lang.ref.Reference.get() line: 361` |
| 27 | 0.60% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 28 | 0.59% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 29 | 0.59% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 30 | 0.59% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 31 | 0.58% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2769` |
| 32 | 0.58% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7808` |
| 33 | 0.55% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 34 | 0.53% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 35 | 0.51% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 36 | 0.50% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 37 | 0.50% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 38 | 0.48% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 39 | 0.46% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 40 | 0.45% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 41 | 0.45% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7683` |
| 42 | 0.45% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4787` |
| 43 | 0.44% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 44 | 0.43% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 45 | 0.42% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 46 | 0.42% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 47 | 0.42% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 48 | 0.41% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1944` |
| 49 | 0.39% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 50 | 0.39% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1947` |
| 51 | 0.39% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 52 | 0.38% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 53 | 0.38% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 54 | 0.35% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 55 | 0.34% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 56 | 0.34% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 57 | 0.33% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 58 | 0.31% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 59 | 0.31% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 60 | 0.29% | `java.util.EnumMap.isValidKey(Object) line: 320` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.76% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.93% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 3.71% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 3.36% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 5 | 2.49% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7718` |
| 6 | 2.39% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 7 | 2.36% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 8 | 2.17% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 9 | 2.03% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1486` |
| 10 | 1.64% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 11 | 1.63% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 12 | 1.34% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7710` |
| 13 | 1.28% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 14 | 1.28% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 15 | 1.21% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 16 | 1.08% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1948` |
| 17 | 1.01% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 18 | 0.96% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 19 | 0.96% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4110` |
| 20 | 0.91% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 21 | 0.86% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 22 | 0.80% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 23 | 0.77% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 24 | 0.76% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 25 | 0.68% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |
| 26 | 0.64% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2692` |
| 27 | 0.63% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 28 | 0.63% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 29 | 0.62% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 30 | 0.57% | `java.util.Map.getOrDefault(Object, Object) line: 691` |

GC pauses: 0, total 0 ms

