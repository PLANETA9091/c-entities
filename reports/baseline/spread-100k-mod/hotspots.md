# JVM hotspots (JFR ExecutionSample)

Samples: 30606

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 4.11% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10182` |
| 2 | 3.44% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 3 | 2.72% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 2.33% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 5 | 2.24% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6305` |
| 6 | 2.15% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 7 | 1.54% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 8 | 1.46% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 9 | 1.46% | `java.util.ArrayDeque.pollFirst() line: 378` |
| 10 | 1.30% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9580` |
| 11 | 1.19% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 12 | 1.18% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 13 | 1.16% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 14 | 1.10% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5537` |
| 15 | 1.09% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 16 | 1.06% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 17 | 0.99% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 18 | 0.96% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 19 | 0.89% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 20 | 0.81% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 21 | 0.76% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6812` |
| 22 | 0.74% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 23 | 0.71% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 24 | 0.70% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 25 | 0.69% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9598` |
| 26 | 0.68% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6880` |
| 27 | 0.67% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2739` |
| 28 | 0.66% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 29 | 0.64% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 30 | 0.61% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 31 | 0.59% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 32 | 0.59% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 33 | 0.59% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6315` |
| 34 | 0.57% | `java.util.EnumMap.get(Object) line: 248` |
| 35 | 0.56% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 36 | 0.55% | `org.spigotmc.WatchdogThread.run() line: 69` |
| 37 | 0.54% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5646` |
| 38 | 0.52% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 39 | 0.52% | `java.lang.String.equals(Object) line: 1926` |
| 40 | 0.50% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 320` |
| 41 | 0.50% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 42 | 0.49% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 43 | 0.49% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 6478` |
| 44 | 0.49% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 45 | 0.47% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 46 | 0.47% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 47 | 0.46% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 48 | 0.45% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 6656` |
| 49 | 0.45% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 50 | 0.45% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 51 | 0.43% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 52 | 0.43% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2777` |
| 53 | 0.43% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9591` |
| 54 | 0.42% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 55 | 0.41% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 56 | 0.40% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 57 | 0.39% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 58 | 0.37% | `ca.spottedleaf.moonrise.patches.chunk_system.scheduling.ChunkHolderManager.processTicketUpdates() line: 1517` |
| 59 | 0.36% | `net.minecraft.core.Vec3i.getX() line: 63` |
| 60 | 0.35% | `java.util.Collections$UnmodifiableCollection$1.<init>(Collections$UnmodifiableCollection) line: 1077` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.13% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 2 | 3.90% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 3.47% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 4 | 3.04% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 5 | 2.93% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9728` |
| 6 | 2.33% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 7 | 2.15% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4292` |
| 8 | 2.14% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 9 | 1.96% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 10 | 1.69% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2121` |
| 11 | 1.69% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6895` |
| 12 | 1.68% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 13 | 1.46% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 14 | 1.46% | `java.util.ArrayDeque.removeFirst() line: 358` |
| 15 | 1.30% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 16 | 1.26% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnEquipment(LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 159` |
| 17 | 1.17% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 18 | 1.08% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 19 | 0.95% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 20 | 0.89% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |
| 21 | 0.86% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 22 | 0.81% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 23 | 0.81% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3728` |
| 24 | 0.75% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 25 | 0.74% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6811` |
| 26 | 0.73% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6880` |
| 27 | 0.72% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 28 | 0.72% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 29 | 0.67% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 30 | 0.66% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |

GC pauses: 0, total 0 ms

