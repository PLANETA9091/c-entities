# JVM hotspots (JFR ExecutionSample)

Samples: 39531

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.15% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.get(VarHandle, Object) line: 92` |
| 2 | 3.54% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.26% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 3099` |
| 4 | 2.74% | `dev.planeta9091.centities.CeHooks.worldId(Level) line: 1467` |
| 5 | 2.73% | `net.minecraft.world.entity.Entity.moonrise$getTrackedEntity() line: 542` |
| 6 | 2.65% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 2962` |
| 7 | 2.27% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 2972` |
| 8 | 2.09% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 9 | 1.77% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.42% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1419` |
| 11 | 1.42% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 12 | 1.32% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 13 | 1.26% | `net.minecraft.world.entity.LivingEntity.tickEffects() line: 947` |
| 14 | 1.26% | `dev.planeta9091.centities.CeHooks.trackerTickNoOp(Object, Object) line: 3633` |
| 15 | 1.21% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 16 | 1.18% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1403` |
| 17 | 1.15% | `net.minecraft.world.entity.Entity.setSharedFlag(int, boolean) line: 3868` |
| 18 | 1.06% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3578` |
| 19 | 1.05% | `java.util.EnumMap.get(Object) line: 248` |
| 20 | 0.97% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3696` |
| 21 | 0.90% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.flushStep() line: 70` |
| 22 | 0.89% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2403` |
| 23 | 0.82% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 24 | 0.80% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 25 | 0.79% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 26 | 0.79% | `jdk.internal.reflect.MethodHandleObjectFieldAccessorImpl.get(Object) line: 57` |
| 27 | 0.75% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 28 | 0.72% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 29 | 0.71% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 3332` |
| 30 | 0.71% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 31 | 0.68% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 32 | 0.67% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 33 | 0.66% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 34 | 0.65% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1387` |
| 35 | 0.63% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1384` |
| 36 | 0.62% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 37 | 0.61% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 38 | 0.60% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 39 | 0.50% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 40 | 0.49% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4706` |
| 41 | 0.46% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2062` |
| 42 | 0.45% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 43 | 0.43% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 44 | 0.40% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |
| 45 | 0.40% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 46 | 0.39% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 47 | 0.39% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 48 | 0.38% | `java.lang.invoke.VarHandleReferences$Array.getVolatile(VarHandle, Object, int) line: 603` |
| 49 | 0.37% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 50 | 0.37% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 51 | 0.37% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 52 | 0.37% | `net.minecraft.world.entity.Entity.isRemoved() line: 5328` |
| 53 | 0.37% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 54 | 0.36% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 55 | 0.36% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 56 | 0.34% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 57 | 0.33% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 58 | 0.33% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 59 | 0.32% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 60 | 0.32% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.31% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 5.10% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 3 | 4.21% | `dev.planeta9091.centities.CeHooks$RefField.getObj(Object) line: 2218` |
| 4 | 3.36% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 3.07% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1047` |
| 6 | 2.73% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1103` |
| 7 | 2.55% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity) line: 599` |
| 8 | 1.60% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 9 | 1.56% | `net.minecraft.server.level.ChunkMap.newTrackerTick() line: 1110` |
| 10 | 1.42% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 11 | 1.34% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 12 | 1.31% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 13 | 1.29% | `net.minecraft.world.entity.LivingEntity.baseTick() line: 532` |
| 14 | 1.27% | `net.minecraft.server.level.ChunkMap$TrackedEntity.moonrise$tick(NearbyPlayers$TrackedChunk)` |
| 15 | 1.24% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 16 | 1.19% | `net.minecraft.world.entity.InsideBlockEffectApplier$StepBasedCollector.applyAndClear(Entity) line: 54` |
| 17 | 1.18% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1388` |
| 18 | 1.11% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 19 | 1.10% | `net.minecraft.world.entity.Entity.setSharedFlagOnFire(boolean) line: 969` |
| 20 | 1.07% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3685` |
| 21 | 0.92% | `net.minecraft.world.entity.Entity.setOldPos() line: 2395` |
| 22 | 0.87% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 23 | 0.84% | `java.lang.reflect.Field.get(Object) line: 440` |
| 24 | 0.82% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 25 | 0.80% | `com.google.common.collect.MapMakerInternalMap$Segment.getEntry(Object, int) line: 1415` |
| 26 | 0.75% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 27 | 0.75% | `net.minecraft.world.entity.LivingEntity.tick() line: 3432` |
| 28 | 0.74% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2212` |
| 29 | 0.72% | `net.minecraft.world.level.Level.guardEntityTick(Consumer, Entity) line: 1532` |
| 30 | 0.69% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 3332` |

GC pauses: 0, total 0 ms

