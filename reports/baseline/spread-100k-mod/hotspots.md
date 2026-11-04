# JVM hotspots (JFR ExecutionSample)

Samples: 35315

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 12.46% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3100` |
| 2 | 3.20% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 3 | 3.00% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 4 | 2.68% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3138` |
| 5 | 2.03% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10280` |
| 6 | 1.59% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2670` |
| 7 | 1.42% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 8 | 1.36% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 9 | 1.15% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 10 | 1.12% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 11 | 1.09% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3103` |
| 12 | 1.08% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 13 | 1.01% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6784` |
| 14 | 0.96% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 15 | 0.91% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 16 | 0.81% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 17 | 0.76% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 18 | 0.71% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 19 | 0.71% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 20 | 0.70% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7359` |
| 21 | 0.69% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 22 | 0.69% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3179` |
| 23 | 0.67% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 24 | 0.65% | `java.util.EnumMap.get(Object) line: 248` |
| 25 | 0.63% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 26 | 0.63% | `dev.planeta9091.centities.CeHooks.vtabHash(int) line: 376` |
| 27 | 0.62% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7291` |
| 28 | 0.61% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 29 | 0.61% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3085` |
| 30 | 0.57% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 31 | 0.57% | `java.util.concurrent.CompletableFuture.isDone() line: 2076` |
| 32 | 0.57% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 33 | 0.55% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 34 | 0.54% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3097` |
| 35 | 0.52% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 36 | 0.51% | `dev.planeta9091.centities.CeHooks.isViewAfter(int) line: 424` |
| 37 | 0.50% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 38 | 0.50% | `dev.planeta9091.centities.CeHooks.lambda$ordSortVanilla$0(Entity, Entity) line: 2401` |
| 39 | 0.48% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 40 | 0.48% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3707` |
| 41 | 0.46% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3112` |
| 42 | 0.44% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 43 | 0.43% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 10317` |
| 44 | 0.43% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 45 | 0.42% | `net.minecraft.world.entity.Entity.isInterpolating() line: 3584` |
| 46 | 0.41% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 47 | 0.40% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 48 | 0.40% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 49 | 0.39% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 50 | 0.39% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 51 | 0.39% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 52 | 0.38% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 53 | 0.37% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2887` |
| 54 | 0.37% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7901` |
| 55 | 0.37% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 56 | 0.36% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6794` |
| 57 | 0.35% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 58 | 0.34% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 59 | 0.33% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 60 | 0.33% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 20.13% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2482` |
| 2 | 4.09% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 3 | 3.20% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4689` |
| 4 | 2.92% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 10463` |
| 5 | 1.82% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate) line: 2443` |
| 6 | 1.69% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 7 | 1.56% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 8 | 1.44% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 9 | 1.16% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7374` |
| 10 | 1.04% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2817` |
| 11 | 1.04% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 12 | 1.04% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 13 | 1.02% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 14 | 0.98% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2821` |
| 15 | 0.89% | `java.util.TimSort.countRunAndMakeAscending(Object[], int, int, Comparator) line: 360` |
| 16 | 0.86% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 17 | 0.80% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3792` |
| 18 | 0.75% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 19 | 0.71% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 20 | 0.71% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 21 | 0.69% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 22 | 0.68% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3739` |
| 23 | 0.68% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 24 | 0.67% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5956` |
| 25 | 0.63% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 26 | 0.63% | `dev.planeta9091.centities.CeHooks.vtabGet(int) line: 390` |
| 27 | 0.62% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 3117` |
| 28 | 0.61% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 29 | 0.60% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 30 | 0.59% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7359` |

GC pauses: 0, total 0 ms

