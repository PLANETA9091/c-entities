# JVM hotspots (JFR ExecutionSample)

Samples: 28085

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.49% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6328` |
| 2 | 3.44% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.30% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 4 | 2.92% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10216` |
| 5 | 2.70% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 6 | 1.94% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 7 | 1.94% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5546` |
| 8 | 1.93% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9614` |
| 9 | 1.67% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 10 | 1.50% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 11 | 1.43% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 12 | 1.35% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 1.04% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 14 | 1.03% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 15 | 0.92% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 16 | 0.91% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 17 | 0.89% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 18 | 0.85% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 10128` |
| 19 | 0.79% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 20 | 0.74% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 21 | 0.71% | `java.util.EnumMap.get(Object) line: 248` |
| 22 | 0.68% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 23 | 0.67% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 24 | 0.61% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 25 | 0.61% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 26 | 0.61% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 27 | 0.60% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6835` |
| 28 | 0.56% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 29 | 0.54% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4077` |
| 30 | 0.51% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9588` |
| 31 | 0.50% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 32 | 0.50% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 320` |
| 33 | 0.48% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 34 | 0.48% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 35 | 0.48% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 36 | 0.47% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 37 | 0.47% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 38 | 0.46% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 39 | 0.46% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 40 | 0.46% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9585` |
| 41 | 0.41% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 42 | 0.40% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9632` |
| 43 | 0.40% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 44 | 0.39% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 45 | 0.38% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 6501` |
| 46 | 0.38% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 47 | 0.38% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6903` |
| 48 | 0.37% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 6679` |
| 49 | 0.36% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 50 | 0.35% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 51 | 0.35% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 10136` |
| 52 | 0.35% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 53 | 0.35% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 54 | 0.34% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 55 | 0.34% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 56 | 0.34% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 57 | 0.32% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 58 | 0.32% | `dev.planeta9091.centities.CeHooks.equipUpdates$(LivingEntity) line: 10024` |
| 59 | 0.31% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 60 | 0.30% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3950` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.38% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 4.17% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9762` |
| 3 | 3.91% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 2.95% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 5 | 2.94% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 6 | 2.70% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4298` |
| 7 | 2.67% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 8 | 2.33% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 9 | 1.85% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 10 | 1.38% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 11 | 1.31% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 12 | 1.30% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 13 | 1.28% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 14 | 1.26% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 15 | 1.13% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 16 | 1.08% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 17 | 1.03% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 18 | 0.93% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 19 | 0.92% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 20 | 0.91% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 21 | 0.76% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6918` |
| 22 | 0.76% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |
| 23 | 0.75% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6338` |
| 24 | 0.70% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 25 | 0.67% | `net.minecraft.world.entity.ai.goal.WrappedGoal.canUse() line: 22` |
| 26 | 0.65% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 7321` |
| 27 | 0.65% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 28 | 0.63% | `net.minecraft.world.entity.Entity.baseTick() line: 903` |
| 29 | 0.61% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 30 | 0.61% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3983` |

GC pauses: 0, total 0 ms

