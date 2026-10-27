# JVM hotspots (JFR ExecutionSample)

Samples: 29684

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.60% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.40% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7462` |
| 3 | 2.58% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 4 | 2.49% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.48% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4522` |
| 6 | 2.03% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 7 | 1.84% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 1.69% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7409` |
| 9 | 1.55% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4377` |
| 10 | 1.53% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 11 | 1.46% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 12 | 1.36% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 13 | 1.07% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 14 | 0.99% | `java.util.EnumMap$EnumMapIterator.hasNext() line: 524` |
| 15 | 0.94% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 16 | 0.92% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1715` |
| 17 | 0.86% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 18 | 0.84% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 19 | 0.84% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 20 | 0.80% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 21 | 0.76% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 22 | 0.75% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 23 | 0.68% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 24 | 0.67% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 25 | 0.65% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 26 | 0.64% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 27 | 0.63% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 28 | 0.58% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2381` |
| 29 | 0.57% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 30 | 0.56% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1696` |
| 31 | 0.56% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 32 | 0.55% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 33 | 0.55% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7423` |
| 34 | 0.53% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 4700` |
| 35 | 0.52% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 36 | 0.52% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7548` |
| 37 | 0.49% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 38 | 0.49% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 39 | 0.49% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 40 | 0.49% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 41 | 0.48% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4527` |
| 42 | 0.47% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 43 | 0.47% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 44 | 0.43% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 45 | 0.43% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 46 | 0.42% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 47 | 0.42% | `java.util.EnumMap.get(Object) line: 248` |
| 48 | 0.41% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 49 | 0.40% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 50 | 0.39% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 51 | 0.39% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1699` |
| 52 | 0.37% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 53 | 0.37% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 54 | 0.36% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 55 | 0.34% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 56 | 0.34% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 57 | 0.34% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 58 | 0.33% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 59 | 0.30% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 60 | 0.29% | `net.minecraft.world.entity.LivingEntity$SwingState.tick() line: 5015` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.44% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.53% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 3.18% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 2.66% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 5 | 2.44% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1307` |
| 6 | 2.36% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7458` |
| 7 | 2.03% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 8 | 1.59% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 9 | 1.56% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4524` |
| 10 | 1.55% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 11 | 1.52% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 12 | 1.46% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 13 | 1.21% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 14 | 1.15% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 15 | 1.08% | `net.minecraft.world.entity.EntityEquipment.tick(Entity) line: 50` |
| 16 | 1.01% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7450` |
| 17 | 1.01% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1700` |
| 18 | 1.00% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 19 | 0.79% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 20 | 0.77% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4987` |
| 21 | 0.76% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 22 | 0.74% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 898` |
| 23 | 0.74% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |
| 24 | 0.71% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 25 | 0.68% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 26 | 0.68% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 27 | 0.67% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 28 | 0.67% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 29 | 0.65% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 5416` |
| 30 | 0.64% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |

GC pauses: 0, total 0 ms

