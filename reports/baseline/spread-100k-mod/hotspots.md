# JVM hotspots (JFR ExecutionSample)

Samples: 31613

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.59% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7176` |
| 2 | 3.51% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4236` |
| 3 | 3.42% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 2.55% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.53% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 6 | 2.18% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7317` |
| 7 | 2.17% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 1.78% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7123` |
| 9 | 1.72% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.62% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4091` |
| 11 | 1.31% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 12 | 1.06% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 1.01% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 14 | 0.99% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 15 | 0.92% | `java.util.EnumMap.get(Object) line: 248` |
| 16 | 0.81% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 17 | 0.79% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 18 | 0.79% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 19 | 0.79% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 20 | 0.78% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 21 | 0.77% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 22 | 0.71% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 23 | 0.70% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 24 | 0.68% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7137` |
| 25 | 0.67% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7262` |
| 26 | 0.62% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 27 | 0.62% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 28 | 0.60% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 29 | 0.59% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 30 | 0.59% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 31 | 0.58% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 32 | 0.58% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 33 | 0.57% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 34 | 0.56% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 35 | 0.54% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 36 | 0.52% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 4414` |
| 37 | 0.48% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 38 | 0.48% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 39 | 0.48% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 40 | 0.46% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 41 | 0.45% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 42 | 0.45% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 43 | 0.43% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 44 | 0.41% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 45 | 0.40% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1464` |
| 46 | 0.40% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 47 | 0.40% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 48 | 0.39% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 49 | 0.38% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 50 | 0.37% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 51 | 0.37% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1448` |
| 52 | 0.37% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2114` |
| 53 | 0.36% | `net.minecraft.world.entity.SteppedInterpolationTracker.updateTracking(Vec3) line: 40` |
| 54 | 0.35% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 55 | 0.35% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 56 | 0.35% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 57 | 0.34% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 58 | 0.34% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 59 | 0.33% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 60 | 0.33% | `java.util.EnumMap.isValidKey(Object) line: 320` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.15% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.77% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 3.72% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 2.63% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7172` |
| 5 | 2.63% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 6 | 2.18% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 7 | 2.17% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 1.65% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 9 | 1.61% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 10 | 1.45% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4238` |
| 11 | 1.38% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1067` |
| 12 | 1.36% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 13 | 1.23% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7189` |
| 14 | 1.20% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 15 | 1.10% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 16 | 1.08% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 17 | 1.02% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7164` |
| 18 | 1.01% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 19 | 0.92% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 20 | 0.85% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4701` |
| 21 | 0.81% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 22 | 0.81% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 23 | 0.79% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 24 | 0.70% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 25 | 0.69% | `net.minecraft.world.entity.LivingEntity.detectEquipmentUpdates() line: 3558` |
| 26 | 0.67% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 5130` |
| 27 | 0.64% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 28 | 0.62% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 29 | 0.62% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 30 | 0.60% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2195` |

GC pauses: 0, total 0 ms

