# JVM hotspots (JFR ExecutionSample)

Samples: 21647

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.07% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 2.21% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 3 | 1.93% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 4 | 1.82% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 5 | 1.77% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 1.65% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10239` |
| 7 | 1.58% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 8 | 1.31% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9637` |
| 9 | 1.27% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 10 | 1.23% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 11 | 1.13% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 12 | 1.08% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6926` |
| 13 | 1.03% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 14 | 1.02% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 15 | 1.02% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6351` |
| 16 | 1.00% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 17 | 0.97% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 18 | 0.94% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 19 | 0.93% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 20 | 0.89% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 21 | 0.85% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6858` |
| 22 | 0.84% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 23 | 0.82% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 24 | 0.78% | `java.util.EnumMap.get(Object) line: 248` |
| 25 | 0.73% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 26 | 0.71% | `java.util.concurrent.CompletableFuture.isDone() line: 2076` |
| 27 | 0.69% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 28 | 0.68% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 29 | 0.63% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2739` |
| 30 | 0.63% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 31 | 0.62% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 32 | 0.62% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 33 | 0.59% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9655` |
| 34 | 0.56% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 35 | 0.56% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 36 | 0.56% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 37 | 0.55% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 38 | 0.55% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 39 | 0.54% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 40 | 0.53% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 41 | 0.53% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 3950` |
| 42 | 0.53% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 43 | 0.53% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 911` |
| 44 | 0.52% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 45 | 0.49% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 46 | 0.49% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 47 | 0.46% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 48 | 0.46% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 49 | 0.45% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 697` |
| 50 | 0.44% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 51 | 0.43% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 52 | 0.42% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 6702` |
| 53 | 0.41% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 320` |
| 54 | 0.41% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 55 | 0.40% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 10151` |
| 56 | 0.40% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 10159` |
| 57 | 0.39% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |
| 58 | 0.37% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 59 | 0.37% | `java.util.Optional.ifPresent(Consumer) line: 178` |
| 60 | 0.36% | `java.util.EnumMap.isValidKey(Object) line: 320` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.84% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 2.86% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 3 | 2.60% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9785` |
| 4 | 2.08% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6941` |
| 5 | 1.94% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 6 | 1.66% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 7 | 1.61% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 8 | 1.58% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 9 | 1.51% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 10 | 1.28% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 11 | 1.27% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 12 | 1.27% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 13 | 1.22% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 14 | 1.19% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2121` |
| 15 | 1.15% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 16 | 1.13% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 17 | 1.04% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 18 | 1.00% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6926` |
| 19 | 0.99% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 20 | 0.97% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 21 | 0.94% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 22 | 0.91% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |
| 23 | 0.91% | `net.minecraft.world.level.EntityGetter.findNearbyPlayer(Entity, double, Predicate) line: 117` |
| 24 | 0.91% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 25 | 0.88% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 26 | 0.86% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 27 | 0.85% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 7344` |
| 28 | 0.80% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 29 | 0.73% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4302` |
| 30 | 0.71% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 160` |

GC pauses: 0, total 0 ms

