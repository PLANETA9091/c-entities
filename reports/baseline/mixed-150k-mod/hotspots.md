# JVM hotspots (JFR ExecutionSample)

Samples: 28275

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.99% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 2 | 3.61% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8729` |
| 3 | 3.55% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8535` |
| 4 | 3.34% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 3.04% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8588` |
| 6 | 2.95% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5624` |
| 7 | 2.51% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 2.03% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4833` |
| 9 | 1.47% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 10 | 1.46% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5479` |
| 11 | 1.42% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 12 | 1.25% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 13 | 1.14% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 14 | 1.14% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 15 | 0.95% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 16 | 0.90% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 374` |
| 17 | 0.89% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 18 | 0.85% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 19 | 0.83% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 20 | 0.76% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5626` |
| 21 | 0.74% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 22 | 0.73% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 23 | 0.70% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 24 | 0.65% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 25 | 0.60% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 26 | 0.59% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8674` |
| 27 | 0.59% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 28 | 0.58% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 29 | 0.57% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 30 | 0.57% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 31 | 0.54% | `dev.planeta9091.centities.CeHooks.probeDeltaCommon(Entity, double, double, double) line: 1106` |
| 32 | 0.52% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 33 | 0.50% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 34 | 0.50% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 35 | 0.49% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 36 | 0.47% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 37 | 0.46% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 38 | 0.46% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3534` |
| 39 | 0.45% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 40 | 0.44% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 41 | 0.41% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 42 | 0.41% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 43 | 0.40% | `java.util.EnumMap.get(Object) line: 248` |
| 44 | 0.40% | `java.lang.ref.Reference.get() line: 361` |
| 45 | 0.40% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getAttributesToSync() line: 34` |
| 46 | 0.39% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 47 | 0.38% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 48 | 0.38% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 49 | 0.37% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 50 | 0.36% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 51 | 0.35% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 52 | 0.35% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 53 | 0.34% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 54 | 0.34% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4934` |
| 55 | 0.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 56 | 0.33% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 57 | 0.33% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 58 | 0.32% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 59 | 0.32% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 60 | 0.32% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.06% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 2 | 3.86% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8584` |
| 3 | 3.65% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 3.62% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 5 | 3.26% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 6 | 2.53% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 7 | 2.48% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 8 | 2.01% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 9 | 1.80% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 10 | 1.76% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8576` |
| 11 | 1.67% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4934` |
| 12 | 1.49% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 13 | 1.46% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 14 | 1.28% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 15 | 0.95% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 16 | 0.91% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 17 | 0.85% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 18 | 0.84% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 19 | 0.84% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 20 | 0.82% | `dev.planeta9091.centities.CeHooks.probeDelta(Entity, Vec3) line: 1060` |
| 21 | 0.76% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 22 | 0.73% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 23 | 0.71% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2155` |
| 24 | 0.70% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3584` |
| 25 | 0.68% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 26 | 0.65% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 6518` |
| 27 | 0.63% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 28 | 0.60% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 29 | 0.60% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 30 | 0.59% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5626` |

GC pauses: 0, total 0 ms

