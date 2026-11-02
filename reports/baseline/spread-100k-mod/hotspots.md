# JVM hotspots (JFR ExecutionSample)

Samples: 26320

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.87% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10199` |
| 2 | 3.15% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 3 | 2.48% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 2.27% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6311` |
| 5 | 2.11% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 6 | 1.65% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 1.44% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5543` |
| 8 | 1.40% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9597` |
| 9 | 1.25% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 10 | 1.22% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 11 | 1.17% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 12 | 1.04% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 0.97% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 14 | 0.95% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 15 | 0.91% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 16 | 0.88% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 17 | 0.87% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2739` |
| 18 | 0.82% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 19 | 0.81% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 20 | 0.80% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 21 | 0.77% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 22 | 0.73% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9615` |
| 23 | 0.70% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6886` |
| 24 | 0.69% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 25 | 0.69% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 26 | 0.68% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6818` |
| 27 | 0.61% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5652` |
| 28 | 0.58% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6321` |
| 29 | 0.55% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 30 | 0.55% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 31 | 0.53% | `java.util.EnumMap.get(Object) line: 248` |
| 32 | 0.53% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 33 | 0.51% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 34 | 0.50% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 35 | 0.49% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 36 | 0.49% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 10111` |
| 37 | 0.48% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 6484` |
| 38 | 0.47% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 324` |
| 39 | 0.46% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 40 | 0.46% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9608` |
| 41 | 0.45% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2780` |
| 42 | 0.43% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 43 | 0.43% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 44 | 0.43% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 6662` |
| 45 | 0.43% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 46 | 0.43% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 47 | 0.42% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 48 | 0.41% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4077` |
| 49 | 0.41% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 50 | 0.41% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 320` |
| 51 | 0.40% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 52 | 0.40% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 53 | 0.38% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 54 | 0.38% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 55 | 0.37% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 56 | 0.36% | `net.minecraft.world.entity.Entity.getType() line: 633` |
| 57 | 0.36% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 58 | 0.36% | `net.minecraft.world.entity.EntityEquipment.get(EquipmentSlot) line: 36` |
| 59 | 0.35% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 60 | 0.34% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 3.89% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 2 | 3.52% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 3.28% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9745` |
| 4 | 3.19% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 5 | 3.16% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 6 | 2.67% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 7 | 2.14% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 8 | 2.11% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4298` |
| 9 | 2.04% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2121` |
| 10 | 1.72% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 11 | 1.65% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 12 | 1.46% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 13 | 1.31% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 14 | 1.22% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 15 | 1.17% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 16 | 1.07% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6901` |
| 17 | 1.04% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 18 | 0.91% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 19 | 0.84% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 20 | 0.83% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 21 | 0.79% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |
| 22 | 0.79% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 23 | 0.77% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |
| 24 | 0.73% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 25 | 0.71% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 26 | 0.69% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6817` |
| 27 | 0.68% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 28 | 0.64% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 29 | 0.60% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 30 | 0.60% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |

GC pauses: 0, total 0 ms

