# JVM hotspots (JFR ExecutionSample)

Samples: 32774

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.20% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 2 | 3.85% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8788` |
| 3 | 3.23% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8929` |
| 4 | 3.07% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.86% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5824` |
| 6 | 2.61% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.14% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8735` |
| 8 | 2.14% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 2.03% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5033` |
| 10 | 1.52% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.44% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5679` |
| 12 | 1.32% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 13 | 1.20% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 14 | 1.18% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 389` |
| 15 | 1.13% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 16 | 1.10% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 17 | 1.03% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 18 | 1.01% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 19 | 0.96% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 20 | 0.89% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 21 | 0.81% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 22 | 0.80% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8874` |
| 23 | 0.75% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 24 | 0.69% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 25 | 0.67% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 26 | 0.62% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 27 | 0.61% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 28 | 0.60% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 29 | 0.59% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 30 | 0.54% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 31 | 0.53% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 32 | 0.53% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 33 | 0.50% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 34 | 0.49% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 35 | 0.48% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 36 | 0.48% | `java.util.EnumMap.get(Object) line: 248` |
| 37 | 0.48% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 38 | 0.43% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 39 | 0.43% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 40 | 0.42% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 41 | 0.40% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8749` |
| 42 | 0.40% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3734` |
| 43 | 0.38% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 44 | 0.37% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 45 | 0.36% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 46 | 0.36% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 47 | 0.36% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 48 | 0.35% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 49 | 0.35% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 50 | 0.33% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 51 | 0.33% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4080` |
| 52 | 0.33% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 53 | 0.32% | `java.util.ImmutableCollections$ListN.get(int) line: 725` |
| 54 | 0.30% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 55 | 0.29% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 56 | 0.29% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 57 | 0.28% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 58 | 0.27% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 59 | 0.27% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 60 | 0.27% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.13% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.96% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 3.24% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 4 | 3.20% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 2.81% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 6 | 2.71% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8784` |
| 7 | 2.25% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 8 | 2.14% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 1.62% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8776` |
| 10 | 1.60% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 11 | 1.51% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 12 | 1.47% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 13 | 1.42% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 14 | 1.40% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 15 | 1.35% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5134` |
| 16 | 1.30% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 17 | 1.20% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 18 | 1.06% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 19 | 0.98% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 20 | 0.89% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2228` |
| 21 | 0.83% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 22 | 0.79% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3584` |
| 23 | 0.75% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 24 | 0.75% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 25 | 0.74% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 26 | 0.70% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 27 | 0.67% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |
| 28 | 0.66% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 29 | 0.62% | `com.google.common.collect.Sets$3.isEmpty() line: 1060` |
| 30 | 0.60% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |

GC pauses: 0, total 0 ms

