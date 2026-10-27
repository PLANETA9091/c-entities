# JVM hotspots (JFR ExecutionSample)

Samples: 25709

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.25% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.74% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 7695` |
| 3 | 3.17% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 4 | 2.90% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7554` |
| 5 | 2.70% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.63% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4614` |
| 7 | 2.46% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7501` |
| 8 | 2.24% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4469` |
| 9 | 2.09% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 10 | 1.87% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 11 | 1.64% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 12 | 1.57% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 1.44% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 14 | 1.42% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 15 | 1.23% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 16 | 1.14% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 17 | 1.10% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 18 | 1.10% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 19 | 0.88% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 20 | 0.87% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2601` |
| 21 | 0.86% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 22 | 0.85% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 23 | 0.83% | `java.util.EnumMap.get(Object) line: 248` |
| 24 | 0.81% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3905` |
| 25 | 0.70% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 26 | 0.64% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 27 | 0.63% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 28 | 0.61% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 29 | 0.60% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 30 | 0.59% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 4792` |
| 31 | 0.58% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 32 | 0.56% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1778` |
| 33 | 0.54% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 34 | 0.53% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 35 | 0.53% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 36 | 0.53% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 37 | 0.49% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 38 | 0.49% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 39 | 0.49% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 40 | 0.48% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 7640` |
| 41 | 0.47% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 42 | 0.46% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 5137` |
| 43 | 0.46% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 44 | 0.46% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7515` |
| 45 | 0.44% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 46 | 0.42% | `net.minecraft.world.entity.Entity.computeSpeed() line: 961` |
| 47 | 0.41% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.values() line: 715` |
| 48 | 0.40% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 1797` |
| 49 | 0.40% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2503` |
| 50 | 0.39% | `com.google.common.collect.MapMakerInternalMap$Segment.getFirst(int) line: 1408` |
| 51 | 0.38% | `net.minecraft.world.entity.Entity.isRemoved() line: 5329` |
| 52 | 0.38% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 53 | 0.38% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 54 | 0.36% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 55 | 0.36% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 56 | 0.35% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 57 | 0.35% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 58 | 0.35% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 59 | 0.34% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 60 | 0.34% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 8.42% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.75% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 3 | 3.21% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 3.14% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7550` |
| 5 | 3.02% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 6 | 2.27% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 7 | 2.08% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 8 | 2.05% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 9 | 1.87% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 10 | 1.68% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 11 | 1.64% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 12 | 1.52% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 13 | 1.47% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1320` |
| 14 | 1.44% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 15 | 1.42% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 16 | 1.16% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 17 | 1.15% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 18 | 1.15% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 19 | 1.10% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 20 | 1.08% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 3942` |
| 21 | 1.05% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 22 | 0.96% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 898` |
| 23 | 0.91% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 2524` |
| 24 | 0.89% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 25 | 0.86% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7542` |
| 26 | 0.81% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 5079` |
| 27 | 0.79% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 4616` |
| 28 | 0.79% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 29 | 0.77% | `net.minecraft.world.entity.LivingEntity.detectEquipmentUpdates() line: 3558` |
| 30 | 0.77% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 5508` |

GC pauses: 0, total 0 ms

