# JVM hotspots (JFR ExecutionSample)

Samples: 29901

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.95% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4571` |
| 2 | 3.94% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.50% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7511` |
| 4 | 2.90% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.45% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 6 | 2.19% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 2.17% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 8 | 2.15% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7458` |
| 9 | 1.99% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 10 | 1.78% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4426` |
| 11 | 1.56% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 12 | 1.42% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 13 | 1.38% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 14 | 1.29% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 15 | 1.17% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 16 | 1.16% | `java.util.EnumMap$EnumMapIterator.hasNext() line: 524` |
| 17 | 0.94% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 18 | 0.91% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 19 | 0.90% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 20 | 0.89% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 21 | 0.81% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 22 | 0.81% | `java.util.EnumMap.get(Object) line: 248` |
| 23 | 0.81% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1764` |
| 24 | 0.79% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 25 | 0.77% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 26 | 0.77% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 27 | 0.72% | `java.lang.ref.Reference.get() line: 361` |
| 28 | 0.71% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 29 | 0.66% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 30 | 0.64% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 31 | 0.64% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 32 | 0.63% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 33 | 0.62% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 34 | 0.61% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 35 | 0.59% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 36 | 0.59% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 37 | 0.59% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 38 | 0.55% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1748` |
| 39 | 0.54% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 40 | 0.50% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 41 | 0.47% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 42 | 0.46% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7472` |
| 43 | 0.46% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 44 | 0.45% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2430` |
| 45 | 0.43% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 46 | 0.43% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 47 | 0.42% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 48 | 0.41% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 49 | 0.41% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 50 | 0.40% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 51 | 0.39% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 52 | 0.39% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1745` |
| 53 | 0.38% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 54 | 0.36% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 55 | 0.36% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 56 | 0.35% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 57 | 0.35% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7597` |
| 58 | 0.34% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 59 | 0.33% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 60 | 0.31% | `dev.planeta9091.centities.CeHooks.aiPackSel(int[], int, WrappedGoal[], int[], int[], long[], int[], int[], int[], float[], Mob) line: 6545` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.11% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.27% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 3.64% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 2.79% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7507` |
| 5 | 2.50% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 6 | 2.33% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1316` |
| 7 | 2.19% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 1.99% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 9 | 1.99% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 10 | 1.80% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 11 | 1.57% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 12 | 1.38% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4573` |
| 13 | 1.28% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1749` |
| 14 | 1.27% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 15 | 1.26% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 16 | 1.22% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7499` |
| 17 | 1.21% | `net.minecraft.world.entity.EntityEquipment.tick(Entity) line: 50` |
| 18 | 1.11% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 19 | 1.10% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 20 | 0.94% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 21 | 0.92% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 22 | 0.92% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 23 | 0.91% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |
| 24 | 0.89% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 25 | 0.78% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 26 | 0.77% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 5465` |
| 27 | 0.76% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2195` |
| 28 | 0.75% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 29 | 0.68% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 30 | 0.68% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 898` |

GC pauses: 0, total 0 ms

