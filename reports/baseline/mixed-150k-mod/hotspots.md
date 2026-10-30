# JVM hotspots (JFR ExecutionSample)

Samples: 33225

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.53% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 2 | 4.05% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8717` |
| 3 | 3.21% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 5753` |
| 4 | 3.02% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 2.99% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 8858` |
| 6 | 2.93% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 7 | 2.24% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 4962` |
| 8 | 2.00% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8664` |
| 9 | 1.94% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 1.57% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.53% | `dev.planeta9091.centities.CeHooks.trackerDispatch(ServerEntity) line: 5608` |
| 12 | 1.26% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 389` |
| 13 | 1.25% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 14 | 1.24% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 15 | 1.13% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 16 | 1.09% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 17 | 1.06% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 18 | 0.91% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 19 | 0.88% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2404` |
| 20 | 0.83% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5074` |
| 21 | 0.78% | `net.minecraft.server.level.ServerEntity.sendDirtyEntityData() line: 442` |
| 22 | 0.78% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 23 | 0.72% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 24 | 0.69% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 25 | 0.69% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 26 | 0.66% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 27 | 0.63% | `dev.planeta9091.centities.CeHooks.ttlFastPath(Entity, Object, Object) line: 8678` |
| 28 | 0.61% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 29 | 0.56% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 30 | 0.55% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1334` |
| 31 | 0.53% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 32 | 0.52% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 33 | 0.51% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 8803` |
| 34 | 0.51% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 35 | 0.51% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 36 | 0.49% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 37 | 0.49% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 3663` |
| 38 | 0.47% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 39 | 0.46% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 40 | 0.45% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 41 | 0.42% | `net.minecraft.world.entity.Entity.getEntityData() line: 692` |
| 42 | 0.40% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 43 | 0.39% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 44 | 0.39% | `net.minecraft.world.entity.Entity.getX() line: 5095` |
| 45 | 0.36% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 46 | 0.36% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 47 | 0.35% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3)` |
| 48 | 0.35% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4707` |
| 49 | 0.33% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 50 | 0.32% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 51 | 0.32% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 52 | 0.32% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 53 | 0.31% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 54 | 0.31% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 55 | 0.31% | `java.util.EnumMap.get(Object) line: 248` |
| 56 | 0.31% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 57 | 0.31% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 58 | 0.30% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 59 | 0.30% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 60 | 0.29% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.50% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.14% | `net.minecraft.server.level.ChunkMap.newTrackerTick()` |
| 3 | 3.51% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 3.02% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 5 | 2.78% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8713` |
| 6 | 2.77% | `net.minecraft.network.protocol.game.VecDelta$Stepped.tryEncode(VecDeltaCodec, List) line: 156` |
| 7 | 2.54% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 8 | 1.94% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 9 | 1.75% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 10 | 1.74% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 11 | 1.56% | `net.minecraft.server.level.ServerEntity.sendChanges()` |
| 12 | 1.51% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 8705` |
| 13 | 1.32% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 14 | 1.24% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 15 | 1.16% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 16 | 1.15% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 17 | 1.10% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5063` |
| 18 | 1.07% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2173` |
| 19 | 1.07% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 20 | 1.01% | `net.minecraft.world.entity.LivingEntity.collectEquipmentChanges(Map) line: 3584` |
| 21 | 0.90% | `net.minecraft.world.entity.Entity.setOldPos() line: 2396` |
| 22 | 0.85% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 23 | 0.80% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 24 | 0.74% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 25 | 0.73% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 26 | 0.70% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 27 | 0.62% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 28 | 0.61% | `net.minecraft.server.level.ServerEntity.sendChanges() line: 226` |
| 29 | 0.60% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2213` |
| 30 | 0.60% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |

GC pauses: 0, total 0 ms

