# JVM hotspots (JFR ExecutionSample)

Samples: 37622

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.14% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 7107` |
| 2 | 3.10% | `jdk.internal.misc.ScopedMemoryAccess.getLongUnalignedInternal(MemorySessionImpl, Object, long, boolean) line: 3858` |
| 3 | 3.08% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10970` |
| 4 | 2.89% | `jdk.internal.misc.Unsafe.getLongUnaligned(Object, long, boolean) line: 3571` |
| 5 | 2.62% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 6 | 2.04% | `jdk.internal.misc.Unsafe.convEndian(boolean, long) line: 3835` |
| 7 | 1.82% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 8 | 1.69% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 9 | 1.65% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 10 | 1.57% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 11 | 1.40% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2731` |
| 12 | 1.39% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 1.25% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 14 | 1.24% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 15 | 1.16% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 16 | 1.08% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6344` |
| 17 | 1.07% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getAcquire(VarHandle, Object) line: 113` |
| 18 | 0.93% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 19 | 0.85% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 20 | 0.83% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 21 | 0.79% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3584` |
| 22 | 0.78% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 11430` |
| 23 | 0.72% | `java.util.HashMap.getNode(Object) line: 586` |
| 24 | 0.70% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 25 | 0.65% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 26 | 0.62% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 27 | 0.61% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 28 | 0.56% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 29 | 0.54% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 30 | 0.54% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 31 | 0.54% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2889` |
| 32 | 0.53% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 33 | 0.53% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 34 | 0.52% | `java.util.EnumMap.get(Object) line: 248` |
| 35 | 0.52% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 36 | 0.51% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 37 | 0.50% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 38 | 0.50% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 39 | 0.49% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 40 | 0.47% | `dev.planeta9091.centities.CeHooks.vtabHash(int) line: 411` |
| 41 | 0.45% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2638` |
| 42 | 0.45% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3401` |
| 43 | 0.40% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7627` |
| 44 | 0.39% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3147` |
| 45 | 0.37% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 46 | 0.37% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 47 | 0.37% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 48 | 0.36% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4764` |
| 49 | 0.36% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 50 | 0.35% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 51 | 0.35% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 52 | 0.34% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 7280` |
| 53 | 0.33% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10832` |
| 54 | 0.32% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 55 | 0.32% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 56 | 0.31% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 57 | 0.30% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2821` |
| 58 | 0.30% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 59 | 0.30% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 60 | 0.28% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.01% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.50% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 3.41% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2599` |
| 4 | 3.11% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 5 | 3.10% | `jdk.internal.misc.ScopedMemoryAccess.getLongUnaligned(MemorySessionImpl, Object, long, boolean) line: 3849` |
| 6 | 2.89% | `jdk.internal.misc.ScopedMemoryAccess.getLongUnalignedInternal(MemorySessionImpl, Object, long, boolean) line: 3861` |
| 7 | 2.23% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2638` |
| 8 | 2.04% | `jdk.internal.misc.Unsafe.getLongUnaligned(Object, long, boolean) line: 3571` |
| 9 | 1.73% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 10 | 1.69% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 11 | 1.58% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 12 | 1.56% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4984` |
| 13 | 1.51% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 6262` |
| 14 | 1.51% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 15 | 1.29% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 16 | 1.04% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 17 | 1.03% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getNextAcquire() line: 1837` |
| 18 | 0.97% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 19 | 0.93% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2817` |
| 20 | 0.87% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 21 | 0.87% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 22 | 0.85% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 23 | 0.83% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3696` |
| 24 | 0.78% | `java.util.HashMap.get(Object) line: 565` |
| 25 | 0.78% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 26 | 0.73% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 27 | 0.70% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 28 | 0.69% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 11065` |
| 29 | 0.66% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 8126` |
| 30 | 0.65% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |

GC pauses: 0, total 0 ms

