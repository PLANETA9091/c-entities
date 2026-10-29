# JVM hotspots (JFR ExecutionSample)

Samples: 36511

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 6.13% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 2 | 2.78% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8049` |
| 3 | 2.71% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 4 | 2.58% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 5 | 2.48% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5138` |
| 6 | 2.24% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8102` |
| 7 | 1.58% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 1.34% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 9 | 1.30% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4420` |
| 10 | 1.16% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 11 | 1.13% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.09% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 13 | 1.05% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 4993` |
| 14 | 1.05% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 15 | 0.91% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 16 | 0.88% | `java.util.EnumMap$EnumMapIterator.hasNext() line: 524` |
| 17 | 0.86% | `java.util.EnumMap.get(Object) line: 248` |
| 18 | 0.85% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 19 | 0.83% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 20 | 0.76% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 21 | 0.70% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 22 | 0.67% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 23 | 0.64% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 24 | 0.62% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 25 | 0.61% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 184` |
| 26 | 0.61% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 27 | 0.61% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 28 | 0.58% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 29 | 0.57% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 30 | 0.56% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5140` |
| 31 | 0.53% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 32 | 0.51% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 33 | 0.50% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate) line: 2228` |
| 34 | 0.50% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 35 | 0.49% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 36 | 0.48% | `net.minecraft.world.entity.Entity.level() line: 5416` |
| 37 | 0.47% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 38 | 0.46% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 39 | 0.46% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 40 | 0.46% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 41 | 0.45% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8188` |
| 42 | 0.44% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 43 | 0.42% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 44 | 0.41% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 45 | 0.41% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 46 | 0.40% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.setDirty() line: 114` |
| 47 | 0.39% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 48 | 0.38% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 49 | 0.38% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 50 | 0.38% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 51 | 0.37% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 52 | 0.37% | `java.lang.ref.Reference.get() line: 361` |
| 53 | 0.35% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 54 | 0.35% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 55 | 0.34% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.hasCommonElements(OptimizedSmallEnumSet) line: 62` |
| 56 | 0.34% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3115` |
| 57 | 0.34% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 58 | 0.33% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 59 | 0.33% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 60 | 0.33% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.13% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 2 | 3.72% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 3.39% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 3.03% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8098` |
| 5 | 2.43% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 6 | 1.87% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 7 | 1.61% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 8 | 1.59% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 9 | 1.39% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices$EntityCollectionBySection.getEntities(Entity, AABB, List, Predicate) line: 546` |
| 10 | 1.39% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4457` |
| 11 | 1.37% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8090` |
| 12 | 1.22% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8115` |
| 13 | 1.08% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 14 | 0.98% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 1732` |
| 15 | 0.96% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 16 | 0.94% | `net.minecraft.world.entity.EntityEquipment.tick(Entity) line: 50` |
| 17 | 0.85% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 18 | 0.82% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 19 | 0.81% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 20 | 0.77% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 21 | 0.76% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 22 | 0.76% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 23 | 0.70% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 24 | 0.68% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 25 | 0.67% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 26 | 0.64% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 27 | 0.64% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |
| 28 | 0.64% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |
| 29 | 0.64% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 30 | 0.62% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |

GC pauses: 0, total 0 ms

