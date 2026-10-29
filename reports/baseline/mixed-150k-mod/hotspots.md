# JVM hotspots (JFR ExecutionSample)

Samples: 40781

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.05% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 3.61% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.31% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5020` |
| 4 | 3.14% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8125` |
| 5 | 3.12% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7984` |
| 6 | 3.02% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 7 | 1.87% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 8 | 1.76% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7931` |
| 9 | 1.71% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 10 | 1.59% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4875` |
| 11 | 1.47% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 12 | 1.36% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4302` |
| 13 | 1.24% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 14 | 1.11% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 15 | 0.99% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 16 | 0.99% | `java.util.EnumMap.get(Object) line: 248` |
| 17 | 0.97% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 18 | 0.95% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 19 | 0.95% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 20 | 0.92% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 21 | 0.92% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 22 | 0.90% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 23 | 0.82% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 24 | 0.77% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 25 | 0.75% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 26 | 0.71% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 27 | 0.69% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 28 | 0.68% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 29 | 0.65% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 30 | 0.63% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 993` |
| 31 | 0.61% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8070` |
| 32 | 0.60% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 33 | 0.56% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 34 | 0.54% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 35 | 0.53% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 36 | 0.53% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3585` |
| 37 | 0.53% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 38 | 0.52% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 39 | 0.51% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 40 | 0.50% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 41 | 0.49% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 42 | 0.48% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 7945` |
| 43 | 0.48% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 44 | 0.48% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 45 | 0.46% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 46 | 0.45% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 47 | 0.43% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 48 | 0.43% | `java.lang.ref.Reference.get() line: 361` |
| 49 | 0.42% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 2997` |
| 50 | 0.41% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 51 | 0.41% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 52 | 0.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 53 | 0.38% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 54 | 0.37% | `java.util.EnumMap.isValidKey(Object) line: 320` |
| 55 | 0.37% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 56 | 0.36% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 6037` |
| 57 | 0.35% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 58 | 0.34% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.NewChunkHolder$ChunkCompletion.chunk() line: 464` |
| 59 | 0.34% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 60 | 0.34% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.32% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.05% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 3 | 3.67% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 3.19% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 3.14% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 6 | 2.39% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7980` |
| 7 | 1.87% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 8 | 1.75% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 9 | 1.65% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 10 | 1.62% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 11 | 1.48% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 12 | 1.40% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7997` |
| 13 | 1.35% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 14 | 1.29% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 15 | 1.27% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 7972` |
| 16 | 1.11% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 17 | 1.05% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4339` |
| 18 | 0.99% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 19 | 0.98% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 20 | 0.90% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 21 | 0.83% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 22 | 0.82% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 23 | 0.82% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 24 | 0.82% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 25 | 0.75% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 26 | 0.72% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 27 | 0.68% | `net.minecraft.world.entity.LivingEntity.detectEquipmentUpdates() line: 3558` |
| 28 | 0.67% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 29 | 0.66% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 30 | 0.64% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |

GC pauses: 0, total 0 ms

