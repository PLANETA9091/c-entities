# JVM hotspots (JFR ExecutionSample)

Samples: 10200

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 2.14% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 2 | 1.57% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 1.51% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 4 | 1.49% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 11848` |
| 5 | 1.44% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 6 | 1.34% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 7 | 1.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 1.33% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 9 | 1.21% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 10 | 1.20% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 11 | 1.13% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7890` |
| 12 | 1.11% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 13 | 1.06% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7346` |
| 14 | 0.94% | `java.util.HashMap.getNode(Object) line: 586` |
| 15 | 0.93% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 5267` |
| 16 | 0.92% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$hasPlayers() line: 1347` |
| 17 | 0.86% | `sun.misc.Unsafe.putInt(Object, long, int) line: 210` |
| 18 | 0.83% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 19 | 0.81% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 20 | 0.76% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 5204` |
| 21 | 0.75% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 22 | 0.74% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 23 | 0.71% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 7543` |
| 24 | 0.67% | `java.util.EnumMap.get(Object) line: 248` |
| 25 | 0.67% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3707` |
| 26 | 0.66% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 27 | 0.66% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 28 | 0.65% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 62` |
| 29 | 0.64% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 285` |
| 30 | 0.61% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 8513` |
| 31 | 0.60% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 32 | 0.59% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 33 | 0.55% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 34 | 0.54% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 7721` |
| 35 | 0.52% | `net.minecraft.world.phys.Vec3.<init>(double, double, double) line: 74` |
| 36 | 0.50% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7968` |
| 37 | 0.50% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3460` |
| 38 | 0.48% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 39 | 0.45% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 40 | 0.45% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 5513` |
| 41 | 0.44% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 42 | 0.43% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 43 | 0.43% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 44 | 0.41% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |
| 45 | 0.41% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 46 | 0.41% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 47 | 0.41% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 48 | 0.40% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3596` |
| 49 | 0.40% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 237` |
| 50 | 0.37% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 51 | 0.37% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 5581` |
| 52 | 0.34% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 248` |
| 53 | 0.34% | `net.minecraft.world.entity.ai.goal.GoalSelector.getAvailableGoals() line: 134` |
| 54 | 0.33% | `net.minecraft.util.SimpleBitStorage.get(int) line: 348` |
| 55 | 0.30% | `net.minecraft.server.level.ServerChunkCache$MainThreadExecutor.pollTask() line: 853` |
| 56 | 0.30% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1376` |
| 57 | 0.30% | `dev.planeta9091.centities.CeHooks.aiShadowPeek(Object) line: 7570` |
| 58 | 0.29% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.getAndMoveToFrontDecode() line: 742` |
| 59 | 0.29% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 60 | 0.29% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 2.44% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 2 | 2.17% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 3 | 2.16% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 2.15% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 5 | 1.91% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 12072` |
| 6 | 1.86% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 7 | 1.70% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7983` |
| 8 | 1.52% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.get(long) line: 264` |
| 9 | 1.52% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7968` |
| 10 | 1.50% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3739` |
| 11 | 1.47% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2683` |
| 12 | 1.44% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 13 | 1.35% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 14 | 1.34% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 15 | 1.22% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 16 | 1.21% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 17 | 1.11% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 18 | 1.07% | `net.minecraft.world.level.chunk.LevelChunk.getBlockState(BlockPos) line: 295` |
| 19 | 0.95% | `java.util.HashMap.get(Object) line: 565` |
| 20 | 0.92% | `dev.planeta9091.centities.CeHooks.mobTravelNative(LivingEntity, Vec3) line: 5539` |
| 21 | 0.91% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 22 | 0.89% | `net.minecraft.world.entity.Mob.aiStep() line: 538` |
| 23 | 0.84% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 24 | 0.83% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7889` |
| 25 | 0.78% | `dev.planeta9091.centities.CeHooks.pushDispatch(Entity) line: 4351` |
| 26 | 0.76% | `net.minecraft.world.entity.Entity.push(double, double, double, Entity) line: 2515` |
| 27 | 0.75% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |
| 28 | 0.68% | `dev.planeta9091.centities.CeHooks.goalSelectorTickRunning(GoalSelector, boolean) line: 8059` |
| 29 | 0.68% | `net.minecraft.world.entity.ai.sensing.Sensing.tick()` |
| 30 | 0.67% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.BZip2CompressorInputStream.initBlock() line: 326` |

GC pauses: 0, total 0 ms

