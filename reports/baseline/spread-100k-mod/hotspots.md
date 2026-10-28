# JVM hotspots (JFR ExecutionSample)

Samples: 28331

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.18% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 4.11% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7720` |
| 3 | 2.81% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 4 | 2.72% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7914` |
| 5 | 2.44% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.31% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7773` |
| 7 | 2.17% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 2.00% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 9 | 1.84% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.54% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 11 | 1.34% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4688` |
| 12 | 1.24% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4833` |
| 13 | 1.08% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 14 | 1.01% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 15 | 1.01% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 16 | 0.89% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 17 | 0.88% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 18 | 0.87% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 19 | 0.81% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 20 | 0.80% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 21 | 0.80% | `java.util.EnumMap.get(Object) line: 248` |
| 22 | 0.78% | `java.lang.String.hashCode() line: 2433` |
| 23 | 0.74% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 24 | 0.73% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1986` |
| 25 | 0.71% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 26 | 0.69% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 27 | 0.68% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 28 | 0.66% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4838` |
| 29 | 0.65% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 30 | 0.65% | `java.lang.ref.Reference.get() line: 361` |
| 31 | 0.64% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 32 | 0.63% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 33 | 0.62% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2713` |
| 34 | 0.61% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 35 | 0.60% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 36 | 0.59% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 37 | 0.58% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 38 | 0.56% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 5356` |
| 39 | 0.56% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 40 | 0.55% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2005` |
| 41 | 0.53% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 42 | 0.53% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 43 | 0.52% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 44 | 0.51% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7859` |
| 45 | 0.49% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 46 | 0.48% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 47 | 0.48% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 48 | 0.48% | `net.minecraft.world.entity.Entity.commonTick() line: 877` |
| 49 | 0.47% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4115` |
| 50 | 0.47% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 51 | 0.46% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 52 | 0.44% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 53 | 0.43% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 54 | 0.42% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 55 | 0.41% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 56 | 0.41% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 57 | 0.40% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 58 | 0.38% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 320` |
| 59 | 0.38% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 60 | 0.37% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 7.02% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.27% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7769` |
| 3 | 2.81% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 4 | 2.72% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 5 | 2.51% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 6 | 2.34% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 7 | 2.18% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 8 | 2.00% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 9 | 1.81% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1528` |
| 10 | 1.68% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 11 | 1.37% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 12 | 1.24% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 5366` |
| 13 | 1.23% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 14 | 1.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4152` |
| 15 | 1.08% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 5727` |
| 16 | 1.06% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 17 | 1.04% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 18 | 1.02% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 19 | 1.01% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 20 | 1.00% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 21 | 0.90% | `net.minecraft.world.level.EntityGetter.findNearbyPlayer(Entity, double, Predicate) line: 117` |
| 22 | 0.89% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 23 | 0.89% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 24 | 0.89% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 25 | 0.88% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 26 | 0.81% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2195` |
| 27 | 0.79% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4835` |
| 28 | 0.72% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 29 | 0.72% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 30 | 0.70% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |

GC pauses: 0, total 0 ms

