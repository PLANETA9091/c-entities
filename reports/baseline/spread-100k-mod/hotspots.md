# JVM hotspots (JFR ExecutionSample)

Samples: 36040

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.62% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7863` |
| 2 | 3.27% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7722` |
| 3 | 3.08% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 4 | 3.00% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 2.47% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 2.13% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4782` |
| 7 | 2.09% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 1.98% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.52% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7669` |
| 10 | 1.40% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4637` |
| 11 | 1.35% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4073` |
| 12 | 1.01% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 13 | 0.88% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 14 | 0.87% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 15 | 0.87% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 16 | 0.84% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 17 | 0.82% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 18 | 0.82% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 19 | 0.82% | `java.util.EnumMap.get(Object) line: 248` |
| 20 | 0.78% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 21 | 0.77% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 22 | 0.75% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 23 | 0.75% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 24 | 0.74% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 25 | 0.74% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 26 | 0.70% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 27 | 0.70% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 28 | 0.69% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 29 | 0.68% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 30 | 0.63% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2769` |
| 31 | 0.60% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 32 | 0.58% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7808` |
| 33 | 0.57% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 34 | 0.56% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1963` |
| 35 | 0.54% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 36 | 0.52% | `jdk.internal.util.ArraysSupport.hashCodeOfUnsigned(byte[], int, int, int) line: 273` |
| 37 | 0.52% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7683` |
| 38 | 0.49% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 39 | 0.49% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 40 | 0.49% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 41 | 0.49% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 42 | 0.47% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 43 | 0.47% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 44 | 0.47% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 45 | 0.46% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1947` |
| 46 | 0.45% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 324` |
| 47 | 0.45% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 4960` |
| 48 | 0.44% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 49 | 0.44% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 50 | 0.42% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 51 | 0.42% | `net.minecraft.world.entity.SteppedInterpolationTracker.updateTracking(Vec3) line: 40` |
| 52 | 0.41% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 53 | 0.40% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 54 | 0.39% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 55 | 0.38% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 56 | 0.37% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 57 | 0.37% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 58 | 0.34% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 59 | 0.33% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1944` |
| 60 | 0.32% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 320` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.98% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.62% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 3 | 3.38% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 3.08% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 5 | 2.55% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 6 | 2.47% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 2.19% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7718` |
| 8 | 1.75% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1486` |
| 9 | 1.65% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 10 | 1.51% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 11 | 1.42% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 12 | 1.42% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 13 | 1.25% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7735` |
| 14 | 1.03% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 15 | 1.01% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 16 | 1.00% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 17 | 0.92% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4110` |
| 18 | 0.87% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 19 | 0.82% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 20 | 0.82% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7710` |
| 21 | 0.80% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 22 | 0.80% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 23 | 0.79% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 24 | 0.78% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 25 | 0.77% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 26 | 0.74% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |
| 27 | 0.70% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2692` |
| 28 | 0.69% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 5247` |
| 29 | 0.67% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |
| 30 | 0.65% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |

GC pauses: 0, total 0 ms

