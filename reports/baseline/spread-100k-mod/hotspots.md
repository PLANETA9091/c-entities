# JVM hotspots (JFR ExecutionSample)

Samples: 35793

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.63% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 2 | 3.23% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8242` |
| 3 | 3.07% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8383` |
| 4 | 2.84% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.65% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 6 | 2.27% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.03% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5278` |
| 8 | 1.68% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 1.65% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8189` |
| 10 | 1.32% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4560` |
| 11 | 1.32% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 12 | 1.28% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 13 | 1.22% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 14 | 1.20% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 15 | 1.16% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5133` |
| 16 | 1.04% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 17 | 0.85% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 18 | 0.80% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 19 | 0.79% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 20 | 0.74% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 21 | 0.72% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 22 | 0.71% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 23 | 0.68% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 24 | 0.68% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 25 | 0.68% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 26 | 0.66% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 27 | 0.63% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 28 | 0.63% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 29 | 0.62% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 30 | 0.61% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 31 | 0.56% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8328` |
| 32 | 0.55% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 33 | 0.52% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 34 | 0.51% | `java.util.EnumMap.get(Object) line: 248` |
| 35 | 0.50% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 36 | 0.49% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 37 | 0.47% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 38 | 0.45% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8203` |
| 39 | 0.44% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 40 | 0.44% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 41 | 0.44% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 42 | 0.43% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 43 | 0.40% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 44 | 0.39% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 45 | 0.39% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 46 | 0.36% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 47 | 0.36% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 48 | 0.36% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 49 | 0.35% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5283` |
| 50 | 0.35% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 51 | 0.34% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 6295` |
| 52 | 0.34% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 53 | 0.34% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3255` |
| 54 | 0.34% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2336` |
| 55 | 0.33% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 56 | 0.33% | `java.lang.ref.Reference.get() line: 361` |
| 57 | 0.32% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 58 | 0.32% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 59 | 0.31% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3601` |
| 60 | 0.30% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2317` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 3.48% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.35% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 3.07% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 4 | 2.74% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 5 | 2.60% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 6 | 2.57% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 7 | 2.20% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8238` |
| 8 | 1.70% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 9 | 1.68% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 10 | 1.67% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 11 | 1.41% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 12 | 1.19% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 13 | 1.19% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8230` |
| 14 | 1.07% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1809` |
| 15 | 0.93% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4597` |
| 16 | 0.92% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 17 | 0.91% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 18 | 0.90% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 19 | 0.87% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 20 | 0.87% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 21 | 0.83% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 22 | 0.82% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 23 | 0.80% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 24 | 0.74% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |
| 25 | 0.71% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 26 | 0.71% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 27 | 0.70% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 28 | 0.69% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 29 | 0.69% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 30 | 0.69% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |

GC pauses: 0, total 0 ms

