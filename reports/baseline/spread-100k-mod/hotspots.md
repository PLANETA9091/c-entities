# JVM hotspots (JFR ExecutionSample)

Samples: 28065

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 3.68% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 2 | 3.43% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.10% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6305` |
| 4 | 2.52% | `sun.misc.Unsafe.getInt(Object, long) line: 180` |
| 5 | 1.92% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, int) line: 9580` |
| 6 | 1.89% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 7 | 1.71% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 8 | 1.70% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 9 | 1.66% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 10 | 1.61% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 10182` |
| 11 | 1.41% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 12 | 1.39% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 13 | 1.36% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 14 | 1.26% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 15 | 1.18% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5537` |
| 16 | 1.14% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 17 | 1.06% | `net.minecraft.server.level.ServerChunkCache.syncLoad(int, int, ChunkStatus) line: 123` |
| 18 | 1.01% | `java.util.EnumMap.get(Object) line: 248` |
| 19 | 0.97% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 20 | 0.95% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 21 | 0.93% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 22 | 0.88% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 23 | 0.84% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 24 | 0.83% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 25 | 0.79% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 26 | 0.76% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6812` |
| 27 | 0.73% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 28 | 0.73% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 29 | 0.72% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 30 | 0.71% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 31 | 0.66% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 32 | 0.66% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 33 | 0.65% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 34 | 0.63% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 35 | 0.61% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 36 | 0.56% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 37 | 0.55% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 38 | 0.54% | `ca.spottedleaf.moonrise.common.set.OptimizedSmallEnumSet.getBackingSet() line: 58` |
| 39 | 0.54% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 40 | 0.51% | `ca.spottedleaf.moonrise.patches.fast_palette.FastHashMapPalette.valueFor(int) line: 161` |
| 41 | 0.48% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7422` |
| 42 | 0.47% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6880` |
| 43 | 0.47% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4071` |
| 44 | 0.47% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 45 | 0.47% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 10094` |
| 46 | 0.46% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 47 | 0.46% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 48 | 0.42% | `dev.planeta9091.centities.CeHooks.shadowOf(Object) line: 6478` |
| 49 | 0.42% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 50 | 0.41% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4438` |
| 51 | 0.41% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6318` |
| 52 | 0.41% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 53 | 0.38% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |
| 54 | 0.38% | `net.minecraft.world.entity.ai.goal.MoveThroughVillageGoal.updateVisited() line: 148` |
| 55 | 0.38% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 56 | 0.38% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 57 | 0.37% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 324` |
| 58 | 0.37% | `dev.planeta9091.centities.CeHooks.tickRunningImpl(GoalSelector, boolean) line: 6656` |
| 59 | 0.36% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 60 | 0.35% | `dev.planeta9091.centities.CeHooks.equipUpdates$(LivingEntity) line: 9990` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 5.14% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.74% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 3.69% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 4 | 2.70% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9728` |
| 5 | 2.51% | `dev.planeta9091.centities.CeHooks.entId(Entity) line: 4292` |
| 6 | 2.37% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 7 | 1.89% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 8 | 1.80% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick()` |
| 9 | 1.77% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 10 | 1.62% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 11 | 1.59% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 12 | 1.35% | `java.util.Map.computeIfAbsent(Object, Function) line: 1065` |
| 13 | 1.32% | `net.minecraft.world.level.chunk.LevelChunk.getBlockStateFinal(int, int, int) line: 289` |
| 14 | 1.26% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 15 | 1.21% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6895` |
| 16 | 1.13% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |
| 17 | 1.12% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 18 | 1.12% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2121` |
| 19 | 1.06% | `net.minecraft.server.level.ServerChunkCache.getChunkFallback(int, int, ChunkStatus, boolean) line: 157` |
| 20 | 1.04% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 21 | 1.00% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 22 | 0.99% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 23 | 0.95% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 24 | 0.95% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 25 | 0.88% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |
| 26 | 0.86% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 27 | 0.86% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 28 | 0.79% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |
| 29 | 0.77% | `dev.planeta9091.centities.CeHooks.goalSelectorTick(GoalSelector) line: 6880` |
| 30 | 0.76% | `net.minecraft.world.entity.Entity.setOldPos() line: 2401` |

GC pauses: 0, total 0 ms

