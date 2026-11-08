# JVM hotspots (JFR ExecutionSample)

Samples: 37919

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.07% | `jdk.internal.misc.ScopedMemoryAccess.getLongUnalignedInternal(MemorySessionImpl, Object, long, boolean) line: 3858` |
| 2 | 3.37% | `jdk.internal.misc.Unsafe.convEndian(boolean, long) line: 3835` |
| 3 | 3.16% | `jdk.internal.misc.Unsafe.getLongUnaligned(Object, long, boolean) line: 3571` |
| 4 | 3.08% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10965` |
| 5 | 3.06% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7102` |
| 6 | 2.42% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.09% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2726` |
| 8 | 1.55% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 9 | 1.53% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 10 | 1.34% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 11 | 1.26% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 12 | 1.24% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 13 | 1.19% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 14 | 1.14% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 15 | 1.11% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 16 | 1.10% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6339` |
| 17 | 1.05% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 18 | 0.87% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3584` |
| 19 | 0.77% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 20 | 0.75% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 21 | 0.70% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 22 | 0.68% | `java.util.HashMap.getNode(Object) line: 586` |
| 23 | 0.68% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 11425` |
| 24 | 0.62% | `java.util.EnumMap.get(Object) line: 248` |
| 25 | 0.61% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 26 | 0.55% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 27 | 0.53% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 28 | 0.52% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 29 | 0.51% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 30 | 0.50% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 31 | 0.50% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 32 | 0.50% | `dev.planeta9091.centities.CeHooks.getEntitiesOfClass(Level, Class, AABB, Predicate) line: 1326` |
| 33 | 0.50% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 34 | 0.46% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 35 | 0.46% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 36 | 0.45% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7622` |
| 37 | 0.44% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 38 | 0.44% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 39 | 0.43% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 40 | 0.43% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2633` |
| 41 | 0.41% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 42 | 0.40% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 43 | 0.39% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 44 | 0.37% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 45 | 0.37% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 46 | 0.36% | `net.minecraft.world.entity.LivingEntity.getItemBySlot(EquipmentSlot) line: 2901` |
| 47 | 0.36% | `dev.planeta9091.centities.CeHooks.vtabHash(int) line: 406` |
| 48 | 0.36% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 7275` |
| 49 | 0.35% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3396` |
| 50 | 0.31% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 51 | 0.29% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 52 | 0.29% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4759` |
| 53 | 0.28% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 54 | 0.28% | `java.lang.ref.Reference.refersToImpl(Object) line: 391` |
| 55 | 0.28% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1484` |
| 56 | 0.28% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 57 | 0.28% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 58 | 0.27% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 59 | 0.27% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 60 | 0.27% | `java.util.EnumMap$EnumMapIterator.hasNext() line: 524` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.07% | `jdk.internal.misc.ScopedMemoryAccess.getLongUnaligned(MemorySessionImpl, Object, long, boolean) line: 3849` |
| 2 | 3.61% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 3.52% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2594` |
| 4 | 3.37% | `jdk.internal.misc.Unsafe.getLongUnaligned(Object, long, boolean) line: 3571` |
| 5 | 3.34% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 6 | 3.16% | `jdk.internal.misc.ScopedMemoryAccess.getLongUnalignedInternal(MemorySessionImpl, Object, long, boolean) line: 3861` |
| 7 | 3.11% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 8 | 1.97% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2633` |
| 9 | 1.70% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 10 | 1.61% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 11 | 1.55% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4979` |
| 12 | 1.41% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6257` |
| 13 | 1.40% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 14 | 1.19% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2817` |
| 15 | 1.01% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 16 | 0.96% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 17 | 0.94% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 18 | 0.92% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 19 | 0.90% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3696` |
| 20 | 0.89% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 21 | 0.83% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 22 | 0.83% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 23 | 0.80% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 24 | 0.75% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 25 | 0.73% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 26 | 0.71% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate)` |
| 27 | 0.70% | `java.util.HashMap.get(Object) line: 565` |
| 28 | 0.64% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 8121` |
| 29 | 0.63% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 30 | 0.60% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 11060` |

GC pauses: 0, total 0 ms

