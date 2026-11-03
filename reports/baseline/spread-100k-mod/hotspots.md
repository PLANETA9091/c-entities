# JVM hotspots (JFR ExecutionSample)

Samples: 27273

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.09% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 2 | 3.84% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10361` |
| 3 | 2.54% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6473` |
| 4 | 2.24% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 5 | 1.88% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 6 | 1.84% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9759` |
| 7 | 1.73% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 8 | 1.61% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getAtIndexAcquire(ConcurrentChainedLong2ReferenceHashTable$TableEntry[], int) line: 187` |
| 9 | 1.57% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 10 | 1.48% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 11 | 1.46% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 12 | 1.35% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 1.07% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 14 | 1.00% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 15 | 1.00% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 16 | 0.99% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7048` |
| 17 | 0.95% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 18 | 0.92% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 19 | 0.92% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 20 | 0.92% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6980` |
| 21 | 0.91% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 22 | 0.88% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 23 | 0.88% | `java.util.EnumMap.get(Object) line: 248` |
| 24 | 0.86% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 25 | 0.85% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 26 | 0.81% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 27 | 0.79% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 28 | 0.73% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 10273` |
| 29 | 0.72% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 30 | 0.71% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2835` |
| 31 | 0.70% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 32 | 0.69% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 33 | 0.69% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 34 | 0.65% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 35 | 0.63% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 36 | 0.63% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 37 | 0.63% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 38 | 0.62% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 39 | 0.60% | `jdk.internal.util.ArraysSupport.hashCodeOfUnsigned(byte[], int, int, int) line: 275` |
| 40 | 0.58% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9777` |
| 41 | 0.57% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 42 | 0.54% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7590` |
| 43 | 0.53% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 6824` |
| 44 | 0.52% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 45 | 0.51% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 46 | 0.51% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 47 | 0.48% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 48 | 0.47% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5710` |
| 49 | 0.47% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 50 | 0.46% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 51 | 0.45% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4204` |
| 52 | 0.45% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 53 | 0.44% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4569` |
| 54 | 0.44% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 6646` |
| 55 | 0.43% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 56 | 0.42% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4506` |
| 57 | 0.41% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 58 | 0.41% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 320` |
| 59 | 0.40% | `java.util.Optional.ifPresent(Consumer) line: 178` |
| 60 | 0.40% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9770` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.44% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.85% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 3 | 3.23% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9907` |
| 4 | 2.96% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 2.51% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 6 | 2.27% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 7 | 1.78% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 8 | 1.73% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7063` |
| 9 | 1.61% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable.getNode(long) line: 230` |
| 10 | 1.57% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4424` |
| 11 | 1.55% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2217` |
| 12 | 1.50% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 13 | 1.50% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 14 | 1.46% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 15 | 1.41% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 16 | 1.40% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 17 | 1.16% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 18 | 1.11% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 19 | 1.03% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 20 | 1.00% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 21 | 0.95% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 22 | 0.93% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 23 | 0.89% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 24 | 0.87% | `dev.planeta9091.centities.CeHooks.entityDataValueIdx(Entity, int) line: 7466` |
| 25 | 0.84% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 26 | 0.81% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 27 | 0.81% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 28 | 0.80% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |
| 29 | 0.72% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 30 | 0.72% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7048` |

GC pauses: 0, total 0 ms

