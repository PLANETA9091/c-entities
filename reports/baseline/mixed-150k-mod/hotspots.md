# JVM hotspots (JFR ExecutionSample)

Samples: 39305

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.09% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.16% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 5454` |
| 3 | 2.87% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 2.39% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 5 | 2.17% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3627` |
| 6 | 1.94% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 1.84% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3791` |
| 8 | 1.69% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 9 | 1.64% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 5413` |
| 10 | 1.41% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 11 | 1.35% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 12 | 1.32% | `dev.planeta9091.centities.CeHooks.setSharedFlag$(Entity, int, boolean) line: 5317` |
| 13 | 1.26% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 14 | 1.23% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 15 | 1.18% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1425` |
| 16 | 1.13% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 3655` |
| 17 | 0.99% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 18 | 0.95% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4131` |
| 19 | 0.91% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 20 | 0.91% | `java.util.EnumMap.get(Object) line: 248` |
| 21 | 0.91% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 22 | 0.90% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 23 | 0.84% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 24 | 0.84% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 25 | 0.71% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1409` |
| 26 | 0.71% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 27 | 0.71% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 28 | 0.70% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1406` |
| 29 | 0.67% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 30 | 0.67% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 31 | 0.63% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 32 | 0.61% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 33 | 0.60% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 34 | 0.58% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 35 | 0.57% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 36 | 0.53% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 37 | 0.50% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 38 | 0.50% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 39 | 0.50% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 40 | 0.46% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 20` |
| 41 | 0.45% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1496` |
| 42 | 0.43% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 43 | 0.42% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 44 | 0.42% | `net.minecraft.world.entity.ai.sensing.Sensing.tick() line: 21` |
| 45 | 0.42% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 46 | 0.41% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 47 | 0.41% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 48 | 0.40% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 49 | 0.38% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 50 | 0.38% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 51 | 0.37% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 52 | 0.36% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 53 | 0.36% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 54 | 0.36% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 55 | 0.35% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 56 | 0.35% | `java.lang.ThreadLocal$ThreadLocalMap.getEntryAfterMiss(ThreadLocal, int, ThreadLocal$ThreadLocalMap$Entry) line: 514` |
| 57 | 0.34% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 3959` |
| 58 | 0.34% | `net.minecraft.world.entity.Entity.computeSpeed() line: 961` |
| 59 | 0.33% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 60 | 0.33% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.48% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.46% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 3 | 3.32% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1059` |
| 4 | 3.16% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 5 | 2.63% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 5413` |
| 6 | 2.14% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 7 | 1.94% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 1.66% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk)` |
| 9 | 1.64% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 10 | 1.57% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 11 | 1.48% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 12 | 1.43% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 13 | 1.35% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 14 | 1.34% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean)` |
| 15 | 1.31% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1410` |
| 16 | 1.28% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3793` |
| 17 | 1.26% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 18 | 1.24% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 19 | 1.14% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 20 | 0.94% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 21 | 0.93% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 22 | 0.92% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 23 | 0.91% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 24 | 0.88% | `net.minecraft.world.entity.Mob.serverAiStep() line: 831` |
| 25 | 0.87% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 26 | 0.81% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 27 | 0.77% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 4131` |
| 28 | 0.67% | `net.minecraft.world.entity.LivingEntity.detectEquipmentUpdates() line: 3558` |
| 29 | 0.60% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 30 | 0.58% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |

GC pauses: 0, total 0 ms

