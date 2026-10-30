# JVM hotspots (JFR ExecutionSample)

Samples: 37900

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.83% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 2 | 3.59% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8729` |
| 3 | 3.37% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8588` |
| 4 | 3.19% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 5 | 2.47% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.18% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 2.16% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5624` |
| 8 | 2.06% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 9 | 1.78% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8535` |
| 10 | 1.71% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4833` |
| 11 | 1.30% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 12 | 1.26% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5479` |
| 13 | 1.10% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 14 | 1.06% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 15 | 1.01% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 16 | 1.01% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 17 | 1.01% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 18 | 0.93% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 19 | 0.91% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 20 | 0.91% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 21 | 0.90% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 22 | 0.86% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 23 | 0.78% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 24 | 0.72% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 25 | 0.69% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8674` |
| 26 | 0.68% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 374` |
| 27 | 0.67% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 28 | 0.62% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 29 | 0.61% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 30 | 0.60% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 31 | 0.60% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 32 | 0.58% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 33 | 0.52% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 34 | 0.52% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 35 | 0.51% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8549` |
| 36 | 0.51% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 37 | 0.51% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 38 | 0.50% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 39 | 0.49% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 40 | 0.45% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 41 | 0.42% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 42 | 0.40% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 43 | 0.40% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 44 | 0.39% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 45 | 0.38% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 46 | 0.38% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3880` |
| 47 | 0.37% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 48 | 0.37% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 49 | 0.35% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 50 | 0.34% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 51 | 0.33% | `java.lang.ref.Reference.get() line: 361` |
| 52 | 0.32% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 53 | 0.31% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3534` |
| 54 | 0.31% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 55 | 0.31% | `java.util.EnumMap.get(Object) line: 248` |
| 56 | 0.31% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 57 | 0.30% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 58 | 0.30% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 59 | 0.30% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 60 | 0.28% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 3.59% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 2 | 3.50% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 3.29% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 4 | 3.16% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 5 | 2.97% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 6 | 2.45% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8584` |
| 7 | 2.44% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 8 | 2.18% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 1.90% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 10 | 1.45% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 11 | 1.30% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 12 | 1.29% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 13 | 1.27% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8576` |
| 14 | 1.17% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 15 | 1.11% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 16 | 1.02% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4934` |
| 17 | 1.00% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 18 | 0.86% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 19 | 0.86% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 20 | 0.85% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 21 | 0.78% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 22 | 0.75% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 23 | 0.75% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 24 | 0.75% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 25 | 0.74% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |
| 26 | 0.72% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 27 | 0.70% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 28 | 0.69% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 29 | 0.68% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2516` |
| 30 | 0.63% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |

GC pauses: 0, total 0 ms

