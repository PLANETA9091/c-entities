# JVM hotspots (JFR ExecutionSample)

Samples: 27262

## Top 60 by self-time (leaf frames)

| # | % | method |
|---:|---:|---|
| 1 | 5.18% | `sun.misc.Unsafe.getObject(Object, long) line: 222` |
| 2 | 3.26% | `net.minecraft.network.syncher.SynchedEntityData$DataItem.getValue() line: 216` |
| 3 | 3.22% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6222` |
| 4 | 2.74% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 5 | 2.41% | `dev.planeta9091.centities.CeHooks.setTicksFrozen$(Entity, int) line: 9838` |
| 6 | 1.77% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 7 | 1.67% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9363` |
| 8 | 1.62% | `it.unimi.dsi.fastutil.objects.ObjectLinkedOpenHashSet$SetIterator.next() line: 989` |
| 9 | 1.62% | `dev.planeta9091.centities.CeHooks.itemTickFlush(Level) line: 5453` |
| 10 | 1.51% | `net.minecraft.network.syncher.SynchedEntityData.getItem(EntityDataAccessor) line: 49` |
| 11 | 1.38% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getInstance(Holder) line: 46` |
| 12 | 1.32% | `net.minecraft.world.phys.AABB.intersects(double, double, double, double, double, double) line: 235` |
| 13 | 1.27% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 87` |
| 14 | 1.23% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 15 | 1.17% | `it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap.get(Object) line: 323` |
| 16 | 1.07% | `net.minecraft.world.level.Level.getChunk(int, int, ChunkStatus, boolean) line: 1040` |
| 17 | 0.97% | `net.minecraft.world.level.NaturalSpawner.createState(int, ServerLevel, NaturalSpawner$ChunkGetter, LocalMobCapCalculator, boolean) line: 88` |
| 18 | 0.91% | `org.spigotmc.WatchdogThread.run() line: 72` |
| 19 | 0.89% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getModifier(Identifier) line: 66` |
| 20 | 0.89% | `net.minecraft.world.entity.ai.goal.GoalSelector.tickRunningGoals(boolean) line: 124` |
| 21 | 0.87% | `it.unimi.dsi.fastutil.objects.Reference2ObjectOpenHashMap.get(Object) line: 323` |
| 22 | 0.86% | `net.minecraft.world.level.chunk.LevelChunkSection.hasFluid() line: 177` |
| 23 | 0.86% | `net.minecraft.world.entity.Entity.getId() line: 651` |
| 24 | 0.83% | `java.lang.invoke.VarHandleReferences$FieldInstanceReadOnly.getVolatile(VarHandle, Object) line: 99` |
| 25 | 0.76% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap$ValuesCollection.forEach(Consumer) line: 698` |
| 26 | 0.73% | `net.minecraft.world.entity.LivingEntity.refreshDirtyAttributes() line: 1332` |
| 27 | 0.72% | `net.minecraft.world.level.chunk.ChunkAccess.getSectionIndexFromSectionY(int) line: 535` |
| 28 | 0.68% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 718` |
| 29 | 0.66% | `net.minecraft.core.component.DataComponentPatch.getFromPatchAndPrototype(Reference2ObjectMap, DataComponentGetter, DataComponentType) line: 167` |
| 30 | 0.66% | `java.util.EnumMap.get(Object) line: 248` |
| 31 | 0.63% | `net.minecraft.world.entity.Entity.setOldPos(Vec3) line: 2409` |
| 32 | 0.62% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks() line: 1420` |
| 33 | 0.62% | `net.minecraft.world.level.chunk.PalettedContainer.get(int) line: 236` |
| 34 | 0.61% | `net.minecraft.world.entity.ai.attributes.AttributeInstance.getValue() line: 140` |
| 35 | 0.57% | `net.minecraft.world.entity.EntityFluidInteraction.isEyeInFluid(TagKey) line: 190` |
| 36 | 0.55% | `net.minecraft.world.entity.ai.goal.WrappedGoal.getFlags() line: 73` |
| 37 | 0.55% | `dev.planeta9091.centities.CeHooks.tickEffectsNoOp(LivingEntity) line: 9783` |
| 38 | 0.52% | `java.util.ImmutableCollections$ListItr.next() line: 397` |
| 39 | 0.51% | `dev.planeta9091.centities.CeHooks.mobBlockEffectsNoOp(Entity) line: 4379` |
| 40 | 0.47% | `it.unimi.dsi.fastutil.objects.Reference2ObjectArrayMap.get(Object) line: 407` |
| 41 | 0.47% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 42 | 0.47% | `net.minecraft.world.phys.Vec3.add(double, double, double) line: 123` |
| 43 | 0.46% | `java.lang.invoke.VarHandleReferences$Array.getAcquire(VarHandle, Object, int) line: 637` |
| 44 | 0.45% | `net.minecraft.world.entity.Entity.getX() line: 5100` |
| 45 | 0.45% | `dev.planeta9091.centities.CeHooks.appendDispatch(ItemEntity, boolean) line: 4033` |
| 46 | 0.44% | `java.lang.ref.Reference.get() line: 361` |
| 47 | 0.42% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9354` |
| 48 | 0.42% | `net.minecraft.server.level.ServerChunkCache.runDistanceManagerUpdates() line: 453` |
| 49 | 0.41% | `net.minecraft.world.entity.LivingEntity.getHealth() line: 1395` |
| 50 | 0.40% | `dev.planeta9091.centities.CeHooks.sensingTick$(Sensing) line: 7265` |
| 51 | 0.39% | `java.util.AbstractCollection.containsAll(Collection) line: 310` |
| 52 | 0.39% | `java.util.Collections.unmodifiableSet(Set) line: 1239` |
| 53 | 0.39% | `net.minecraft.world.item.enchantment.EnchantmentHelper.runIterationOnItem(ItemStack, EquipmentSlot, LivingEntity, EnchantmentHelper$EnchantmentInSlotVisitor) line: 144` |
| 54 | 0.38% | `net.minecraft.world.entity.Entity.level() line: 5421` |
| 55 | 0.37% | `dev.planeta9091.centities.CeHooks.tickListForEach(EntityTickList, Consumer) line: 6235` |
| 56 | 0.37% | `net.minecraft.world.entity.Entity.computeSpeed() line: 965` |
| 57 | 0.36% | `dev.planeta9091.centities.CeHooks.rowQuery(ServerLevel, int, Entity, AABB, Predicate, boolean) line: 2750` |
| 58 | 0.36% | `net.minecraft.world.item.enchantment.Enchantment.entityContext(ServerLevel, int, Entity, Vec3) line: 509` |
| 59 | 0.36% | `net.minecraft.world.entity.Mob.checkDespawn() line: 786` |
| 60 | 0.35% | `net.minecraft.world.entity.Entity.getBoundingBox() line: 4712` |

## Top 30 callers of hot leaves

| # | % | method |
|---:|---:|---|
| 1 | 4.77% | `net.minecraft.network.syncher.SynchedEntityData.get(EntityDataAccessor) line: 53` |
| 2 | 3.87% | `net.minecraft.world.level.entity.EntityTickList.forEach(Consumer)` |
| 3 | 2.75% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.EntityLookup.getEntities(Class, Entity, AABB, List, Predicate) line: 722` |
| 4 | 2.74% | `dev.planeta9091.centities.CeHooks.ttlCachedPath(Entity, Object, int) line: 9353` |
| 5 | 2.61% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9456` |
| 6 | 2.42% | `net.minecraft.world.entity.Entity.setTicksFrozen(int)` |
| 7 | 2.26% | `net.minecraft.server.level.ServerChunkCache.tickChunks(ProfilerFiller) line: 567` |
| 8 | 2.07% | `net.minecraft.world.level.Level.tickBlockEntities()` |
| 9 | 1.36% | `net.minecraft.world.entity.LivingEntity.getAttribute(Holder) line: 2806` |
| 10 | 1.32% | `net.minecraft.world.phys.AABB.intersects(AABB) line: 231` |
| 11 | 1.25% | `net.minecraft.world.entity.LivingEntity.getAttributeValue(Holder) line: 2810` |
| 12 | 1.21% | `net.minecraft.world.entity.ai.goal.GoalSelector.tick() line: 88` |
| 13 | 1.21% | `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.ChunkEntitySlices.getEntities(Class, Entity, AABB, List, Predicate) line: 381` |
| 14 | 1.11% | `net.minecraft.world.entity.EntityFluidInteraction.update(Entity, boolean) line: 39` |
| 15 | 1.11% | `net.minecraft.world.entity.Mob.serverAiStep() line: 846` |
| 16 | 1.08% | `dev.planeta9091.centities.CeHooks.fastQuery(Level, Entity, AABB, Predicate, boolean) line: 2094` |
| 17 | 1.06% | `net.minecraft.world.entity.Entity.applyEffectsFromBlocks()` |
| 18 | 0.94% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 125` |
| 19 | 0.91% | `net.minecraft.world.level.Level.getEntitiesOfClass(Class, AABB, Predicate) line: 224` |
| 20 | 0.89% | `net.minecraft.world.entity.LivingEntity.removeFrost() line: 558` |
| 21 | 0.86% | `dev.planeta9091.centities.CeHooks.newTrackerTick$(Object) line: 9446` |
| 22 | 0.86% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 135` |
| 23 | 0.83% | `ca.spottedleaf.concurrentutil.map.concurrent.longs.ConcurrentChainedLong2ReferenceHashTable$TableEntry.getValueVolatile() line: 1812` |
| 24 | 0.79% | `net.minecraft.world.entity.ai.goal.GoalSelector.goalContainsAnyFlags(WrappedGoal, OptimizedSmallEnumSet) line: 66` |
| 25 | 0.77% | `net.minecraft.world.entity.LivingEntity.aiStep() line: 3781` |
| 26 | 0.73% | `net.minecraft.world.entity.LivingEntity.tick() line: 3480` |
| 27 | 0.72% | `net.minecraft.world.entity.EntityFluidInteraction.hasFluidAndLoaded(Level, int, int, int, int, int, int) line: 133` |
| 28 | 0.70% | `net.minecraft.world.entity.ai.attributes.AttributeMap.getValue(Holder) line: 59` |
| 29 | 0.69% | `net.minecraft.world.entity.LivingEntity.tickEffects()` |
| 30 | 0.69% | `net.minecraft.world.entity.LivingEntity.travel(Vec3)` |

GC pauses: 0, total 0 ms

