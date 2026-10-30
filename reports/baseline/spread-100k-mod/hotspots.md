# JVM hotspots (JFR ExecutionSample)

Samples: 37534

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.40% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8657` |
| 2 | 3.28% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 3 | 3.13% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8516` |
| 4 | 3.06% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 5 | 2.05% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5552` |
| 6 | 2.04% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 7 | 1.76% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 8 | 1.67% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8463` |
| 9 | 1.61% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 1.38% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4761` |
| 11 | 1.27% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 12 | 1.24% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 13 | 1.16% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 14 | 1.11% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 374` |
| 15 | 1.11% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5407` |
| 16 | 1.02% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 17 | 0.93% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 18 | 0.91% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 19 | 0.91% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 20 | 0.89% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 21 | 0.86% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 22 | 0.85% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 23 | 0.84% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 24 | 0.82% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 25 | 0.77% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 26 | 0.76% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 27 | 0.71% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 28 | 0.66% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 29 | 0.66% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 30 | 0.65% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 31 | 0.63% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 32 | 0.62% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 33 | 0.60% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8602` |
| 34 | 0.57% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 35 | 0.56% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 36 | 0.52% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 37 | 0.52% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 38 | 0.52% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 39 | 0.51% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 40 | 0.50% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 41 | 0.47% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 42 | 0.47% | `java.util.EnumMap.get(Object) line: 248` |
| 43 | 0.43% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 44 | 0.40% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 45 | 0.38% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8477` |
| 46 | 0.37% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 47 | 0.37% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2474` |
| 48 | 0.36% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 49 | 0.35% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 50 | 0.34% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 51 | 0.33% | `java.lang.ref.Reference.get() line: 361` |
| 52 | 0.33% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3462` |
| 53 | 0.32% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 54 | 0.32% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 55 | 0.32% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 56 | 0.31% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3808` |
| 57 | 0.31% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 58 | 0.30% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 59 | 0.30% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 60 | 0.30% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 3.41% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 2 | 3.25% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 3.16% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 4 | 2.78% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 2.33% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 6 | 2.17% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8512` |
| 7 | 1.71% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8504` |
| 8 | 1.61% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8529` |
| 9 | 1.61% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 10 | 1.51% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 11 | 1.49% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 12 | 1.41% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 13 | 1.15% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 14 | 1.14% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 15 | 1.12% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2470` |
| 16 | 1.08% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 17 | 0.90% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 18 | 0.90% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4862` |
| 19 | 0.86% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 20 | 0.84% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1935` |
| 21 | 0.80% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 22 | 0.79% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |
| 23 | 0.79% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 24 | 0.79% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 25 | 0.78% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 26 | 0.78% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 125` |
| 27 | 0.76% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 28 | 0.75% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 29 | 0.68% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 30 | 0.63% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2487` |

GC pauses: 0, total 0 ms

