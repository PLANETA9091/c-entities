# JVM hotspots (JFR ExecutionSample)

Samples: 31873

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.76% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.33% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7544` |
| 3 | 3.30% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7685` |
| 4 | 3.22% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4604` |
| 5 | 2.42% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 6 | 2.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 7 | 2.27% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 8 | 2.19% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 9 | 1.85% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.61% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4459` |
| 11 | 1.55% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7491` |
| 12 | 1.17% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 1.08% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 14 | 0.92% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 15 | 0.81% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 16 | 0.78% | `java.util.EnumMap.get(Object) line: 248` |
| 17 | 0.77% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 18 | 0.77% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 19 | 0.71% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 20 | 0.68% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 21 | 0.66% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 22 | 0.63% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 23 | 0.63% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 24 | 0.60% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 25 | 0.58% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 26 | 0.57% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 27 | 0.56% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7505` |
| 28 | 0.56% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 29 | 0.56% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 30 | 0.55% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 31 | 0.53% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 32 | 0.51% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 33 | 0.50% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 34 | 0.48% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 35 | 0.48% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 36 | 0.48% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 37 | 0.46% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 38 | 0.46% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7630` |
| 39 | 0.44% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 40 | 0.43% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 41 | 0.43% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4609` |
| 42 | 0.42% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 43 | 0.42% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2463` |
| 44 | 0.41% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 45 | 0.40% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 4782` |
| 46 | 0.39% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1781` |
| 47 | 0.39% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 48 | 0.37% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 49 | 0.37% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 50 | 0.35% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 51 | 0.35% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1778` |
| 52 | 0.34% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1797` |
| 53 | 0.34% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 54 | 0.33% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 55 | 0.31% | `jdk.internal.util.ArraysSupport.hashCodeOfUnsigned(byte[], int, int, int) line: 275` |
| 56 | 0.31% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 57 | 0.29% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 58 | 0.29% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 59 | 0.29% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 60 | 0.28% | `dev.planeta9091.centities.CeHooks.aiPackSel(int[], int, WrappedGoal[], int[], int[], long[], int[], int[], int[], float[], Mob) line: 6578` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.61% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.83% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 3.44% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 3.30% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 5 | 2.41% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 6 | 2.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 7 | 2.28% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7540` |
| 8 | 2.27% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 1.64% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 10 | 1.51% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 11 | 1.47% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1320` |
| 12 | 1.37% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4606` |
| 13 | 1.15% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 14 | 1.12% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 15 | 1.08% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 16 | 1.02% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 17 | 0.99% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 18 | 0.98% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 19 | 0.83% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7532` |
| 20 | 0.75% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 21 | 0.70% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 22 | 0.69% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 23 | 0.69% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 5069` |
| 24 | 0.66% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 898` |
| 25 | 0.66% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 26 | 0.63% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 27 | 0.58% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 5498` |
| 28 | 0.56% | `net.minecraft.world.entity.LivingEntity.detectEquipmentUpdates() line: 3558` |
| 29 | 0.56% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 30 | 0.56% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |

GC pauses: 0, total 0 ms

