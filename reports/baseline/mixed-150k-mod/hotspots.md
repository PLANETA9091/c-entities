# JVM hotspots (JFR ExecutionSample)

Samples: 22831

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.44% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6351` |
| 2 | 2.97% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10239` |
| 3 | 2.78% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 2.56% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 5 | 2.29% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 6 | 2.02% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 7 | 1.69% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9637` |
| 8 | 1.64% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 9 | 1.40% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getAcquire(VarHandle, Object) line: 113` |
| 10 | 1.35% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 11 | 1.33% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 12 | 1.30% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 13 | 1.30% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 14 | 1.25% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 15 | 1.02% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 16 | 0.94% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6361` |
| 17 | 0.85% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 18 | 0.84% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5588` |
| 19 | 0.83% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9655` |
| 20 | 0.83% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 21 | 0.66% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 22 | 0.66% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 23 | 0.65% | `java.util.EnumMap.get(Object) line: 248` |
| 24 | 0.65% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 25 | 0.65% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 26 | 0.61% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 27 | 0.60% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 28 | 0.60% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6858` |
| 29 | 0.57% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 30 | 0.57% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 31 | 0.56% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4082` |
| 32 | 0.54% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 33 | 0.53% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6926` |
| 34 | 0.52% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 35 | 0.52% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9608` |
| 36 | 0.52% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9648` |
| 37 | 0.49% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 38 | 0.49% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 10151` |
| 39 | 0.49% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 40 | 0.47% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 41 | 0.47% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 42 | 0.45% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 43 | 0.45% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 44 | 0.44% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 45 | 0.44% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9611` |
| 46 | 0.44% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 47 | 0.44% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 48 | 0.42% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 6524` |
| 49 | 0.39% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5562` |
| 50 | 0.39% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 51 | 0.38% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 52 | 0.38% | `sun.misc.Unsafe.putObject(Object, long, Object) line: 240` |
| 53 | 0.38% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 54 | 0.37% | `java.lang.String.equals(Object) line: 1926` |
| 55 | 0.36% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 56 | 0.35% | `net.minecraft.server.level.ServerLevel.moonrise$midTickTasks() line: 357` |
| 57 | 0.35% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 58 | 0.35% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 59 | 0.35% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 60 | 0.35% | `paperclip.libs.org.apache.commons.compress.compressors.bzip2.CRC.updateCRC(int) line: 119` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.81% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 2 | 4.75% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9785` |
| 3 | 4.03% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 4 | 2.97% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 5 | 2.55% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4302` |
| 6 | 2.33% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 7 | 2.33% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 8 | 1.94% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5553` |
| 9 | 1.79% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 10 | 1.75% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 11 | 1.35% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 12 | 1.34% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getNextAcquire() line: 1837` |
| 13 | 1.33% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 14 | 1.28% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 15 | 1.13% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 16 | 1.01% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 17 | 0.98% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 18 | 0.95% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6941` |
| 19 | 0.88% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 20 | 0.84% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 21 | 0.75% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6361` |
| 22 | 0.71% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 23 | 0.67% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 24 | 0.67% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 25 | 0.67% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 26 | 0.66% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |
| 27 | 0.62% | `net.minecraft.world.entity.Entity.updateFluidInteraction() line: 2218` |
| 28 | 0.62% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 29 | 0.61% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3983` |
| 30 | 0.58% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6857` |

GC pauses: 0, total 0 ms

