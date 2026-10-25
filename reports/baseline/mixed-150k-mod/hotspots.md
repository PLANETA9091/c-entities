# JVM hotspots (JFR ExecutionSample)

Samples: 39963

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.54% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.30% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 3 | 3.14% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 5480` |
| 4 | 2.59% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.55% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3671` |
| 6 | 2.13% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 2.09% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3817` |
| 8 | 1.79% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 9 | 1.62% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 10 | 1.47% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1425` |
| 11 | 1.36% | `dev.planeta9091.centities.CeHooks.setSharedFlag$(Entity, int, boolean) line: 5343` |
| 12 | 1.32% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 13 | 1.28% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 5439` |
| 14 | 1.26% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 15 | 1.21% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 16 | 1.11% | `java.util.EnumMap.get(Object) line: 248` |
| 17 | 1.06% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3681` |
| 18 | 1.05% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1496` |
| 19 | 0.98% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 20 | 0.98% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1409` |
| 21 | 0.91% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4157` |
| 22 | 0.89% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 23 | 0.87% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 24 | 0.84% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 25 | 0.79% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 26 | 0.78% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 27 | 0.74% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 28 | 0.73% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 29 | 0.71% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 30 | 0.68% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 31 | 0.66% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 32 | 0.65% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 33 | 0.63% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 34 | 0.59% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 35 | 0.58% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 36 | 0.56% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1406` |
| 37 | 0.53% | `java.util.EnumMap$EnumMapIterator.hasNext() line: 523` |
| 38 | 0.52% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 39 | 0.51% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 40 | 0.49% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 41 | 0.44% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 42 | 0.43% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 43 | 0.42% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 44 | 0.41% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 45 | 0.40% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 46 | 0.38% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 47 | 0.37% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 48 | 0.37% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 3985` |
| 49 | 0.36% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 50 | 0.36% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 51 | 0.36% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 52 | 0.36% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 53 | 0.35% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 54 | 0.35% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 55 | 0.35% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 56 | 0.35% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 57 | 0.34% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 58 | 0.34% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 59 | 0.33% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 60 | 0.31% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.67% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.81% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1059` |
| 3 | 3.77% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 4 | 3.30% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 5 | 3.14% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 6 | 2.43% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 5439` |
| 7 | 2.32% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 8 | 1.74% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 9 | 1.40% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 10 | 1.39% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean)` |
| 11 | 1.37% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 12 | 1.36% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1410` |
| 13 | 1.31% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 14 | 1.29% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk)` |
| 15 | 1.27% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 16 | 1.26% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 17 | 1.25% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 18 | 1.16% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 19 | 1.11% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3819` |
| 20 | 1.00% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 21 | 0.92% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 22 | 0.87% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 23 | 0.82% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity) line: 603` |
| 24 | 0.81% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 25 | 0.81% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 26 | 0.81% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 27 | 0.80% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 28 | 0.78% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4157` |
| 29 | 0.77% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 30 | 0.66% | `net.minecraft.world.entity.LivingEntity.isAlive() line: 2195` |

GC pauses: 0, total 0 ms

