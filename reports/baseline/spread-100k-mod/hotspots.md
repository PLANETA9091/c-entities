# JVM hotspots (JFR ExecutionSample)

Samples: 32133

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.31% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8558` |
| 2 | 3.21% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8699` |
| 3 | 2.91% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5594` |
| 4 | 2.78% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.69% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 2.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 7 | 2.27% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 8 | 2.02% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 2.01% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8505` |
| 10 | 1.82% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.66% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5449` |
| 12 | 1.62% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 13 | 1.48% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4803` |
| 14 | 1.33% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 15 | 1.18% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 16 | 1.13% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 17 | 1.06% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 18 | 1.04% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 19 | 1.01% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 20 | 0.91% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 21 | 0.90% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 22 | 0.83% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 23 | 0.81% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 24 | 0.80% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 374` |
| 25 | 0.78% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 26 | 0.77% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 27 | 0.77% | `java.util.concurrent.CompletableFuture.isDone() line: 2076` |
| 28 | 0.75% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 29 | 0.68% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 30 | 0.64% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 31 | 0.64% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 32 | 0.63% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 33 | 0.63% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 34 | 0.61% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 35 | 0.58% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 36 | 0.55% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 37 | 0.54% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 38 | 0.54% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 39 | 0.51% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8519` |
| 40 | 0.49% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 41 | 0.49% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 42 | 0.48% | `java.util.EnumMap.get(Object) line: 248` |
| 43 | 0.48% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 44 | 0.48% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 45 | 0.48% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 46 | 0.45% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 47 | 0.45% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 48 | 0.44% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8644` |
| 49 | 0.44% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 50 | 0.43% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 51 | 0.43% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 52 | 0.42% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 53 | 0.40% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5599` |
| 54 | 0.40% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 55 | 0.40% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3504` |
| 56 | 0.39% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 57 | 0.39% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 58 | 0.38% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 3850` |
| 59 | 0.37% | `java.lang.ref.Reference.get() line: 361` |
| 60 | 0.37% | `net.minecraft.core.Vec3i.getX() line: 63` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.51% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.58% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 3.42% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 3.22% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 5 | 2.70% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8554` |
| 6 | 2.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 7 | 2.02% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 1.83% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 9 | 1.70% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 10 | 1.61% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 11 | 1.48% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8571` |
| 12 | 1.45% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 13 | 1.33% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 14 | 1.26% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8546` |
| 15 | 1.15% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4904` |
| 16 | 1.07% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 17 | 1.01% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 18 | 0.99% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 19 | 0.87% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 20 | 0.80% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2512` |
| 21 | 0.80% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 195` |
| 22 | 0.78% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 23 | 0.77% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 24 | 0.77% | `net.minecraft.util.thread.BlockableEventLoop.managedBlock(BooleanSupplier) line: 190` |
| 25 | 0.75% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |
| 26 | 0.74% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 27 | 0.72% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 28 | 0.72% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3584` |
| 29 | 0.71% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 30 | 0.71% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |

GC pauses: 0, total 0 ms

