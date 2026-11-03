# JVM hotspots (JFR ExecutionSample)

Samples: 27772

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.83% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10301` |
| 2 | 4.72% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.15% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6413` |
| 4 | 2.98% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 5 | 2.12% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 6 | 2.11% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 7 | 2.05% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 8 | 1.48% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 9 | 1.47% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9699` |
| 10 | 1.43% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 11 | 1.37% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 12 | 1.29% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 13 | 1.25% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 14 | 1.21% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 15 | 1.13% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 16 | 1.12% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 17 | 1.07% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 18 | 1.06% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 19 | 0.97% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 20 | 0.92% | `net.minecraft.world.entity.Entity.isRemoved() line: 5334` |
| 21 | 0.89% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 22 | 0.86% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6920` |
| 23 | 0.83% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 24 | 0.82% | `net.minecraft.server.level.ServerChunkCache.getChunk(int, int, ChunkStatus, boolean) line: 312` |
| 25 | 0.82% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6988` |
| 26 | 0.79% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 27 | 0.73% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 28 | 0.70% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 29 | 0.70% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4144` |
| 30 | 0.70% | `java.util.EnumMap.get(Object) line: 248` |
| 31 | 0.68% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 32 | 0.67% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 33 | 0.65% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 34 | 0.61% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 35 | 0.61% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 36 | 0.57% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 37 | 0.57% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 38 | 0.57% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 39 | 0.56% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 40 | 0.53% | `net.minecraft.world.entity.ElytraAnimationState.tick() line: 28` |
| 41 | 0.53% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 42 | 0.50% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4510` |
| 43 | 0.49% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 44 | 0.49% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 6586` |
| 45 | 0.49% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 46 | 0.49% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4446` |
| 47 | 0.49% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 48 | 0.48% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 6764` |
| 49 | 0.48% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 50 | 0.47% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7530` |
| 51 | 0.45% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9717` |
| 52 | 0.44% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 53 | 0.41% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 320` |
| 54 | 0.38% | `ca.spottedleaf.moonrise.common.util.TickThread.isTickThread() line: 122` |
| 55 | 0.37% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 56 | 0.37% | `net.minecraft.world.entity.Entity.setDeltaMovement(Vec3) line: 5079` |
| 57 | 0.36% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6426` |
| 58 | 0.36% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4012` |
| 59 | 0.35% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2786` |
| 60 | 0.35% | `org.spigotmc.WatchdogThread.run() line: 72` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 6.83% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 5.84% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 3 | 3.89% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 4 | 2.98% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 5 | 2.61% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9847` |
| 6 | 2.35% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 7 | 2.32% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 8 | 2.04% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4364` |
| 9 | 2.03% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 7003` |
| 10 | 1.69% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 11 | 1.60% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 12 | 1.51% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 13 | 1.48% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 14 | 1.43% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 15 | 1.23% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 16 | 1.18% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6988` |
| 17 | 1.11% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 18 | 1.10% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 19 | 0.99% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 20 | 0.91% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2168` |
| 21 | 0.89% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 22 | 0.89% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 23 | 0.86% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |
| 24 | 0.78% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 25 | 0.78% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |
| 26 | 0.77% | `net.minecraft.server.level.ServerLevel.lambda$tick$0(TickRateManager, ProfilerFiller, Entity) line: 898` |
| 27 | 0.76% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6919` |
| 28 | 0.73% | `net.minecraft.world.entity.Entity.isEyeInFluid(TagKey) line: 2311` |
| 29 | 0.72% | `dev.planeta9091.centities.CeHooks.itemDispatch(Entity) line: 4045` |
| 30 | 0.70% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |

GC pauses: 0, total 0 ms

