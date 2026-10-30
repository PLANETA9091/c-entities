# JVM hotspots (JFR ExecutionSample)

Samples: 34481

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.48% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 2 | 3.37% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8840` |
| 3 | 3.24% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8699` |
| 4 | 2.65% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.54% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 6 | 2.41% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.22% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5735` |
| 8 | 2.04% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 1.64% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8646` |
| 10 | 1.58% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4944` |
| 11 | 1.37% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 12 | 1.31% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 1.20% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5590` |
| 14 | 1.13% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 15 | 1.02% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 16 | 1.01% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 17 | 1.00% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 18 | 0.95% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 389` |
| 19 | 0.92% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 20 | 0.83% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 21 | 0.81% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 22 | 0.81% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 23 | 0.77% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 24 | 0.77% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 25 | 0.74% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 26 | 0.71% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 27 | 0.70% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 28 | 0.67% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 29 | 0.65% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 30 | 0.60% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 31 | 0.59% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8785` |
| 32 | 0.58% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 33 | 0.58% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 34 | 0.56% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 35 | 0.56% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 36 | 0.55% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 37 | 0.48% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 38 | 0.47% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 39 | 0.45% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8660` |
| 40 | 0.44% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 41 | 0.41% | `java.util.EnumMap.get(Object) line: 248` |
| 42 | 0.40% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 43 | 0.40% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 44 | 0.39% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 45 | 0.39% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 46 | 0.38% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3991` |
| 47 | 0.38% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 48 | 0.37% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 49 | 0.37% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 50 | 0.36% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3645` |
| 51 | 0.36% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 52 | 0.36% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 53 | 0.35% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 54 | 0.34% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2601` |
| 55 | 0.33% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 56 | 0.32% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 57 | 0.32% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 58 | 0.31% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 59 | 0.31% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 6752` |
| 60 | 0.30% | `net.minecraft.world.entity.Entity.level() line: 5416` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 3.54% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.38% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 3 | 3.33% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 2.63% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 5 | 2.54% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 6 | 2.49% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 7 | 2.20% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8695` |
| 8 | 2.04% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 1.76% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 10 | 1.61% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 11 | 1.32% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 12 | 1.30% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8687` |
| 13 | 1.24% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 14 | 1.00% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 15 | 0.99% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 16 | 0.95% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5045` |
| 17 | 0.95% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2597` |
| 18 | 0.92% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 19 | 0.85% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 20 | 0.82% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 21 | 0.81% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 22 | 0.79% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 23 | 0.78% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 24 | 0.78% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 25 | 0.73% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 26 | 0.71% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 27 | 0.68% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 125` |
| 28 | 0.67% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |
| 29 | 0.63% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 30 | 0.63% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3584` |

GC pauses: 0, total 0 ms

