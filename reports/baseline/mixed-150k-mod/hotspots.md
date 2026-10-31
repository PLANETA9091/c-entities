# JVM hotspots (JFR ExecutionSample)

Samples: 35312

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.88% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8788` |
| 2 | 3.35% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5824` |
| 3 | 3.09% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8929` |
| 4 | 3.03% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 3.03% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 2.72% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 7 | 2.56% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5033` |
| 8 | 1.82% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8735` |
| 9 | 1.65% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 1.53% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 11 | 1.50% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.48% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 1.40% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5679` |
| 14 | 1.34% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 389` |
| 15 | 1.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 16 | 1.15% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 17 | 1.07% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 18 | 0.97% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 19 | 0.97% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 20 | 0.82% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 21 | 0.79% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 22 | 0.78% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 23 | 0.75% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8874` |
| 24 | 0.67% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 25 | 0.66% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 26 | 0.65% | `java.util.EnumMap.get(Object) line: 248` |
| 27 | 0.63% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 28 | 0.63% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 29 | 0.61% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 30 | 0.60% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8749` |
| 31 | 0.54% | `dev.planeta9091.centities.CeHooks.deltaNoOp(Entity, Vec3) line: 8431` |
| 32 | 0.54% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 33 | 0.53% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 34 | 0.51% | `net.minecraft.world.entity.SteppedInterpolationTracker.updateTracking(Vec3) line: 40` |
| 35 | 0.50% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 36 | 0.50% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 37 | 0.49% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 38 | 0.46% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 39 | 0.46% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 40 | 0.45% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 41 | 0.43% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 42 | 0.42% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5829` |
| 43 | 0.42% | `net.minecraft.world.phys.Vec3.distanceToSqr(Vec3) line: 138` |
| 44 | 0.40% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 45 | 0.39% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 46 | 0.38% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 47 | 0.38% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 48 | 0.38% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3734` |
| 49 | 0.38% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 50 | 0.37% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 51 | 0.36% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 52 | 0.36% | `java.util.ArrayList.add(Object, Object[], int) line: 483` |
| 53 | 0.35% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 54 | 0.35% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 55 | 0.34% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 56 | 0.33% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 57 | 0.33% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 58 | 0.32% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4080` |
| 59 | 0.31% | `java.util.ImmutableCollections$ListN.get(int) line: 725` |
| 60 | 0.31% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.53% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.02% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 3.99% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 4 | 3.10% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 5 | 2.83% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 6 | 2.61% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8784` |
| 7 | 2.10% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 8 | 1.91% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 9 | 1.70% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8801` |
| 10 | 1.65% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 11 | 1.46% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8776` |
| 12 | 1.44% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 13 | 1.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 14 | 1.22% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 15 | 1.17% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2228` |
| 16 | 1.16% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 17 | 1.15% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 18 | 1.10% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 19 | 0.98% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 20 | 0.85% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 21 | 0.82% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5134` |
| 22 | 0.80% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 23 | 0.78% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 24 | 0.69% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3584` |
| 25 | 0.69% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 26 | 0.68% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 27 | 0.68% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 28 | 0.64% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 29 | 0.63% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 30 | 0.61% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 6718` |

GC pauses: 0, total 0 ms

