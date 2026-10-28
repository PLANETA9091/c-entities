# JVM hotspots (JFR ExecutionSample)

Samples: 27780

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.38% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.58% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7776` |
| 3 | 3.23% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7635` |
| 4 | 2.80% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4695` |
| 5 | 2.49% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.19% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 2.19% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 2.17% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4550` |
| 9 | 2.11% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7582` |
| 10 | 1.90% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 11 | 1.47% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 12 | 1.30% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 13 | 1.20% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 14 | 1.01% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 15 | 0.99% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 16 | 0.95% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 17 | 0.92% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 18 | 0.92% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 19 | 0.90% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 20 | 0.87% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 21 | 0.86% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 22 | 0.81% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3986` |
| 23 | 0.78% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 24 | 0.76% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 25 | 0.75% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 26 | 0.72% | `java.util.EnumMap.get(Object) line: 248` |
| 27 | 0.70% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1876` |
| 28 | 0.68% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 29 | 0.65% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 30 | 0.63% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 31 | 0.61% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2682` |
| 32 | 0.57% | `java.lang.String.equals(Object) line: 1926` |
| 33 | 0.56% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 34 | 0.55% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1860` |
| 35 | 0.55% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 36 | 0.55% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 4873` |
| 37 | 0.54% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 38 | 0.53% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4700` |
| 39 | 0.51% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 40 | 0.51% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 41 | 0.50% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 42 | 0.50% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 43 | 0.49% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 44 | 0.48% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 45 | 0.45% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 46 | 0.44% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 47 | 0.44% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1857` |
| 48 | 0.43% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7596` |
| 49 | 0.42% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 50 | 0.38% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 51 | 0.37% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 52 | 0.36% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7721` |
| 53 | 0.34% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 54 | 0.34% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 55 | 0.34% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 5161` |
| 56 | 0.33% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 57 | 0.33% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 58 | 0.33% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 59 | 0.33% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 60 | 0.32% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.57% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.70% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 3.58% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 4 | 3.35% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 2.68% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7631` |
| 6 | 2.28% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1399` |
| 7 | 2.19% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 8 | 2.19% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 2.14% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 10 | 1.90% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 11 | 1.47% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 12 | 1.29% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 13 | 1.20% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 14 | 1.20% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 15 | 1.15% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 16 | 1.08% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 17 | 1.06% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 18 | 0.99% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4023` |
| 19 | 0.96% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 20 | 0.95% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 21 | 0.90% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1861` |
| 22 | 0.80% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 5160` |
| 23 | 0.80% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7623` |
| 24 | 0.79% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 5589` |
| 25 | 0.76% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 26 | 0.75% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 898` |
| 27 | 0.74% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2195` |
| 28 | 0.71% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4697` |
| 29 | 0.67% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2605` |
| 30 | 0.67% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |

GC pauses: 0, total 0 ms

