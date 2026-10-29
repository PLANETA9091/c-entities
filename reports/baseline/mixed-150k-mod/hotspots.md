# JVM hotspots (JFR ExecutionSample)

Samples: 44966

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.53% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.53% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8056` |
| 3 | 3.28% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 2.68% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5092` |
| 5 | 2.62% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 2.49% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8197` |
| 7 | 1.84% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 1.62% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4374` |
| 9 | 1.47% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8003` |
| 10 | 1.28% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.24% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 12 | 1.20% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4947` |
| 13 | 1.07% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 14 | 0.91% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 15 | 0.86% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 16 | 0.82% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 17 | 0.82% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 18 | 0.78% | `java.util.EnumMap.get(Object) line: 248` |
| 19 | 0.75% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 20 | 0.75% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 21 | 0.73% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 22 | 0.68% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 23 | 0.64% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 24 | 0.63% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 25 | 0.61% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8142` |
| 26 | 0.60% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 27 | 0.57% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 28 | 0.56% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 29 | 0.56% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 30 | 0.56% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 31 | 0.54% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 32 | 0.53% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 993` |
| 33 | 0.52% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 34 | 0.52% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 35 | 0.51% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 36 | 0.48% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 7699` |
| 37 | 0.45% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8017` |
| 38 | 0.45% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 39 | 0.45% | `net.minecraft.world.entity.SteppedInterpolationTracker.updateTracking(Vec3) line: 40` |
| 40 | 0.44% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 41 | 0.43% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 42 | 0.41% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 43 | 0.39% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 44 | 0.39% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 45 | 0.38% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 46 | 0.36% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 47 | 0.36% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 48 | 0.35% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 49 | 0.35% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 50 | 0.35% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 51 | 0.34% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 52 | 0.34% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 53 | 0.34% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 54 | 0.32% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 55 | 0.32% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 56 | 0.32% | `net.minecraft.util.SimpleBitStorage.get(int) line: 350` |
| 57 | 0.31% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 58 | 0.30% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 59 | 0.30% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 60 | 0.29% | `dev.planeta9091.centities.CeHooks.setConverting$(Object, boolean) line: 8264` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.53% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 3.90% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 3.64% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 2.99% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 2.50% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 6 | 2.09% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8052` |
| 7 | 2.03% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8044` |
| 8 | 1.98% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 9 | 1.84% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 10 | 1.73% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 11 | 1.38% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8069` |
| 12 | 1.22% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 13 | 1.09% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 14 | 1.01% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 15 | 0.95% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 16 | 0.88% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 17 | 0.85% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 18 | 0.83% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 19 | 0.82% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 20 | 0.81% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 21 | 0.75% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 22 | 0.75% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 23 | 0.71% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 24 | 0.70% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Entity, AABB, List, Predicate) line: 327` |
| 25 | 0.68% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 26 | 0.66% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 27 | 0.63% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 28 | 0.63% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 29 | 0.63% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 30 | 0.61% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |

GC pauses: 0, total 0 ms

