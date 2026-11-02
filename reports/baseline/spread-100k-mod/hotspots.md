# JVM hotspots (JFR ExecutionSample)

Samples: 38013

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.64% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9207` |
| 2 | 3.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 3 | 3.25% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9543` |
| 4 | 3.04% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.40% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 2.29% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6166` |
| 7 | 1.81% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9121` |
| 8 | 1.57% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5427` |
| 9 | 1.29% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 1.27% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 6012` |
| 11 | 1.22% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 1.18% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 13 | 1.14% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 14 | 1.06% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 15 | 1.04% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 16 | 1.03% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 17 | 1.01% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 18 | 0.99% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 19 | 0.94% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 20 | 0.90% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 21 | 0.86% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 22 | 0.85% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 23 | 0.71% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 24 | 0.69% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 25 | 0.64% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 26 | 0.64% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 27 | 0.63% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 28 | 0.63% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 29 | 0.63% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 30 | 0.62% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 31 | 0.61% | `java.util.EnumMap.get(Object) line: 248` |
| 32 | 0.61% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 33 | 0.60% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 34 | 0.59% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 35 | 0.58% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 36 | 0.54% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 37 | 0.50% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 38 | 0.49% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 9166` |
| 39 | 0.46% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 40 | 0.43% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 41 | 0.42% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 9488` |
| 42 | 0.41% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 43 | 0.40% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 44 | 0.40% | `net.minecraft.world.entity.SteppedInterpolationTracker.updateTracking(Vec3) line: 40` |
| 45 | 0.40% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 46 | 0.39% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 47 | 0.39% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2742` |
| 48 | 0.37% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 49 | 0.37% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 50 | 0.37% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 320` |
| 51 | 0.36% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 52 | 0.35% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 53 | 0.35% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 54 | 0.33% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 55 | 0.33% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4020` |
| 56 | 0.32% | `java.lang.ref.Reference.get() line: 361` |
| 57 | 0.31% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 58 | 0.31% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 59 | 0.31% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 60 | 0.31% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 151` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 3.81% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 2 | 3.54% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 3 | 3.44% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 3.26% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 5 | 2.52% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 6 | 2.51% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9203` |
| 7 | 1.95% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 8 | 1.64% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 9 | 1.59% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9220` |
| 10 | 1.44% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9195` |
| 11 | 1.30% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 12 | 1.29% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 13 | 1.29% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 14 | 1.25% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2086` |
| 15 | 1.02% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 16 | 1.01% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 17 | 0.99% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 18 | 0.90% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |
| 19 | 0.90% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5465` |
| 20 | 0.89% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 21 | 0.78% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 22 | 0.73% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 23 | 0.73% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 125` |
| 24 | 0.70% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 25 | 0.68% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 26 | 0.66% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 27 | 0.66% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 28 | 0.66% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 29 | 0.66% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 30 | 0.63% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |

GC pauses: 0, total 0 ms

