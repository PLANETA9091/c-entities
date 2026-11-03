# JVM hotspots (JFR ExecutionSample)

Samples: 37384

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.11% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 2.82% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 3 | 2.66% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10380` |
| 4 | 2.61% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6473` |
| 5 | 1.90% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 6 | 1.81% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9778` |
| 7 | 1.69% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 8 | 1.57% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |
| 9 | 1.50% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 10 | 1.48% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 11 | 1.45% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.16% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 13 | 1.16% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 14 | 1.12% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 15 | 1.11% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 16 | 1.02% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 17 | 0.97% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 18 | 0.87% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 19 | 0.81% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 20 | 0.73% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 21 | 0.72% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 22 | 0.71% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 23 | 0.69% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 24 | 0.69% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 25 | 0.69% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 26 | 0.68% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 27 | 0.68% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 28 | 0.68% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6980` |
| 29 | 0.58% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 30 | 0.58% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 10292` |
| 31 | 0.57% | `java.util.EnumMap.get(Object) line: 248` |
| 32 | 0.57% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5710` |
| 33 | 0.57% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 34 | 0.56% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 35 | 0.53% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 36 | 0.53% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 37 | 0.52% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 38 | 0.51% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 39 | 0.51% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 40 | 0.48% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 41 | 0.47% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 42 | 0.47% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 43 | 0.45% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 44 | 0.45% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7048` |
| 45 | 0.45% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 46 | 0.44% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 47 | 0.41% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4072` |
| 48 | 0.40% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 49 | 0.40% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9789` |
| 50 | 0.40% | `java.util.Arrays.fill(Object[], int, int, Object) line: 3450` |
| 51 | 0.39% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 52 | 0.38% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 6646` |
| 53 | 0.38% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 54 | 0.37% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4204` |
| 55 | 0.37% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2873` |
| 56 | 0.37% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 6824` |
| 57 | 0.36% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4570` |
| 58 | 0.35% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5781` |
| 59 | 0.34% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 60 | 0.34% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.56% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.00% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 3 | 2.91% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 2.83% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9926` |
| 5 | 2.69% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 6 | 1.93% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 7 | 1.90% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4424` |
| 8 | 1.89% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 9 | 1.57% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 10 | 1.39% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 11 | 1.36% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 12 | 1.34% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 13 | 1.17% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7063` |
| 14 | 1.11% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 15 | 1.08% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2217` |
| 16 | 1.04% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7048` |
| 17 | 0.99% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 18 | 0.99% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 19 | 0.98% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 20 | 0.93% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 21 | 0.92% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 22 | 0.85% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 23 | 0.84% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 24 | 0.82% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 25 | 0.78% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 7466` |
| 26 | 0.77% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5675` |
| 27 | 0.77% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 28 | 0.74% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6483` |
| 29 | 0.71% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 30 | 0.70% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |

GC pauses: 0, total 0 ms

