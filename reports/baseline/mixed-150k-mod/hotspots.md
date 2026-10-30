# JVM hotspots (JFR ExecutionSample)

Samples: 35658

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.90% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8492` |
| 2 | 3.24% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 2.98% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5528` |
| 4 | 2.78% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 2.50% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 6 | 2.42% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8633` |
| 7 | 2.28% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4737` |
| 8 | 1.85% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8439` |
| 9 | 1.67% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 10 | 1.64% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 11 | 1.58% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5383` |
| 12 | 1.44% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 1.35% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 14 | 1.18% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 15 | 1.07% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 16 | 1.04% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 17 | 0.97% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 18 | 0.96% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 19 | 0.91% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 374` |
| 20 | 0.86% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 21 | 0.82% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 22 | 0.77% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8578` |
| 23 | 0.76% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 24 | 0.71% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 25 | 0.71% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 26 | 0.67% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 27 | 0.66% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8453` |
| 28 | 0.62% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 29 | 0.61% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 30 | 0.59% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 31 | 0.59% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 32 | 0.56% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 33 | 0.55% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 34 | 0.54% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 35 | 0.50% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 36 | 0.49% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 37 | 0.47% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 38 | 0.47% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 39 | 0.45% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 40 | 0.45% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 41 | 0.42% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3438` |
| 42 | 0.42% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 43 | 0.42% | `net.minecraft.world.entity.SteppedInterpolationTracker.updateTracking(Vec3) line: 40` |
| 44 | 0.41% | `java.util.EnumMap.get(Object) line: 248` |
| 45 | 0.40% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 46 | 0.39% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 47 | 0.38% | `java.util.ArrayList.add(Object, Object[], int) line: 483` |
| 48 | 0.37% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 49 | 0.36% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 50 | 0.36% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 51 | 0.36% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 52 | 0.36% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 53 | 0.34% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 54 | 0.34% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 55 | 0.33% | `java.lang.ref.Reference.get() line: 361` |
| 56 | 0.29% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 57 | 0.29% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 58 | 0.29% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 59 | 0.29% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 60 | 0.29% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3784` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.22% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.02% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 3.36% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 2.69% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8488` |
| 5 | 2.61% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 6 | 2.43% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 7 | 1.94% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 8 | 1.82% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8505` |
| 9 | 1.64% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 10 | 1.63% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 11 | 1.61% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 12 | 1.55% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8480` |
| 13 | 1.49% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 14 | 1.38% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4838` |
| 15 | 1.25% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 16 | 1.10% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 17 | 1.10% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 18 | 0.96% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 19 | 0.78% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 20 | 0.77% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3584` |
| 21 | 0.75% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 22 | 0.75% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 23 | 0.73% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2092` |
| 24 | 0.73% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 25 | 0.72% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 26 | 0.71% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 27 | 0.70% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 28 | 0.68% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |
| 29 | 0.67% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 30 | 0.67% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |

GC pauses: 0, total 0 ms

