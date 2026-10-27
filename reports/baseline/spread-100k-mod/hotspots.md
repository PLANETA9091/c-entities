# JVM hotspots (JFR ExecutionSample)

Samples: 34736

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.87% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7666` |
| 2 | 3.48% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7525` |
| 3 | 3.48% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 3.35% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4585` |
| 5 | 3.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 6 | 2.35% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 2.21% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 8 | 1.95% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 1.77% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7472` |
| 10 | 1.60% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.55% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4440` |
| 12 | 0.98% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 13 | 0.90% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 14 | 0.87% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 15 | 0.83% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 16 | 0.78% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 17 | 0.77% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 18 | 0.77% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 19 | 0.74% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 20 | 0.73% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1778` |
| 21 | 0.73% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 22 | 0.71% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 23 | 0.71% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 24 | 0.71% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 25 | 0.69% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 26 | 0.67% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 27 | 0.62% | `jdk.internal.util.ArraysSupport.hashCodeOfUnsigned(byte[], int, int, int) line: 275` |
| 28 | 0.61% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7486` |
| 29 | 0.59% | `java.util.EnumMap.get(Object) line: 248` |
| 30 | 0.58% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 31 | 0.58% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 4763` |
| 32 | 0.57% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 33 | 0.55% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 34 | 0.55% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 35 | 0.54% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7611` |
| 36 | 0.52% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 37 | 0.51% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 38 | 0.50% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 39 | 0.50% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2444` |
| 40 | 0.49% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 41 | 0.49% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 42 | 0.48% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 43 | 0.47% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1762` |
| 44 | 0.47% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4590` |
| 45 | 0.46% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 46 | 0.46% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 47 | 0.46% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 48 | 0.44% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 49 | 0.43% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 50 | 0.41% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 51 | 0.40% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 52 | 0.39% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1759` |
| 53 | 0.37% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 54 | 0.37% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 55 | 0.36% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 56 | 0.36% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 57 | 0.36% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 58 | 0.35% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 59 | 0.34% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 60 | 0.33% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.08% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.08% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 3.89% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 4 | 3.60% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 3.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 6 | 2.51% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7521` |
| 7 | 2.21% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 8 | 2.07% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1316` |
| 9 | 1.95% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 10 | 1.58% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 11 | 1.53% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 12 | 1.42% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4587` |
| 13 | 1.03% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 14 | 1.03% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 15 | 0.98% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 16 | 0.98% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 17 | 0.93% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 18 | 0.89% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 5050` |
| 19 | 0.87% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7513` |
| 20 | 0.82% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1763` |
| 21 | 0.79% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 22 | 0.78% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 23 | 0.76% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 24 | 0.75% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 25 | 0.73% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 26 | 0.73% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 27 | 0.69% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 28 | 0.69% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |
| 29 | 0.63% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 898` |
| 30 | 0.63% | `java.lang.StringLatin1.hashCode(byte[]) line: 195` |

GC pauses: 0, total 0 ms

